use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};

#[test]
fn squeezing_inputs() {
    let op = Squeezing::new(1, 0.1.into(), 0.0.into());
    assert_eq!(op.mode(), &1_usize);
    assert_eq!(op.squeezing(), &CalculatorFloat::from(0.1));
    assert_eq!(op.phase(), &CalculatorFloat::from(0.0));
}

#[test]
fn operate_analog_const_spin() {
    let pp = PauliProduct::new().z(0);
    let mut unparam_hamiltonian = SpinHamiltonian::new();
    unparam_hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let mut param_hamlitonian = SpinHamiltonian::new();
    param_hamlitonian
        .add_operator_product(pp, "parametrized".into())
        .unwrap();
    let time = CalculatorFloat::from(1.0);

    let name = "ApplyConstantSpinHamiltonian";
    let unparam_analog = ApplyConstantSpinHamiltonian::new(unparam_hamiltonian, time.clone());
    let param_analog = ApplyConstantSpinHamiltonian::new(param_hamlitonian, time.clone());

    // (1) Test tags function
    let tags: &[&str; 4] = &["Operation", "ModeGateOperation", "OperateSpinsAnalog", name];

    assert_eq!(unparam_analog.tags(), tags);
    assert_eq!(param_analog.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(unparam_analog.hqslang(), String::from(name));
    assert_eq!(param_analog.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!unparam_analog.is_parametrized());
    assert!(param_analog.is_parametrized());
}
