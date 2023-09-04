use derive_builder::{Builder, UninitializedFieldError};
use rand::Rng;

use crate::types::tables;

use super::{
	enums::{Buff, StepState},
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
	// private fails: number[] = [],

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
	available_cp: u32,
	#[builder(setter(skip), default = "self.build_max_cp()?")]
	max_cp: u32,

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

	pub fn add_inner_quiet_stacks(&mut self, stacks: u32) {
		if !self.has_buff(Buff::InnerQuiet) {
			self.buffs.push(EffectiveBuff {
				duration: u32::MAX,
				stacks: stacks.min(10),
				buff: Buff::InnerQuiet,
				applied_step: self.step_states.len() as u32,
			});
		} else {
			/*
			let iq = self.get_buff(Buff::InnerQuiet);
			iq.stacks = (iq.stacks + stacks).min(10);
			*/
			let iq = self.get_buff(Buff::InnerQuiet).unwrap();
			iq.stacks = (iq.stacks + stacks).min(10);
		}
	}

	pub fn run(mut self, linear: bool) -> SimulationResult {
		self.last_possible_reclaim_step = None;
		self.actions.clone().iter().for_each(|action| {
			self.state = StepState::Normal; // TODO: this.step_states[index] || Normal;
			let result: ActionResult;
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
			if self.success == None
                    && has_enough_cp
                    // TODO: && self.steps.len() < max_turns
                    && can_use_action
			{
				result = self.run_action(action, linear);
			} else {
				result = ActionResult {
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
				}
			}

			// TODO: if self.steps.len() < max_turns
			if self.steps.len() < usize::MAX {
				let quality_before = self.quality;
				let progression_before = self.progression;
				let durability_before = self.durability;
				let cp_before = self.available_cp;
				let skip_ticks_on_fail = !result.success.unwrap_or(false) && action.skip_on_fail();
				if self.success.is_none() && !action.skips_buff_ticks() && !skip_ticks_on_fail {
					// self.tick_buffs(linear, action);
				}
				// result.after_buff_tick = {
				//     added_progression: self.progression - progression_before,
				// };
			}

			use crate::types::enums::CraftingActionEnum as Action;
			// if (!linear && action != Action::FinalAppraisal && action != Action::RemoveFinalAppraisal) {
			if !linear {
				// self.tick_state();
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
			success: success,
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

	pub fn run_action(&mut self, action: &Box<dyn CraftingAction>, linear: bool) -> ActionResult {
		let probability_roll: u32 = if linear {
			0
		} else {
			rand::thread_rng().gen_range(0..100)
		};
		// if self.fails.includes(index) {
		let probability_roll: u32 = if false {
			999
		} else {
			if linear {
				0
			} else {
				rand::thread_rng().gen_range(0..100)
			}
		};
		let quality_before = self.quality;
		let progression_before = self.progression;
		let durability_before = self.durability;
		let cp_before = self.available_cp;
		let combo = action.has_combo(self);

		let mut fail_cause: Option<&str> = None;
		let mut success = false;

		// if safe_mode &&
		if (action.get_success_rate(self) < 100
			|| (action.requires_good() && !self.has_buff(Buff::HeartAndSoul)))
		{
			fail_cause = Some("Unsafe action");
			action.on_fail(self);
			self.safe = false;
		} else {
			if action.get_success_rate(self) >= probability_roll {
				action.execute(self);
				success = true;
			} else {
				action.on_fail(self);
			}
		}

		// even if failed, remove durability cost and CP
		self.durability -= action.get_durability_cost(self) as i32;
		self.available_cp -= action.get_cp_cost(self);
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
		}
	}

	pub fn has_buff(&self, buff: Buff) -> bool {
		self.buffs.iter().any(|x| x.buff == buff)
	}

	pub fn get_buff(&mut self, buff: Buff) -> Option<&mut EffectiveBuff> {
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
