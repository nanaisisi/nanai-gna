use nanai_gna::gna_rs::gna_api::inference_api::*;
use nanai_gna::gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint;

#[test]
fn request_state_transitions() {
    let cfg = gna2_request_config_create();
    let request_id = gna2_request_enqueue(&cfg);

    assert_eq!(
        gna2_request_get_state(request_id),
        Some(Gna2RequestState::Pending)
    );
    assert!(!gna2_request_is_in_flight(request_id));

    let handle = std::thread::spawn(move || {
        let ok = gna2_request_wait(request_id, 1000);
        assert!(ok);
    });

    // Poll until the request completes or the timeout expires.
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
    while std::time::Instant::now() < deadline {
        let state = gna2_request_get_state(request_id);
        if state == Some(Gna2RequestState::Completed) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
    assert_eq!(
        gna2_request_get_state(request_id),
        Some(Gna2RequestState::Completed)
    );
    assert!(!gna2_request_is_in_flight(request_id));
}

#[test]
fn request_config_acceleration_mode_roundtrip() {
    let mut cfg = gna2_request_config_create();
    assert_eq!(
        gna2_request_config_get_acceleration_mode(&cfg),
        Gna2AccelerationMode::Auto
    );

    gna2_request_config_set_acceleration_mode(&mut cfg, Gna2AccelerationMode::Hardware);
    assert_eq!(
        gna2_request_config_get_acceleration_mode(&cfg),
        Gna2AccelerationMode::Hardware
    );
}

#[test]
fn request_simulation_uses_acceleration_mode() {
    let mut cfg = gna2_request_config_create();
    gna2_request_config_set_instrumentation_points(
        &mut cfg,
        &[
            Gna2InstrumentationPoint::HwTotalCycles,
            Gna2InstrumentationPoint::HwStallCycles,
        ],
    );
    gna2_request_config_set_acceleration_mode(&mut cfg, Gna2AccelerationMode::Hardware);

    let request_id = gna2_request_enqueue(&cfg);
    assert!(gna2_request_wait(request_id, 1000));

    let results = gna2_request_get_instrumentation_results(request_id).unwrap();
    assert_eq!(results, vec![800u64, 80u64]);
}
