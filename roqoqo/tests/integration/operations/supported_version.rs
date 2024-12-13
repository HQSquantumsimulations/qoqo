// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//
//! Integration test for supported version trait

use ndarray::array;
use qoqo_calculator::CalculatorFloat;
#[cfg(feature = "circuitdag")]
use roqoqo::measurements::Cheated;
#[cfg(feature = "circuitdag")]
use roqoqo::measurements::CheatedPauliZProduct;
#[cfg(feature = "circuitdag")]
use roqoqo::measurements::ClassicalRegister;
#[cfg(feature = "circuitdag")]
use roqoqo::measurements::PauliZProduct;
use roqoqo::operations;
use roqoqo::prelude::*;
#[cfg(feature = "circuitdag")]
use roqoqo::QuantumProgram;
use std::collections::HashMap;
#[cfg(feature = "unstable_analog_operations")]
use struqture::prelude::*;
#[cfg(feature = "unstable_analog_operations")]
use struqture::spins;
use test_case::test_case;

#[test_case(operations::TwoQubitGateOperation::from(operations::CNOT::new(1,0)); "CNOT")]
#[test_case(operations::TwoQubitGateOperation::from(operations::SWAP::new(1,0)); "SWAP")]
#[test_case(operations::TwoQubitGateOperation::from(operations::FSwap::new(1,0)); "FSwap")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ISwap::new(1,0)); "ISwap")]
#[test_case(operations::TwoQubitGateOperation::from(operations::SqrtISwap::new(1,0)); "SqrtISwap")]
#[test_case(operations::TwoQubitGateOperation::from(operations::InvSqrtISwap::new(1,0)); "InvSqrtISwap")]
#[test_case(operations::TwoQubitGateOperation::from(operations::XY::new(1,0, 1.0.into())); "XY")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ControlledPauliY::new(1,0)); "ControlledPauliY")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ControlledPauliZ::new(1,0)); "ControlledPauliZ")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ControlledPhaseShift::new(1,0, 1.0.into())); "ControlledPhaseShift")]
#[test_case(operations::TwoQubitGateOperation::from(operations::PMInteraction::new(1,0, 1.0.into())); "PMInteraction")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ComplexPMInteraction::new(1,0, 1.0.into(), 2.0.into())); "ComplexPMInteraction")]
#[test_case(operations::TwoQubitGateOperation::from(operations::MolmerSorensenXX::new(1,0,)); "MolmerSorensenXX")]
#[test_case(operations::TwoQubitGateOperation::from(operations::VariableMSXX::new(1,0, 1.0.into())); "VariableMSXX")]
#[test_case(operations::TwoQubitGateOperation::from(operations::GivensRotation::new(1,0, 1.0.into(), 2.0.into())); "GivensRotation")]
#[test_case(operations::TwoQubitGateOperation::from(operations::GivensRotationLittleEndian::new(1,0, 1.0.into(), 2.0.into())); "GivensRotationLittleEndian")]
#[test_case(operations::TwoQubitGateOperation::from(operations::Qsim::new(1,0, 0.5.into(), 1.0.into(), 0.5.into())); "Qsim")]
#[test_case(operations::TwoQubitGateOperation::from(operations::Fsim::new(1,0, 0.5.into(), 1.0.into(), 0.5.into())); "Fsim")]
#[test_case(operations::TwoQubitGateOperation::from(operations::SpinInteraction::new(1,0, 1.0.into(), 2.0.into(), 3.0.into())); "SpinInteraction")]
#[test_case(operations::TwoQubitGateOperation::from(operations::Bogoliubov::new(1,0, 1.0.into(), 2.0.into())); "Bogoliubov")]
#[test_case(operations::TwoQubitGateOperation::from(operations::PhaseShiftedControlledZ::new(1,0, 3.0.into())); "PhaseShifterControlledZ")]
fn test_version_1_0_0_two_qubit_gate(operation: operations::TwoQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 0, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 0, 0));
}

#[test_case(operations::TwoQubitGateOperation::from(operations::PhaseShiftedControlledPhase::new(1,0, 3.0.into(), 2.0.into())); "PhaseShiftedControlledPhase")]
fn test_version_1_2_0_two_qubit_gate(operation: operations::TwoQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 2, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 2, 0));
}

