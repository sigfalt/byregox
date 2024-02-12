use derive_builder::{Builder, UninitializedFieldError};
use rand::Rng;

use crate::types::tables;

use super::{
	enums::{Buff, CraftingActionEnum, StepState},
	structs::*,
	traits::CraftingAction,
};

#[derive(Builder)]
pub struct Simulation {
	pub recipe: Craft,
	pub actions: Vec<Box<dyn CraftingAction>>,
	pub crafter_stats: CrafterStats,
	// private hqIngredients: {id: number; amount: number}[] = []
	#[builder(default = "vec![]")]
	step_states: Vec<StepState>,
	#[builder(default = "vec![]")]
	fails: Vec<usize>,

	// Auto-initialized fields
	#[builder(setter(skip), default = "0")]
	pub progression: u32,
	#[builder(setter(skip), default = "0")]
	pub quality: u32,
	#[builder(setter(skip), default = "self.build_starting_quality()?")]
	starting_quality: u32,
	#[builder(setter(skip), default = "self.build_durability()?")]
	pub durability: i32,

	#[builder(setter(skip), default = "StepState::Normal")]
	state: StepState,

	#[builder(setter(skip), default = "self.build_available_cp()?")]
	pub available_cp: u32,
	#[builder(setter(skip), default = "self.build_max_cp()?")]
	pub max_cp: u32,

	#[builder(setter(skip), default = "vec![]")]
	buffs: Vec<EffectiveBuff>,
	#[builder(setter(skip), default = "None")]
	pub success: Option<bool>,
	#[builder(setter(skip), default = "vec![]")]
	pub steps: Vec<ActionResult>,

	// the index of the last step where you have CP/durability for Reclaim,
	// or None if Reclaim is uncastable (i.e. not enough CP)
	#[builder(setter(skip), default = "None")]
	last_possible_reclaim_step: Option<u32>,

	#[builder(setter(skip), default = "false")]
	pub safe: bool,
}

impl Simulation {
	pub fn state(&self) -> StepState {
		self.state
	}

	pub fn has_combo_available(&self, action: &dyn CraftingAction) -> bool {
		// starting from the most recent action
		for step in self.steps.iter().rev() {
			// if we find the action that we're looking for and it was successful, we can combo
			if step.action.get_enum() == action.get_enum() && step.success.is_some_and(|x| x) {
				return true;
			}
			// if any previous action that isn't what we're looking for wasn't skipped, combo is broken
			if !step.skipped {
				return false;
			}
		}
		false
	}

	pub fn add_inner_quiet_stacks(&mut self, stacks: u32) {
		if let Some(buff) = self.get_mut_buff(Buff::InnerQuiet) {
			buff.stacks = (buff.stacks + stacks).min(10);
		} else {
			self.buffs.push(EffectiveBuff {
				duration: i32::MAX,
				stacks: stacks.min(10),
				buff: Buff::InnerQuiet,
				applied_step: self.step_states.len() as u32,
				tick: None,
				on_expire: None,
			});
		}
	}

	pub fn run(self) -> SimulationResult {
		self.run_linear(false)
	}

	pub fn run_linear(self, linear: bool) -> SimulationResult {
		self.run_max_steps(linear, usize::MAX)
	}

	pub fn run_max_steps(self, linear: bool, max_steps: usize) -> SimulationResult {
		self.run_with_flags(linear, max_steps, false)
	}

