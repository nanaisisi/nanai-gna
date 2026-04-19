use nanai_gna::gna_rs::gna_api::inference_api::*;
use nanai_gna::gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint::{
    HwStallCycles, HwTotalCycles,
};
use std::{thread, time::Duration};

fn main() {
    let mut cfg = Gna2RequestConfigCreate();
    Gna2RequestConfigSetInstrumentationPoints(&mut cfg, &[HwTotalCycles, HwStallCycles]);
    Gna2RequestConfigSetAccelerationMode(&mut cfg, Gna2AccelerationMode::Hardware);

    println!(
        "request config acceleration mode: {:?}",
        Gna2RequestConfigGetAccelerationMode(&cfg)
    );

    let request_id = Gna2RequestEnqueue(&cfg);
    println!("request {} enqueued", request_id);
    println!("initial state: {:?}", Gna2RequestGetState(request_id));

    let waiter = thread::spawn(move || {
        let success = Gna2RequestWait(request_id, 1000);
        println!("request {} wait returned: {}", request_id, success);
        success
    });

    loop {
        let state = Gna2RequestGetState(request_id);
        println!("polled state: {:?}", state);
        if state == Some(Gna2RequestState::Completed) {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }

    waiter.join().unwrap();
    println!("final in-flight? {}", Gna2RequestIsInFlight(request_id));

    match Gna2RequestGetInstrumentationResults(request_id) {
        Some(results) => println!("instrumentation results: {:?}", results),
        None => println!("no instrumentation results available"),
    }
}
