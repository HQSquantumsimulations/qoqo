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

use std::collections::HashMap;

use test_case::test_case;
use test_roqoqo_1_13;

// 1.0 version
#[test_case(test_roqoqo_1_13::operations::SingleQubitGate::new(0, 1.0.into(), 0.0.into(), 0.0.into(), 0.0.into(), 0.0.into(),).into(); "SingleQubitGate")]
#[test_case(test_roqoqo_1_13::operations::RotateZ::new(0, 0.1.into()).into(); "RotateZ")]
#[test_case(test_roqoqo_1_13::operations::RotateY::new(0, 0.1.into()).into(); "RotateY")]
#[test_case(test_roqoqo_1_13::operations::RotateX::new(0, 0.1.into()).into(); "RotateX")]
#[test_case(test_roqoqo_1_13::operations::RotateXY::new(0,1.0.into(), 0.1.into()).into(); "RotateXY")]
#[test_case(test_roqoqo_1_13::operations::RotateAroundSphericalAxis::new(0, 1.0.into(), 1.0.into(), 1.0.into()).into(); "RotateAroundSphericalAxis")]
#[test_case(test_roqoqo_1_13::operations::PauliZ::new(0).into(); "PauliZ")]
#[test_case(test_roqoqo_1_13::operations::PauliY::new(0).into(); "PauliY")]
#[test_case(test_roqoqo_1_13::operations::PauliX::new(0).into(); "PauliX")]
#[test_case(test_roqoqo_1_13::operations::SqrtPauliX::new(0).into(); "SqrtPauliX")]
#[test_case(test_roqoqo_1_13::operations::InvSqrtPauliX::new(0).into(); "InvSqrtPauliX")]
#[test_case(test_roqoqo_1_13::operations::Hadamard::new(0).into(); "Hadamard")]
#[test_case(test_roqoqo_1_13::operations::TGate::new(0).into(); "TGate")]
#[test_case(test_roqoqo_1_13::operations::SGate::new(0).into(); "SGate")]
#[test_case(test_roqoqo_1_13::operations::DefinitionBit::new("ro".to_string(), 1, false).into(); "DefinitionBit")]
#[test_case(test_roqoqo_1_13::operations::DefinitionComplex::new("ro".to_string(), 1, true).into(); "DefinitionComplex")]
#[test_case(test_roqoqo_1_13::operations::DefinitionUsize::new("ro".to_string(), 1, true).into(); "DefinitionUsize")]
#[test_case(test_roqoqo_1_13::operations::DefinitionFloat::new("ro".to_string(), 1, true).into(); "DefinitionFloat")]
#[test_case(test_roqoqo_1_13::operations::InputSymbolic::new("ro".to_string(), 1.0).into(); "InputSymbolic")]
#[test_case(test_roqoqo_1_13::operations::MeasureQubit::new(0,"ro".to_string(), 1).into(); "MeasureQubit")]
#[test_case(test_roqoqo_1_13::operations::PragmaGetStateVector::new("ro".to_string(), None).into(); "PragmaGetStateVector")]
#[test_case(test_roqoqo_1_13::operations::PragmaGetDensityMatrix::new("ro".to_string(), None).into(); "PragmaGetDensityMatrix")]
#[test_case(test_roqoqo_1_13::operations::PragmaGetOccupationProbability::new("ro".to_string(), None).into(); "PragmaGetOccupationProbability")]
#[test_case(test_roqoqo_1_13::operations::PragmaGetPauliProduct::new(std::collections::HashMap::new(),"ro".to_string(), test_roqoqo_1_13::Circuit::new()).into(); "PragmaGetPauliProduct")]
#[test_case(test_roqoqo_1_13::operations::PragmaRepeatedMeasurement::new("ro".to_string(), 10, None).into(); "PragmaRepeatedMeasurement")]
#[test_case(test_roqoqo_1_13::operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string()).into(); "PragmaSetNumberOfMeasurements")]
#[test_case(test_roqoqo_1_13::operations::PragmaSetStateVector::new(ndarray::array![1.0.into(), 0.0.into(), 0.0.into()]).into(); "PragmaSetStateVector")]
#[test_case(test_roqoqo_1_13::operations::PragmaSetDensityMatrix::new(ndarray::array![[1.0.into(), 0.0.into(), 0.0.into()]]).into(); "PragmaSetDensityMatrix")]
#[test_case(test_roqoqo_1_13::operations::PragmaRepeatGate::new(10).into(); "PragmaRepeatGate")]
#[test_case(test_roqoqo_1_13::operations::PragmaOverrotation::new("RotateZ".to_string(), vec![0], 1.0, 1.0).into(); "PragmaOverrotation")]
#[test_case(test_roqoqo_1_13::operations::PragmaBoostNoise::new(1.0.into()).into(); "PragmaBoostNoise")]
#[test_case(test_roqoqo_1_13::operations::PragmaStopParallelBlock::new(vec![0], 1.0.into()).into(); "PragmaStopParallelBlock")]
#[test_case(test_roqoqo_1_13::operations::PragmaGlobalPhase::new(1.0.into()).into(); "PragmaGlobalPhase")]
#[test_case(test_roqoqo_1_13::operations::PragmaSleep::new(vec![0], 1.0.into()).into(); "PragmaSleep")]
#[test_case(test_roqoqo_1_13::operations::PragmaActiveReset::new(0).into(); "PragmaActiveReset")]
#[test_case(test_roqoqo_1_13::operations::PragmaStartDecompositionBlock::new(vec![0], HashMap::new()).into(); "PragmaStartDecompositionBlock")]
#[test_case(test_roqoqo_1_13::operations::PragmaStopDecompositionBlock::new(vec![0]).into(); "PragmaStopDecompositionBlock")]
#[test_case(test_roqoqo_1_13::operations::PragmaDamping::new(0,1.0.into(), 1.0.into()).into(); "PragmaDamping")]
#[test_case(test_roqoqo_1_13::operations::PragmaDepolarising::new(0,1.0.into(), 1.0.into()).into(); "PragmaDepolarising")]
#[test_case(test_roqoqo_1_13::operations::PragmaDephasing::new(0,1.0.into(), 1.0.into()).into(); "PragmaDephasing")]
#[test_case(test_roqoqo_1_13::operations::PragmaRandomNoise::new(0,1.0.into(), 1.0.into(), 1.0.into()).into(); "PragmaRandomNoise")]
#[test_case(test_roqoqo_1_13::operations::PragmaGeneralNoise::new(0, 0.1.into(), ndarray::array![[1.0.into(), 0.0.into()], [1.0.into(), 2.0.into()]]).into(); "PragmaGeneralNoise")]
#[test_case(test_roqoqo_1_13::operations::PragmaConditional::new("ro".to_string(),0, test_roqoqo_1_13::Circuit::new()).into(); "PragmaConditional")]
// #[test_case(test_roqoqo_1_13::operations::PragmaChangeDevice::new(&"PragmaTest".into()).unwrap().into(); "PragmaChangeDevice")]
#[test_case(test_roqoqo_1_13::operations::CNOT::new(0,1).into(); "CNOT")]
#[test_case(test_roqoqo_1_13::operations::SWAP::new(0,1).into(); "SWAP")]
#[test_case(test_roqoqo_1_13::operations::FSwap::new(0,1).into(); "FSwap")]
#[test_case(test_roqoqo_1_13::operations::ISwap::new(0,1).into(); "ISwap")]
#[test_case(test_roqoqo_1_13::operations::SqrtISwap::new(0,1).into(); "SqrtISWAP")]
#[test_case(test_roqoqo_1_13::operations::InvSqrtISwap::new(0,1).into(); "InvSqrtISWAP")]
#[test_case(test_roqoqo_1_13::operations::XY::new(0,1, 0.1.into()).into(); "XY")]
#[test_case(test_roqoqo_1_13::operations::ControlledPhaseShift::new(0,1, 0.1.into()).into(); "ControlledPhase")]
#[test_case(test_roqoqo_1_13::operations::ControlledPauliY::new(0,1).into(); "ControlledPauliY")]
#[test_case(test_roqoqo_1_13::operations::ControlledPauliZ::new(0,1).into(); "ControlledPauliZ")]
#[test_case(test_roqoqo_1_13::operations::MolmerSorensenXX::new(0,1).into(); "MolmerSorensenXX")]
#[test_case(test_roqoqo_1_13::operations::VariableMSXX::new(0,1, 0.1.into()).into(); "VariableMSXX")]
#[test_case(test_roqoqo_1_13::operations::GivensRotation::new(0,1, 0.1.into(), 0.1.into()).into(); "GivensRotation")]
#[test_case(test_roqoqo_1_13::operations::GivensRotationLittleEndian::new(0,1, 0.1.into(), 0.1.into()).into(); "GivensRotationLittleEndian")]
#[test_case(test_roqoqo_1_13::operations::Qsim::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "Qsim")]
#[test_case(test_roqoqo_1_13::operations::Fsim::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "Fsim")]
#[test_case(test_roqoqo_1_13::operations::SpinInteraction::new(0,1, 0.1.into(), 0.1.into(), 0.1.into()).into(); "SpinInteraction")]
#[test_case(test_roqoqo_1_13::operations::Bogoliubov::new(0,1, 0.1.into(), 0.1.into()).into(); "Bogoliubov")]
#[test_case(test_roqoqo_1_13::operations::PMInteraction::new(0,1, 0.1.into()).into(); "PMInteraction")]
#[test_case(test_roqoqo_1_13::operations::ComplexPMInteraction::new(0,1, 0.1.into(), 0.1.into()).into(); "ComplexPMInteraction")]
#[test_case(test_roqoqo_1_13::operations::PhaseShiftedControlledZ::new(0,1, 1.0.into()).into(); "PhaseShiftedControlledZ")]
#[test_case(test_roqoqo_1_13::operations::PhaseShiftState1::new(0, 1.0.into()).into(); "PhaseShiftState1")]
#[test_case(test_roqoqo_1_13::operations::PhaseShiftState0::new(0, 1.0.into()).into(); "PhaseShiftState0")]
#[test_case(test_roqoqo_1_13::operations::MultiQubitMS::new(vec![0,2,3], 1.0.into()).into(); "MultiQubitMS")]
#[test_case(test_roqoqo_1_13::operations::MultiQubitZZ::new(vec![0,2,3], 1.0.into()).into(); "MultiQubitZZ")]
// 1.1 and 1.2
#[test_case(test_roqoqo_1_13::operations::InputBit::new("input".to_string(), 1, true).into(); "InputBit")]
#[test_case(test_roqoqo_1_13::operations::PragmaLoop::new(2.0.into(), test_roqoqo_1_13::Circuit::new()).into(); "PragmaLoop")]
#[test_case(test_roqoqo_1_13::operations::PhaseShiftedControlledPhase::new(0,1, 1.0.into(), 1.0.into()).into(); "PhaseShiftedControlledPhase")]
// 1.3
#[test_case(test_roqoqo_1_13::operations::ControlledRotateX::new(0,1, 1.0.into()).into(); "ControlledRotateX")]
#[test_case(test_roqoqo_1_13::operations::ControlledRotateXY::new(0,1, 1.0.into(), 1.0.into()).into(); "ControlledRotateXY")]
#[test_case(test_roqoqo_1_13::operations::ControlledControlledPauliZ::new(0,1,2).into(); "ControlledControlledPauliZ")]
#[test_case(test_roqoqo_1_13::operations::ControlledControlledPhaseShift::new(0,1,2, 1.0.into()).into(); "ControlledControlledPhaseShift")]
#[test_case(test_roqoqo_1_13::operations::Toffoli::new(0,1,2).into(); "Toffoli")]
// 1.4
#[test_case(test_roqoqo_1_13::operations::GPi::new(0, 0.1.into()).into(); "GPi")]
#[test_case(test_roqoqo_1_13::operations::GPi2::new(0, 0.1.into()).into(); "GPi2")]
// 1.5
#[test_case(test_roqoqo_1_13::operations::PragmaControlledCircuit::new(0, test_roqoqo_1_13::Circuit::new()).into(); "PragmaControlledCircuit")]
// Operations from 1.6
#[test_case(test_roqoqo_1_13::operations::Squeezing::new(0, 0.1.into(), 0.1.into()).into(); "Squeezing")]
#[test_case(test_roqoqo_1_13::operations::PhaseShift::new(0, 0.1.into()).into(); "PhaseShift")]
#[test_case(test_roqoqo_1_13::operations::BeamSplitter::new(0, 1, 0.1.into(), 0.2.into()).into(); "BeamSplitter")]
#[test_case(test_roqoqo_1_13::operations::PhotonDetection::new(0, "ro".into(), 0).into(); "PhotonDetection")]
// Operations from 1.7
#[test_case(test_roqoqo_1_13::operations::Identity::new(0).into(); "Identity")]
// Operations from 1.8
#[test_case(test_roqoqo_1_13::operations::PhaseDisplacement::new(0, 0.1.into(), 0.1.into()).into(); "PhaseDisplacement")]
#[test_case(test_roqoqo_1_13::operations::EchoCrossResonance::new(0, 1).into(); "EchoCrossResonance")]
#[test_case(test_roqoqo_1_13::operations::PragmaAnnotatedOp::new(test_roqoqo_1_13::operations::PauliX::new(0).into(), "test".to_string()).into(); "PragmaAnnotatedOp")]
// Operations from 1.9 - nothing was added
// Operations from 1.10
// QuantumRabi, LongitudinalCoupling, JaynesCummings, SingleExcitationLoad, SingleExcitationStore and CZQubitResonator were all added
// as unstable, but have been added as stable in 1.11
// Operations from 1.11
#[test_case(test_roqoqo_1_13::operations::QuantumRabi::new(0, 1, 0.1.into()).into(); "QuantumRabi")]
#[test_case(test_roqoqo_1_13::operations::LongitudinalCoupling::new(0, 1, 0.1.into()).into(); "LongitudinalCoupling")]
#[test_case(test_roqoqo_1_13::operations::JaynesCummings::new(0, 1, 0.1.into()).into(); "JaynesCummings")]
#[test_case(test_roqoqo_1_13::operations::SingleExcitationLoad::new(0, 1).into(); "SingleExcitationLoad")]
#[test_case(test_roqoqo_1_13::operations::SingleExcitationStore::new(0, 1).into(); "SingleExcitationStore")]
#[test_case(test_roqoqo_1_13::operations::CZQubitResonator::new(0, 1).into(); "CZQubitResonator")]
// Operations from 1.11 - ApplyConstantPauliHamiltonian and ApplyTimeDependentHamiltonian are unstable in 1.11
// #[test_case(create_apply_constant_spin_hamiltonian(); "ApplyConstantPauliHamiltonian")]
// #[test_case(create_apply_timedependent_spin_hamiltonian(); "ApplyTimeDependentHamiltonian")]
// Operations from 1.13 - GateDefinition and CallDefined gate are unstable, uncomment when stable.
// #[test_case(test_roqoqo_1_13::operations::GateDefinition::new(test_roqoqo_1_13::Circuit::new(), "name".into(), vec![0, 1], vec!["param".into()]).into(); "GateDefinition")]
// #[test_case(test_roqoqo_1_13::operations::CallDefinedGate::new("name".into(), vec![0, 1], vec![0.0]).into(); "CallDefinedGate")]
fn test_bincode_compatibility_1_13(operation: test_roqoqo_1_13::operations::Operation) {
    let mut test_circuit = test_roqoqo_1_13::Circuit::new();
    test_circuit += operation;

    let test_measurement_input = test_roqoqo_1_13::measurements::PauliZProductInput::new(3, false);
    let test_measurement = test_roqoqo_1_13::measurements::PauliZProduct {
        constant_circuit: Some(test_circuit.clone()),
        circuits: vec![test_circuit],
        input: test_measurement_input,
    };
    let test_program = test_roqoqo_1_13::QuantumProgram::PauliZProduct {
        measurement: test_measurement,
        input_parameter_names: vec!["test".to_string()],
    };
    let test_serialisation: Vec<u8> = bincode::serialize(&test_program).unwrap();

    let _test_deserialisation: roqoqo::QuantumProgram =
        bincode::deserialize(&test_serialisation).unwrap();
}

