use crate::types::{structs::CraftingLevel, traits::{BuffAction, CraftingAction}, enums::*, Simulation};

#[derive(Clone)]
pub struct WasteNot;

impl BuffAction for WasteNot {
    fn get_duration(&self, _simulation_state: &Simulation) -> u32 {
        4
    }

	fn can_be_clipped(&self) -> bool {
		true
	}

	fn get_overrides(&self) -> Vec<Buff> {
		vec![self.get_buff(), Buff::WasteNotII]
	}

    fn get_buff(&self) -> Buff {
        Buff::WasteNot
    }

    fn get_initial_stacks(&self) -> u32 {
        0
    }
}

impl CraftingAction for WasteNot {
	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_type(&self) -> ActionType {
		ActionType::Buff
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(15).unwrap())
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		if self.can_be_clipped() {
			true
		} else {
			!simulation_state.has_buff(self.get_buff())
		}
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		56
	}

	fn get_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		self.get_overrides().into_iter().for_each(|b| simulation_state.remove_buff(b));
		simulation_state.add_buff(self.get_applied_buff(simulation_state));
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::WasteNot
	}
}
