use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::types::{
	actions,
	enums::{Buff, CraftingActionEnum, StepState},
	structs::{Craft, CrafterLevels, CrafterStats, CraftingLevel},
	tables,
	traits::CraftingAction,
	Simulation,
};

#[test]
fn test_reflect() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe_lvl(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::BasicTouch.into(),
			actions::CarefulSynthesis.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert!(result
		.simulation
		.get_buff(Buff::InnerQuiet)
		.is_some_and(|b| b.stacks == 3));

	Ok(())
}

#[test]
fn test_low_level() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe_lvl(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::BasicTouch.into(),
			actions::ByregotsBlessing.into(),
			actions::CarefulSynthesis.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[3].added_progression, 685);
	assert_eq!(result.simulation.steps[0].added_quality, 2451);
	assert_eq!(result.simulation.steps[1].added_quality, 980);
	assert_eq!(result.simulation.steps[2].added_quality, 1699);

	Ok(())
}

#[test]
fn test_innovation() -> Result<()> {
	// generateRecipe(517, 2000, 5200, 121, 105)
	let recipe = generate_recipe_rlvl(3864, 80, 517, 80, 2000, 5200, 121, 105);
	// generateStats(80, 2763, 2780, 545)
	let stats = generate_stats(80, 2763, 2780, 545);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::DelicateSynthesis.into(),
			actions::DelicateSynthesis.into(),
			actions::WasteNot.into(),
			actions::Groundwork.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::MastersMend.into(),
			actions::PreparatoryTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[0].added_quality, 897);
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
	let recipe = generate_recipe_rlvl(3864, 80, 517, 80, 2000, 5200, 121, 105);
	// generateStats(80, 1645, 1532, 400)
	let stats = generate_stats(80, 1645, 1532, 400);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.quality, 828);

	// generateStarRecipe(580, 3900, 10920, 130, 115, 80, 70)
	let recipe = generate_star_recipe(580, 3900, 10920, 130, 115, 80, 70);
	// generateStats(90, 3289, 3420, 400)
	let stats = generate_stats(90, 3289, 3420, 400);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::MuscleMemory.into(),
			actions::Veneration.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::Observe.into(),
			actions::Observe.into(),
			actions::CarefulSynthesis.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[0].added_progression, 609);
	assert_eq!(result.simulation.progression, 3897);

	Ok(())
}

#[test]
fn test_dawntrail_flooring() -> Result<()> {
	// generateRecipe(685, 6300, 11400, 167, 147)
	let recipe = generate_recipe_rlvl(3864, 80, 685, 80, 6300, 11400, 167, 147);
	// generateStats(94, 3957, 3896, 563)
	let stats = generate_stats(94, 3957, 3896, 563);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::PrudentTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.quality, 2610);

	// generateRecipe(685, 6300, 11400, 167, 147),
	let recipe = generate_recipe_rlvl(3864, 80, 685, 80, 6300, 11400, 167, 147);

	// generateStats(100, 4045, 3902, 601)
	let stats = generate_stats(100, 4045, 3902, 601);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::PrudentTouch.into(),
			actions::GreatStrides.into(),
			actions::PreparatoryTouch.into(),
			actions::GreatStrides.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::ImmaculateMend.into(),
			actions::GreatStrides.into(),
			actions::ByregotsBlessing.into(),
			actions::WasteNot.into(),
			actions::Veneration.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::Veneration.into(),
			actions::Groundwork.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.quality, 11400);
	assert_eq!(result.simulation.progression, 6585);

	Ok(())
}

#[test]
fn test_combo_refinedtouch_with_basictouch() -> Result<()> {
	// generateRecipe(517, 1000, 5200, 121, 105)
	let recipe = generate_recipe_lvl(3864, 81, 80, 1000, 5200, 121, 105);
	// generateStats(100, 2763, 2780, 545)
	let stats = generate_stats(100, 2763, 2780, 545);

	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.actions(vec![
			actions::BasicTouch.into(),
			actions::RefinedTouch.into(),
		])
		.crafter_stats(stats.clone())
		.build();

	let result = sim.start().linear(true).run();
	assert!(result
		.simulation
		.get_buff(Buff::InnerQuiet)
		.is_some_and(|b| b.stacks == 3));

	Ok(())
}

