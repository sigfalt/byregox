use crate::types::{
	enums::{ActionType, CraftingActionEnum, CraftingJob},
	structs::CraftingLevel,
	traits::CraftingAction,
	Simulation,
};

#[derive(Clone)]
pub struct Observe;

impl CraftingAction for Observe {
	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::unchecked_new(13))
	}

	fn get_type(&self) -> ActionType {
		ActionType::Other
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, _simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		true
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		7
	}

	fn get_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn execute_with_flags(
		&self,
		_simulation_state: &mut Simulation,
		_safe: bool,
		_skip_stack_addition: bool,
	) {
		// nothing
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::Observe
	}
}
