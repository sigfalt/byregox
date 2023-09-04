use dyn_clone::DynClone;
use enum_dispatch::enum_dispatch;

use super::{enums::*, Simulation};

use crate::types::actions::*;

// #[enum_dispatch(CraftingActionEnum)]
pub trait CraftingAction: DynClone {
	fn can_be_moved(&self, current_index: u32) -> bool {
		true
	}

	fn get_wait_duration(&self) -> u32 {
		if self.get_type() == ActionType::Buff {
			2
		} else {
			3
		}
	}

	fn skip_on_fail(&self) -> bool {
		false
	}

	fn requires_good(&self) -> bool {
		false
	}

	fn has_combo(&self, simulation_state: &Simulation) -> bool {
		false
	}

	fn get_level_requirement(&self) -> (CraftingJob, u32);

	fn get_type(&self) -> ActionType;

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32;

	fn get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		let base_rate = self._get_success_rate(simulation_state);
		if simulation_state.state() == StepState::Centered {
			base_rate + 25
		} else {
			base_rate
		}
	}

	fn can_be_used(&self, simulation_state: &Simulation) -> bool {
		// TODO: add linear, safeMode args
		let level_requirement = self.get_level_requirement();
		let craftsmanship_requirement = simulation_state.recipe.craftsmanship_req;
		let control_requirement = simulation_state.recipe.control_req;

		(if level_requirement.0 != CraftingJob::Any {
			simulation_state.crafter_stats.levels[level_requirement.0] >= level_requirement.1
		} else {
			simulation_state.crafter_stats.craftsmanship
				>= craftsmanship_requirement.unwrap_or_default()
				&& simulation_state.crafter_stats.control >= control_requirement.unwrap_or_default()
				&& simulation_state.crafter_stats.level >= level_requirement.1
		}) && self._can_be_used(simulation_state)
	}

	fn get_fail_cause(&self, simulation_state: &Simulation) -> Option<&str> {
		let level_requirement = self.get_level_requirement();
		let craftsmanship_requirement = simulation_state.recipe.craftsmanship_req;
		let control_requirement = simulation_state.recipe.control_req;

		if level_requirement.0 != CraftingJob::Any
			&& simulation_state.crafter_stats.levels[level_requirement.0] < level_requirement.1
		{
			Some("Missing level requirement")
		} else if simulation_state.crafter_stats.level < level_requirement.1 {
			Some("Missing level requirement")
		} else if craftsmanship_requirement
			.is_some_and(|x| x > simulation_state.crafter_stats.craftsmanship)
			|| control_requirement.is_some_and(|x| x > simulation_state.crafter_stats.control)
		{
			Some("Missing stats requirement")
		} else {
			None
		}
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool;

	fn get_cp_cost(&self, simulation_state: &Simulation) -> u32 {
		let base_cost = self.get_base_cp_cost(simulation_state);
		if simulation_state.state() == StepState::Pliant {
			(base_cost as f64 / 2.0).ceil() as u32
		} else {
			base_cost
		}
	}

	fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32;

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32;

	fn execute(&self, simulation_state: &mut Simulation);

	fn on_fail(&self, simulation_state: &Simulation) {}

	fn skips_buff_ticks(&self) -> bool {
		false
	}

	fn get_base_progression(&self, simulation_state: &Simulation) -> u32 {
		let stats = &simulation_state.crafter_stats;
		let base_value = ((stats.craftsmanship * 10) as f64
			/ simulation_state.recipe.progress_divider as f64)
			+ 2.0;
		if super::tables::level_to_ilevel(stats.level) <= simulation_state.recipe.rlvl {
			(base_value
				* simulation_state.recipe.progress_modifier.unwrap_or(100.0)
				* (0.01f32 as f64))
				.floor() as u32
		} else {
			base_value.floor() as u32
		}
	}

	fn get_base_quality(&self, simulation_state: &Simulation) -> u32 {
		let stats = &simulation_state.crafter_stats;
		let base_value =
			((stats.control * 10) as f64 / simulation_state.recipe.quality_divider as f64) + 35.0;
		if super::tables::level_to_ilevel(stats.level) <= simulation_state.recipe.rlvl {
			(base_value
				* simulation_state.recipe.quality_modifier.unwrap_or(100.0)
				* (0.01f32 as f64))
				.floor() as u32
		} else {
			base_value.floor() as u32
		}
	}
}
dyn_clone::clone_trait_object!(CraftingAction);

