use crate::types::{
	enums::{ActionType, Buff, CraftingActionEnum, CraftingJob, StepState},
	structs::CraftingLevel,
	traits::CraftingAction,
	Simulation,
};

#[derive(Clone)]
pub struct TricksOfTheTrade;

impl CraftingAction for TricksOfTheTrade {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(13).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::CPRecovery
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		// if linear (defaults to false)
		if false {
			true
		} else if simulation_state.safe {
			true
		} else {
			simulation_state.state() == StepState::Good
				|| simulation_state.state() == StepState::Excellent
		}
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn get_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		if simulation_state.has_buff(Buff::HeartAndSoul)
			|| simulation_state.state() == StepState::Good
			|| simulation_state.state() == StepState::Excellent
		{
			simulation_state.available_cp += 20;
			if simulation_state.available_cp > simulation_state.max_cp {
				simulation_state.available_cp = simulation_state.max_cp;
			}
		}
	}

	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::TricksOfTheTrade
	}
}