#[test_case(operations::TwoQubitGateOperation::from(operations::ControlledRotateX::new(0, 1, 0.1.into())); "ControlledRotateX")]
#[test_case(operations::TwoQubitGateOperation::from(operations::ControlledRotateXY::new(0, 1, 0.1.into(), 0.2.into())); "ControlledRotateXY")]
fn test_version_1_3_0_two_qubit_gate(operation: operations::TwoQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 3, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 3, 0));
}

#[test_case(operations::TwoQubitGateOperation::from(operations::EchoCrossResonance::new(0, 1)); "EchoCrossResonance")]
fn test_version_1_8_0_two_qubit_gate(operation: operations::TwoQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 8, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 8, 0));
}

#[test_case(operations::ThreeQubitGateOperation::from(operations::ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(operations::ThreeQubitGateOperation::from(operations::ControlledControlledPhaseShift::new(0, 1, 2, 0.1.into())); "ControlledControlledPhaseShift")]
#[test_case(operations::ThreeQubitGateOperation::from(operations::Toffoli::new(0, 1, 2)); "Toffoli")]
fn test_version_1_3_0_three_qubit_gate(operation: operations::ThreeQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 3, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 3, 0));
}

#[test_case(operations::SingleQubitGateOperation::from(operations::Hadamard::new(0)); "Hadamard")]
#[test_case(operations::SingleQubitGateOperation::from(operations::PauliX::new(0));"PauliX")]
#[test_case(operations::SingleQubitGateOperation::from(operations::PauliY::new(0));"PauliY")]
#[test_case(operations::SingleQubitGateOperation::from(operations::PauliZ::new(0));"PauliZ")]
#[test_case(operations::SingleQubitGateOperation::from(operations::RotateX::new(0, 0.0.into()));"RotateX")]
#[test_case(operations::SingleQubitGateOperation::from(operations::RotateY::new(0, 1.0.into()));"RotateY")]
#[test_case(operations::SingleQubitGateOperation::from(operations::RotateZ::new(0, 2.0.into()));"RotateZ")]
#[test_case(operations::SingleQubitGateOperation::from(operations::PhaseShiftState0::new(0, 3.0.into()));"PhaseShiftState0")]
#[test_case(operations::SingleQubitGateOperation::from(operations::PhaseShiftState1::new(0, 4.0.into()));"PhaseShiftState1")]
#[test_case(operations::SingleQubitGateOperation::from(operations::SGate::new(0)); "SGate")]
#[test_case(operations::SingleQubitGateOperation::from(operations::TGate::new(0)); "TGate")]
#[test_case(operations::SingleQubitGateOperation::from(operations::SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(operations::SingleQubitGateOperation::from(operations::InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(operations::SingleQubitGateOperation::from(operations::RotateAroundSphericalAxis::new(0, 1.0.into(), 0.5.into(), 1.0.into())); "RotateAroundSphericalAxis")]
#[test_case(operations::SingleQubitGateOperation::from(operations::SingleQubitGate::new(0,0.5.into(),  0.5.into(), 0.5.into(), 0.5.into(), 0.5.into()));"SingleQubitGate")]
fn test_version_1_0_0_single_qubit_gate(operation: operations::SingleQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 0, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 0, 0));
}

#[test_case(operations::SingleQubitGateOperation::from(operations::GPi::new(0, 0.1.into()));"GPi")]
#[test_case(operations::SingleQubitGateOperation::from(operations::GPi2::new(0, 0.1.into()));"GPi2")]
fn test_version_1_4_0_single_qubit_gate(operation: operations::SingleQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 4, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 4, 0));
}

#[test_case(operations::MultiQubitGateOperation::from(operations::MultiQubitMS::new(vec![0,1,2,3], 1.0.into())); "MultiQubitMS")]
fn test_version_1_0_0_multi_qubit_gate(operation: operations::MultiQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 0, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 0, 0));
}

#[cfg(feature = "unstable_operation_definition")]
#[test_case(operations::MultiQubitOperation::from(operations::CallDefinedGate::new("test".into(), vec![0,1,2,3], vec![CalculatorFloat::Float(1.0)])); "CallDefinedGate")]
fn test_version_1_13_0_multi_qubit_gate(operation: operations::MultiQubitOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 13, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 13, 0));
}