#[test]
fn test_combo_advancedtouch_with_observe() -> Result<()> {
	// generateRecipe(517, 1000, 5200, 121, 105)
	let recipe = generate_recipe_lvl(3864, 81, 80, 1000, 5200, 121, 105);
	// generateStats(90, 2763, 2780, 545)
	let stats = generate_stats(90, 2763, 2780, 545);

	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.actions(vec![actions::Observe.into(), actions::AdvancedTouch.into()])
		.crafter_stats(stats.clone())
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[1].cp_difference, -18);

	Ok(())
}

#[test]
fn test_advanced_touch_combo() -> Result<()> {
	// generateRecipe(517, 1000, 5200, 121, 105)
	let recipe = generate_recipe_lvl(3864, 81, 80, 1000, 5200, 121, 105);
	// generateStats(90, 2763, 2780, 545)
	let stats = generate_stats(90, 2763, 2780, 545);
	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.actions(vec![
			actions::StandardTouch.into(),
			actions::AdvancedTouch.into(),
		])
		.crafter_stats(stats.clone())
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[1].cp_difference, -46);

	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::BasicTouch.into(),
			actions::StandardTouch.into(),
			actions::AdvancedTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[1].cp_difference, -18);

	Ok(())
}

#[test]
fn test_level_90_accuracy() -> Result<()> {
	// generateStarRecipe(560, 1000, 5200, 130, 115, 90, 80)
	let recipe = generate_star_recipe(560, 1000, 5200, 130, 115, 90, 80);
	// generateStats(90, 2659, 2803, 548)
	let stats = generate_stats(90, 2659, 2803, 548);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::BasicSynthesis.into(),
			actions::BasicTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[0].added_quality, 666);
	assert_eq!(result.simulation.steps[1].added_progression, 222);
	assert_eq!(result.simulation.steps[2].added_quality, 266);

	Ok(())
}

#[test]
fn test_innovation_great_strides_interaction() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe_lvl(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 532)
	let stats = generate_stats(80, 2278, 2348, 532);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::Innovation.into(),
			actions::GreatStrides.into(),
			actions::BasicTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[0].added_quality, 2451);
	assert_eq!(result.simulation.steps[3].added_quality, 2451);

	Ok(())
}

#[test]
fn test_lv80_2star_craft() -> Result<()> {
	// generateStarRecipe(450, 2050, 9000, 110, 90, 80, 70)
	let recipe = generate_star_recipe(56450, 2050, 9000, 110, 90, 80, 70);
	// generateStats(80, 2626, 2477, 522)
	let stats = generate_stats(80, 2626, 2477, 522);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::BasicSynthesis.into(),
			actions::BasicTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[0].added_progression, 230);
	assert_eq!(result.simulation.steps[1].added_quality, 217);

	Ok(())
}

#[test]
fn test_high_byregots_stacks() -> Result<()> {
	// generateRecipe(16, 31, 866, 50, 30)
	let recipe = generate_recipe_lvl(3864, 16, 80, 31, 866, 50, 30);
	// generateStats(80, 2278, 2348, 10000)
	let stats = generate_stats(80, 2278, 2348, 10000);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::Reflect.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::MastersMend.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::MastersMend.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
			actions::ByregotsBlessing.into(),
			actions::CarefulSynthesis.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.simulation.success.is_some_and(|x| x));
	assert_eq!(result.simulation.steps[11].added_quality, 4902);

	Ok(())
}

#[test]
fn test_pliant_step_state_reducing_cp_cost() -> Result<()> {
	// generateStarRecipe(480, 4943, 32328, 2480, 2195, 80, 70, true)
	let recipe = Craft {
		expert: Some(true),
		..generate_star_recipe(480, 4943, 32328, 2480, 2195, 80, 70)
	};
	// generateStats(80, 2800, 2500, 541)
	let stats = generate_stats(80, 2800, 2500, 541);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![actions::PrudentTouch.into()])
		.crafter_stats(stats)
		.step_states(vec![StepState::Pliant])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.available_cp, 541 - 13);

	Ok(())
}

