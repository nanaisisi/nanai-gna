use gna_rs::gna_api::inference_api::*;

#[test]
fn request_state_transitions() {
    let cfg = Gna2RequestConfigCreate();
    let request_id = Gna2RequestEnqueue(&cfg);

    assert_eq!(Gna2RequestGetState(request_id), Some(Gna2RequestState::Pending));
    assert!(!Gna2RequestIsInFlight(request_id));

    let handle = std::thread::spawn(move || {
        let ok = Gna2RequestWait(request_id, 1000);
        assert!(ok);
    });

    // Wait briefly for the spawned thread to begin processing.
    std::thread::sleep(std::time::Duration::from_millis(2));
    let state = Gna2RequestGetState(request_id);
    assert!(state == Some(Gna2RequestState::InFlight) || state == Some(Gna2RequestState::Completed));

    handle.join().unwrap();
    assert_eq!(Gna2RequestGetState(request_id), Some(Gna2RequestState::Completed));
    assert!(!Gna2RequestIsInFlight(request_id));
}