pub trait GeneralAction: CraftingAction {
	fn get_base_bonus(&self, simulation_state: &Simulation) -> f64 {
		1.0
	}

	fn get_base_condition(&self, simulation_state: &Simulation) -> f64 {
		1.0
	}

	fn get_potency(&self, simulation_state: &Simulation) -> u32;

	fn get_base_durability_cost(&self, simulation_state: &Simulation) -> u32;

	fn get_base_success_rate(&self, simulation_state: &Simulation) -> u32;
}
// any class that implements GeneralAction should inherit the following defaults
/*
impl CraftingAction for Class {
	fn get_level_requirement(&self) -> (CraftingJob, u32) {
		todo!()
	}

	fn get_type(&self) -> ActionType {
		todo!()
	}

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		todo!()
	}

	fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII) { divider *= 2.0 }
		if simulation_state.state() == StepState::Sturdy { divider *= 2.0 }
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		todo!()
	}
}
*/

pub trait ProgressAction: GeneralAction {}
// any class that implements ProgressAction should inherit the following defaults
/*
impl CraftingAction for Class {
	fn get_level_requirement(&self) -> (CraftingJob, u32) {
		todo!()
	}

	fn get_type(&self) -> ActionType { ActionType::Progression }

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		todo!()
	}

	fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII) { divider *= 2.0 }
		if simulation_state.state() == StepState::Sturdy { divider *= 2.0 }
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let progression_increase = self.get_base_progression(simulation_state);

		if simulation_state.state() == StepState::Malleable { condition_mod *= 1.5; }
		if simulation_state.has_buff(Buff::MuscleMemory) {
			buff_mod += 1.0;
			simulation_state.remove_buff(Buff::MuscleMemory);
		}
		if simulation_state.has_buff(Buff::Veneration) {
			buff_mod += 0.5;
		}

		let efficiency = potency as f64 * buff_mod;
		simulation_state.progression += (progression_increase as f64 * condition_mod * efficiency / 100.0).floor() as u32;

		if simulation_state.has_buff(Buff::FinalAppraisal) && simulation_state.progression >= simulation_state.recipe.progress {
			simulation_state.progression = simulation_state.progression.min(simulation_state.recipe.progress - 1);
			simulation_state.remove_buff(Buff::FinalAppraisal);
		}
	}
}

impl GeneralAction for Class {
	fn get_potency(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_base_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_base_success_rate(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}
}
*/

pub trait QualityAction: GeneralAction {}
// any class that implements QualityAction should inherit the following defaults
/*
impl CraftingAction for Class {
	fn get_level_requirement(&self) -> (CraftingJob, u32) {
		todo!()
	}

	fn get_type(&self) -> ActionType { ActionType::Quality }

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		todo!()
	}

	fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII) { divider *= 2.0 }
		if simulation_state.state() == StepState::Sturdy { divider *= 2.0 }
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let quality_increase = self.get_base_quality(simulation_state).floor(); // TODO: convert get_base_quality return type for f64?

		match simulation_state.state() {
			StepState::Excellent => condition_mod *= 4,
			StepState::Poor => condition_mod *= 0.5,
			StepState::Good => condition_mod *= if simulation_state.crafter_stats.splendorous { 1.75 } else { 1.5 },
			_ => ()
		};

		buff_mod += simulation_state.get_buff(Buff::InnerQuiet).map(|b| b.stacks).unwrap_or_default();

		let mut buff_mult = 1.0;
		if simulation_state.has_buff(Buff::GreatStrides) {
			buff_mult += 1.0;
			simulation_state.remove_buff(Buff::GreatStrides);
		}
		if simulation_state.has_buff(Buff::Innovation) {
			buff_mult += 0.5;
		}

		let buff_mod: f64 = (buff_mod as f32) * (buff_mult as f32);
		let efficiency = (potency * buff_mod) as f32;
		simulation_state.quality += (quality_increase * condition_mod * efficiency / 100.0).floor() as u32;

		// if !skipStackAddition { // argument to function, defaults to false
		if true {
			simulation_state.add_inner_quiet_stacks(1);
		}
	}
}

impl GeneralAction for Class {
	fn get_potency(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_base_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}

	fn get_base_success_rate(&self, simulation_state: &Simulation) -> u32 {
		todo!()
	}
}
*/