#[test]
fn test_pliant_step_state_reducing_cp_cost_two() -> Result<()> {
	// generateStarRecipe(480, 4943, 32328, 2480, 2195, 80, 70, true)
	let recipe = Craft {
		expert: Some(true),
		..generate_star_recipe(480, 4943, 32328, 2480, 2195, 80, 70)
	};
	// generateStats(80, 2800, 2500, 541)
	let stats = generate_stats(80, 2800, 2500, 541);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![actions::MuscleMemory.into(), actions::WasteNot.into()])
		.crafter_stats(stats)
		.step_states(vec![StepState::Normal, StepState::Pliant])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.available_cp, 541 - 6 - (56 / 2));

	Ok(())
}

#[test]
fn test_sturdy_step_state_reducing_durability_cost() -> Result<()> {
	// generateStarRecipe(480, 4943, 32328, 2480, 2195, 80, 70, true)
	let recipe = Craft {
		expert: Some(true),
		..generate_star_recipe(480, 4943, 32328, 2480, 2195, 80, 70)
	};
	// generateStats(80, 2800, 2500, 541)
	let stats = generate_stats(80, 2800, 2500, 541);
	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.actions(vec![actions::PrudentTouch.into()])
		.crafter_stats(stats.clone())
		.step_states(vec![StepState::Sturdy])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.durability, 70 - 3);

	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::WasteNot.into(),
			actions::CarefulSynthesis.into(),
		])
		.crafter_stats(stats)
		.step_states(vec![StepState::Normal, StepState::Sturdy])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.durability, 70 - 3);

	Ok(())
}

#[test]
fn test_not_tick_buffs_if_buff_set_to_fail() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90);
	// generateStats(80, 2800, 2500, 541)
	let stats = generate_stats(80, 2800, 2500, 541);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::GreatStrides.into(),
			actions::TricksOfTheTrade.into(),
		])
		.crafter_stats(stats)
		.step_states(vec![StepState::Normal])
		.fails(vec![1])
		.build();

	let result = sim.start().linear(true).run();
	assert!(result
		.simulation
		.get_buff(Buff::GreatStrides)
		.is_some_and(|b| b.duration == 3));

	Ok(())
}

#[test]
fn test_not_ticking_buffs_with_certain_abilities() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90);
	// generateStats(80, 2486, 2318, 613)
	let stats = generate_stats(80, 2486, 2318, 613);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::GreatStrides.into(),
			actions::FinalAppraisal.into(),
			actions::CarefulObservation.into(),
			actions::RemoveFinalAppraisal.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert!(result
		.simulation
		.get_buff(Buff::GreatStrides)
		.is_some_and(|b| b.duration == 3));

	Ok(())
}

#[test]
fn test_5point4_standard_touch_combo_bonus() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90);
	// generateStats(80, 2486, 2318, 613)
	let stats = generate_stats(80, 2486, 2318, 613);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::BasicTouch.into(),
			actions::StandardTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[1].cp_difference, -18);

	Ok(())
}

#[test]
fn test_count_buffs_properly_in_step_by_step_mode() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90);
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let sim = Simulation::builder()
		.recipe(recipe)
		.actions(vec![
			actions::MuscleMemory.into(),
			actions::Manipulation.into(),
			actions::Observe.into(),
			actions::Veneration.into(),
			actions::Groundwork.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
		])
		.crafter_stats(stats)
		.build();

	let result = sim.start().linear(true).max_steps(4).run();
	assert!(result
		.simulation
		.get_buff(Buff::Manipulation)
		.is_some_and(|b| b.duration == 6));

	Ok(())
}

#[test]
fn test_conditions_for_normal_recipe() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90);
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.build();

	assert_eq!(
		sim.possible_conditions(),
		&HashSet::from([
			StepState::Normal,
			StepState::Good,
			StepState::Excellent,
			StepState::Poor,
		])
	);

	Ok(())
}

#[test]
fn test_conditions_switch() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90, 995)
	let recipe = Craft {
		conditions_flag: 995,
		..generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90)
	};
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let actions: Vec<CraftingActionEnum> = vec![actions::Observe.into(), actions::Observe.into()];

	let mut excellent_test = Simulation::builder()
		.recipe(recipe.clone())
		.crafter_stats(stats.clone())
		.actions(actions.clone())
		.build();
	excellent_test.override_state(StepState::Excellent);
	excellent_test.tick_state();
	assert_eq!(excellent_test.state(), StepState::Poor);

	let mut good_omen_test = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(actions)
		.build();
	good_omen_test.override_state(StepState::GoodOmen);
	good_omen_test.tick_state();
	assert_eq!(good_omen_test.state(), StepState::Good);

	Ok(())
}

