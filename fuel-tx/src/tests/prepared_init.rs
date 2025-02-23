use fuel_tx::{
    field::Outputs,
    Executable,
    *,
};
use rand::{
    rngs::StdRng,
    Rng,
    SeedableRng,
};

#[test]
fn output_variable_prepare_init_zeroes_recipient_and_amount() {
    let rng = &mut StdRng::seed_from_u64(8586);

    let variable = Output::variable(rng.gen(), rng.gen(), rng.gen());
    let zeroed = Output::variable(Address::zeroed(), 0, AssetId::zeroed());

    let tx = TransactionBuilder::script(vec![], vec![])
        .prepare_script(false)
        .add_output(variable)
        .finalize();

    let output = tx
        .clone()
        .prepare_init_predicate()
        .outputs()
        .first()
        .cloned()
        .expect("failed to fetch output");

    let output_p = tx
        .outputs()
        .first()
        .cloned()
        .expect("failed to fetch output");

    assert_ne!(zeroed, variable);
    assert_eq!(zeroed, output);
    assert_eq!(variable, output_p);
}
