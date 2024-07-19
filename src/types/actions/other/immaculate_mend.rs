use crate::types::enums::{ActionType, CraftingActionEnum, CraftingJob};
use crate::types::Simulation;
use crate::types::structs::CraftingLevel;
use crate::types::traits::CraftingAction;

#[derive(Clone)]
pub struct ImmaculateMend;

impl CraftingAction for ImmaculateMend {
    fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
        todo!()
    }

    fn get_type(&self) -> ActionType {
        todo!()
    }

    fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
        todo!()
    }

    fn _can_be_used(&self, simulation_state: &Simulation, linear: Option<bool>) -> bool {
        todo!()
    }

    fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
        todo!()
    }

    fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
        todo!()
    }

    fn execute_with_flags(&self, simulation_state: &mut Simulation, safe: bool, skip_stack_addition: bool) {
        todo!()
    }

    fn get_enum(&self) -> CraftingActionEnum {
        todo!()
    }
}