#[test]
fn test_expert_one_conditions() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90, 115)
	let recipe = Craft {
		conditions_flag: 115,
		..generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90)
	};
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.build();

	assert_eq!(
		sim.possible_conditions(),
		&HashSet::from([
			StepState::Normal,
			StepState::Good,
			StepState::Centered,
			StepState::Sturdy,
			StepState::Pliant,
		])
	);

	Ok(())
}

#[test]
fn test_expert_two_conditions() -> Result<()> {
	// generateRecipe(480, 6178, 36208, 110, 90, 483)
	let recipe = Craft {
		conditions_flag: 483,
		..generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90)
	};
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.build();

	assert_eq!(
		sim.possible_conditions(),
		&HashSet::from([
			StepState::Normal,
			StepState::Good,
			StepState::Sturdy,
			StepState::Pliant,
			StepState::Malleable,
			StepState::Primed,
		])
	);

	Ok(())
}

#[test]
fn test_expert_two_condition_rates() -> Result<()> {
	println!("testing expert 2 recipe condition rates");
	// generateRecipe(480, 6178, 36208, 110, 90, 483)
	let recipe = Craft {
		expert: Some(true),
		conditions_flag: 483,
		..generate_recipe_rlvl(3864, 80, 480, 80, 6178, 36208, 110, 90)
	};
	// generateStats(80, 2745, 2885, 626)
	let stats = generate_stats(80, 2745, 2885, 626);
	let mut sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.build();

	let mut condition_rates: HashMap<_, _> =
		HashMap::from_iter(sim.possible_conditions().iter().map(|&cond| (cond, 0)));
	let num_samples = 100_000;
	for _ in 0..num_samples {
		sim.tick_state();
		condition_rates
			.entry(sim.state())
			.and_modify(|val| *val += 1);
	}
	println!("rates: {:#?}", condition_rates);

	let expected_rates = HashMap::from([
		(StepState::Normal, 0.37),
		(StepState::Good, 0.12),
		(StepState::Sturdy, 0.15),
		(StepState::Pliant, 0.12),
		(StepState::Malleable, 0.12),
		(StepState::Primed, 0.12),
	]);
	let num_samples = num_samples as f64;
	let deviation = (num_samples * 0.005) as u64; // allow .5% random deviation
	expected_rates.into_iter().for_each(|(state, rate)| {
		let ideal_count = (rate * num_samples) as u64;
		let acceptable_range = (ideal_count - deviation)..(ideal_count + deviation);
		println!("range for {:?}: {:?}", state, acceptable_range);
		assert!(acceptable_range.contains(condition_rates.get(&state).unwrap()));
	});

	println!("finished expert 2 recipe condition rates");
	Ok(())
}

#[test]
fn test_heart_and_soul() -> Result<()> {
	// generateRecipe(480, 900, 36208, 110, 90)
	let recipe = generate_recipe_rlvl(3864, 80, 480, 80, 900, 36208, 110, 90);
	// generateStats(90, 2745, 2885, 500)
	let stats = CrafterStats {
		specialist: true,
		..generate_stats(90, 2745, 2885, 500)
	};
	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.actions(vec![
			actions::Observe.into(),
			actions::HeartAndSoul.into(),
			actions::PreciseTouch.into(),
		])
		.crafter_stats(stats.clone())
		.step_states(vec![
			StepState::Normal,
			StepState::Normal,
			StepState::Normal,
		])
		.build();

	let result = sim.start().run();
	assert!(result.simulation.quality > 0);

	Ok(())
}

#[test]
fn test_progress_flooring() -> Result<()> {
	// generateRecipe(535, 3000, 6700, 125, 109)
	let recipe = generate_recipe_rlvl(3864, 80, 535, 80, 3000, 6700, 125, 109);
	// generateStats(90, 2606, 2457, 507)
	let stats = generate_stats(90, 2606, 2457, 507);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![actions::CarefulSynthesis.into()])
		.build();

	let result = sim.start().run();
	assert_eq!(result.simulation.progression, 378);

	Ok(())
}

