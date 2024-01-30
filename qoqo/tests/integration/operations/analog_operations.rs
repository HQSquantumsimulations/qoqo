// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    ApplyConstantSpinHamiltonianWrapper, ApplyTimeDependentSpinHamiltonianWrapper,
};
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};
use struqture_py::spins::SpinHamiltonianSystemWrapper;

fn new_system(py: Python, number_spins: Option<usize>) -> &PyCell<SpinHamiltonianSystemWrapper> {
    let system_type = py.get_type::<SpinHamiltonianSystemWrapper>();
    system_type
        .call1((number_spins,))
        .unwrap()
        .downcast::<PyCell<SpinHamiltonianSystemWrapper>>()
        .unwrap()
}

/// Test new() function for ApplyConstantSpinHamiltonian
#[test]
fn test_new_constantspinhamiltionian() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();

    let input_operation = Operation::from(ApplyConstantSpinHamiltonian::new(
        hamiltonian.clone(),
        (0.1).into(),
    ));
    let method = "__eq__";
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let new_system = new_system(py, number_spins);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system
            .extract::<SpinHamiltonianSystemWrapper>()
            .unwrap();

        let operation_type = py.get_type::<ApplyConstantSpinHamiltonianWrapper>();
        let operation_py = operation_type
            .call1((system_wrapper.clone(), 0.1))
            .unwrap()
            .downcast::<PyCell<ApplyConstantSpinHamiltonianWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((system_wrapper.clone(), 0.0))
            .unwrap()
            .downcast::<PyCell<ApplyConstantSpinHamiltonianWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ApplyConstantSpinHamiltonianWrapper { internal: ApplyConstantSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(0.0) } }"
        );
    })
}

/// Test new() function for ApplyTimeDepenendentSpinHamiltonian
#[test]
fn test_new_timedependentspinhamiltionian() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();

    let times = vec![1.0];
    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    let input_operation = Operation::from(ApplyTimeDependentSpinHamiltonian::new(
        hamiltonian.clone(),
        times.clone(),
        values.clone(),
    ));
    let method = "__eq__";
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let new_system = new_system(py, number_spins);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system
            .extract::<SpinHamiltonianSystemWrapper>()
            .unwrap();

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![1.0]);

        let operation_type = py.get_type::<ApplyTimeDependentSpinHamiltonianWrapper>();
        let operation_py = operation_type
            .call1((system_wrapper.clone(), vec![1.0], values.clone()))
            .unwrap()
            .downcast::<PyCell<ApplyTimeDependentSpinHamiltonianWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![0.1]);

        let def_wrapper = operation_py
            .extract::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((system_wrapper.clone(), vec![0.1], values.clone()))
            .unwrap()
            .downcast::<PyCell<ApplyTimeDependentSpinHamiltonianWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ApplyTimeDependentSpinHamiltonianWrapper { internal: ApplyTimeDependentSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: [0.1], values: {\"omega\": [0.1]} } }"
        );
    })
}
