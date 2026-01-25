use gna_rs::gna_api::inference_api::*;
use gna_rs::gna_api::memory_api::*;
use gna_rs::common::BaseAddress;

#[test]
fn request_enqueue_and_wait_works() {
    let mut cfg = Gna2RequestConfigCreate();
    // set an input buffer
    let ba = BaseAddress::from_ptr(0x1000usize as *mut u8);
    Gna2RequestConfigSetOperandBuffer(&mut cfg, 0, ba);
    let req_id = Gna2RequestEnqueue(&cfg);
    let ok = Gna2RequestWait(req_id, 1000);
    assert!(ok, "Request should complete");
}
