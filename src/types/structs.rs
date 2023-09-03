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
	pub lvl: u32,
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
	pub level: u32,
	pub levels: CrafterLevels,
}

#[derive(Clone)]
pub struct CrafterLevels {
	pub crp: u32,
	pub bsm: u32,
	pub arm: u32,
	pub gsm: u32,
	pub ltw: u32,
	pub wvr: u32,
	pub alc: u32,
	pub cul: u32,
}

impl From<[u32; 8]> for CrafterLevels {
	fn from(value: [u32; 8]) -> Self {
		CrafterLevels {
			crp: value[0],
			bsm: value[1],
			arm: value[2],
			gsm: value[3],
			ltw: value[4],
			wvr: value[5],
			alc: value[6],
			cul: value[7],
		}
	}
}
impl std::ops::Index<CraftingJob> for CrafterLevels {
	type Output = u32;
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

pub struct EffectiveBuff {
	pub duration: u32,
	pub stacks: u32,
	pub buff: Buff,
	pub applied_step: u32,
}
pub trait EffectiveBuffT {
	fn tick(&self, simulation_state: &Simulation);
}
impl EffectiveBuffT for EffectiveBuff {
	fn tick(&self, simulation_state: &Simulation) {}
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
