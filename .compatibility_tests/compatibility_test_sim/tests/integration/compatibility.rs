// Copyright © 2023-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use roqoqo::operations::*;

use test_roqoqo_1_2::backends::EvaluatingBackend;

use qoqo_calculator::CalculatorFloat;

use roqoqo_quest::Backend;

use ndarray::{array, Array1};
use num_complex::{Complex, Complex64};
use test_case::test_case;

// helper function
fn is_close(a: Complex64, b: Complex64) -> bool {
    (a - b).norm() < 1e-10
}

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
    let mut final_matrix: Array1<Complex64> = array![c0, c0, c0, c0, c0, c0, c0, c0];
    final_matrix[value] = c1;
    let final_matrix_ccz = unitary_ccz.dot(&final_matrix);
    let final_matrix_ccps = unitary_ccps.dot(&final_matrix);

    // Serializing .circuit()
    let ccz_circuit = ccz.circuit();
    let ccps_circuit = ccps.circuit();
    let json_ccz_circuit = serde_json::to_string(&ccz_circuit).unwrap();
    let json_ccps_circuit = serde_json::to_string(&ccps_circuit).unwrap();

    // Preparing initial vector
    let mut initial_state = test_roqoqo_1_2::Circuit::new();
    if iteration.0 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(2);
    }
    if iteration.1 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(1);
    }
    if iteration.2 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(0);
    }

    // Building initial vector -> .circuit() -> PragmaGetStateVector
    let mut circuit_ccz_fromj = test_roqoqo_1_2::Circuit::new();
    circuit_ccz_fromj += initial_state.clone();
    circuit_ccz_fromj +=
        serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_ccz_circuit).unwrap();
    circuit_ccz_fromj +=
        test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_ccz_fromj +=
        test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);
    let mut circuit_ccps_fromj = test_roqoqo_1_2::Circuit::new();
    circuit_ccps_fromj += initial_state.clone();
    circuit_ccps_fromj +=
        serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_ccps_circuit).unwrap();
    circuit_ccps_fromj +=
        test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_ccps_fromj +=
        test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);

    // Get results from circuits()
    let backend = Backend::new(3);
    let (_, _, result_ccz) = backend.run_circuit(&circuit_ccz_fromj).unwrap();
    let (_, _, result_ccps) = backend.run_circuit(&circuit_ccps_fromj).unwrap();

    for (el1, el2) in result_ccz["out"][0]
        .iter()
        .zip(final_matrix_ccz.iter().cloned().collect::<Vec<Complex64>>())
    {
        assert!(is_close(*el1, el2));
    }
    for (el1, el2) in result_ccps["out"][0].iter().zip(
        final_matrix_ccps
            .iter()
            .cloned()
            .collect::<Vec<Complex64>>(),
    ) {
        assert!(is_close(*el1, el2));
    }
}

#[test_case((false, false, false), 0)]
#[test_case((false, false, true), 1)]
#[test_case((false, true, false), 2)]
#[test_case((false, true, true), 3)]
#[test_case((true, false, false), 4)]
#[test_case((true, false, true), 5)]
#[test_case((true, true, false), 6)]
#[test_case((true, true, true), 7)]
fn test_circuits_mq(iteration: (bool, bool, bool), value: usize) {
    let mqzz = MultiQubitZZ::new([0, 1, 2].to_vec(), CalculatorFloat::from(0.3));
    let mqms = MultiQubitMS::new([0, 1, 2].to_vec(), CalculatorFloat::from(0.3));
    let unitary_mqzz = mqzz.unitary_matrix().unwrap();
    let unitary_mqms = mqms.unitary_matrix().unwrap();

    // Preparing matrices for later simulation
    let c0: Complex64 = Complex::new(0.0, 0.0);
    let c1: Complex64 = Complex::new(1.0, 0.0);
    let mut final_matrix: Array1<Complex64> = array![c0, c0, c0, c0, c0, c0, c0, c0];
    final_matrix[value] = c1;
    let final_matrix_mqzz = unitary_mqzz.dot(&final_matrix);
    let final_matrix_mqms = unitary_mqms.dot(&final_matrix);

    // Serializing .circuit()

    let mqzz_circuit = mqzz.circuit();
    let mqms_circuit = mqms.circuit();
    let json_mqzz_circuit = serde_json::to_string(&mqzz_circuit).unwrap();
    let json_mqms_circuit = serde_json::to_string(&mqms_circuit).unwrap();

    // Preparing initial ve<ctor
    let mut initial_state = test_roqoqo_1_2::Circuit::new();
    if iteration.0 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(2);
    }
    if iteration.1 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(1);
    }
    if iteration.2 {
        initial_state += test_roqoqo_1_2::operations::PauliX::new(0);
    }

    // Building initial vector -> .circuit() -> PragmaGetStateVector
    let mut circuit_mqzz_fromj = test_roqoqo_1_2::Circuit::new();
    circuit_mqzz_fromj += initial_state.clone();
    circuit_mqzz_fromj +=
        serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_mqzz_circuit).unwrap();
    circuit_mqzz_fromj +=
        test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_mqzz_fromj +=
        test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);
    let mut circuit_mqms_fromj = test_roqoqo_1_2::Circuit::new();
    circuit_mqms_fromj += initial_state.clone();
    circuit_mqms_fromj +=
        serde_json::from_str::<test_roqoqo_1_2::Circuit>(&json_mqms_circuit).unwrap();
    circuit_mqms_fromj +=
        test_roqoqo_1_2::operations::DefinitionComplex::new("out".to_string(), 3, true);
    circuit_mqms_fromj +=
        test_roqoqo_1_2::operations::PragmaGetStateVector::new("out".to_string(), None);

    // Get results from circuits()
    let backend = Backend::new(3);
    let (_, _, result_mqzz) = backend.run_circuit(&circuit_mqzz_fromj).unwrap();
    let (_, _, result_mqms) = backend.run_circuit(&circuit_mqms_fromj).unwrap();


    for (el1, el2) in result_mqzz["out"][0].iter().zip(
        final_matrix_mqzz
            .iter()
            .cloned()
            .collect::<Vec<Complex64>>(),
    ) {
        assert!(is_close(*el1, el2));
    }
    for (el1, el2) in result_mqms["out"][0].iter().zip(
        final_matrix_mqms
            .iter()
            .cloned()
            .collect::<Vec<Complex64>>(),
    ) {
        assert!(is_close(*el1, el2));
    }
}
