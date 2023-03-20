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


#[test_case((false, false, false))]
#[test_case((false, false, true))]
#[test_case((false, true, false))]
#[test_case((false, true, true))]
#[test_case((true, false, false))]
#[test_case((true, false, true))]
#[test_case((true, true, false))]
#[test_case((true, true, true))]
fn test_circuits_3q(iteration: (bool, bool, bool)) {
    let ccz = ControlledControlledPauliZ::new(0, 1, 2);
    let ccps = ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.3));
    let unitary_ccz = ccz.unitary_matrix();
    let unitary_ccps = ccps.unitary_matrix();
    
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

    // Get results
    let backend = Backend::new(3);
    let (_, _, result_ccz) = backend.run_circuit(&circuit_ccz_fromj).unwrap();
    let (_, _, result_ccps) = backend.run_circuit(&circuit_ccps_fromj).unwrap();

    println!("test");
}