#[test]
fn test_quality_buff_flooring() -> Result<()> {
	// generateRecipe(285, 980, 3420, 88, 68)
	let recipe = generate_recipe_rlvl(3864, 80, 285, 80, 980, 3420, 88, 68);
	// generateStats(66, 813, 683, 283)
	let stats = generate_stats(66, 813, 683, 283);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::Innovation.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.quality, 667);

	Ok(())
}

#[test]
fn test_quality_flooring() -> Result<()> {
	// generateRecipe(145, 3000, 6700, 68, 48)
	let recipe = generate_recipe_rlvl(3864, 80, 145, 80, 3000, 6700, 68, 48);
	// generateStats(58, 2606, 434, 507)
	let stats = generate_stats(58, 2606, 434, 507);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::Innovation.into(),
			actions::BasicTouch.into(),
			actions::StandardTouch.into(),
			actions::BasicTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[3].added_quality, 225);

	// generateStarRecipe(610, 5060, 12628, 130, 115, 80, 70)
	let recipe = generate_star_recipe(610, 5060, 12628, 130, 115, 80, 70);
	// generateStats(90, 3702, 3792, 588)
	let stats = generate_stats(90, 3702, 3792, 588);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::MuscleMemory.into(),
			actions::Manipulation.into(),
			actions::Veneration.into(),
			actions::WasteNotII.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::DelicateSynthesis.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[8].added_quality, 663);

	// generateStarRecipe(625, 5280, 13050, 130, 115, 80, 70)
	let recipe = generate_star_recipe(625, 5280, 13050, 130, 115, 80, 70);
	// generateStats(90, 3702, 4073, 588)
	let stats = generate_stats(90, 3702, 4073, 588);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::Reflect.into(),
			actions::Innovation.into(),
			actions::BasicTouch.into(),
			actions::StandardTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.steps[3].added_quality, 663);

	Ok(())
}

#[test]
fn test_required_quality_unmet_fails() -> Result<()> {
	// generateStarRecipe(590, 4300, 12800, 130, 115, 80, 70, false, 15, { requiredQuality: 12800 })
	let recipe = Craft {
		required_quality: Some(12800),
		..generate_star_recipe(590, 4300, 12800, 130, 115, 80, 15)
	};
	// generateStats(90, 3392, 3338, 675)
	let stats = generate_stats(90, 3392, 3338, 675);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::MuscleMemory.into(),
			actions::Manipulation.into(),
			actions::Veneration.into(),
			actions::WasteNotII.into(),
			actions::FinalAppraisal.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::CarefulSynthesis.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::Innovation.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::Observe.into(),
			actions::AdvancedTouch.into(),
			actions::Innovation.into(),
			actions::TrainedFinesse.into(),
			actions::TrainedFinesse.into(),
			actions::GreatStrides.into(),
			actions::ByregotsBlessing.into(),
			actions::BasicSynthesis.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert!(!result.success);

	// generateStarRecipe(590, 4300, 12800, 130, 115, 80, 70, false, 15, { requiredQuality: 6400 })
	let recipe = Craft {
		required_quality: Some(6400),
		..generate_star_recipe(590, 4300, 12800, 130, 115, 80, 15)
	};
	// generateStats(90, 3392, 3338, 675)
	let stats = generate_stats(90, 3392, 3338, 675);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::MuscleMemory.into(),
			actions::Manipulation.into(),
			actions::Veneration.into(),
			actions::WasteNotII.into(),
			actions::FinalAppraisal.into(),
			actions::Groundwork.into(),
			actions::Groundwork.into(),
			actions::CarefulSynthesis.into(),
			actions::Innovation.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::PreparatoryTouch.into(),
			actions::Innovation.into(),
			actions::PrudentTouch.into(),
			actions::PrudentTouch.into(),
			actions::Observe.into(),
			actions::AdvancedTouch.into(),
			actions::Innovation.into(),
			actions::TrainedFinesse.into(),
			actions::TrainedFinesse.into(),
			actions::GreatStrides.into(),
			actions::ByregotsBlessing.into(),
			actions::BasicSynthesis.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert!(!result.success);

	Ok(())
}

