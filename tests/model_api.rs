use nanai_gna::gna_rs::gna_api::model_api::*;

#[test]
fn create_add_compile_model() {
    let mut m = gna2_model_create();
    // add a simple copy operation
    let mut op = Gna2Operation::default();
    op.op_type = nanai_gna::gna_rs::gna_api::types::OperationType::Copy;
    op.number_of_operands = 2;
    let idx = gna2_model_add_operation(&mut m, op);
    assert_eq!(idx, 0);
    assert_eq!(m.operation_count(), 1);
    let compiled = gna2_model_compile(&m);
    assert!(compiled.id() > 0);
}
