use std::error::Error;

use crate::types::{
	actions,
	structs::{Craft, CrafterLevels, CrafterStats, CraftingLevel},
	tables, SimulationBuilder,
};

#[test]
fn test_basics() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(1035, 15, 70, 55, 360, 50, 30);
	let stats = generate_stats(90, 1208, 698, 187, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_quality, 267);
	assert_eq!(result.simulation.steps[1].added_quality, 293);
	assert_eq!(result.simulation.steps[2].added_progression, 291);

	Ok(())
}

#[test]
fn test_muscle_memory() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(1960, 61, 80, 630, 2790, 81, 58);
	let stats = generate_stats(90, 1208, 698, 187, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::MuscleMemory),
			Box::new(actions::BasicSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_progression, 453);
	assert_eq!(result.simulation.steps[1].added_progression, 362);

	Ok(())
}

#[test]
fn test_careful_synthesis() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(3997, 72, 80, 1220, 3800, 102, 82);
	let stats = generate_stats(90, 1208, 698, 187, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::MuscleMemory),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::CarefulSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_progression, 360);
	assert_eq!(result.simulation.steps[1].added_progression, 432);
	assert_eq!(result.simulation.steps[2].added_progression, 216);
	assert_eq!(result.simulation.steps[3].added_progression, 216);

	Ok(())
}

#[test]
fn test_groundwork() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(3997, 72, 80, 1220, 3800, 102, 82);
	let stats = generate_stats(90, 1208, 698, 187, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Groundwork),
			Box::new(actions::Groundwork),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicTouch),
			Box::new(actions::Groundwork),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_progression, 432);
	assert_eq!(result.simulation.steps[1].added_progression, 432);
	assert_eq!(result.simulation.steps[5].added_progression, 216);

	Ok(())
}

#[test]
fn test_quality_and_buffs() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(3997, 72, 80, 1220, 3800, 102, 82);
	let stats = generate_stats(90, 1208, 698, 534, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::MuscleMemory),
			Box::new(actions::Manipulation),
			Box::new(actions::Veneration),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::PrudentTouch),
			Box::new(actions::WasteNot),
			Box::new(actions::AdvancedTouch),
			Box::new(actions::StandardTouch),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::Innovation),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::GreatStrides),
			Box::new(actions::ByregotsBlessing),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::CarefulSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_progression, 360);
	assert_eq!(result.simulation.steps[3].added_progression, 540);
	assert_eq!(result.simulation.steps[15].added_progression, 216);
	assert_eq!(result.simulation.steps[16].added_progression, 216);
	assert_eq!(result.simulation.steps[4].added_quality, 120);
	assert_eq!(result.simulation.steps[6].added_quality, 198);
	assert_eq!(result.simulation.steps[7].added_quality, 180);
	assert_eq!(result.simulation.steps[8].added_quality, 312);
	assert_eq!(result.simulation.steps[9].added_quality, 360);
	assert_eq!(result.simulation.steps[11].added_quality, 612);
	assert_eq!(result.simulation.steps[12].added_quality, 684);
	assert_eq!(result.simulation.steps[14].added_quality, 1800);

	Ok(())
}

#[test]
fn test_combos() -> Result<(), Box<dyn Error>> {
	let recipe = generate_recipe(3997, 72, 80, 1220, 3800, 102, 82);
	let stats = generate_stats(90, 1208, 698, 534, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::MuscleMemory),
			Box::new(actions::Manipulation),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::CarefulSynthesis),
			Box::new(actions::BasicTouch),
			Box::new(actions::StandardTouch),
			Box::new(actions::AdvancedTouch),
			Box::new(actions::StandardTouch),
			Box::new(actions::AdvancedTouch),
			Box::new(actions::Observe),
			Box::new(actions::FocusedTouch),
			Box::new(actions::Observe),
			Box::new(actions::FocusedSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[0].added_progression, 360);
	assert_eq!(result.simulation.steps[2].added_progression, 432);
	assert_eq!(result.simulation.steps[3].added_progression, 216);
	assert_eq!(result.simulation.steps[12].added_progression, 240);
	assert_eq!(result.simulation.steps[4].added_quality, 120);
	assert_eq!(result.simulation.steps[5].added_quality, 165);
	assert_eq!(result.simulation.steps[6].added_quality, 216);
	assert_eq!(result.simulation.steps[7].added_quality, 195);
	assert_eq!(result.simulation.steps[8].added_quality, 252);
	assert_eq!(result.simulation.steps[10].added_quality, 270);

	Ok(())
}

fn generate_recipe(
	id: u32,
	lvl: u8,
	durability: u32,
	progress: u32,
	quality: u32,
	progress_divider: u32,
	quality_divider: u32,
) -> Craft {
	Craft {
		id: id.to_string(),
		job: 14, // CRP
		lvl: CraftingLevel::new(lvl).unwrap(),
		rlvl: tables::level_to_ilevel(CraftingLevel::new(lvl).unwrap()),
		durability,
		progress,
		quality,
		progress_divider,
		quality_divider,
		hq: Some(true),
		quick_synth: Some(true),
		ingredients: vec![],
		..Default::default()
	}
}

fn generate_stats(
	level: u8,
	craftsmanship: u32,
	control: u32,
	cp: u32,
	splendorous: bool,
) -> CrafterStats {
	CrafterStats {
		job_id: 14,
		craftsmanship,
		control,
		cp,
		specialist: false,
		splendorous,
		level: CraftingLevel::new(level).unwrap(),
		levels: CrafterLevels {
			crp: CraftingLevel::new(level).unwrap(),
			bsm: CraftingLevel::new(level).unwrap(),
			arm: CraftingLevel::new(level).unwrap(),
			gsm: CraftingLevel::new(level).unwrap(),
			ltw: CraftingLevel::new(level).unwrap(),
			wvr: CraftingLevel::new(level).unwrap(),
			alc: CraftingLevel::new(level).unwrap(),
			cul: CraftingLevel::new(level).unwrap(),
		},
	}
}
