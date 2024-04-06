use crate::types::{
	enums::{ActionType, CraftingActionEnum, CraftingJob},
	structs::CraftingLevel,
	traits::CraftingAction,
	Simulation,
};

#[derive(Clone)]
pub struct TrainedEye;

impl CraftingAction for TrainedEye {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(80).unwrap())
	}

	fn get_type(&self) -> crate::types::enums::ActionType {
		ActionType::Quality
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		simulation_state.recipe.expert.is_some_and(|x| !x)
			&& simulation_state.crafter_stats.level - simulation_state.recipe.lvl >= 10
			&& simulation_state.steps.is_empty()
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		250
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
		simulation_state.quality = simulation_state.recipe.quality;
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::TrainedEye
	}
}
