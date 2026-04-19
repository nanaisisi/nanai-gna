use gna_rs::gna_api::inference_api::*;
use gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint;
use gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint::HwTotalCycles;
use gna_rs::gna_api::instrumentation_api::Gna2InstrumentationPoint::HwStallCycles;

#[test]
fn load_test_and_compute_usage() {
    // configure a request to collect total and stall cycles
    let mut cfg = Gna2RequestConfigCreate();
    Gna2RequestConfigSetInstrumentationPoints(&mut cfg, &[HwTotalCycles, HwStallCycles]);

    // enqueue a batch of requests
    let mut total_active = 0u128;
    let mut total_total = 0u128;

    for _ in 0..10u32 {
        let id = Gna2RequestEnqueue(&cfg);
        let ok = Gna2RequestWait(id, 1000);
        assert!(ok);
        if let Some(results) = Gna2RequestGetInstrumentationResults(id) {
            // results is [total, stall]
            assert_eq!(results.len(), 2);
            let total = results[0] as u128;
            let stall = results[1] as u128;
            let active = total.saturating_sub(stall);
            total_active += active;
            total_total += total;
        }
    }

    // compute usage across requests
    assert!(total_total > 0);
    let usage = (total_active as f64) / (total_total as f64);
    // Our simulation returns total=1000, stall=200 -> usage=0.8
    assert!((usage - 0.8).abs() < 1e-9);
}