use nanai_gna::gna_rs::common::BaseAddress;
use nanai_gna::gna_rs::gna_api::inference_api::*;

#[test]
fn request_enqueue_and_wait_works() {
    let mut cfg = gna2_request_config_create();
    // set an input buffer
    let ba = BaseAddress::from_ptr(0x1000usize as *mut u8);
    gna2_request_config_set_operand_buffer(&mut cfg, 0, ba);
    let req_id = gna2_request_enqueue(&cfg);
    let ok = gna2_request_wait(req_id, 1000);
    assert!(ok, "Request should complete");
}
