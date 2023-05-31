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

use std::collections::HashMap;

use test_case::test_case;
use test_roqoqo_1_4;

// 1.0 version
#[test_case(test_roqoqo_1_4::operations::SingleQubitGate::new(0, 1.0.into(), 0.0.into(), 0.0.into(), 0.0.into(), 0.0.into(),).into(); "SingleQubitGate")]
#[test_case(test_roqoqo_1_4::operations::RotateZ::new(0, 0.1.into()).into(); "RotateZ")]
#[test_case(test_roqoqo_1_4::operations::RotateY::new(0, 0.1.into()).into(); "RotateY")]
#[test_case(test_roqoqo_1_4::operations::RotateX::new(0, 0.1.into()).into(); "RotateX")]
#[test_case(test_roqoqo_1_4::operations::RotateXY::new(0,1.0.into(), 0.1.into()).into(); "RotateXY")]
#[test_case(test_roqoqo_1_4::operations::RotateAroundSphericalAxis::new(0, 1.0.into(), 1.0.into(), 1.0.into()).into(); "RotateAroundSphericalAxis")]
#[test_case(test_roqoqo_1_4::operations::PauliZ::new(0).into(); "PauliZ")]
#[test_case(test_roqoqo_1_4::operations::PauliY::new(0).into(); "PauliY")]
#[test_case(test_roqoqo_1_4::operations::PauliX::new(0).into(); "PauliX")]
#[test_case(test_roqoqo_1_4::operations::SqrtPauliX::new(0).into(); "SqrtPauliX")]
#[test_case(test_roqoqo_1_4::operations::InvSqrtPauliX::new(0).into(); "InvSqrtPauliX")]
#[test_case(test_roqoqo_1_4::operations::Hadamard::new(0).into(); "Hadamard")]
#[test_case(test_roqoqo_1_4::operations::TGate::new(0).into(); "TGate")]
#[test_case(test_roqoqo_1_4::operations::SGate::new(0).into(); "SGate")]
#[test_case(test_roqoqo_1_4::operations::DefinitionBit::new("ro".to_string(), 1, false).into(); "DefinitionBit")]
#[test_case(test_roqoqo_1_4::operations::DefinitionComplex::new("ro".to_string(), 1, true).into(); "DefinitionComplex")]
#[test_case(test_roqoqo_1_4::operations::DefinitionUsize::new("ro".to_string(), 1, true).into(); "DefinitionUsize")]
#[test_case(test_roqoqo_1_4::operations::DefinitionFloat::new("ro".to_string(), 1, true).into(); "DefinitionFloat")]
#[test_case(test_roqoqo_1_4::operations::InputSymbolic::new("ro".to_string(), 1.0).into(); "InputSymbolic")]
#[test_case(test_roqoqo_1_4::operations::MeasureQubit::new(0,"ro".to_string(), 1).into(); "MeasureQubit")]
#[test_case(test_roqoqo_1_4::operations::PragmaGetStateVector::new("ro".to_string(), None).into(); "PragmaGetStateVector")]
#[test_case(test_roqoqo_1_4::operations::PragmaGetDensityMatrix::new("ro".to_string(), None).into(); "PragmaGetDensityMatrix")]
#[test_case(test_roqoqo_1_4::operations::PragmaGetOccupationProbability::new("ro".to_string(), None).into(); "PragmaGetOccupationProbability")]
#[test_case(test_roqoqo_1_4::operations::PragmaGetPauliProduct::new(std::collections::HashMap::new(),"ro".to_string(), test_roqoqo_1_4::Circuit::new()).into(); "PragmaGetPauliProduct")]
#[test_case(test_roqoqo_1_4::operations::PragmaRepeatedMeasurement::new("ro".to_string(), 10, None).into(); "PragmaRepeatedMeasurement")]
#[test_case(test_roqoqo_1_4::operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string()).into(); "PragmaSetNumberOfMeasurements")]
#[test_case(test_roqoqo_1_4::operations::PragmaSetStateVector::new(ndarray::array![1.0.into(), 0.0.into(), 0.0.into()]).into(); "PragmaSetStateVector")]
#[test_case(test_roqoqo_1_4::operations::PragmaSetDensityMatrix::new(ndarray::array![[1.0.into(), 0.0.into(), 0.0.into()]]).into(); "PragmaSetDensityMatrix")]
#[test_case(test_roqoqo_1_4::operations::PragmaRepeatGate::new(10).into(); "PragmaRepeatGate")]
#[test_case(test_roqoqo_1_4::operations::PragmaOverrotation::new("RotateZ".to_string(), vec![0], 1.0, 1.0).into(); "PragmaOverrotation")]
#[test_case(test_roqoqo_1_4::operations::PragmaBoostNoise::new(1.0.into()).into(); "PragmaBoostNoise")]
#[test_case(test_roqoqo_1_4::operations::PragmaStopParallelBlock::new(vec![0], 1.0.into()).into(); "PragmaStopParallelBlock")]
#[test_case(test_roqoqo_1_4::operations::PragmaGlobalPhase::new(1.0.into()).into(); "PragmaGlobalPhase")]
#[test_case(test_roqoqo_1_4::operations::PragmaSleep::new(vec![0], 1.0.into()).into(); "PragmaSleep")]
#[test_case(test_roqoqo_1_4::operations::PragmaActiveReset::new(0).into(); "PragmaActiveReset")]
#[test_case(test_roqoqo_1_4::operations::PragmaStartDecompositionBlock::new(vec![0], HashMap::new()).into(); "PragmaStartDecompositionBlock")]
#[test_case(test_roqoqo_1_4::operations::PragmaStopDecompositionBlock::new(vec![0]).into(); "PragmaStopDecompositionBlock")]
#[test_case(test_roqoqo_1_4::operations::PragmaDamping::new(0,1.0.into(), 1.0.into()).into(); "PragmaDamping")]
#[test_case(test_roqoqo_1_4::operations::PragmaDepolarising::new(0,1.0.into(), 1.0.into()).into(); "PragmaDepolarising")]
#[test_case(test_roqoqo_1_4::operations::PragmaDephasing::new(0,1.0.into(), 1.0.into()).into(); "PragmaDephasing")]
#[test_case(test_roqoqo_1_4::operations::PragmaRandomNoise::new(0,1.0.into(), 1.0.into(), 1.0.into()).into(); "PragmaRandomNoise")]
#[test_case(test_roqoqo_1_4::operations::PragmaGeneralNoise::new(0, 0.1.into(), ndarray::array![[1.0.into(), 0.0.into()], [1.0.into(), 2.0.into()]]).into(); "PragmaGeneralNoise")]
#[test_case(test_roqoqo_1_4::operations::PragmaConditional::new("ro".to_string(),0, test_roqoqo_1_4::Circuit::new()).into(); "PragmaConditional")]
// #[test_case(test_roqoqo_1_4::operations::PragmaChangeDevice::new(&"PragmaTest".into()).unwrap().into(); "PragmaChangeDevice")]
#[test_case(test_roqoqo_1_4::operations::CNOT::new(0,1).into(); "CNOT")]
#[test_case(test_roqoqo_1_4::operations::SWAP::new(0,1).into(); "SWAP")]
#[test_case(test_roqoqo_1_4::operations::FSwap::new(0,1).into(); "FSwap")]
#[test_case(test_roqoqo_1_4::operations::ISwap::new(0,1).into(); "ISwap")]
#[test_case(test_roqoqo_1_4::operations::SqrtISwap::new(0,1).into(); "SqrtISWAP")]
#[test_case(test_roqoqo_1_4::operations::InvSqrtISwap::new(0,1).into(); "InvSqrtISWAP")]
#[test_case(test_roqoqo_1_4::operations::XY::new(0,1, 0.1.into()).into(); "XY")]
#[test_case(test_roqoqo_1_4::operations::ControlledPhaseShift::new(0,1, 0.1.into()).into(); "ControlledPhase")]
#[test_case(test_roqoqo_1_4::operations::ControlledPauliY::new(0,1).into(); "ControlledPauliY")]
#[test_case(test_roqoqo_1_4::operations::ControlledPauliZ::new(0,1).into(); "ControlledPauliZ")]
#[test_case(test_roqoqo_1_4::operations::MolmerSorensenXX::new(0,1).into(); "MolmerSorensenXX")]
#[test_case(test_roqoqo_1_4::operations::VariableMSXX::new(0,1, 0.1.into()).into(); "VariableMSXX")]
#[test_case(test_roqoqo_1_4::operations::GivensRotation::new(0,1, 0.1.into(), 0.1.into()).into(); "GivensRotation")]
#[test_case(test_roqoqo_1_4::operations::GivensRotationLittleEndian::new(0,1, 0.1.into(), 0.1.into()).into(); "GivensRotationLittleEndian")]
#[test_case(test_roqoqo_1_4::operations::Qsim::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "Qsim")]
#[test_case(test_roqoqo_1_4::operations::Fsim::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "Fsim")]
#[test_case(test_roqoqo_1_4::operations::SpinInteraction::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "SpinInteraction")]
#[test_case(test_roqoqo_1_4::operations::Bogoliubov::new(0,1, 0.1.into(), 0.1.into()).into(); "Bogoliubov")]
#[test_case(test_roqoqo_1_4::operations::PMInteraction::new(0,1, 0.1.into()).into(); "PMInteraction")]
#[test_case(test_roqoqo_1_4::operations::ComplexPMInteraction::new(0,1, 0.1.into(), 0.1.into()).into(); "ComplexPMInteraction")]
#[test_case(test_roqoqo_1_4::operations::PhaseShiftedControlledZ::new(0,1, 1.0.into()).into(); "PhaseShiftedControlledZ")]
#[test_case(test_roqoqo_1_4::operations::PhaseShiftState1::new(0, 1.0.into()).into(); "PhaseShiftState1")]
#[test_case(test_roqoqo_1_4::operations::PhaseShiftState0::new(0, 1.0.into()).into(); "PhaseShiftState0")]
#[test_case(test_roqoqo_1_4::operations::MultiQubitMS::new(vec![0,2,3], 1.0.into()).into(); "MultiQubitMS")]
#[test_case(test_roqoqo_1_4::operations::MultiQubitZZ::new(vec![0,2,3], 1.0.into()).into(); "MultiQubitZZ")]
// 1.1 and 1.2
#[test_case(test_roqoqo_1_4::operations::InputBit::new("input".to_string(), 1, true).into(); "InputBit")]
#[test_case(test_roqoqo_1_4::operations::PragmaLoop::new(2.0.into(), test_roqoqo_1_4::Circuit::new()).into(); "PragmaLoop")]
#[test_case(test_roqoqo_1_4::operations::PhaseShiftedControlledPhase::new(0,1, 1.0.into(), 1.0.into()).into(); "PhaseShiftedControlledPhase")]
// 1.3
#[test_case(test_roqoqo_1_4::operations::ControlledRotateX::new(0,1, 1.0.into()).into(); "ControlledRotateX")]
#[test_case(test_roqoqo_1_4::operations::ControlledRotateXY::new(0,1, 1.0.into(), 1.0.into()).into(); "ControlledRotateXY")]
#[test_case(test_roqoqo_1_4::operations::ControlledControlledPauliZ::new(0,1,2).into(); "ControlledControlledPauliZ")]
#[test_case(test_roqoqo_1_4::operations::ControlledControlledPhaseShift::new(0,1,2, 1.0.into()).into(); "ControlledControlledPhaseShift")]
#[test_case(test_roqoqo_1_4::operations::Toffoli::new(0,1,2).into(); "Toffoli")]
// 1.4 - add this into the next compatibility crate
#[test_case(test_roqoqo_1_4::operations::GPi::new(0, 0.1.into()).into(); "GPi")]
#[test_case(test_roqoqo_1_4::operations::GPi2::new(0, 0.1.into()).into(); "GPi2")]