	pub fn run_with_flags(
		mut self,
		linear: bool,
		max_steps: usize,
		safe: bool,
	) -> SimulationResult {
		self.last_possible_reclaim_step = None;
		self.actions
			.clone()
			.iter()
			.enumerate()
			.for_each(|(i, action)| {
				self.state = self
					.step_states
					.get(i)
					.map_or_else(|| StepState::Normal, |s| *s);
				let mut fail_cause: Option<&str> = None;

				let can_use_action = action.can_be_used(&self);
				if !can_use_action {
					fail_cause = action.get_fail_cause(&self);
				}
				let has_enough_cp = action.get_base_cp_cost(&self) <= self.available_cp;
				if !has_enough_cp {
					fail_cause = Some("Not enough CP");
				}
				// we can use the action
				let mut result = if self.success.is_none()
					&& has_enough_cp && self.steps.len() < max_steps
					&& can_use_action
				{
					self.run_action_with_flags(action, linear, safe, i)
				} else {
					ActionResult {
						action: action.clone(),
						success: None,
						fail_cause: fail_cause.map(|x| x.to_string()),
						added_progression: 0,
						added_quality: 0,
						cp_difference: 0,
						solidity_difference: 0,
						skipped: true,
						combo: None,
						state: self.state,
						after_buff_tick: None,
					}
				};

				if self.steps.len() < max_steps {
					let quality_before = self.quality;
					let progression_before = self.progression;
					let durability_before = self.durability;
					let cp_before = self.available_cp as i32;
					let skip_ticks_on_fail =
						!result.success.unwrap_or(false) && action.skip_on_fail();
					if self.success.is_none() && !action.skips_buff_ticks() && !skip_ticks_on_fail {
						self.tick_buffs(action.as_ref());
					}
					result.after_buff_tick = Some(BuffTickResult {
						added_progression: self.progression - progression_before,
						added_quality: self.quality - quality_before,
						cp_difference: self.available_cp as i32 - cp_before,
						solidity_difference: self.durability - durability_before,
					});
				}

				if !linear
					&& action.get_enum() != CraftingActionEnum::FinalAppraisal
					&& action.get_enum() != CraftingActionEnum::RemoveFinalAppraisal
				{
					self.tick_state();
				}
				self.steps.push(result);
			});

		let failed_action = self
			.steps
			.iter()
			.find(|step| step.fail_cause.is_some())
			.cloned();
		let has_required_quality = self.recipe.required_quality.is_some();
		let success = self.progression >= self.recipe.progress
			&& if let Some(required_quality) = self.recipe.required_quality {
				self.quality > required_quality
			} else {
				true
			};
		let mut res = SimulationResult {
			steps: self.steps.clone(),
			hq_percent: self.get_hq_percent(),
			success,
			simulation: self,
			fail_cause: if has_required_quality && !success {
				Some("Quality too low".to_string())
			} else {
				None
			},
		};
		if let Some(failed_action) = failed_action {
			if failed_action.fail_cause.is_some() {
				res.fail_cause = failed_action.fail_cause.clone();
			}
		}
		res
	}

	pub fn run_action(&mut self, action: &Box<dyn CraftingAction>, index: usize) -> ActionResult {
		self.run_action_linear(action, false, index)
	}

	pub fn run_action_linear(
		&mut self,
		action: &Box<dyn CraftingAction>,
		linear: bool,
		index: usize,
	) -> ActionResult {
		self.run_action_with_flags(action, linear, false, index)
	}

	pub fn run_action_with_flags(
		&mut self,
		action: &Box<dyn CraftingAction>,
		linear: bool,
		safe: bool,
		index: usize,
	) -> ActionResult {
		let probability_roll: u32 = if self.fails.contains(&index) {
			999
		} else if linear {
			0
		} else {
			rand::thread_rng().gen_range(0..100)
		};
		let quality_before = self.quality;
		let progression_before = self.progression;
		let durability_before = self.durability;
		let cp_before = self.available_cp;
		let combo = action.has_combo(self);

		let mut fail_cause: Option<&str> = None;
		let mut success = false;

		// TODO: if safe_mode &&
		if action.get_success_rate(self) < 100
			|| (action.requires_good() && !self.has_buff(Buff::HeartAndSoul))
		{
			fail_cause = Some("Unsafe action");
			action.on_fail(self);
			self.safe = false;
		} else if action.get_success_rate(self) >= probability_roll {
			action.execute(self);
			success = true;
		} else {
			action.on_fail(self);
		}

		// even if failed, remove durability cost and CP
		self.durability -= action.get_durability_cost(self) as i32;
		self.available_cp -= action.get_cp_cost_linear(self, linear);
		if self.progression >= self.recipe.progress {
			self.success = Some(true);
		} else if self.durability <= 0 {
			fail_cause = Some("Durability reached zero");
			self.success = Some(false);
		}

		ActionResult {
			action: action.clone(),
			success: Some(success),
			fail_cause: fail_cause.map(|x| x.to_string()),
			added_progression: self.progression - progression_before,
			added_quality: self.quality - quality_before,
			cp_difference: 0i32
				.saturating_add_unsigned(self.available_cp)
				.saturating_sub_unsigned(cp_before),
			solidity_difference: self.durability - durability_before,
			skipped: false,
			combo: Some(combo),
			state: self.state,
			after_buff_tick: None,
		}
	}