#[test_case(operations::SingleModeGateOperation::from(operations::Squeezing::new(0, 1.0.into(), 0.0.into())); "Squeezing")]
#[test_case(operations::SingleModeGateOperation::from(operations::PhaseShift::new(0, 1.0.into())); "PhaseShift")]
fn test_version_1_6_0_single_mode_gate(operation: operations::SingleModeGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 6, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 6, 0));
}

#[test_case(operations::SingleModeGateOperation::from(operations::PhaseDisplacement::new(0, 1.0.into(), 0.1.into())); "PhaseDisplacement")]
fn test_version_1_8_0_single_mode_gate(operation: operations::SingleModeGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 8, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 8, 0));
}

#[test_case(operations::SingleModeGateOperation::from(operations::QuantumRabi::new(1, 0, 1.0.into()));"QuantumRabi")]
#[test_case(operations::SingleModeGateOperation::from(operations::LongitudinalCoupling::new(1, 0, 1.0.into()));"LongitudinalCoupling")]
#[test_case(operations::SingleModeGateOperation::from(operations::JaynesCummings::new(1, 0, 1.0.into()));"JaynesCummings")]
#[test_case(operations::SingleModeGateOperation::from(operations::SingleExcitationLoad::new(1, 0));"SingleExcitationLoad")]
#[test_case(operations::SingleModeGateOperation::from(operations::SingleExcitationStore::new(1, 0));"SingleExcitationStore")]
#[test_case(operations::SingleModeGateOperation::from(operations::CZQubitResonator::new(1, 0));"CZQubitResonator")]
fn test_version_1_11_0_single_mode_gate(operation: operations::SingleModeGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 11, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 11, 0));
}

#[cfg(feature = "unstable_analog_operations")]
fn create_apply_constant_spin_hamiltonian<T>(p: T) -> operations::ApplyConstantSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = spins::PauliProduct::new().z(0);
    let mut hamiltonian = spins::SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();
    operations::ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into())
}
#[cfg(feature = "unstable_analog_operations")]
fn create_apply_timedependent_spin_hamiltonian<T>(
    p: T,
) -> operations::ApplyTimeDependentSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = spins::PauliProduct::new().z(0);
    let mut hamiltonian = spins::SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    operations::ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

#[cfg(feature = "unstable_analog_operations")]
#[test_case(operations::SpinsAnalogOperation::from(create_apply_constant_spin_hamiltonian(1.0));"ApplyConstantSpinHamiltonian")]
#[test_case(operations::SpinsAnalogOperation::from(create_apply_timedependent_spin_hamiltonian("omega"));"ApplyTimeDependentHamiltonian")]
fn test_version_1_11_0_spin_analog_operations(operation: operations::SpinsAnalogOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 11, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 11, 0));
}

#[test_case(operations::TwoModeGateOperation::from(operations::BeamSplitter::new(0, 1, 0.5.into(), 1.0.into())); "BeamSplitter")]
fn test_version_1_6_0_two_mode_gate(operation: operations::TwoModeGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 6, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 6, 0));
}

#[test_case(operations::SingleModeOperation::from(operations::PhotonDetection::new(0, "ro".into(), 1)); "PhotonDetection")]
fn test_version_1_6_0_measurement_mode_gate(operation: operations::SingleModeOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 6, 0));
    let op = operations::Operation::from(operation);
    assert_eq!(op.minimum_supported_roqoqo_version(), (1, 6, 0));
}

