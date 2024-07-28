use crate::types::{
	enums::{ActionType, Buff, CraftingActionEnum, CraftingJob, StepState},
	structs::CraftingLevel,
	traits::{CraftingAction, GeneralAction, QualityAction},
	Simulation,
};

#[derive(Clone)]
pub struct AdvancedTouch;

impl QualityAction for AdvancedTouch {}

impl CraftingAction for AdvancedTouch {
	fn has_combo(&self, simulation_state: &Simulation) -> bool {
		// need to check not only for StandardTouch, but that it was also combo'd
		for step in simulation_state.steps.iter().rev() {
			if step.action.get_enum() == CraftingActionEnum::Observe
				|| (step.action.get_enum() == CraftingActionEnum::StandardTouch
					&& step.success.is_some_and(|x| x)
					&& step.combo.is_some_and(|x| x)) {
				return true;
			}
			if !step.skipped {
				return false;
			}
		}
		false
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(84).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::Quality
	}

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, _simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		true
	}

	fn get_base_cp_cost(&self, simulation_state: &Simulation) -> u32 {
		if self.has_combo(simulation_state) {
			18
		} else {
			46
		}
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
		skip_stack_addition: bool,
	) {
		let buff_mod = self.get_base_bonus(simulation_state);
		let potency = self.get_potency(simulation_state) as f64;
		let quality_increase = self.get_base_quality(simulation_state) as f64;

		let mut condition_mod = self.get_base_condition(simulation_state);
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

		let iq_mod = simulation_state
			.get_buff(Buff::InnerQuiet)
			.map(|b| b.stacks)
			.unwrap_or(0);

		let mut buff_mult = 1.0;
		if simulation_state.has_buff(Buff::GreatStrides) {
			buff_mult += 1.0;
			simulation_state.remove_buff(Buff::GreatStrides);
		}
		if simulation_state.has_buff(Buff::Innovation) {
			buff_mult += 0.5;
		}

		let buff_mod = buff_mod * buff_mult * (100 + iq_mod * 10) as f64 / 100.0;
		let efficiency = ((potency * buff_mod) as f32) as f64;
		simulation_state.quality += (quality_increase * condition_mod * efficiency / 100.0) as u32;

		if !skip_stack_addition && simulation_state.crafter_stats.level >= 11 {
			simulation_state.add_inner_quiet_stacks(1);
		}
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::AdvancedTouch
	}
}

impl GeneralAction for AdvancedTouch {
	fn get_potency(&self, _simulation_state: &Simulation) -> u32 {
		150
	}

	fn get_base_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		10
	}

	fn get_base_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}
}
