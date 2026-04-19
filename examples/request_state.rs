use nanai_gna::gna_rs::gna_api::inference_api::*;
use nanai_gna::gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint::{
    HwStallCycles, HwTotalCycles,
};
use std::{thread, time::Duration};

fn main() {
    let mut cfg = gna2_request_config_create();
    gna2_request_config_set_instrumentation_points(&mut cfg, &[HwTotalCycles, HwStallCycles]);
    gna2_request_config_set_acceleration_mode(&mut cfg, Gna2AccelerationMode::Hardware);

    println!(
        "request config acceleration mode: {:?}",
        gna2_request_config_get_acceleration_mode(&cfg)
    );

    let request_id = gna2_request_enqueue(&cfg);
    println!("request {} enqueued", request_id);
    println!("initial state: {:?}", gna2_request_get_state(request_id));

    let waiter = thread::spawn(move || {
        let success = gna2_request_wait(request_id, 1000);
        println!("request {} wait returned: {}", request_id, success);
        success
    });

    loop {
        let state = gna2_request_get_state(request_id);
        println!("polled state: {:?}", state);
        if state == Some(Gna2RequestState::Completed) {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }

    waiter.join().unwrap();
    println!("final in-flight? {}", gna2_request_is_in_flight(request_id));

    match gna2_request_get_instrumentation_results(request_id) {
        Some(results) => println!("instrumentation results: {:?}", results),
        None => println!("no instrumentation results available"),
    }
}
