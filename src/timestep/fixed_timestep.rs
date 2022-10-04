//! Fixed timestep implementation
//! Based on iyes_loopless fixed timestep

#![allow(dead_code)]

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::time::Duration;

/// If you modify the step value, the fixed timestep driver stage will
/// reconfigure itself to respect it. Your new timestep duration will be
/// used starting from the next update cycle.
#[derive(Debug, Default, Inspectable)]
pub struct FixedTimestepConfig {
    pub step: Option<Duration>,
    pub accumulator: Option<Duration>,
}

/// This type will be available as a resource, while a fixed timestep stage
/// runs, to provide info about the current status of the fixed timestep.
///
/// If you modify the step value, the fixed timestep driver stage will
/// reconfigure itself to respect it. Your new timestep duration will be
/// used starting from the next update cycle.
#[derive(Debug, Inspectable)]
pub struct FixedTimestepInfo {
    pub step: Duration,
    pub accumulator: Duration,
}

/// A Stage that runs a number of child stages with a fixed timestep
///
/// You can set the timestep duration. Every frame update, the time delta
/// will be accumulated, and the child stages will run when it goes over
/// the timestep threshold. If multiple timesteps have been accumulated,
/// the child stages will be run multiple times.
///
/// You can add multiple child stages, allowing you to use `Commands` in
/// your fixed timestep systems, and have their effects applied.
///
/// A good place to add the `FixedTimestepStage` is usually before
/// `CoreStage::Update`.
pub struct FixedTimestepStage {
    step: Duration,
    accumulator: Duration,
    pub stages: Vec<Box<dyn Stage>>,
}

impl FixedTimestepStage {
    /// Helper to create a `FixedTimestepStage` with a single child stage
    pub fn from_stage<S: Stage>(timestep: Duration, stage: S) -> Self {
        Self::new(timestep).with_stage(stage)
    }

    /// Create a new empty `FixedTimestepStage` with no child stages
    pub fn empty(timestep: Duration) -> Self {
        Self {
            step: timestep,
            accumulator: Duration::default(),
            stages: Vec::new(),
        }
    }

    /// Create a new `FixedTimestepStage`
    pub fn new(timestep: Duration) -> Self {
        Self::empty(timestep)
            .with_stage(SystemStage::parallel())
            .with_stage(SystemStage::parallel())
            .with_stage(SystemStage::parallel())
    }

    /// Add a child stage
    pub fn add_stage<S: Stage>(&mut self, stage: S) {
        self.stages.push(Box::new(stage));
    }

    /// Builder method for adding a child stage
    pub fn with_stage<S: Stage>(mut self, stage: S) -> Self {
        self.add_stage(stage);
        self
    }

    pub fn get_system_stage(&mut self, idx: usize) -> &mut SystemStage {
        self.stages[idx]
            .downcast_mut::<SystemStage>()
            .expect("Its is not a system stage")
    }
}

impl Stage for FixedTimestepStage {
    fn run(&mut self, world: &mut World) {
        if let Some(config) = world.get_resource::<FixedTimestepConfig>() {
            // update our actual timestep configuration, in case the user has modified it
            if let Some(step) = config.step {
                self.step = step;
            }
            if let Some(accumulator) = config.accumulator {
                self.accumulator = accumulator;
            }

            world.remove_resource::<FixedTimestepConfig>();
        }

        self.accumulator += {
            let time = world.get_resource::<Time>();
            if let Some(time) = time {
                time.delta()
            } else {
                return;
            }
        };

        while self.accumulator >= self.step {
            self.accumulator -= self.step;

            world.insert_resource(FixedTimestepInfo {
                step: self.step,
                accumulator: self.accumulator,
            });

            for stage in self.stages.iter_mut() {
                stage.run(world);
            }
        }
    }
}
