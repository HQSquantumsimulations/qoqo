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

use test_case::test_case;

// Helper functions
fn new_circuitdag(py: Python) -> &PyCell<CircuitDagWrapper> {
    let circuitdag_type = py.get_type::<CircuitDagWrapper>();
    circuitdag_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitDagWrapper>>()
        .unwrap()
}

#[test]
fn test_default() {
    let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (operation.clone(),)).unwrap();
        let circuitdag_wrapper = dag.extract::<CircuitDagWrapper>();

        //assert_ne!(CircuitDagWrapper::default(), circuitdag_wrapper.unwrap());
        //assert_eq!(CircuitDagWrapper::default(), CircuitDagWrapper::new());
    })
}

#[test]
fn test_add_to_back() {}

#[test]
fn test_add_to_front() {}

#[test]
fn test_get() {}
