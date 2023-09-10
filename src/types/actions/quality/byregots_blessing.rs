use crate::types::{structs::CraftingLevel, traits::{QualityAction, GeneralAction, CraftingAction}, enums::{CraftingJob, ActionType, Buff, StepState}, Simulation};

#[derive(Clone)]
pub struct ByregotsBlessing;

impl QualityAction for ByregotsBlessing {}

impl CraftingAction for ByregotsBlessing {
	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(50).unwrap())
	}

	fn get_type(&self) -> ActionType { ActionType::Quality }

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		simulation_state.get_buff(Buff::InnerQuiet).is_some_and(|buff| buff.stacks > 0)
	}

	fn get_fail_cause(&self, simulation_state: &Simulation) -> Option<&str> {
		let level_requirement = self.get_level_requirement();
		let craftsmanship_requirement = simulation_state.recipe.craftsmanship_req;
		let control_requirement = simulation_state.recipe.control_req;

		if (level_requirement.0 != CraftingJob::Any
			&& simulation_state.crafter_stats.levels[level_requirement.0] < level_requirement.1)
			|| simulation_state.crafter_stats.level < level_requirement.1
		{
			Some("Missing level requirement")
		} else if craftsmanship_requirement
			.is_some_and(|x| x > simulation_state.crafter_stats.craftsmanship)
			|| control_requirement.is_some_and(|x| x > simulation_state.crafter_stats.control)
		{
			Some("Missing stats requirement")
		} else
		// Byregots Blessing specific addition to blanket `get_fail_cause` impl
		if simulation_state.success.is_some_and(|x| !x) && !simulation_state.has_buff(Buff::InnerQuiet) {
			Some("No Inner Quiet")
		} else
		// end specific impl
		{
			None
		}
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		24
	}

	fn get_durability_cost(&self, simulation_state: &Simulation) -> u32 {
		let mut divider = 1.0;
		if simulation_state.has_buff(Buff::WasteNot) || simulation_state.has_buff(Buff::WasteNotII) { divider *= 2.0 }
		if simulation_state.state() == StepState::Sturdy { divider *= 2.0 }
		(self.get_base_durability_cost(simulation_state) as f64 / divider).ceil() as u32
	}

	fn execute(&self, simulation_state: &mut Simulation) {
		let mut buff_mod = self.get_base_bonus(simulation_state);
		let mut condition_mod = self.get_base_condition(simulation_state);
		let potency = self.get_potency(simulation_state);
		let quality_increase = self.get_base_quality(simulation_state) as f64;

		match simulation_state.state() {
			StepState::Excellent => condition_mod *= 4.0,
			StepState::Poor => condition_mod *= 0.5,
			StepState::Good => condition_mod *= if simulation_state.crafter_stats.splendorous { 1.75 } else { 1.5 },
			_ => ()
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
		simulation_state.quality += (quality_increase * condition_mod * efficiency / 100.0).floor() as u32;

		// if !skipStackAddition { // argument to function, defaults to false
		if true {
			simulation_state.add_inner_quiet_stacks(1);
		}

		// Byregots Blessing specific addition to blanket `execute` impl
		simulation_state.remove_buff(Buff::InnerQuiet);
	}
}

impl GeneralAction for ByregotsBlessing {
	fn get_potency(&self, simulation_state: &Simulation) -> u32 {
		300.min((simulation_state.get_buff(Buff::InnerQuiet).unwrap().stacks * 20) + 10)
	}

	fn get_base_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		10
	}

	fn get_base_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}
}