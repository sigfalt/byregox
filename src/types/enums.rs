use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, PartialEq)]
pub enum StepState {
	None, // Fails the step
	Normal,
	Good,
	Excellent, // Not available on expert recipes
	Poor,      // Not available on expert recipes

	// Only for expert recipes
	Centered, // Increase success rate by 25%
	Sturdy,   // Reduces loss of durability by 50%, stacks with WN/WN2
	Pliant,   // Reduces CP cost by 50%

	// Only for super expert recipes
	Malleable, // Good, but for Progress. Doesn't proc Intensive/Precise
	Primed,    // Next status is +2 duration
	GoodOmen,  // Next step is GOOD condition
}

#[derive(Clone, Copy, PartialEq)]
pub enum ActionType {
	Progression,
	Quality,
	CPRecovery,
	Buff,
	Specialty,
	Repair,
	Other,
}

#[derive(Clone, Copy, PartialEq)]
// #[enum_dispatch]
pub enum CraftingActionEnum {
	// Progress actions
	BasicSynthesis,
	CarefulSynthesis,
	PrudentSynthesis,
	RapidSynthesis,
	Groundwork,
	MuscleMemory,
	IntensiveSynthesis,

	// Quality actions
	BasicTouch,
	StandardTouch,
	AdvancedTouch,
	HastyTouch,
	ByregotsBlessing,
	PreciseTouch,
	PrudentTouch,
	TrainedEye,
	PreparatoryTouch,
	Reflect,
	TrainedFinesse,
	DaringTouch,
	RefinedTouch,

	// CP recovery
	TricksOfTheTrade,

	// Repair
	MastersMend,
	Manipulation,
	ImmaculateMend,

	// Buffs
	WasteNot,
	WasteNotII,
	GreatStrides,
	Innovation,
	Veneration,
	FinalAppraisal,
	QuickInnovation,
	TrainedPerfection,

	// Other
	Observe,
	HeartAndSoul,
	CarefulObservation,
	DelicateSynthesis,
	RemoveFinalAppraisal,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Buff {
	InnerQuiet,

	WasteNot,
	WasteNotII,

	Manipulation,

	GreatStrides,

	Innovation,

	Veneration,

	MakersMark,

	MuscleMemory,

	FinalAppraisal,

	HeartAndSoul,

	TrainedPerfection,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CraftingJob {
	Any,
	Carpenter,
	Blacksmith,
	Armorer,
	Goldsmith,
	Leatherworker,
	Weaver,
	Alchemist,
	Culinarian,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FailCause {
	UnsafeAction,
	DurabilityReachedZero,
	NotEnoughCP,
	MissingLevelRequirement,
	MissingStatsRequirement,
	NotSpecialist,
	NoInnerQuiet,
	QualityTooLow
}