#[test]
fn test_device_compat() {
    let test_device = test_roqoqo_1_13::devices::AllToAllDevice::new(
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

// Operations from 1.11 - ApplyConstantPauliHamiltonian and ApplyTimeDependentHamiltonian are unstable in 1.11
// use struqture;
// use struqture::prelude::*;
// fn create_apply_constant_spin_hamiltonian(
// ) -> test_roqoqo_1_13::operations::ApplyConstantPauliHamiltonian {
//     let pp = struqture::spins::PauliProduct::new().z(0);
//     let mut hamiltonian = struqture::spins::PauliHamiltonian::new();
//     hamiltonian
//         .add_operator_product(pp.clone(), 1.0.into())
//         .unwrap();
//     return test_roqoqo_1_13::operations::ApplyConstantPauliHamiltonian::new(
//         hamiltonian,
//         1.0.into(),
//     );
// }

// fn create_apply_timedependent_spin_hamiltonian(
// ) -> test_roqoqo_1_13::operations::ApplyTimeDependentPauliHamiltonian {
//     let pp = struqture::spins::PauliProduct::new().z(0);
//     let mut hamiltonian = struqture::spins::PauliHamiltonian::new();
//     hamiltonian
//         .add_operator_product(pp.clone(), "omega".into())
//         .unwrap();
//     let mut values = HashMap::new();
//     values.insert("omega".to_string(), vec![1.0]);
//     return test_roqoqo_1_13::operations::ApplyTimeDependentPauliHamiltonian::new(
//         hamiltonian,
//         vec![1.0],
//         values.clone(),
//     );
// }
