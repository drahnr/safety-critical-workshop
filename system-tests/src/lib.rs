use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use embsinth::{connection::Connection, probe::AttachedProbe};

/// Serial number of the debug probe attached to the RAD device
const RAD_PROBE_SER_NR: &str = "001050203466";
/// Probe ID of the debug probe attached to the RAD device
const RAD_PROBE_ID: u16 = 0x1060;
/// Serial number of the debug probe attached to the SIM device
const SIM_PROBE_SER_NR: &str = "001050263183";
/// Probe ID of the debug probe attached to the SIM device
const SIM_PROBE_ID: u16 = 0x1060;

pub static WORKSPACE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let crate_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Tests must be run via cargo"));
    let workspace_dir = crate_dir.parent().expect("Parent path must exist");
    workspace_dir.to_path_buf()
});

pub static BINARY_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| WORKSPACE_DIR.join("target/thumbv7em-none-eabihf/debug"));

pub const RAD_BINARY_NAME: &str = "rad";
pub const CHIP: &str = "nRF52840_xxAA";

pub fn attach_rad_probe() -> AttachedProbe {
    embsinth::probe::ProbeId::with_serial_nr(0x1366, RAD_PROBE_ID, RAD_PROBE_SER_NR)
        .attach_under_reset(CHIP)
        .expect("Failed to attach to rad target")
}

pub fn init(sim_path: &Path) -> (Connection, Connection) {
    let rad_probe = attach_rad_probe();
    let sim_probe_id =
        embsinth::probe::ProbeId::with_serial_nr(0x1366, SIM_PROBE_ID, SIM_PROBE_SER_NR);

    {
        let sim_reset = sim_probe_id
            .clone()
            .attach_under_reset(CHIP)
            .expect("Failed to attach to sim target");
        let _ = sim_reset
            .flash_and_connect(BINARY_DIR.join("reset"))
            .expect("Failed reseting sim target");
    }

    let sim_probe = sim_probe_id
        .attach_under_reset(CHIP)
        .expect("Failed to attach to sim target");

    let rad_connection = rad_probe
        .flash_once_and_connect(BINARY_DIR.join(RAD_BINARY_NAME))
        .expect("Failed to flash rad binary");
    let sim_connection = sim_probe
        .flash_and_connect(sim_path)
        .expect("Failed flashing to sim target");

    (rad_connection, sim_connection)
}