#[test_case(operations::Operation::from(operations::PragmaSetNumberOfMeasurements::new(3, "ro".into())); "PragmaSetNumberOfMeasurements")]
#[test_case(operations::Operation::from(operations::PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(operations::Operation::from(operations::PragmaGeneralNoise::new(0, 1.0.into(),  array![[0.1, 0.0, 0.0],[0.0, 0.0, 0.0],[0.0, 0.0, 0.0]])); "PragmaGeneralNoise")]
#[test_case(operations::Operation::from(operations::PragmaBoostNoise::new(0.5.into())); "PragmaBoostNoise")]
#[test_case(operations::Operation::from(operations::PragmaStopParallelBlock::new(vec![0, 1], 0.5.into())); "PragmaStopParallelBlock")]
#[test_case(operations::Operation::from(operations::PragmaGlobalPhase::new(0.5.into())); "PragmaGlobalPhase")]
#[test_case(operations::Operation::from(operations::PragmaStartDecompositionBlock::new(vec![0, 1], HashMap::new())); "PragmaStartDecompositionBlock")]
#[test_case(operations::Operation::from(operations::PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(operations::Operation::from(operations::DefinitionUsize::new("ro".into(), 2, false)); "DefinitionUsize")]
#[test_case(operations::Operation::from(operations::InputSymbolic::new("ro".into(), 2.0)); "InputSymbolic")]
#[test_case(operations::Operation::from(operations::PragmaDamping::new(0, 0.01.into(),  2.0.into())); "PragmaDamping001")]
#[test_case(operations::Operation::from(operations::PragmaDephasing::new(0, 0.01.into(),  2.0.into())); "PragmaDephasing")]
#[test_case(operations::Operation::from(operations::PragmaGetPauliProduct::new(HashMap::from([(0, 0)]), "ro".into(), roqoqo::Circuit::new(),)); "PragmaGetPauliProduct")]
#[test_case(operations::Operation::from(operations::PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(operations::Operation::from(operations::PragmaSleep::new(vec![0],0.0.into())); "PragmaSleep")]
#[test_case(operations::Operation::from(operations::PragmaRepeatedMeasurement::new( "ro".to_string(), 10, None)); "PragmaRepeatedMeasurement")]
#[test_case(operations::Operation::from(operations::DefinitionBit::new("ro".into(), 2, false)); "DefinitionBit")]
#[test_case(operations::Operation::from(operations::DefinitionFloat::new("ro".into(), 2, false)); "DefinitionFloat")]
#[test_case(operations::Operation::from(operations::DefinitionComplex::new("ro".into(), 2, false)); "DefinitionComplex")]
#[test_case(operations::Operation::from(operations::PragmaGetOccupationProbability::new("ro".into(), None)); "PragmaGetOccupationProbability")]
fn test_version_1_0_0_pragmas(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 0, 0));
}

#[test_case(operations::Operation::from(operations::PragmaLoop::new(10.into(), roqoqo::Circuit::new())); "PragmaLoop")]
#[test_case(operations::Operation::from(operations::InputBit::new(String::from("test"), 1, false)); "InputBit")]
fn test_version_1_1_0_pragmas(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 1, 0));
}

#[test_case(operations::Operation::from(operations::PragmaControlledCircuit::new(10, roqoqo::Circuit::new())); "PragmaControlledCircuit")]
fn test_version_1_5_0_pragmas(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 5, 0));
}

#[test_case(operations::Operation::from(operations::PragmaAnnotatedOp::new(operations::PauliX::new(0).into(), "test".to_string())))]
fn test_version_1_8_0_pragmas(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 8, 0));
}

