use gna_rs::gna_api::inference_api::*;
use gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint;

#[test]
fn request_state_transitions() {
    let cfg = Gna2RequestConfigCreate();
    let request_id = Gna2RequestEnqueue(&cfg);

    assert_eq!(
        Gna2RequestGetState(request_id),
        Some(Gna2RequestState::Pending)
    );
    assert!(!Gna2RequestIsInFlight(request_id));

    let handle = std::thread::spawn(move || {
        let ok = Gna2RequestWait(request_id, 1000);
        assert!(ok);
    });

    // Poll until the request completes or the timeout expires.
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
    while std::time::Instant::now() < deadline {
        let state = Gna2RequestGetState(request_id);
        if state == Some(Gna2RequestState::Completed) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
    assert_eq!(
        Gna2RequestGetState(request_id),
        Some(Gna2RequestState::Completed)
    );
    assert!(!Gna2RequestIsInFlight(request_id));
}

#[test]
fn request_config_acceleration_mode_roundtrip() {
    let mut cfg = Gna2RequestConfigCreate();
    assert_eq!(
        Gna2RequestConfigGetAccelerationMode(&cfg),
        Gna2AccelerationMode::Auto
    );

    Gna2RequestConfigSetAccelerationMode(&mut cfg, Gna2AccelerationMode::Hardware);
    assert_eq!(
        Gna2RequestConfigGetAccelerationMode(&cfg),
        Gna2AccelerationMode::Hardware
    );
}

#[test]
fn request_simulation_uses_acceleration_mode() {
    let mut cfg = Gna2RequestConfigCreate();
    Gna2RequestConfigSetInstrumentationPoints(
        &mut cfg,
        &[
            Gna2InstrumentationPoint::HwTotalCycles,
            Gna2InstrumentationPoint::HwStallCycles,
        ],
    );
    Gna2RequestConfigSetAccelerationMode(&mut cfg, Gna2AccelerationMode::Hardware);

    let request_id = Gna2RequestEnqueue(&cfg);
    assert!(Gna2RequestWait(request_id, 1000));

    let results = Gna2RequestGetInstrumentationResults(request_id).unwrap();
    assert_eq!(results, vec![800u64, 80u64]);
}
