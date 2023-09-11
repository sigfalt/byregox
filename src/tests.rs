use std::error::Error;

use crate::types::{
	actions,
	structs::{Craft, CrafterLevels, CrafterStats, CraftingLevel},
	SimulationBuilder,
};

#[test]
fn test_simulation_construction() -> Result<(), Box<dyn Error>> {
	let recipe = Craft {
		id: "1035".to_string(),
		job: 14,
		rlvl: 15,
		durability: 70,
		quality: 360,
		progress: 55,
		lvl: CraftingLevel::new(15).unwrap(),
		hq: Some(true),
		quick_synth: Some(true),
		ingredients: vec![],
		progress_divider: 50,
		quality_divider: 30,
		..Default::default()
	};
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
	let recipe = Craft {
		id: "1960".to_string(),
		job: 14,
		rlvl: 255,
		durability: 80,
		quality: 2790,
		progress: 630,
		lvl: CraftingLevel::new(61).unwrap(),
		hq: Some(true),
		quick_synth: Some(true),
		ingredients: vec![],
		progress_divider: 81,
		quality_divider: 58,
		..Default::default()
	};
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
	let recipe = Craft {
		id: "3997".to_string(),
		job: 14,
		rlvl: 395,
		durability: 80,
		quality: 3800,
		progress: 1220,
		lvl: CraftingLevel::new(72).unwrap(),
		hq: Some(true),
		quick_synth: Some(true),
		ingredients: vec![],
		progress_divider: 102,
		quality_divider: 82,
		..Default::default()
	};
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
	let recipe = Craft {
		id: "3997".to_string(),
		job: 14,
		rlvl: 395,
		durability: 80,
		quality: 3800,
		progress: 1220,
		lvl: CraftingLevel::new(72).unwrap(),
		hq: Some(true),
		quick_synth: Some(true),
		ingredients: vec![],
		progress_divider: 102,
		quality_divider: 82,
		..Default::default()
	};
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
