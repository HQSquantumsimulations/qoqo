use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};

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

    // (1) Test tags functionval
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

#[test]
fn analog_spins() {
    let pp1 = PauliProduct::new().z(0).x(2);
    let pp2 = PauliProduct::new().y(3).z(0);
    let mut hamlitonian = SpinHamiltonian::new();
    hamlitonian.add_operator_product(pp1, (1.0).into()).unwrap();
    hamlitonian.add_operator_product(pp2, (1.0).into()).unwrap();
    let time = CalculatorFloat::from(1.0);

    let analog = ApplyConstantSpinHamiltonian::new(hamlitonian, time);

    assert_eq!(analog.spin(), vec![0, 2, 3]);
}

#[test]
fn operate_analog_timedependent_spin() {
    let pp = PauliProduct::new().z(0);
    let mut unparam_hamiltonian = SpinHamiltonian::new();
    unparam_hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let mut param_hamlitonian = SpinHamiltonian::new();
    param_hamlitonian
        .add_operator_product(pp, "parametrized".into())
        .unwrap();
    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    let name = "ApplyTimeDependentSpinHamiltonian";
    let unparam_analog =
        ApplyTimeDependentSpinHamiltonian::new(unparam_hamiltonian, vec![1.0], values.clone());
    let param_analog =
        ApplyTimeDependentSpinHamiltonian::new(param_hamlitonian, vec![1.0], values.clone());

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