fn test_bincode_compatibility_1_4(operation: test_roqoqo_1_4::operations::Operation) {
    let mut test_circuit = test_roqoqo_1_4::Circuit::new();
    test_circuit += operation;

    let test_measurement_input = test_roqoqo_1_4::measurements::PauliZProductInput::new(3, false);
    let test_measurement = test_roqoqo_1_4::measurements::PauliZProduct {
        constant_circuit: Some(test_circuit.clone()),
        circuits: vec![test_circuit],
        input: test_measurement_input,
    };
    let test_program = test_roqoqo_1_4::QuantumProgram::PauliZProduct {
        measurement: test_measurement,
        input_parameter_names: vec!["test".to_string()],
    };
    let test_serialisation: Vec<u8> = bincode::serialize(&test_program).unwrap();

    let _test_deserialisation: roqoqo::QuantumProgram =
        bincode::deserialize(&test_serialisation).unwrap();
}

#[test]
fn test_device_compat() {
    let test_device = test_roqoqo_1_4::devices::AllToAllDevice::new(
        3,
        &["RotateZ".to_string()],
        &["CNOT".to_string()],
        1.0,
    );
    let test_serialisation: Vec<u8> = bincode::serialize(&test_device).unwrap();

    let test_deserialisation: roqoqo::devices::AllToAllDevice =
        bincode::deserialize(&test_serialisation).unwrap();

    let comparsion_device = roqoqo::devices::AllToAllDevice::new(
        3,
        &["RotateZ".to_string()],
        &["CNOT".to_string()],
        1.0,
    );
    assert_eq!(test_deserialisation, comparsion_device);
}
