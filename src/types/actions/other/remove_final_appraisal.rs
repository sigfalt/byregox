use crate::types::{
	enums::{ActionType, Buff, CraftingActionEnum, CraftingJob},
	structs::CraftingLevel,
	traits::CraftingAction,
	Simulation,
};

#[derive(Clone)]
pub struct RemoveFinalAppraisal;

impl CraftingAction for RemoveFinalAppraisal {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(42).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::Other
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		simulation_state.has_buff(Buff::FinalAppraisal)
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
		simulation_state.remove_buff(Buff::FinalAppraisal);
	}

	fn skips_buff_ticks(&self) -> bool {
		true
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::RemoveFinalAppraisal
	}
}
