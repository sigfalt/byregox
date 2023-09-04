pub mod tests {
	use crate::types::{structs::*, *};

	#[test]
	fn test_simulation_construction() -> Result<(), SimulationBuilderError> {
		let recipe = Craft {
			id: "1035".to_string(),
			job: 14,
			rlvl: 15,
			durability: 70,
			quality: 360,
			progress: 55,
			lvl: 15,
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

	fn generate_recipe(
		rlvl: u32,
		progress: u32,
		quality: u32,
		progress_divider: u32,
		quality_divider: u32,
	) -> Craft {
		Craft {
			id: "3864".to_string(),
			job: 14,
			rlvl,
			durability: 80,
			quality: if quality != 0 { quality } else { 20287 },
			progress: if progress != 0 { progress } else { 3943 },
			lvl: 80,
			hq: Some(true),
			quick_synth: Some(true),
			ingredients: vec![],
			conditions_flag: 15,
			progress_divider,
			quality_divider,
			..Default::default()
		}
	}

	fn generate_stats(
		level: u32,
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
			level,
			levels: CrafterLevels {
				crp: level,
				bsm: level,
				arm: level,
				gsm: level,
				ltw: level,
				wvr: level,
				alc: level,
				cul: level,
			},
		}
	}
}