#[cfg(feature = "circuitdag")]
#[test_case(roqoqo::Circuit::from_iter(vec![
    operations::Operation::from(operations::DefinitionBit::new("ro".to_string(), 2, true))
].into_iter()), (1, 0, 0); "1 0 0")]
#[test_case(roqoqo::Circuit::from_iter(vec![
    operations::Operation::from(operations::DefinitionBit::new("ro".to_string(), 2, true)),
    operations::Operation::from(operations::PragmaLoop::new( 2.into(), roqoqo::Circuit::new())),
].into_iter()), (1, 1, 0); "1 1 0")]
#[test_case(roqoqo::Circuit::from_iter(vec![
    operations::Operation::from(operations::DefinitionBit::new("ro".to_string(), 2, true)),
    operations::Operation::from(operations::PhaseShiftedControlledPhase::new(1,2, 3.0.into(), 4.0.into())),
    operations::Operation::from(operations::PragmaLoop::new( 2.into(), roqoqo::Circuit::new())),
].into_iter()), (1, 2, 0); "1 2 0")]
fn test_version_circuit(circuit: roqoqo::Circuit, version: (u32, u32, u32)) {
    assert_eq!(circuit.minimum_supported_roqoqo_version(), version);
    let circuit_dag = roqoqo::CircuitDag::from(circuit.clone());
    assert_eq!(circuit_dag.minimum_supported_roqoqo_version(), version);

    let input = roqoqo::measurements::PauliZProductInput::new(1, false);
    let measurement = PauliZProduct {
        input: input.clone(),
        constant_circuit: Some(circuit.clone()),
        circuits: vec![],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);
    let measurement = PauliZProduct {
        input,
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);

    let program = QuantumProgram::PauliZProduct {
        measurement,
        input_parameter_names: vec![],
    };
    assert_eq!(program.minimum_supported_roqoqo_version(), version);

    let input = roqoqo::measurements::CheatedPauliZProductInput::new();
    let measurement = CheatedPauliZProduct {
        input: input.clone(),
        constant_circuit: Some(circuit.clone()),
        circuits: vec![],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);
    let measurement = CheatedPauliZProduct {
        input,
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);

    let program = QuantumProgram::CheatedPauliZProduct {
        measurement,
        input_parameter_names: vec![],
    };
    assert_eq!(program.minimum_supported_roqoqo_version(), version);

    let input = roqoqo::measurements::CheatedInput::new(1);
    let measurement = Cheated {
        input: input.clone(),
        constant_circuit: Some(circuit.clone()),
        circuits: vec![],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);
    let measurement = Cheated {
        input,
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);

    let program = QuantumProgram::Cheated {
        measurement,
        input_parameter_names: vec![],
    };
    assert_eq!(program.minimum_supported_roqoqo_version(), version);

    let measurement = ClassicalRegister {
        constant_circuit: Some(circuit.clone()),
        circuits: vec![],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit],
    };
    assert_eq!(measurement.minimum_supported_roqoqo_version(), version);

    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    assert_eq!(program.minimum_supported_roqoqo_version(), version);
}

#[cfg(feature = "unstable_operation_definition")]
#[test_case(operations::Operation::from(operations::GateDefinition::new(roqoqo::Circuit::new(), "name".to_string(), vec![2], vec!["name".to_owned()])); "GateDefinition")]
fn test_version_1_13_0_gate_definition(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 13, 0));
}

#[test_case(operations::SingleQubitGateOperation::from(operations::SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(operations::SingleQubitGateOperation::from(operations::InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
fn test_version_1_15_0_single_qubit_gate(operation: operations::SingleQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 15, 0));
}

#[test_case(operations::SingleQubitGateOperation::from(operations::InvTGate::new(0)); "SqrtPaInvTGateuliY")]
#[test_case(operations::SingleQubitGateOperation::from(operations::InvSGate::new(0)); "InvSGate")]
#[test_case(operations::SingleQubitGateOperation::from(operations::SXGate::new(0)); "SXGate")]
#[test_case(operations::SingleQubitGateOperation::from(operations::InvSXGate::new(0)); "InvSXGate")]
fn test_version_1_16_0_single_qubit_gate(operation: operations::SingleQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 16, 0));
}

#[test_case(operations::ThreeQubitGateOperation::from(operations::ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(operations::ThreeQubitGateOperation::from(operations::PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledZ")]
#[test_case(operations::ThreeQubitGateOperation::from(operations::PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::PI, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_version_1_16_0_three_qubit_gate(operation: operations::ThreeQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 16, 0));
}

#[test_case(
    operations::FourQubitGateOperation::from(operations::TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX"
)]
#[test_case(
    operations::FourQubitGateOperation::from(operations::TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ"
)]
#[test_case(
    operations::FourQubitGateOperation::from(operations::TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::PI)); "TripleControlledPhaseShift"
)]
fn test_version_1_16_0_four_qubit_gate(operation: operations::FourQubitGateOperation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 16, 0));
}

#[cfg(feature = "unstable_simulation_repetitions")]
#[test_case(operations::Operation::from(operations::PragmaSimulationRepetitions::new(100)))]
fn test_version_1_17_0_pragmas(operation: operations::Operation) {
    assert_eq!(operation.minimum_supported_roqoqo_version(), (1, 17, 0));
}
