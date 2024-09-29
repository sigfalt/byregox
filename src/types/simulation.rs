use bon::bon;
use num_traits::FromPrimitive;
use rand::{random, Rng};
use std::collections::{HashMap, HashSet};

use crate::types::{
	actions,
	enums::{Buff, CraftingActionEnum, FailCause, StepState},
	structs::*,
	tables,
	traits::CraftingAction,
};

pub struct Simulation {
	pub recipe: Craft,
	pub crafter_stats: CrafterStats,

	// optional fields
	pub actions: Vec<CraftingActionEnum>,
	step_states: Vec<StepState>,
	fails: Vec<usize>,

	// auto-initialized fields
	pub progression: u32,

	starting_quality: u32,
	pub quality: u32,

	pub durability: i32,

	state: StepState,

	pub max_cp: u32,
	pub available_cp: u32,

	buffs: Vec<EffectiveBuff>,
	pub success: Option<bool>,
	pub steps: Vec<ActionResult>,

	// the index of the last step where you have CP/durability for Reclaim,
	// or None if Reclaim is uncastable (i.e. not enough CP)
	last_possible_reclaim_step: Option<u32>,

	pub safe: bool,

	possible_conditions: HashSet<StepState>,
}

#[bon]
impl Simulation {

	#[builder]
	pub fn new(
		recipe: Craft,
		crafter_stats: CrafterStats,
		actions: Option<Vec<CraftingActionEnum>>,
		hq_ingredients: Option<Vec<Ingredient>>,
		step_states: Option<Vec<StepState>>,
		fails: Option<Vec<usize>>,
	) -> Self {
		let mut starting_quality = 0;
		if let Some(hq_ingredients) = &hq_ingredients {
			for ingredient in hq_ingredients {
				if let Some(ingredient_details) = &recipe
					.ingredients
					.iter()
					.find(|recipe_ingredient| recipe_ingredient.id == ingredient.id)
				{
					starting_quality += ingredient_details.quality.unwrap_or(0) * ingredient.amount;
				}
			}
		}

		let conditions_flag = recipe.conditions_flag;
		let binary_string = format!("{:b}", conditions_flag);
		let possible_conditions = binary_string
			.chars()
			.rev()
			.enumerate()
			.filter_map(|(ix, chr)| {
				if chr == '1' {
					StepState::from_usize(ix + 1)
				} else {
					None
				}
			})
			.collect();

		let durability = recipe.durability as i32;
		let max_cp = crafter_stats.cp;

		Self {
			recipe,
			crafter_stats,
			actions: actions.unwrap_or(vec![]),
			step_states: step_states.unwrap_or(vec![]),
			fails: fails.unwrap_or(vec![]),
			progression: 0,
			starting_quality,
			quality: starting_quality,
			durability,
			state: StepState::Normal,
			max_cp,
			available_cp: max_cp,
			buffs: vec![],
			success: None,
			steps: vec![],
			last_possible_reclaim_step: None,
			safe: false,
			possible_conditions,
		}
	}

	pub fn state(&self) -> StepState {
		self.state
	}

	pub fn override_state(&mut self, new_state: StepState) {
		self.state = new_state;
	}

