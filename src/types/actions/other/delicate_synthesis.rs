use crate::types::{
	enums::{ActionType, Buff, CraftingJob, StepState},
	structs::CraftingLevel,
	traits::{CraftingAction, GeneralAction},
	Simulation,
};

#[derive(Clone, Copy, PartialEq)]
pub struct DelicateSynthesis;

impl GeneralAction for DelicateSynthesis {
	fn get_potency(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn get_base_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		10
	}

	fn get_base_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}
}

impl CraftingAction for DelicateSynthesis {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::unchecked_new(76))
	}

	fn get_type(&self) -> ActionType {
		ActionType::Other
	}

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, _simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		true
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		32
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
		// progress
		let progression_increase = self.get_base_progression(simulation_state);
		let progress_potency = self.get_potency(simulation_state);
		let mut progress_buff_mod = self.get_base_bonus(simulation_state);
		let mut progress_condition_mod = self.get_base_condition(simulation_state);

		if simulation_state.state() == StepState::Malleable {
			progress_condition_mod *= 1.5;
		}

		if simulation_state.has_buff(Buff::MuscleMemory) {
			progress_buff_mod += 1.0;
			simulation_state.remove_buff(Buff::MuscleMemory);
		}
		if simulation_state.has_buff(Buff::Veneration) {
			progress_buff_mod += 0.5;
		}

		let progress_efficiency = progress_potency as f64 * progress_buff_mod;
		simulation_state.progression +=
			(progression_increase as f64 * progress_condition_mod * progress_efficiency / 100.0)
				.floor() as u32;

		if simulation_state.has_buff(Buff::FinalAppraisal)
			&& simulation_state.progression >= simulation_state.recipe.progress
		{
			simulation_state.progression = simulation_state
				.progression
				.min(simulation_state.recipe.progress - 1);
			simulation_state.remove_buff(Buff::FinalAppraisal);
		}

		// quality
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let quality_increase = self.get_base_quality(simulation_state) as f64;

		match simulation_state.state() {
			StepState::Excellent => condition_mod *= 4.0,
			StepState::Poor => condition_mod *= 0.5,
			StepState::Good => {
				condition_mod *= if simulation_state.crafter_stats.splendorous {
					1.75
				} else {
					1.5
				}
			}
			_ => (),
		};

		buff_mod += simulation_state
			.get_buff(Buff::InnerQuiet)
			.map(|b| b.stacks)
			.unwrap_or_default() as f64
			/ 10.0;

		let mut buff_mult = 1.0;
		if simulation_state.has_buff(Buff::GreatStrides) {
			buff_mult += 1.0;
			simulation_state.remove_buff(Buff::GreatStrides);
		}
		if simulation_state.has_buff(Buff::Innovation) {
			buff_mult += 0.5;
		}

		let buff_mod: f64 = ((buff_mod as f32) * (buff_mult as f32)) as f64;
		let efficiency = ((potency as f64 * buff_mod) as f32) as f64;
		simulation_state.quality +=
			(quality_increase * condition_mod * efficiency / 100.0).floor() as u32;

		simulation_state.add_inner_quiet_stacks(1);
	}
}