	pub fn has_buff(&self, buff: Buff) -> bool {
		self.buffs.iter().any(|x| x.buff == buff)
	}

	pub fn get_buff(&self, buff: Buff) -> Option<&EffectiveBuff> {
		self.buffs.iter().find(|x| x.buff == buff)
	}

	pub fn get_mut_buff(&mut self, buff: Buff) -> Option<&mut EffectiveBuff> {
		self.buffs.iter_mut().find(|x| x.buff == buff)
	}

	pub fn add_buff(&mut self, buff: EffectiveBuff) {
		self.buffs.push(buff);
	}

	pub fn remove_buff(&mut self, buff: Buff) {
		let ix = self.buffs.iter().position(|b| b.buff == buff);
		if let Some(ix) = ix {
			self.buffs.swap_remove(ix);
		}
	}

	pub fn repair(&mut self, amt: u32) {
		self.durability = (self.recipe.durability as i32).min(self.durability + (amt as i32));
	}

	pub fn get_hq_percent(&self) -> u32 {
		let quality_percent =
			(((self.quality as f64 / self.recipe.quality as f64) * 100.0).floor() as u32).min(100);
		if quality_percent == 0 {
			1
		} else if quality_percent >= 100 {
			100
		} else {
			*tables::HQ_TABLE.get(quality_percent as usize).unwrap()
		}
	}

	fn tick_buffs(&mut self, action: &dyn CraftingAction) {
		let mut curr_buffs = self.buffs.clone();
		curr_buffs.iter_mut().for_each(|b| {
			if b.applied_step < self.steps.len() as u32 {
				b.tick(self, action);
				b.duration -= 1;
			}
		});
		curr_buffs
			.iter()
			.filter(|b| b.duration <= 0 && b.on_expire.is_some())
			.for_each(|b| b.on_expire(self, action));
		self.buffs = curr_buffs.into_iter().filter(|b| b.duration > 0).collect();
	}

	fn tick_state(&mut self) {
		// if current state is EXCELLENT, next is always POOR
		if self.state == StepState::Excellent {
			self.state = StepState::Poor;
			return;
		} else
		// if current state is GOOD OMEN, next is always GOOD
		if self.state == StepState::GoodOmen {
			self.state = StepState::Good;
			return;
		}

		// Quality Assurance trait, level 63
		let good_chance = if self.crafter_stats.level >= 63 {
			0.25
		} else {
			0.2
		};

		// TODO: roll for next state
		self.state = StepState::Normal;
	}
}

impl SimulationBuilder {
	fn build_starting_quality(&self) -> Result<u32, SimulationBuilderError> {
		// TODO: Incorproate HQ ingredients calculation
		Ok(0)
	}

	fn build_durability(&self) -> Result<i32, SimulationBuilderError> {
		match &self.recipe {
			Some(craft) => Ok(craft.durability as i32),
			_ => Err(SimulationBuilderError::from(UninitializedFieldError::new(
				"durability",
			))),
		}
	}

	fn build_available_cp(&self) -> Result<u32, SimulationBuilderError> {
		match &self.crafter_stats {
			Some(stats) => Ok(stats.cp),
			_ => Err(SimulationBuilderError::from(UninitializedFieldError::new(
				"available_cp",
			))),
		}
	}

	fn build_max_cp(&self) -> Result<u32, SimulationBuilderError> {
		match &self.crafter_stats {
			Some(s) => Ok(s.cp),
			_ => Err(SimulationBuilderError::from(UninitializedFieldError::new(
				"max_cp",
			))),
		}
	}
}