	pub fn has_combo_available(&self, action: &CraftingActionEnum) -> bool {
		// starting from the most recent action
		for step in self.steps.iter().rev() {
			// if we find the action that we're looking for and it was successful, we can combo
			if step.action == *action && step.success.is_some_and(|x| x) {
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

	pub fn reset(&mut self) {
		self.success = None;
		self.progression = 0;
		self.durability = self.recipe.durability as i32;
		self.quality = self.starting_quality;
		self.buffs = vec![];
		self.steps = vec![];
		self.max_cp = self.crafter_stats.cp;
		self.available_cp = self.max_cp;
		self.state = StepState::Normal;
		self.safe = false;
	}

	#[builder(finish_fn = run)]
	pub fn start(
		mut self,
		linear: Option<bool>,
		max_steps: Option<usize>,
		safe: Option<bool>
	) -> SimulationResult {
		let linear = linear.unwrap_or(false);
		let max_steps = max_steps.unwrap_or(usize::MAX);
		let safe = safe.unwrap_or(false);

		self.last_possible_reclaim_step = None;
		self.actions
			.clone()
			.iter()
			.enumerate()
			.for_each(|(i, action)| {
				self.state = self.step_states.get(i).map_or_else(
					|| StepState::Normal,
					|&s| {
						if s == StepState::None {
							StepState::Normal
						} else {
							s
						}
					},
				);
				let mut fail_cause: Option<FailCause> = None;

				let can_use_action = action.can_be_used_with_flags(&self, Some(linear), Some(safe));
				if !can_use_action {
					fail_cause = action.get_fail_cause_with_flags(&self, Some(linear), Some(safe));
				}
				let has_enough_cp = action.get_base_cp_cost(&self) <= self.available_cp;
				if !has_enough_cp {
					fail_cause = Some(FailCause::NotEnoughCP);
				}
				// we can use the action
				let mut result = if self.success.is_none()
					&& has_enough_cp
					&& self.steps.len() < max_steps
					&& can_use_action
				{
					self.run_action_with_flags(action, linear, safe, i)
				} else {
					ActionResult {
						action: *action,
						success: None,
						fail_cause,
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
						self.tick_buffs(action);
					}
					result.after_buff_tick = Some(BuffTickResult {
						added_progression: self.progression - progression_before,
						added_quality: self.quality - quality_before,
						cp_difference: self.available_cp as i32 - cp_before,
						solidity_difference: self.durability - durability_before,
					});
				}

				if !linear
					&& *action != actions::FinalAppraisal.into()
					&& *action != actions::RemoveFinalAppraisal.into()
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
				Some(FailCause::QualityTooLow)
			} else {
				None
			},
		};
		if let Some(failed_action) = failed_action {
			if failed_action.fail_cause.is_some() {
				res.fail_cause = failed_action.fail_cause;
			}
		}
		res
	}

	pub fn run_action(&mut self, action: &CraftingActionEnum, index: usize) -> ActionResult {
		self.run_action_linear(action, false, index)
	}

	pub fn run_action_linear(
		&mut self,
		action: &CraftingActionEnum,
		linear: bool,
		index: usize,
	) -> ActionResult {
		self.run_action_with_flags(action, linear, false, index)
	}

	pub fn run_action_with_flags(
		&mut self,
		action: &CraftingActionEnum,
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

		let mut fail_cause: Option<FailCause> = None;
		let mut success = false;

		if safe
			&& (action.get_success_rate(self) < 100
				|| (action.requires_good() && !self.has_buff(Buff::HeartAndSoul)))
		{
			fail_cause = Some(FailCause::UnsafeAction);
			action.on_fail(self);
			self.safe = false;
		} else if action.get_success_rate(self) >= probability_roll {
			action.execute(self);
			success = true;
		} else {
			action.on_fail(self);
		}

		// even if failed, remove durability cost and CP
		if self.has_buff(Buff::TrainedPerfection) && action.get_durability_cost(self) > 0 {
			self.remove_buff(Buff::TrainedPerfection);
		} else {
			self.durability -= action.get_durability_cost(self) as i32;
		}
		self.available_cp -= action.get_cp_cost_linear(self, linear);
		if self.progression >= self.recipe.progress {
			self.success = Some(true);
		} else if self.durability <= 0 {
			fail_cause = Some(FailCause::DurabilityReachedZero);
			self.success = Some(false);
		}

		ActionResult {
			action: *action,
			success: Some(success),
			fail_cause,
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

	fn tick_buffs(&mut self, action: &CraftingActionEnum) {
		let buff_vec = self.buffs.clone();
		buff_vec.iter().for_each(|b| {
			if b.applied_step < self.steps.len() as u32 {
				b.tick(self, action);
				if let Some(buff_ref) = self.get_mut_buff(b.buff) {
					buff_ref.duration -= 1;
				}
			};
		});
		buff_vec
			.iter()
			.filter(|b| b.duration <= 0 && b.on_expire.is_some())
			.for_each(|b| b.on_expire(self, action));
		self.buffs = self
			.buffs
			.clone()
			.into_iter()
			.filter(|b| b.duration > 0)
			.collect();
	}

	pub fn possible_conditions(&self) -> &HashSet<StepState> {
		&self.possible_conditions
	}

	pub fn tick_state(&mut self) {
		// if current state is EXCELLENT, next is always POOR
		if self.state == StepState::Excellent {
			self.state = StepState::Poor;
			return;
		}
		// if current state is GOOD OMEN, next is always GOOD
		else if self.state == StepState::GoodOmen {
			self.state = StepState::Good;
			return;
		}

		// Quality Assurance trait, level 63
		let good_chance = if self.crafter_stats.level >= 63 {
			0.25
		} else {
			0.2
		};

		let mut states_and_rates: HashMap<_, _> =
			HashMap::from_iter(self.possible_conditions.iter().filter_map(|&step_state| {
				match step_state {
					StepState::Good => Some(if self.recipe.expert.is_some_and(|b| b) {
						0.12
					} else {
						good_chance
					}),
					StepState::Excellent => Some(if self.recipe.expert.is_some_and(|b| b) {
						0.0
					} else {
						0.04
					}),
					StepState::Poor => Some(0.0),
					StepState::Centered => Some(0.15),
					StepState::Sturdy => Some(0.15),
					StepState::Pliant => Some(0.12),
					StepState::Malleable => Some(0.12),
					StepState::Primed => Some(0.12),
					StepState::GoodOmen => Some(0.1),
					_ => None,
				}
				.map(|rate| (step_state, rate))
			}));
		let non_normal_rate: f64 = states_and_rates.values().sum();
		states_and_rates.insert(StepState::Normal, 1.0 - non_normal_rate);
		self.state = Self::get_weighted_random(states_and_rates).unwrap_or(StepState::Normal);
	}

	fn get_weighted_random<T>(weighted_items: HashMap<T, f64>) -> Option<T> {
		let total_weight: f64 = weighted_items.values().sum();
		let threshold = random::<f64>() * total_weight;

		let mut sum = 0.0;
		for (item, weight) in weighted_items {
			sum += weight;
			if sum > threshold {
				return Some(item);
			}
		}
		None
	}
}
