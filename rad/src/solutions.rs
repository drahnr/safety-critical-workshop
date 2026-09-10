use super::RadMode;
use crate::Hal;

impl super::Rad {
    #[cfg(all(feature = "phase-one", not(feature = "phase-two")))]
    pub(super) fn phase_one_operation(&mut self, hal: &mut impl Hal) {
        mantra_macros::impl_req!("rad.sw.operation.stop", "rad.sw.operation.invariant" => {
            if !self.invariant_violated && super::operation_conditions_fulfilled(hal).is_err() {
                #[cfg(feature = "hw")]
                defmt::info!("Operation invariant violated");
                self.invariant_violated = true;
            }

            if hal.stop_requested() && !self.stop_triggered {
                #[cfg(feature = "hw")]
                defmt::info!("Stop requested");
                self.stop_triggered = true;
            }

            let exit_operation = self.stop_triggered || self.invariant_violated;

            if exit_operation {
                hal.stop_radiation();
            }

            mantra_macros::impl_req!("rad.sw.operation.post-condition" => {
                if exit_operation && !hal.radiation_active() {
                    self.mode = RadMode::Idle;
                    self.prev_mode = Some(RadMode::Operation);
                    self.stop_triggered = false;
                    self.invariant_violated = false;

                    #[cfg(feature = "hw")]
                    defmt::info!("Switching into 'idle' mode");
                }
            })
        });

        // we entered operation mode => start RAD
        if !(self.stop_triggered || self.invariant_violated)
            && self.prev_mode == Some(RadMode::Idle)
        {
            hal.start_radiation();
        }

        self.prev_mode = Some(RadMode::Operation);
    }

    #[cfg(feature = "phase-two")]
    pub(super) fn phase_two_operation(&mut self, hal: &mut impl Hal) {
        if hal.radiation_active() && self.rad_start_time.is_none() {
            self.rad_start_time = Some(hal.sys_time());
        }

        mantra_macros::impl_req!("rad.sw.operation.stop", "rad.sw.operation.invariant" => {
            if !self.invariant_violated && super::operation_conditions_fulfilled(hal).is_err() {
                #[cfg(feature = "hw")]
                defmt::info!("Operation invariant violated");
                self.invariant_violated = true;
            }

            if hal.stop_requested() && !self.stop_triggered {
                #[cfg(feature = "hw")]
                defmt::info!("Stop requested");
                self.stop_triggered = true;
            }

            let exit_operation = self.stop_triggered || self.invariant_violated;

            if exit_operation && hal.radiation_active() {
                hal.stop_radiation();
            }

            mantra_macros::impl_req!("rad.sw.operation.post-condition" => {
                if exit_operation && !hal.radiation_active() {
                    self.mode = RadMode::Idle;
                    self.prev_mode = Some(RadMode::Operation);
                    self.stop_triggered = false;
                    self.invariant_violated = false;

                    #[cfg(feature = "hw")]
                    defmt::info!("Switching into 'idle' mode");
                }
            })
        });

        // we entered operation mode => control radiation
        if !(self.stop_triggered || self.invariant_violated) {
            mantra_macros::impl_req!("rad.sw.limit-radiation" => {
                if !self.intensity_limit_reached
                    && self
                        .rad_start_time
                        .map(|start| start.abs_diff(hal.sys_time()).as_secs() > 5)
                        .unwrap_or_default()
                {
                    #[cfg(feature = "hw")]
                    defmt::warn!("Radiation intensity exceeded");

                    hal.stop_radiation();
                    self.intensity_limit_reached = true;
                } else if self.intensity_limit_reached {
                    if hal.radiation_active() {
                        self.rad_start_time = Some(hal.sys_time());
                    } else if self
                        .rad_start_time
                        .map(|start| start.abs_diff(hal.sys_time()).as_secs() > 3)
                        .unwrap_or_default()
                    {
                        self.intensity_limit_reached = false;
                        self.rad_start_time = None;

                        #[cfg(feature = "hw")]
                        defmt::info!("Radiation again below restart limit");
                    }
                }

                if !self.intensity_limit_reached && !hal.radiation_active() {
                    hal.start_radiation();
                }
            });
        }

        self.prev_mode = Some(RadMode::Operation);
    }
}
