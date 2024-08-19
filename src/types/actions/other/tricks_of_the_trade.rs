use crate::types::{
	enums::{ActionType, Buff, CraftingJob, StepState},
	structs::CraftingLevel,
	traits::CraftingAction,
	Simulation,
};

#[derive(Clone, Copy, PartialEq)]
pub struct TricksOfTheTrade;

impl CraftingAction for TricksOfTheTrade {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::unchecked_new(13))
	}

	fn get_type(&self) -> ActionType {
		ActionType::CPRecovery
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, linear: Option<bool>) -> bool {
		if linear.unwrap_or(false) {
			true
		} else if simulation_state.safe {
			false
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

	fn execute_with_flags(
		&self,
		simulation_state: &mut Simulation,
		_safe: bool,
		_skip_stack_addition: bool,
	) {
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
}
