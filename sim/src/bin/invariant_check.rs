#![no_main]
#![no_std]

use sim::{self as _, RadMode, Sim, UPDATE_DELAY};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut sim = Sim::init();

    sim.wait_update_until(
        |sim| sim.actual_mode() == RadMode::Idle,
        "RAD not in 'idle' after startup delay",
    );

    defmt::info!("Starting RAD operation");
    sim.rad_to_production();

    sim.wait(UPDATE_DELAY);

    defmt::info!("Breaking RAD invariant");
    sim.set_door_sensor(sim::OutputState::Off);
    sim.wait_update();

    // RAD stays in 'operation' until radiation is deactivated
    sim.set_radiation_state(sim::RadiationState::Deactive);
    sim.wait_update();
    sim.wait_update();

    assert_eq!(
        sim.actual_mode(),
        RadMode::Idle,
        "Violating the invariant must move the RAD back to 'idle'"
    );

    defmt::info!("RAD invariant flow done -> exiting");

    nrf_bsp::exit()
}
