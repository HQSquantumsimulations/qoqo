// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use roqoqo::prelude::*;
use roqoqo::operations::*;

use test_roqoqo_1_2;
use test_roqoqo_1_2::backends::EvaluatingBackend;

use qoqo_calculator::CalculatorFloat;

use roqoqo_quest::Backend;

use serde_json;
use test_case::test_case;
use num_complex::{Complex, Complex64};
use ndarray::{array, Array2};


#[test_case((false, false, false), 0)]
#[test_case((false, false, true), 1)]
#[test_case((false, true, false), 2)]
#[test_case((false, true, true), 3)]
#[test_case((true, false, false), 4)]
#[test_case((true, false, true), 5)]
#[test_case((true, true, false), 6)]
#[test_case((true, true, true), 7)]
fn test_circuits_3q(iteration: (bool, bool, bool), value: usize) {
    let ccz = ControlledControlledPauliZ::new(0, 1, 2);
    let ccps = ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.3));
    let unitary_ccz = ccz.unitary_matrix().unwrap();
    let unitary_ccps = ccps.unitary_matrix().unwrap();

    // Preparing matrices for later simulation
    let c0: Complex64 = Complex::new(0.0, 0.0);
    let c1: Complex64 = Complex::new(1.0, 0.0);
    let empty: Array2<Complex64> = array![
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
        [c0, c0, c0, c0, c0, c0, c0, c0],
    ];
    let mut final_matrix: Array2<Complex64> = empty.clone();
    for val in 0..8 {
        if value == val {
            final_matrix[[val, val]] = c1;
        }
    }
    let final_matrix_ccz = final_matrix.dot(&unitary_ccz);
    let final_matrix_ccps = final_matrix.dot(&unitary_ccps);
    
    // Serializing .circuit()
    let ccz_circuit = ccz.circuit();
    let ccps_circuit = ccps.circuit();
    let json_ccz_circuit = serde_json::to_string(&ccz_circuit).unwrap();
    let json_ccps_circuit = serde_json::to_string(&ccps_circuit).unwrap();

    // Preparing initial vector
    let mut initial_state = test_roqoqo_1_2::Circuit::new();
    if iteration.2 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(2);
    }
    if iteration.1 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(1);
    }
    if iteration.0 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(0);
    }

    // Building initial vector -> .circuit() -> PragmaGetStateVector
    let mut circuit_ccz_fromj =  test_roqoqo_1_2::Circuit::new();
    circuit_ccz_fromj += initial_state.clone();
    circuit_ccz_fromj += serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_ccz_circuit).unwrap();
    circuit_ccz_fromj += test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_ccz_fromj += test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);
    let mut circuit_ccps_fromj = test_roqoqo_1_2::Circuit::new();
    circuit_ccps_fromj += initial_state.clone();
    circuit_ccps_fromj += serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_ccps_circuit).unwrap();
    circuit_ccps_fromj += test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_ccps_fromj += test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);

    // Get results from circuits()
    let backend = Backend::new(3);
    let (_, _, result_ccz) = backend.run_circuit(&circuit_ccz_fromj).unwrap();
    let (_, _, result_ccps) = backend.run_circuit(&circuit_ccps_fromj).unwrap();

    assert_eq!(result_ccz["out"][0], final_matrix_ccz.iter().cloned().collect::<Vec<Complex64>>());
    assert_eq!(result_ccps["out"][0], final_matrix_ccps.iter().cloned().collect::<Vec<Complex64>>());
}