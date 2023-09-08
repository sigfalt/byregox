use crate::types::{traits::{QualityAction, CraftingAction, GeneralAction}, enums::{ActionType, CraftingJob, Buff, StepState}, Simulation};


#[derive(Clone)]
pub struct PreciseTouch;

impl QualityAction for PreciseTouch {}

impl CraftingAction for PreciseTouch {
	fn requires_good(&self) -> bool {
		true
	}

	fn get_level_requirement(&self) -> (CraftingJob, u32) {
		(CraftingJob::Any, 53)
	}

	fn get_type(&self) -> ActionType { ActionType::Quality }

	fn _get_success_rate(&self, simulation_state: &Simulation) -> u32 {
		self.get_base_success_rate(simulation_state)
	}

	fn _can_be_used(&self, simulation_state: &Simulation) -> bool {
		// TODO: if linear (default false)
		if false {
			true
		} else if simulation_state.safe && !simulation_state.has_buff(Buff::HeartAndSoul) {
			false
		} else {
			simulation_state.has_buff(Buff::HeartAndSoul)
			|| simulation_state.state() == StepState::Good
			|| simulation_state.state() == StepState::Excellent
		}
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		18
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

		// Precise Touch specific addition to blanket `execute` impl
		simulation_state.add_inner_quiet_stacks(1);
		// TODO: any reason to do this separately in Rust? since we reimpl execute anyway
	}
}

impl GeneralAction for PreciseTouch {
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