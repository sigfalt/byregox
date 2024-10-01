use std::ops::{Index, IndexMut};
use super::{enums::*, Simulation};

#[derive(Clone)]
pub struct ActionResult {
	pub action: CraftingActionEnum,
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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct CrafterLevels {
	crp: CraftingLevel,
	bsm: CraftingLevel,
	arm: CraftingLevel,
	gsm: CraftingLevel,
	ltw: CraftingLevel,
	wvr: CraftingLevel,
	alc: CraftingLevel,
	cul: CraftingLevel,
}
impl CrafterLevels {
	pub fn max() -> Self {
		Self {
			crp: CraftingLevel::max(),
			bsm: CraftingLevel::max(),
			arm: CraftingLevel::max(),
			gsm: CraftingLevel::max(),
			ltw: CraftingLevel::max(),
			wvr: CraftingLevel::max(),
			alc: CraftingLevel::max(),
			cul: CraftingLevel::max(),
		}
	}
}
impl TryFrom<[u8; 8]> for CrafterLevels {
	type Error = &'static str;

	fn try_from(value: [u8; 8]) -> Result<Self, Self::Error> {
		let converted_value_vec = value.into_iter().map(
			|n| CraftingLevel::try_from(n)
		).collect::<Result<Vec<_>, _>>()?;
		let converted_value_array: [_; 8] = converted_value_vec.as_slice().try_into().unwrap();
		Ok(Self::from(converted_value_array))
	}
}
impl From<[CraftingLevel; 8]> for CrafterLevels {
	fn from(value: [CraftingLevel; 8]) -> Self {
		Self {
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
impl Index<CraftingJob> for CrafterLevels {
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
impl IndexMut<CraftingJob> for CrafterLevels {
	fn index_mut(&mut self, index: CraftingJob) -> &mut Self::Output {
		match index {
			CraftingJob::Any => panic!("Crafting job 'ANY' specified as index argument"),
			CraftingJob::Carpenter => &mut self.crp,
			CraftingJob::Blacksmith => &mut self.bsm,
			CraftingJob::Armorer => &mut self.arm,
			CraftingJob::Goldsmith => &mut self.gsm,
			CraftingJob::Leatherworker => &mut self.ltw,
			CraftingJob::Weaver => &mut self.wvr,
			CraftingJob::Alchemist => &mut self.alc,
			CraftingJob::Culinarian => &mut self.cul,
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
		if val <= MAX_LEVEL {
			Some(CraftingLevel { val })
		} else {
			None
		}
	}

	pub fn unchecked_new(val: u8) -> CraftingLevel {
		Self::new(val).unwrap()
	}

	pub fn max() -> CraftingLevel {
		Self::unchecked_new(MAX_LEVEL)
	}
}
impl TryFrom<u8> for CraftingLevel {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		CraftingLevel::new(value).ok_or("")
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
	pub tick: Option<fn(&mut Simulation, &CraftingActionEnum) -> ()>,
	pub on_expire: Option<fn(&mut Simulation, &CraftingActionEnum) -> ()>,
}
impl EffectiveBuff {
	pub fn tick(&self, simulation_state: &mut Simulation, action: &CraftingActionEnum) {
		if let Some(f) = &self.tick {
			f(simulation_state, action);
		}
	}

	pub fn on_expire(&self, simulation_state: &mut Simulation, action: &CraftingActionEnum) {
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
