use crate::types::enums::{ActionType, CraftingActionEnum, CraftingJob};
use crate::types::Simulation;
use crate::types::structs::CraftingLevel;
use crate::types::traits::CraftingAction;

#[derive(Clone)]
pub struct ImmaculateMend;

impl CraftingAction for ImmaculateMend {
    fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
        (CraftingJob::Any, CraftingLevel::new(98).unwrap())
    }

    fn get_type(&self) -> ActionType {
        ActionType::Repair
    }

    fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
        100
    }

    fn _can_be_used(&self, simulation_state: &Simulation, linear: Option<bool>) -> bool {
        true
    }

    fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
        112
    }

    fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
        0
    }

    fn execute_with_flags(&self, simulation_state: &mut Simulation, safe: bool, skip_stack_addition: bool) {
        simulation_state.durability = simulation_state.recipe.durability as i32;
    }

    fn get_enum(&self) -> CraftingActionEnum {
        CraftingActionEnum::ImmaculateMend
    }
}