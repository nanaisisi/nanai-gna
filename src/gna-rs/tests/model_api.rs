use gna_rs::gna_api::model_api::*;

#[test]
fn create_add_compile_model() {
    let mut m = Gna2ModelCreate();
    // add a simple copy operation
    let mut op = Gna2Operation::default();
    op.op_type = gna_rs::gna_api::types::OperationType::Copy;
    op.number_of_operands = 2;
    let idx = Gna2ModelAddOperation(&mut m, op);
    assert_eq!(idx, 0);
    assert_eq!(m.operation_count(), 1);
    let compiled = Gna2ModelCompile(&m);
    assert!(compiled.id() > 0);
}
