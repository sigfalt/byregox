use crate::types::{traits::CraftingAction, enums::{CraftingJob, ActionType}, Simulation};


#[derive(Clone)]
pub struct TrainedEye;

impl CraftingAction for TrainedEye {
    fn get_level_requirement(&self) -> (CraftingJob, u32) {
        (CraftingJob::Any, 80)
    }

    fn get_type(&self) -> crate::types::enums::ActionType {
        ActionType::Quality
    }

    fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
        100
    }

    fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
        simulation_state.recipe.expert.is_some_and(|x| !x)
		&& simulation_state.crafter_stats.level - simulation_state.recipe.lvl >= 10
		&& simulation_state.steps.len() == 0
    }

    fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
        250
    }

    fn get_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
        0
    }

    fn execute(&self, simulation_state: &mut Simulation) {
        simulation_state.quality = simulation_state.recipe.quality;
    }
}