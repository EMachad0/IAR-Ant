//! Fixed timestep implementation
//! Based on iyes_loopless fixed timestep

#![allow(dead_code)]

use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use std::time::Duration;
use bevy_inspector_egui::Inspectable;

use crate::timestep::diagnostic::{TimeStepDiagnosticsPlugin, TimeStepDiagnosticsState};

/// If you modify the step value, the fixed timestep driver stage will
/// reconfigure itself to respect it. Your new timestep duration will be
/// used starting from the next update cycle.
#[derive(Debug, Inspectable)]
pub struct FixedTimestepConfig {
    pub step: Duration,
}

impl FixedTimestepConfig {
    pub fn new(step: Duration) -> Self {
        Self { step }
    }
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
    stages: Vec<Box<dyn Stage>>,
}

impl FixedTimestepStage {
    /// Helper to create a `FixedTimestepStage` with a single child stage
    pub fn from_stage<S: Stage>(timestep: Duration, stage: S) -> Self {
        Self::new(timestep).with_stage(stage)
    }

    /// Create a new empty `FixedTimestepStage` with no child stages
    pub fn new(timestep: Duration) -> Self {
        Self {
            step: timestep,
            accumulator: Duration::default(),
            stages: Vec::new(),
        }
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

    /// Create a new empty `FixedTimestepStage` with no child stages
    /// Duration is set to zero
    /// Useful when using [`FixedTimestepConfig`]
    pub fn empty() -> Self {
        Self::new(Duration::ZERO)
    }
}

impl Stage for FixedTimestepStage {
    fn run(&mut self, world: &mut World) {
        if let Some(config) = world.get_resource::<FixedTimestepConfig>() {
            // update our actual step duration, in case the user has
            // modified it in the info resource
            self.step = config.step;
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
            for stage in self.stages.iter_mut() {
                {
                    let cell = world.cell();
                    if let Some(mut diagnostics) = cell.get_resource_mut::<Diagnostics>() {
                        if let Some(mut state) = cell.get_resource_mut::<TimeStepDiagnosticsState>()
                        {
                            diagnostics.add_measurement(
                                TimeStepDiagnosticsPlugin::STEP_COUNT,
                                || {
                                    state.update_count = state.update_count.wrapping_add(1);
                                    state.update_count as f64
                                },
                            );
                        }
                        diagnostics.add_measurement(TimeStepDiagnosticsPlugin::STEP_TIME, || {
                            self.step.as_secs_f64()
                        });
                        diagnostics.add_measurement(TimeStepDiagnosticsPlugin::ACCUMULATOR, || {
                            self.accumulator.as_secs_f64()
                        });
                        if self.step > Duration::ZERO {
                            diagnostics.add_measurement(TimeStepDiagnosticsPlugin::SPS, || {
                                1.0 / self.step.as_secs_f64()
                            });
                            diagnostics
                                .add_measurement(TimeStepDiagnosticsPlugin::OVERSTEP, || {
                                    self.accumulator.as_secs_f64() / self.step.as_secs_f64()
                                });
                        }
                    };
                }

                stage.run(world);
            }
        }
    }
}
