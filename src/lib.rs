//! # Byregox
//!
//! A simulation library for the crafting system of FINAL FANTASY XIV.
//! Heavily inspired by (mostly just a Rust rewrite of) the simulator created by the
//! ffxiv-teamcraft group, which can be found [here](https://github.com/ffxiv-teamcraft/simulator).
//!
//! ## Basic Usage
//! ```rust
//! use byregox::types::{actions, Simulation};
//! use byregox::types::structs::{Craft, CrafterLevels, CrafterStats, CraftingLevel};
//!
//! let recipe = Craft {
//!     ..Default::default()
//! };
//!
//! let stats = CrafterStats {
//!     craftsmanship: 4936,
//!     control: 4943,
//!     cp: 627,
//!     level: CraftingLevel::max(),
//!     levels: CrafterLevels::max(),
//!     ..Default::default()
//! };
//!
//! let sim = Simulation::builder()
//!     .recipe(recipe)
//!     .actions(vec![
//!         actions::Reflect.into(),
//!         actions::BasicTouch.into(),
//!         actions::ByregotsBlessing.into(),
//!         actions::CarefulSynthesis.into(),
//!     ])
//!     .crafter_stats(stats)
//!     .build();
//!
//! let result = sim.start().safe(true).run();
//! ```

#![forbid(unsafe_code)]
pub mod types;

#[cfg(test)]
mod tests;