#[test]
fn test_tricksofthetrade_and_heartandsoul() -> Result<()> {
	// generateStarRecipe(590, 4300, 12800, 130, 115, 80, 70, false, 15)
	let recipe = generate_star_recipe(590, 4300, 12800, 130, 115, 80, 15);
	// generateStats(90, 3392, 3338, 675)
	let stats = CrafterStats {
		specialist: true,
		..generate_stats(90, 3392, 3338, 675)
	};
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::HeartAndSoul.into(),
			actions::PreparatoryTouch.into(),
			actions::TricksOfTheTrade.into(),
		])
		.step_states(vec![StepState::None, StepState::None, StepState::Good])
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.simulation.get_buff(Buff::HeartAndSoul).is_some());

	// generateStarRecipe(590, 4300, 12800, 130, 115, 80, 70, false, 15)
	let recipe = generate_star_recipe(590, 4300, 12800, 130, 115, 80, 15);
	// generateStats(90, 500, 500, 675)
	let stats = generate_stats(90, 3392, 3338, 675);
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::HeartAndSoul.into(),
			actions::PreparatoryTouch.into(),
			actions::TricksOfTheTrade.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.simulation.get_buff(Buff::HeartAndSoul).is_none());

	Ok(())
}

// should calculate min stats
/*
const simulation = new Simulation(
	  generateRecipe(525, 1300, 6200, 123, 107, 15, { durability: 40 }),
	  [
		new Reflect(),
		new Groundwork(),
		new MastersMend(),
		new Manipulation(),
		new WasteNot(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new ByregotsBlessing(),
		new BasicSynthesis(),
	  ],
	  generateStats(90, 4021, 3600, 500)
	);
	const stats = simulation.getMinStats();
	expect(stats.found).toBe(true);
	expect(stats.craftsmanship).toBe(3309);
	expect(stats.control).toBe(2793);
	expect(stats.cp).toBe(448);
 */

// should correctly identify tier thresholds for min stats
/*
const simulation = new Simulation(
	  generateRecipe(560, 3500, 7200, 130, 115, 15, { progressModifier: 90, qualityModifier: 80 }),
	  [
		new MuscleMemory(),
		new WasteNotII(),
		new Groundwork(),
		new DelicateSynthesis(),
		new Innovation(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new PreparatoryTouch(),
		new ByregotsBlessing(),
		new CarefulSynthesis(),
	  ],
	  generateStats(90, 4021, 3600, 601)
	);
	const stats = simulation.getMinStats([3960, 5400, 6840]);
	expect(stats.found).toBe(true);
	expect(stats.craftsmanship).toBe(3875);
	expect(stats.control).toBe(2962);
	expect(stats.cp).toBe(363);
 */

#[test]
fn test_enhanced_good_modifier_with_splendorous_tools() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(90, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(90, 4041, 3987, 616)
	};
	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![actions::Observe.into(), actions::BasicTouch.into()])
		.step_states(vec![StepState::None, StepState::Good])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.simulation.quality, 2387);

	Ok(())
}

#[test]
fn test_inner_quiet_below_level_11() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(10, 10, 10, 20),
	let stats = generate_stats(10, 10, 10, 20);

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![actions::BasicTouch.into()])
		.step_states(vec![StepState::Normal])
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.simulation.get_buff(Buff::InnerQuiet).is_none());

	Ok(())
}

#[test]
fn test_trained_perfection() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(100, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(100, 4041, 3987, 616)
	};

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::TrainedPerfection.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.steps[1].solidity_difference, 0);
	assert_eq!(
		result.steps[2].solidity_difference,
		-(actions::BasicTouch.get_durability_cost(&result.simulation) as i32)
	);

	Ok(())
}

#[test]
fn test_trained_perfection_with_moves_between() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(100, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(100, 4041, 3987, 616)
	};

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::TrainedPerfection.into(),
			actions::Innovation.into(),
			actions::BasicTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.steps[2].solidity_difference, 0);

	Ok(())
}

