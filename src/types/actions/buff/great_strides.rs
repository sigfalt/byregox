use crate::types::{
	enums::*,
	structs::CraftingLevel,
	traits::{BuffAction, CraftingAction},
	Simulation,
};

#[derive(Clone)]
pub struct GreatStrides;

impl BuffAction for GreatStrides {
	fn get_duration(&self, _simulation_state: &Simulation) -> i32 {
		3
	}

	fn can_be_clipped(&self) -> bool {
		true
	}

	fn get_buff(&self) -> Buff {
		Buff::GreatStrides
	}

	fn get_initial_stacks(&self) -> u32 {
		0
	}
}

impl CraftingAction for GreatStrides {
	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(21).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::Buff
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		if self.can_be_clipped() {
			true
		} else {
			!simulation_state.has_buff(self.get_buff())
		}
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		32
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
		self.get_overrides()
			.into_iter()
			.for_each(|b| simulation_state.remove_buff(b));
		simulation_state.add_buff(self.get_applied_buff(simulation_state));
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::GreatStrides
	}
}
