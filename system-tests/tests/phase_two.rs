use std::time::Duration;

use mantra_macros::assert_req;
use system_tests::{BINARY_DIR, init};

#[cfg(feature = "phase-two")]
#[mantra_macros::req_verified("rad.sw.limit-radiation")]
#[embsinth::test]
fn radiation_limit() {
    let (mut rad_connection, mut sim_connection) = init(&BINARY_DIR.join("limit_radiation"));

    assert_req!("rad.sw.operation.start" =>
        rad_connection
            .search_msg_for(Duration::from_secs(10), |msg| {
                msg.message.starts_with("Start requested")
            })
            .is_some(),
        "No start was requested"
    );

    assert_req!("rad.sw.operation.pre-condition" =>
        rad_connection
            .search_msg_for(Duration::from_secs(5), |msg| {
                msg.message.starts_with("Switching into 'operation' mode")
            })
            .is_some(),
        "Rad did not switch into 'operation' mode"
    );

    assert!(
        rad_connection
            .search_msg_for(Duration::from_secs(15), |msg| {
                msg.message.starts_with("Radiation intensity exceeded")
            })
            .is_some(),
        "Radiation intensity was not exceeded after 15s"
    );

    assert!(
        rad_connection
            .search_msg_for(Duration::from_secs(10), |msg| {
                msg.message
                    .starts_with("Radiation again below restart limit")
            })
            .is_some(),
        "Radiation intensity did not fall below restart limit after 10s"
    );

    assert!(
        sim_connection
            .search_msg_for(Duration::from_secs(5), |msg| {
                msg.message
                    .starts_with("RAD limit-radiation done -> exiting")
            })
            .is_some(),
        "Limit radiation simulation did not succeed"
    );
}
