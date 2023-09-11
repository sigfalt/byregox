use crate::types::{
	enums::{ActionType, Buff, CraftingActionEnum, CraftingJob, StepState},
	structs::CraftingLevel,
	traits::{CraftingAction, GeneralAction, ProgressAction},
	Simulation,
};

#[derive(Clone)]
pub struct Groundwork;

impl ProgressAction for Groundwork {}

impl CraftingAction for Groundwork {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(72).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::Progression
	}

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, _simulation_state: &Simulation) -> bool {
		true
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		18
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII)
		{
			divider *= 2.0
		}
		if simulation_state.state() == StepState::Sturdy {
			divider *= 2.0
		}
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let progression_increase = self.get_base_progression(simulation_state);

		if simulation_state.state() == StepState::Malleable {
			condition_mod *= 1.5;
		}
		if simulation_state.has_buff(Buff::MuscleMemory) {
			buff_mod += 1.0;
			simulation_state.remove_buff(Buff::MuscleMemory);
		}
		if simulation_state.has_buff(Buff::Veneration) {
			buff_mod += 0.5;
		}

		let efficiency = potency as f64 * buff_mod;
		simulation_state.progression +=
			(progression_increase as f64 * condition_mod * efficiency / 100.0).floor() as u32;

		if simulation_state.has_buff(Buff::FinalAppraisal)
			&& simulation_state.progression >= simulation_state.recipe.progress
		{
			simulation_state.progression = simulation_state
				.progression
				.min(simulation_state.recipe.progress - 1);
			simulation_state.remove_buff(Buff::FinalAppraisal);
		}
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::Groundwork
	}
}

impl GeneralAction for Groundwork {
	fn get_potency(&self, simulation_state: &Simulation) -> u32 {
		let base_potency = if simulation_state.crafter_stats.level >= 86 {
			360
		} else {
			300
		};
		if simulation_state.durability >= self.get_durability_cost(simulation_state) as i32 {
			base_potency
		} else {
			base_potency / 2
		}
	}

	fn get_base_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		20
	}

	fn get_base_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}
}
