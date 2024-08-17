use crate::types::enums::{ActionType, Buff, CraftingActionEnum, CraftingJob};
use crate::types::Simulation;
use crate::types::structs::CraftingLevel;
use crate::types::traits::{BuffAction, CraftingAction};

#[derive(Clone)]
pub struct TrainedPerfection;

impl BuffAction for TrainedPerfection {
	fn get_duration(&self, _simulation_state: &Simulation) -> i32 {
		// basically infinity
		// improvement: fix for crafting rotations over 2,147,483,647 steps long
		i32::MAX
	}

	fn get_buff(&self) -> Buff {
		Buff::TrainedPerfection
	}

	fn get_initial_stacks(&self) -> u32 {
		0
	}
}

impl CraftingAction for TrainedPerfection {
	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::unchecked_new(100))
	}

	fn get_type(&self) -> ActionType {
		ActionType::Buff
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		!simulation_state.steps.iter().any(|step|
			step.action.get_enum() == CraftingActionEnum::TrainedPerfection)
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
		self.get_overrides().into_iter().for_each(|b| simulation_state.remove_buff(b));
		simulation_state.add_buff(self.get_applied_buff(simulation_state));
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::TrainedPerfection
	}
}