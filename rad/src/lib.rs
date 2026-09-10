#![cfg_attr(not(test), no_std)]

#[cfg(feature = "phase-two")]
use core::time::Duration;

use mantra_macros::req;

#[cfg(feature = "hw")]
use defmt::assert;

use crate::hal::Hal;
use crate::hal::LedIndicator;
use crate::hal::OutputState;

mod hal;
mod solutions;

#[cfg(test)]
mod tests;

/// The *RAD* control structure.
pub struct Rad {
    /// Current mode *RAD* is in
    mode: RadMode,
    start_triggered: bool,
    stop_triggered: bool,
    prev_mode: Option<RadMode>,
    // ----------------------
    // Fields below used to implement exercise solutions.
    // Feel free to ignore and add your own changes above.
    // ----------------------
    #[cfg(feature = "phase-one")]
    invariant_violated: bool,
    #[cfg(feature = "phase-two")]
    rad_start_time: Option<Duration>,
    #[cfg(feature = "phase-two")]
    intensity_limit_reached: bool,
}

impl Rad {
    /// Initialize *RAD*.
    ///
    /// Starts in `idle` mode.
    pub fn init() -> Self {
        #[cfg(feature = "hw")]
        defmt::info!("Starting up *RAD*");

        Self {
            mode: RadMode::Idle,
            start_triggered: false,
            stop_triggered: false,
            prev_mode: None,
            // ----------------------
            // Fields below used to implement exercise solutions.
            // Feel free to ignore and add your own changes above.
            // ----------------------
            #[cfg(feature = "phase-one")]
            invariant_violated: false,
            #[cfg(feature = "phase-two")]
            rad_start_time: None,
            #[cfg(feature = "phase-two")]
            intensity_limit_reached: false,
        }
    }

    pub fn update(&mut self, hal: &mut impl Hal) {
        match self.mode {
            RadMode::Idle => mantra_macros::impl_req!("rad.sw.idle" => {
                assert!(!hal.radiation_active(), "Radiation must not be active in `idle` mode");

                mantra_macros::impl_req!("rad.sw.operation.start" => {
                    if hal.start_requested() && !self.start_triggered {
                        #[cfg(feature = "hw")]
                        defmt::info!("Start requested");
                        self.start_triggered = true;
                    } else if !hal.start_requested() && self.start_triggered {
                        self.start_triggered = false;
                    }

                    if self.start_triggered {
                        match operation_conditions_fulfilled(hal) {
                            Ok(_) => {
                                self.start_triggered = false;
                                self.prev_mode = Some(self.mode);

                                self.mode = RadMode::Operation;

                                #[cfg(feature = "hw")]
                                defmt::info!("Switching into 'operation' mode");
                            },
                            Err(err) => {
                                #[cfg(feature = "hw")]
                                defmt::warn!("Could not start operation due to error: '{}'", err);
                            }
                        }
                    }
                })
            }),
            RadMode::Operation => mantra_macros::impl_req!("rad.sw.operation" => {
                #[cfg(not(any(feature = "phase-one", feature = "phase-two")))]
                self.base_operation(hal);

                // ----------------------
                // Functions below used to implement exercise solutions.
                // Feel free to ignore and add your own changes to `base_operation()`.
                // ----------------------
                #[cfg(all(feature = "phase-one", not(feature = "phase-two")))]
                self.phase_one_operation(hal);
                #[cfg(feature = "phase-two")]
                self.phase_two_operation(hal);
            }),
        }

        self.update_indicators(hal);
    }

    #[req("rad.sw.indicator")]
    fn update_indicators(&self, hal: &mut impl Hal) {
        match self.mode {
            RadMode::Idle => {
                hal.set_mode_indicator(LedIndicator::Off);
            }
            RadMode::Operation => {
                hal.set_mode_indicator(LedIndicator::On);
            }
        }

        if hal.entrance_door_closed() {
            hal.set_entrance_door_indicator(LedIndicator::On);
        } else {
            hal.set_entrance_door_indicator(LedIndicator::Off);
        }

        if hal.safe_environment_confirmed() {
            hal.set_confirmation_switch_indicator(LedIndicator::On);
        } else {
            hal.set_confirmation_switch_indicator(LedIndicator::Off);
        }

        if hal.radiation_output_state() == OutputState::On {
            hal.set_radiation_output_indicator(LedIndicator::On);
        } else {
            hal.set_radiation_output_indicator(LedIndicator::Off);
        }

        #[cfg(feature = "hw-testing")]
        hal.set_start_stop_indicator();
    }

    #[cfg(not(any(feature = "phase-one", feature = "phase-two")))]
    fn base_operation(&mut self, hal: &mut impl Hal) {
        mantra_macros::impl_req!("rad.sw.operation.stop" => {
            if hal.stop_requested() && !self.stop_triggered {
                #[cfg(feature = "hw")]
                defmt::info!("Stop requested");
                self.stop_triggered = true;
            }

            if self.stop_triggered {
                hal.stop_radiation();
            }

            mantra_macros::impl_req!("rad.sw.operation.post-condition" => {
                if self.stop_triggered && !hal.radiation_active() {
                    self.mode = RadMode::Idle;
                    self.prev_mode = Some(RadMode::Operation);
                    self.stop_triggered = false;

                    #[cfg(feature = "hw")]
                    defmt::info!("Switching into 'idle' mode");
                }
            })
        });

        // we entered operation mode => start RAD
        if !self.stop_triggered && self.prev_mode == Some(RadMode::Idle) {
            hal.start_radiation();
        }

        self.prev_mode = Some(RadMode::Operation);
    }
}

#[req("rad.sw.operation.pre-condition")]
fn operation_conditions_fulfilled(hal: &impl Hal) -> Result<(), RadError> {
    if !hal.entrance_door_closed() {
        return Err(RadError::EntranceDoorOpen);
    }

    if !hal.safe_environment_confirmed() {
        return Err(RadError::MissingSafeEnvironmentConfirmation);
    }

    Ok(())
}

/// Possible modes of the *RAD* product.
#[req("rad.mode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadMode {
    /// *RAD* is waiting for input. No radiation is output
    Idle,
    /// *RAD* is performing radiation therapy
    Operation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "hw", derive(defmt::Format))]
pub enum RadError {
    EntranceDoorOpen,
    MissingSafeEnvironmentConfirmation,
}