#[test]
fn test_trained_perfection_buff_consumed() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(100, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(100, 4041, 3987, 616)
	};

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::TrainedPerfection.into(),
			actions::Innovation.into(),
			actions::BasicTouch.into(),
			actions::BasicTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert_eq!(result.steps[2].solidity_difference, 0);
	assert!(!result.simulation.has_buff(Buff::TrainedPerfection));

	Ok(())
}

#[test]
fn test_hasty_touch_only_after_daring_touch() -> Result<()> {
	// generateRecipe(1, 9, 80, 50, 30)
	let recipe = generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30);
	// generateStats(100, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(100, 4041, 3987, 616)
	};

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::HastyTouch.into(),
			actions::DaringTouch.into(),
			actions::DaringTouch.into(),
		])
		.build();

	let result = sim.start().linear(true).run();
	assert!(result.steps[1]
		.success
		.is_some_and(|step_success| step_success));
	assert!(result.steps[2].success.is_none());

	Ok(())
}

#[test]
fn test_groundwork_efficiency_with_trained_perfection() -> Result<()> {
	// { ...generateRecipe(1, 9, 80, 50, 30), durability: 10 }
	let recipe = Craft {
		durability: 10,
		..generate_recipe_lvl(3864, 1, 80, 9, 80, 50, 30)
	};
	// generateStats(100, 4041, 3987, 616, true)
	let stats = CrafterStats {
		splendorous: true,
		..generate_stats(100, 4041, 3987, 616)
	};

	let sim = Simulation::builder()
		.recipe(recipe.clone())
		.crafter_stats(stats.clone())
		.actions(vec![actions::Groundwork.into()])
		.build();
	let result1 = sim.start().linear(true).run();

	let sim = Simulation::builder()
		.recipe(recipe)
		.crafter_stats(stats)
		.actions(vec![
			actions::TrainedPerfection.into(),
			actions::Groundwork.into(),
		])
		.build();
	let result2 = sim.start().linear(true).run();

	assert!(result1.steps[0].added_progression < result2.steps[1].added_progression);

	Ok(())
}

fn generate_recipe_lvl(
	id: u32,
	lvl: u8,
	durability: u32,
	progress: u32,
	quality: u32,
	progress_divider: u32,
	quality_divider: u32,
) -> Craft {
	generate_recipe_rlvl(
		id,
		lvl,
		tables::level_to_ilevel(CraftingLevel::unchecked_new(lvl)),
		durability,
		progress,
		quality,
		progress_divider,
		quality_divider,
	)
}

fn generate_recipe_rlvl(
	id: u32,
	lvl: u8,
	rlvl: u32,
	durability: u32,
	progress: u32,
	quality: u32,
	progress_divider: u32,
	quality_divider: u32,
) -> Craft {
	Craft {
		id: id.to_string(),
		job: 14, // CRP
		lvl: CraftingLevel::unchecked_new(lvl),
		rlvl,
		durability,
		progress,
		quality,
		progress_divider,
		quality_divider,
		hq: Some(true),
		quick_synth: Some(true),
		conditions_flag: 15,
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
		lvl: CraftingLevel::unchecked_new(80),
		rlvl,
		durability: 70,
		quality,
		progress,
		hq: Some(true),
		quick_synth: Some(false),
		expert: Some(false),
		conditions_flag: 15,
		progress_divider,
		quality_divider,
		progress_modifier: Some(progress_modifier as f64),
		quality_modifier: Some(quality_modifier as f64),
		..Default::default()
	}
}

fn generate_stats(level: u8, craftsmanship: u32, control: u32, cp: u32) -> CrafterStats {
	CrafterStats {
		job_id: 14,
		craftsmanship,
		control,
		cp,
		specialist: false,
		splendorous: false,
		level: CraftingLevel::unchecked_new(level),
		levels: CrafterLevels {
			crp: CraftingLevel::unchecked_new(level),
			bsm: CraftingLevel::unchecked_new(level),
			arm: CraftingLevel::unchecked_new(level),
			gsm: CraftingLevel::unchecked_new(level),
			ltw: CraftingLevel::unchecked_new(level),
			wvr: CraftingLevel::unchecked_new(level),
			alc: CraftingLevel::unchecked_new(level),
			cul: CraftingLevel::unchecked_new(level),
		},
	}
}
