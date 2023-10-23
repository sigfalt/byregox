use anyhow::Result;

use crate::types::{
	actions,
	enums::Buff,
	structs::{Craft, CrafterLevels, CrafterStats, CraftingLevel},
	tables, SimulationBuilder,
};

#[test]
fn test_basics() -> Result<()> {
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
fn test_muscle_memory() -> Result<()> {
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
fn test_careful_synthesis() -> Result<()> {
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
fn test_groundwork() -> Result<()> {
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
fn test_quality_and_buffs() -> Result<()> {
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
fn test_combos() -> Result<()> {
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

// from teamcraft

#[test]
fn test_reflect() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Reflect),
			Box::new(actions::BasicTouch),
			Box::new(actions::CarefulSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(
		result
			.simulation
			.get_buff(Buff::InnerQuiet)
			.map(|b| b.stacks),
		Some(3)
	);

	Ok(())
}

#[test]
fn test_low_level() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Reflect),
			Box::new(actions::BasicTouch),
			Box::new(actions::ByregotsBlessing),
			Box::new(actions::CarefulSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[3].added_progression, 685);
	assert_eq!(result.simulation.steps[0].added_quality, 817);
	assert_eq!(result.simulation.steps[1].added_quality, 980);
	assert_eq!(result.simulation.steps[2].added_quality, 1699);

	Ok(())
}

#[test]
fn test_innovation() -> Result<()> {
	// generateRecipe(517, 2000, 5200, 121, 105)
	let mut recipe = generate_recipe(3864, 81, 80, 2000, 5200, 121, 105);
	recipe.lvl = CraftingLevel::new(80).unwrap();
	// generateStats(80, 2763, 2780, 545)
	let stats = generate_stats(80, 2763, 2780, 545, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Reflect),
			Box::new(actions::DelicateSynthesis),
			Box::new(actions::DelicateSynthesis),
			Box::new(actions::WasteNot),
			Box::new(actions::Groundwork),
			Box::new(actions::Innovation),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::PreparatoryTouch),
			Box::new(actions::MastersMend),
			Box::new(actions::PreparatoryTouch),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[0].added_quality, 299);
	assert_eq!(result.simulation.steps[1].added_quality, 358);
	assert_eq!(result.simulation.steps[2].added_quality, 388);
	assert_eq!(result.simulation.steps[6].added_quality, 1255);
	assert_eq!(result.simulation.steps[7].added_quality, 1435);
	assert_eq!(result.simulation.steps[9].added_quality, 1614);

	Ok(())
}

#[test]
fn test_flooring() -> Result<()> {
	// generateRecipe(517, 2000, 5200, 121, 105)
	let mut recipe = generate_recipe(3864, 81, 80, 2000, 5200, 121, 105);
	recipe.lvl = CraftingLevel::new(80).unwrap();
	// generateStats(80, 1645, 1532, 400)
	let stats = generate_stats(80, 1645, 1532, 400, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicTouch),
			Box::new(actions::BasicTouch),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.quality, 828);

	// generateStarRecipe(580, 3900, 10920, 130, 115, 80, 70)
	let recipe = generate_star_recipe(580, 3900, 10920, 130, 115, 80, 70);
	// generateStats(90, 3289, 3420, 400)
	let stats = generate_stats(90, 3289, 3420, 400, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::MuscleMemory),
			Box::new(actions::Veneration),
			Box::new(actions::Groundwork),
			Box::new(actions::Groundwork),
			Box::new(actions::Observe),
			Box::new(actions::Observe),
			Box::new(actions::CarefulSynthesis),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[0].added_progression, 609);
	assert_eq!(result.simulation.progression, 3897);

	Ok(())
}

#[test]
fn test_advanced_touch_combo() -> Result<()> {
	// generateRecipe(517, 1000, 5200, 121, 105)
	let recipe = generate_recipe(3864, 81, 80, 1000, 5200, 121, 105);
	// generateStats(90, 2763, 2780, 545)
	let stats = generate_stats(90, 2763, 2780, 545, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe.clone())
		.actions(vec![
			Box::new(actions::StandardTouch),
			Box::new(actions::AdvancedTouch),
		])
		.crafter_stats(stats.clone())
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[1].cp_difference, -46);

	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::BasicTouch),
			Box::new(actions::StandardTouch),
			Box::new(actions::AdvancedTouch),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[1].cp_difference, -18);

	Ok(())
}

#[test]
fn test_level_90_accuracy() -> Result<()> {
	// generateStarRecipe(560, 1000, 5200, 130, 115, 90, 80)
	let recipe = generate_star_recipe(560, 1000, 5200, 130, 115, 90, 80);
	// generateStats(90, 2659, 2803, 548)
	let stats = generate_stats(90, 2659, 2803, 548, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Reflect),
			Box::new(actions::BasicSynthesis),
			Box::new(actions::BasicTouch),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[0].added_quality, 222);
	assert_eq!(result.simulation.steps[1].added_progression, 222);
	assert_eq!(result.simulation.steps[2].added_quality, 266);

	Ok(())
}

#[test]
fn test_innovation_great_strides_interaction() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532, false);
	let sim = SimulationBuilder::default()
		.recipe(recipe)
		.actions(vec![
			Box::new(actions::Reflect),
			Box::new(actions::Innovation),
			Box::new(actions::GreatStrides),
			Box::new(actions::BasicTouch),
		])
		.crafter_stats(stats)
		.build()?;
	let result = sim.run(true);
	assert_eq!(result.simulation.steps[0].added_quality, 817);
	assert_eq!(result.simulation.steps[3].added_quality, 2451);

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

fn generate_star_recipe(
	rlvl: u32,
	progress: u32,
	quality: u32,
	progress_divider: u32,
	quality_divider: u32,
	progress_modifier: u32,
	quality_modifier: u32,
) -> Craft {
	Craft {
		id: "33904".to_string(),
		job: 14, // CRP
		lvl: CraftingLevel::new(80).unwrap(),
		rlvl,
		durability: 70,
		quality,
		progress,
		hq: Some(true),
		quick_synth: Some(false),
		ingredients: vec![],
		expert: Some(false),
		conditions_flag: 15,
		progress_divider,
		quality_divider,
		progress_modifier: Some(progress_modifier as f64),
		quality_modifier: Some(quality_modifier as f64),
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
