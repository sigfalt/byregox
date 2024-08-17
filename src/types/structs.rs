use super::{enums::*, traits::CraftingAction, Simulation};

#[derive(Clone)]
pub struct ActionResult {
	pub action: Box<dyn CraftingAction>,
	pub success: Option<bool>,
	pub fail_cause: Option<FailCause>,
	pub added_progression: u32,
	pub added_quality: u32,
	pub cp_difference: i32,
	pub solidity_difference: i32,
	pub skipped: bool,
	pub combo: Option<bool>,
	pub state: StepState,
	pub after_buff_tick: Option<BuffTickResult>,
}

#[derive(Clone)]
pub struct BuffTickResult {
	pub added_progression: u32,
	pub added_quality: u32,
	pub cp_difference: i32,
	pub solidity_difference: i32,
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
			crp: CraftingLevel::unchecked_new(value[0]),
			bsm: CraftingLevel::unchecked_new(value[1]),
			arm: CraftingLevel::unchecked_new(value[2]),
			gsm: CraftingLevel::unchecked_new(value[3]),
			ltw: CraftingLevel::unchecked_new(value[4]),
			wvr: CraftingLevel::unchecked_new(value[5]),
			alc: CraftingLevel::unchecked_new(value[6]),
			cul: CraftingLevel::unchecked_new(value[7]),
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

const MAX_LEVEL: u8 = 100;

#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd)]
pub struct CraftingLevel {
	val: u8,
}
impl CraftingLevel {
	pub fn new(val: u8) -> Option<CraftingLevel> {
		if (0..=MAX_LEVEL).contains(&val) {
			Some(CraftingLevel { val })
		} else {
			None
		}
	}

	pub fn unchecked_new(val: u8) -> CraftingLevel {
		Self::new(val).unwrap()
	}
}
impl From<CraftingLevel> for u8 {
	fn from(value: CraftingLevel) -> Self {
		value.val
	}
}
impl std::ops::Sub for CraftingLevel {
	type Output = u8;

	fn sub(self, rhs: Self) -> Self::Output {
		self.val - rhs.val
	}
}
impl PartialEq<u8> for CraftingLevel {
	fn eq(&self, other: &u8) -> bool {
		self.val == *other
	}
}
impl PartialOrd<u8> for CraftingLevel {
	fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
		self.val.partial_cmp(other)
	}
}

#[derive(Clone)]
pub struct EffectiveBuff {
	pub duration: i32,
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
	pub fail_cause: Option<FailCause>,
}
