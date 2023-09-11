use super::{enums::*, traits::CraftingAction, Simulation};

#[derive(Clone)]
pub struct ActionResult {
	pub action: Box<dyn CraftingAction>,
	pub success: Option<bool>,
	pub fail_cause: Option<String>,
	pub added_progression: u32,
	pub added_quality: u32,
	pub cp_difference: i32,
	pub solidity_difference: i32,
	pub skipped: bool,
	pub combo: Option<bool>,
	pub state: StepState,
	// pub after_buff_tick: Option<{
	//     pub added_progression: u32,
	//     pub added_quality: u32,
	//     pub cp_difference: i32,
	//     pub solidity_difference: i32
	// }>
}

#[derive(Clone, Default)]
pub struct Craft {
	pub id: String,
	pub job: u32,
	pub rlvl: u32,
	pub durability: u32,
	pub quality: u32,
	pub progress: u32,
	pub lvl: CraftingLevel,
	pub suggested_craftsmanship: Option<u32>,
	pub suggested_control: Option<u32>,
	pub stars: Option<u32>,
	pub hq: Option<bool>,
	pub quick_synth: Option<bool>,
	pub control_req: Option<u32>,
	pub craftsmanship_req: Option<u32>,
	pub unlock_id: Option<u32>,
	pub ingredients: Vec<Ingredient>,
	pub r#yield: Option<u32>,
	pub expert: Option<bool>,
	pub conditions_flag: u32,
	pub progress_divider: u32,
	pub quality_divider: u32,
	pub progress_modifier: Option<f64>,
	pub quality_modifier: Option<f64>,
	pub required_quality: Option<u32>,
}

#[derive(Clone)]
pub struct CrafterStats {
	pub job_id: u32,
	pub craftsmanship: u32,
	pub control: u32,
	pub cp: u32,
	pub specialist: bool,
	pub splendorous: bool,
	pub level: CraftingLevel,
	pub levels: CrafterLevels,
}

#[derive(Clone)]
pub struct CrafterLevels {
	pub crp: CraftingLevel,
	pub bsm: CraftingLevel,
	pub arm: CraftingLevel,
	pub gsm: CraftingLevel,
	pub ltw: CraftingLevel,
	pub wvr: CraftingLevel,
	pub alc: CraftingLevel,
	pub cul: CraftingLevel,
}

impl From<[u8; 8]> for CrafterLevels {
	fn from(value: [u8; 8]) -> Self {
		CrafterLevels {
			crp: CraftingLevel::new(value[0]).unwrap(),
			bsm: CraftingLevel::new(value[1]).unwrap(),
			arm: CraftingLevel::new(value[2]).unwrap(),
			gsm: CraftingLevel::new(value[3]).unwrap(),
			ltw: CraftingLevel::new(value[4]).unwrap(),
			wvr: CraftingLevel::new(value[5]).unwrap(),
			alc: CraftingLevel::new(value[6]).unwrap(),
			cul: CraftingLevel::new(value[7]).unwrap(),
		}
	}
}
impl std::ops::Index<CraftingJob> for CrafterLevels {
	type Output = CraftingLevel;
	fn index(&self, index: CraftingJob) -> &Self::Output {
		match index {
			CraftingJob::Any => panic!("Crafting job 'ANY' specified as index argument"),
			CraftingJob::Carpenter => &self.crp,
			CraftingJob::Blacksmith => &self.bsm,
			CraftingJob::Armorer => &self.arm,
			CraftingJob::Goldsmith => &self.gsm,
			CraftingJob::Leatherworker => &self.ltw,
			CraftingJob::Weaver => &self.wvr,
			CraftingJob::Alchemist => &self.alc,
			CraftingJob::Culinarian => &self.cul,
		}
	}
}

bounded_integer::bounded_integer! {
	pub struct CraftingLevel { 0..=90 }
}

#[derive(Clone)]
pub struct EffectiveBuff {
	pub duration: u32,
	pub stacks: u32,
	pub buff: Buff,
	pub applied_step: u32,
	pub tick: Option<fn(&mut Simulation, &dyn CraftingAction) -> ()>,
	pub on_expire: Option<fn(&mut Simulation, &dyn CraftingAction) -> ()>,
}
impl EffectiveBuff {
	pub fn tick(&self, simulation_state: &mut Simulation, action: &dyn CraftingAction) {
		if let Some(f) = &self.tick {
			f(simulation_state, action);
		}
	}

	pub fn on_expire(&self, simulation_state: &mut Simulation, action: &dyn CraftingAction) {
		if let Some(f) = &self.on_expire {
			f(simulation_state, action);
		}
	}
}

#[derive(Clone, Default)]
pub struct Ingredient {
	pub id: String,
	pub amount: u32,
	pub quality: Option<u32>,
	pub step_id: Option<String>,
	pub part: Option<String>,
	pub phase: Option<u32>,
	pub custom: Option<bool>,
}

pub struct SimulationResult {
	pub steps: Vec<ActionResult>,
	pub hq_percent: u32,
	pub success: bool,
	pub simulation: Simulation,
	pub fail_cause: Option<String>,
}
