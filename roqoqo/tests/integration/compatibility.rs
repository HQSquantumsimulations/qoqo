// Copyright © 2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

// use bincode;
// use roqoqo;
#[cfg(feature = "compatibility")]
use test_case::test_case;
// use test_roqoqo_1_0;

#[cfg(feature = "compatibility")]
#[test_case(test_roqoqo_1_0::operations::Hadamard::new(0).into(); "Hadamard")]
#[cfg(feature = "compatibility")]
#[test_case(test_roqoqo_1_0::operations::ControlledPhaseShift::new(0,1, 0.1.into()).into(); "ControlledPhase")]
#[cfg(feature = "compatibility")]
#[test_case(test_roqoqo_1_0::operations::VariableMSXX::new(0,1, 0.1.into()).into(); "VariableMSXX")]
#[cfg(feature = "compatibility")]
fn test_bincode_compatibility(operation: test_roqoqo_1_0::operations::Operation) {
    let mut test_circuit = test_roqoqo_1_0::Circuit::new();
    test_circuit += operation;

    let test_measurement_input = test_roqoqo_1_0::measurements::PauliZProductInput::new(3, false);
    let test_measurement = test_roqoqo_1_0::measurements::PauliZProduct {
        constant_circuit: Some(test_circuit.clone()),
        circuits: vec![test_circuit],
        input: test_measurement_input,
    };
    let test_program = test_roqoqo_1_0::QuantumProgram::PauliZProduct {
        measurement: test_measurement,
        input_parameter_names: vec!["test".to_string()],
    };
    let test_serialisation: Vec<u8> = bincode::serialize(&test_program).unwrap();

    let _test_deserialisation: roqoqo::QuantumProgram =
        bincode::deserialize(&test_serialisation).unwrap();
}
