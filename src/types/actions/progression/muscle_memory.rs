use crate::types::{
	enums::{ActionType, Buff, CraftingActionEnum, CraftingJob, StepState},
	structs::{CraftingLevel, EffectiveBuff},
	traits::{CraftingAction, GeneralAction, ProgressAction},
	Simulation,
};

#[derive(Clone)]
pub struct MuscleMemory;

impl ProgressAction for MuscleMemory {}

impl CraftingAction for MuscleMemory {
	fn can_be_moved(&self, current_index: u32) -> bool {
		current_index > 0
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::unchecked_new(54))
	}

	fn get_type(&self) -> ActionType {
		ActionType::Progression
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		simulation_state
			.steps
			.iter()
			.all(|s| s.action.skips_buff_ticks())
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		6
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII)
		{
			divider *= 2.0
		}
		if simulation_state.state() == StepState::Sturdy {
			divider *= 2.0
		}
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute_with_flags(
		&self,
		simulation_state: &mut Simulation,
		_safe: bool,
		_skip_stack_addition: bool,
	) {
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let progression_increase = self.get_base_progression(simulation_state);

		if simulation_state.state() == StepState::Malleable {
			condition_mod *= 1.5;
		}
		if simulation_state.has_buff(Buff::MuscleMemory) {
			buff_mod += 1.0;
			simulation_state.remove_buff(Buff::MuscleMemory);
		}
		if simulation_state.has_buff(Buff::Veneration) {
			buff_mod += 0.5;
		}

		let efficiency = potency as f64 * buff_mod;
		simulation_state.progression +=
			(progression_increase as f64 * condition_mod * efficiency / 100.0).floor() as u32;

		if simulation_state.has_buff(Buff::FinalAppraisal)
			&& simulation_state.progression >= simulation_state.recipe.progress
		{
			simulation_state.progression = simulation_state
				.progression
				.min(simulation_state.recipe.progress - 1);
			simulation_state.remove_buff(Buff::FinalAppraisal);
		}

		// Muscle Memory specific addition to blanket `execute` impl
		simulation_state.add_buff(EffectiveBuff {
			duration: if simulation_state.state() == StepState::Primed {
				7
			} else {
				5
			},
			stacks: 0,
			buff: Buff::MuscleMemory,
			applied_step: simulation_state.steps.len() as u32,
			tick: None,
			on_expire: None,
		});
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::MuscleMemory
	}
}

impl GeneralAction for MuscleMemory {
	fn get_potency(&self, _simulation_state: &Simulation) -> u32 {
		300
	}

	fn get_base_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		10
	}

	fn get_base_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}
}
