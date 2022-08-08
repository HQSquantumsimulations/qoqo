// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::CircuitDagWrapper;

use roqoqo::operations::*;

// Helper functions
fn new_circuitdag(py: Python) -> &PyCell<CircuitDagWrapper> {
    let circuitdag_type = py.get_type::<CircuitDagWrapper>();
    circuitdag_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitDagWrapper>>()
        .unwrap()
}

/// Test default
#[test]
fn test_default() {
    pyo3::prepare_freethreaded_python();
    let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (operation.clone(),))
            .unwrap();
        let circuitdag_wrapper = dag.extract::<CircuitDagWrapper>();

        let helper_ne: bool = CircuitDagWrapper::default() != circuitdag_wrapper.unwrap();
        assert!(helper_ne);
        let helper_eq: bool = CircuitDagWrapper::default() == CircuitDagWrapper::new();
        assert!(helper_eq);
    })
}

/// Test add_to_back and add_to_front
#[test]
fn test_add_to() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_front", (cnot_01.clone(),))
            .unwrap();
        let _circuit_wrapper = dag.extract::<CircuitDagWrapper>();
    })
}

/// Test get
#[test]
fn test_get() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();

        let comp_op = dag.call_method1("get", (0,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

        let helper1 = bool::extract(comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = dag.call_method1("get", (1,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();

        let helper2 = bool::extract(comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(helper2);
    })
}
