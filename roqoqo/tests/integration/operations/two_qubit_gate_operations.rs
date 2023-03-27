// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Integration test for public API of two qubit gate operations

use super::convert_matrix;
use nalgebra as na;
use ndarray::{array, Array2};
use num_complex::Complex64;
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::Circuit;
use roqoqo::RoqoqoError;
use roqoqo::RoqoqoError::QubitMappingError;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::f64::consts::PI;
use test_case::test_case;

// helper function to convert a complex matrix to a matrix with real absolute values
fn convert_normsqr(customarray: na::DMatrix<Complex64>) -> [[f64; 4]; 4] {
    let mut overall_vec: Vec<[f64; 4]> = Vec::new();
    for i in [0, 4, 8, 12].iter() {
        let mut this_vec: Vec<f64> = Vec::new();
        for j in 0..4 {
            this_vec.push(customarray[i + j].norm());
        }
        let this_vec_to_array: [f64; 4] = this_vec.try_into().unwrap();
        overall_vec.push(this_vec_to_array);
    }
    let overall_array: [[f64; 4]; 4] = [
        overall_vec[0],
        overall_vec[1],
        overall_vec[2],
        overall_vec[3],
    ];
    overall_array
}

// helper function to build the sigma matrix used for unit testing KaK decomposition
fn kak_sigma_matrix(
    x: CalculatorFloat,
    y: CalculatorFloat,
    z: CalculatorFloat,
) -> Array2<Complex64> {
    let x: f64 = f64::try_from(x).unwrap();
    let y: f64 = f64::try_from(y).unwrap();
    let z: f64 = f64::try_from(z).unwrap();

    let cm: f64 = (x - y).cos();
    let cp: f64 = (x + y).cos();
    let sm: f64 = (x - y).sin();
    let sp: f64 = (x + y).sin();

    let cz: f64 = z.cos();
    let sz: f64 = z.sin();

    let array: Array2<Complex64> = array![
        [
            Complex64::new(cm * cz, (-1.0) * cm * sz),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new((-1.0) * sm * sz, (-1.0) * sm * cz)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(cp * cz, cp * sz),
            Complex64::new(sp * sz, (-1.0) * sp * cz),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(sp * sz, (-1.0) * sp * cz),
            Complex64::new(cp * cz, cp * sz),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new((-1.0) * sm * sz, (-1.0) * sm * cz),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(cm * cz, (-1.0) * cm * sz)
        ],
    ];
    array
}

//
// Test KaK decompositions for Two Qubit Gates
//
#[test_case(TwoQubitGateOperation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(TwoQubitGateOperation::from(SWAP::new(0, 1)); "Swap")]
#[test_case(TwoQubitGateOperation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(TwoQubitGateOperation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(TwoQubitGateOperation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(TwoQubitGateOperation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(TwoQubitGateOperation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(TwoQubitGateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(TwoQubitGateOperation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(TwoQubitGateOperation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(TwoQubitGateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "VariableMSXX")]
#[test_case(TwoQubitGateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX_pi")]
#[test_case(TwoQubitGateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(TwoQubitGateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(TwoQubitGateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(TwoQubitGateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(TwoQubitGateOperation::from(Fsim::new(0, 1, CalculatorFloat::PI, CalculatorFloat::PI, CalculatorFloat::PI)); "Fsim_PI")]
#[test_case(TwoQubitGateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(TwoQubitGateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(TwoQubitGateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(TwoQubitGateOperation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(TwoQubitGateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ_pi")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::ZERO)); "PhaseShiftedControlledZ_zero")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(PI/(-3.0)))); "PhaseShiftedControlledZ")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase_pi_pi_2")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::PI)); "PhaseShiftedControlledPhase_zero_pi")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::ZERO)); "PhaseShiftedControlledPhase_pi_4_zero")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::ZERO)); "ControlledRotateX_zero")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledRotateX_pi_4")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX_pi_2")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY_zero_pi_4")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::ZERO)); "ControlledRotateXY_pi_4_zero")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_2)); "ControlledRotateXY_pi_2_pi_2")]
fn test_kakdecomposition(gate: TwoQubitGateOperation) {
    // k vector
    let k = gate.kak_decomposition().k_vector;
    let sigma_matrix: na::DMatrix<Complex64> = convert_matrix(kak_sigma_matrix(
        k[0].clone() * (-1.0),
        k[1].clone() * (-1.0),
        k[2].clone() * (-1.0),
    ));

    // global phase
    let g = gate.kak_decomposition().global_phase;
    let phase_factor: f64 = f64::try_from(g).unwrap();
    let phase: Complex64 = Complex64::new(0.0, phase_factor).exp();

    // determine matrix before entanglement
    let circuit_before = gate.kak_decomposition().circuit_before;

    let mut target_before: SingleQubitGate = SingleQubitGate::new(
        *gate.target(),
        CalculatorFloat::from(1.0),
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
    );
    let mut control_before: SingleQubitGate = SingleQubitGate::new(
        *gate.control(),
        CalculatorFloat::from(1.0),
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
    );

    if let Some(circ) = circuit_before {
        let operations_before: Vec<Operation> = circ.operations().clone();
        let range = 0..operations_before.len();
        for i in range {
            let element: SingleQubitGateOperation =
                operations_before[i].clone().try_into().unwrap();
            if element.qubit() == gate.target() {
                target_before = element.clone().mul(&target_before.clone()).unwrap()
            } else {
                control_before = element.clone().mul(&control_before.clone()).unwrap()
            }
        }
    }

    let target_before_matrix: na::DMatrix<Complex64> =
        convert_matrix(target_before.unitary_matrix().unwrap());
    let control_before_matrix: na::DMatrix<Complex64> =
        convert_matrix(control_before.unitary_matrix().unwrap());
    let matrix_before: na::DMatrix<Complex64> =
        control_before_matrix.kronecker(&target_before_matrix);

    // determine matrix after entanglement
    let circuit_after = gate.kak_decomposition().circuit_after;
    let mut target_after: SingleQubitGate = SingleQubitGate::new(
        *gate.target(),
        CalculatorFloat::from(1.0),
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
    );
    let mut control_after: SingleQubitGate = SingleQubitGate::new(
        *gate.control(),
        CalculatorFloat::from(1.0),
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
    );

    if let Some(circ) = circuit_after {
        let operations_after: Vec<Operation> = circ.operations().clone();
        let range = 0..operations_after.len();
        for i in range {
            let element: SingleQubitGateOperation = operations_after[i].clone().try_into().unwrap();
            if element.qubit() == gate.target() {
                target_after = element.clone().mul(&target_after.clone()).unwrap()
            } else {
                control_after = element.clone().mul(&control_after.clone()).unwrap()
            }
        }
    }

    let target_after_matrix: na::DMatrix<Complex64> =
        convert_matrix(target_after.unitary_matrix().unwrap());
    let control_after_matrix: na::DMatrix<Complex64> =
        convert_matrix(control_after.unitary_matrix().unwrap());
    let matrix_after: na::DMatrix<Complex64> = control_after_matrix.kronecker(&target_after_matrix);

    let decomposed_matrix = matrix_after * sigma_matrix * matrix_before * phase;
    let test_matrix: na::DMatrix<Complex64> = convert_matrix(gate.unitary_matrix().unwrap());

    let epsilon = 1e-12;
    for i in 0..16 {
        assert!((decomposed_matrix[i] - test_matrix[i]).norm() < epsilon);
    }
}

//
// Test Unitary Matrix for TwoQubit Gates
//

// Test unitary matrix for TwoQubitGate Operations
#[test_case(GateOperation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(GateOperation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(GateOperation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(GateOperation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(GateOperation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(GateOperation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(GateOperation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(GateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(GateOperation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(GateOperation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(GateOperation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(GateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(GateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(GateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(GateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(GateOperation::from(Qsim::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4, CalculatorFloat::from(1.0))); "Qsim_1")]
#[test_case(GateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(GateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::ZERO)); "Bogoliubov_r")]
#[test_case(GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(1.0))); "Bogoliubov_i")]
#[test_case(GateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(GateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ_pi_4")]
#[test_case(GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::ZERO)); "PhaseShiftedControlledZ_zero")]
#[test_case(GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(1.33))); "PhaseShiftedControlledZ_1_33")]
#[test_case(GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase_pi_4_pi_4")]
#[test_case(GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase_zero_pi_4")]
#[test_case(GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::from(1.33))); "PhaseShiftedControlledPhase_pi_2_1_33")]
#[test_case(GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX_pi_2")]
#[test_case(GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledRotateX_pi_4")]
#[test_case(GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::ZERO)); "ControlledRotateX_zero")]
#[test_case(GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_2)); "ControlledRotateXY_pi_2_pi_2")]
#[test_case(GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::ZERO)); "ControlledRotateXY_pi_4_zero")]
#[test_case(GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_2)); "ControlledRotateXY_zero_pi_2")]
fn test_twoqubitgates_unitarity(gate: GateOperation) {
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    let result_array: Array2<Complex64> = result.unwrap();
    // check unitarity with nalgebra
    // convert ndarray into nalgebra matrix
    let result_matrix: na::DMatrix<Complex64> = convert_matrix(result_array);
    // calculate matrix product A*A_dagger
    let product = result_matrix.clone() * result_matrix.adjoint();
    // convert complex matrix product into real matrix by taking the absolute value of the complex number, which should be sufficient if the matrix is unitary.
    let matrix_norm: na::Matrix4<f64> = na::Matrix4::from(convert_normsqr(product));
    let epsilon = 1e-12;
    assert!(matrix_norm.is_identity(epsilon));
}

//
// Test 'Derive' for TwoQubitGate Operations
//

/// Test clone function for TwoQubitGate Operations
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_twoqubitgates_clone(gate1: Operation) {
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

#[test_case(TwoQubitGateOperation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(TwoQubitGateOperation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(TwoQubitGateOperation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(TwoQubitGateOperation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(TwoQubitGateOperation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(TwoQubitGateOperation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(TwoQubitGateOperation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(TwoQubitGateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(TwoQubitGateOperation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(TwoQubitGateOperation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(TwoQubitGateOperation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(TwoQubitGateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(TwoQubitGateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(TwoQubitGateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(TwoQubitGateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(TwoQubitGateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(TwoQubitGateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(TwoQubitGateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(TwoQubitGateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(TwoQubitGateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(TwoQubitGateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(TwoQubitGateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_qubits_twoqubitgates(gate: TwoQubitGateOperation) {
    let control: &usize = gate.control();
    assert_eq!(control, &0);
    let target: &usize = gate.target();
    assert_eq!(target, &1);
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    assert_eq!(gate.involved_qubits(), test_qubits);
}

#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_is_parametrized_false(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(!bool_parameter);
}

#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("parameter"))); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("parameter"))); "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("parameter"))); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("parameter1"), CalculatorFloat::from("parameter2"))); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("parameter1"), CalculatorFloat::from("parameter2"))); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("x"), CalculatorFloat::from("y"), CalculatorFloat::from("z"))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("x"), CalculatorFloat::from("y"), CalculatorFloat::from("z"))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("x"), CalculatorFloat::from("y"), CalculatorFloat::from("z"))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1,CalculatorFloat::from("parameter1"), CalculatorFloat::from("parameter2"))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("parameter"))); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("parameter1"), CalculatorFloat::from("parameter2"))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("phi"))); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::from("theta"))); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "ControlledRotateXY")]
fn test_is_parametrized_true(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(bool_parameter);
}

#[test_case("CNOT", Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case("SWAP", Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case("ISwap", Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case("FSwap", Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case("SqrtISwap", Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case("InvSqrtISwap", Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case("XY", Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(
    "ControlledPhaseShift", 
    Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4))
; "ControlledPhaseShift")]
#[test_case("ControlledPauliY", Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case("ControlledPauliZ", Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case("MolmerSorensenXX", Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case("VariableMSXX", Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case("GivensRotation", Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case("GivensRotationLittleEndian", Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case("Qsim", Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case("Fsim", Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case("SpinInteraction", Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case("Bogoliubov", Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case("PMInteraction", Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case("ComplexPMInteraction", Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case("PhaseShiftedControlledZ", Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("phi"))); "PhaseShiftedControlledZ")]
#[test_case("PhaseShiftedControlledPhase", Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "PhaseShiftedControlledPhase")]
#[test_case("ControlledRotateX", Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::from("theta"))); "ControlledRotateX")]
#[test_case("ControlledRotateXY", Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "ControlledRotateXY")]
fn test_twoqubitgateoperations_hqslang(name: &'static str, gate: Operation) {
    assert!(!gate.hqslang().is_empty());
    assert_eq!(gate.hqslang(), name);
}

#[test_case(
    GateOperation::from(CNOT::new(0, 1)),
    GateOperation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    GateOperation::from(SWAP::new(0, 1)),
    GateOperation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    GateOperation::from(ISwap::new(0, 1)),
    GateOperation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    GateOperation::from(FSwap::new(0, 1)),
    GateOperation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    GateOperation::from(SqrtISwap::new(0, 1)),
    GateOperation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    GateOperation::from(InvSqrtISwap::new(0, 1)),
    GateOperation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    GateOperation::from(XY::new(0, 1, CalculatorFloat::PI)),
    GateOperation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    GateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)),
    GateOperation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    GateOperation::from(ControlledPauliY::new(0, 1)),
    GateOperation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    GateOperation::from(ControlledPauliZ::new(0, 1)),
    GateOperation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    GateOperation::from(MolmerSorensenXX::new(0, 1)),
    GateOperation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    GateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)),
    GateOperation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    GateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    GateOperation::from(GivensRotation::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(
    GateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    GateOperation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(
    GateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    GateOperation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    GateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    GateOperation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    GateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    GateOperation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    GateOperation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    GateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)),
    GateOperation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    GateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    GateOperation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)),
    GateOperation::from(PhaseShiftedControlledZ::new(1, 0, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(
    GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_2)),
    GateOperation::from(PhaseShiftedControlledPhase::new(1, 0, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(
    GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)),
    GateOperation::from(ControlledRotateX::new(1, 0, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(
    GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)),
    GateOperation::from(ControlledRotateXY::new(1, 0, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn remap_qubits_result(gate: GateOperation, test_gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    qubit_mapping.insert(1, 0);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(&result, &Ok(test_gate));
}

#[test_case(GateOperation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(GateOperation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(GateOperation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(GateOperation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(GateOperation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(GateOperation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(GateOperation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(GateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(GateOperation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(GateOperation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(GateOperation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(GateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(GateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(GateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(GateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(GateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(GateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(GateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(GateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn remap_qubits_error0(gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(1, 0);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 0 }));
}

#[test_case(GateOperation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(GateOperation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(GateOperation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(GateOperation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(GateOperation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(GateOperation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(GateOperation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(GateOperation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(GateOperation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(GateOperation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(GateOperation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(GateOperation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(GateOperation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(GateOperation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(GateOperation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(GateOperation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(GateOperation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(GateOperation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(GateOperation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(GateOperation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(GateOperation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(GateOperation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(GateOperation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(GateOperation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn remap_qubits_error1(gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 2);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 2 }));
}

#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "CNOT",
        ],
    Operation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SWAP",
        ],
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ISwap",
        ],
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "FSwap",
        ],
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SqrtISwap",
        ],
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "InvSqrtISwap",
        ],
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "XY",
        ],
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "ControlledPhaseShift",
        ],
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledPauliY",
        ],
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledPauliZ",
        ],
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "MolmerSorensenXX",
        ],
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "VariableMSXX",
        ],
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "GivensRotation",
        ],
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "GivensRotationLittleEndian",
        ],
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Qsim",
        ],
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Fsim",
        ],
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SpinInteraction",
        ],
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Bogoliubov",
        ],
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PMInteraction",
        ],
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ComplexPMInteraction",
        ],
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PhaseShiftedControlledZ",
        ],
    Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PhaseShiftedControlledPhase",
        ],
    Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "ControlledRotateX"
    ],
    Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "ControlledRotateXY"
    ],
    Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
pub fn test_tags(tags: Vec<&str>, gate: Operation) {
    let range = 0..tags.len();
    for i in range {
        assert_eq!(gate.tags()[i], tags[i]);
    }
}

#[test_case(
    "CNOT(CNOT { control: 1, target: 0 })",
    Operation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    "SWAP(SWAP { control: 1, target: 0 })",
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    "ISwap(ISwap { control: 1, target: 0 })",
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    "FSwap(FSwap { control: 1, target: 0 })",
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    "SqrtISwap(SqrtISwap { control: 1, target: 0 })",
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    "InvSqrtISwap(InvSqrtISwap { control: 1, target: 0 })",
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    "XY(XY { control: 1, target: 0, theta: Float(3.141592653589793) })",
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    "ControlledPhaseShift(ControlledPhaseShift { control: 1, target: 0, theta: Float(0.7853981633974483) })",
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    "ControlledPauliY(ControlledPauliY { control: 1, target: 0 })",
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    "ControlledPauliZ(ControlledPauliZ { control: 1, target: 0 })",
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    "MolmerSorensenXX(MolmerSorensenXX { control: 1, target: 0 })",
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    "VariableMSXX(VariableMSXX { control: 1, target: 0, theta: Float(3.141592653589793) })",
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    "GivensRotation(GivensRotation { control: 1, target: 0, theta: Float(0.0), phi: Float(0.0) })",
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::ZERO, CalculatorFloat::ZERO)); "GivensRotation")]
#[test_case(
    "GivensRotationLittleEndian(GivensRotationLittleEndian { control: 1, target: 0, theta: Float(0.0), phi: Float(0.0) })",
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::ZERO, CalculatorFloat::ZERO)); "GivensRotationLittleEndian")]
#[test_case(
    "Qsim(Qsim { control: 1, target: 0, x: Float(1.0), y: Float(1.0), z: Float(-1.0) })",
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    "Fsim(Fsim { control: 1, target: 0, t: Float(1.0), u: Float(2.0), delta: Float(-1.0) })",
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    "SpinInteraction(SpinInteraction { control: 1, target: 0, x: Float(1.0), y: Float(2.0), z: Float(-1.0) })",
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    "Bogoliubov(Bogoliubov { control: 1, target: 0, delta_real: Float(1.0), delta_imag: Float(-1.0) })",
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    "PMInteraction(PMInteraction { control: 1, target: 0, t: Float(3.141592653589793) })",
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    "ComplexPMInteraction(ComplexPMInteraction { control: 1, target: 0, t_real: Float(1.0), t_imag: Float(-1.0) })",
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    "PhaseShiftedControlledZ(PhaseShiftedControlledZ { control: 1, target: 0, phi: Float(-1.0) })",
    Operation::from(PhaseShiftedControlledZ::new(1, 0, CalculatorFloat::from(-1.0))); "PhaseShiftedControlledZ")]
#[test_case(
    "PhaseShiftedControlledPhase(PhaseShiftedControlledPhase { control: 1, target: 0, theta: Float(-1.0), phi: Float(-1.0) })",
    Operation::from(PhaseShiftedControlledPhase::new(1, 0, CalculatorFloat::from(-1.0), CalculatorFloat::from(-1.0))); "PhaseShiftedControlledPhase")]
#[test_case(
    "ControlledRotateX(ControlledRotateX { control: 1, target: 0, theta: Float(-1.0) })",
    Operation::from(ControlledRotateX::new(1, 0, CalculatorFloat::from(-1.0))); "ControlledRotateX")]
#[test_case(
    "ControlledRotateXY(ControlledRotateXY { control: 1, target: 0, theta: Float(-1.0), phi: Float(1.0) })",
    Operation::from(ControlledRotateXY::new(1, 0, CalculatorFloat::from(-1.0), CalculatorFloat::from(1.0))); "ControlledRotateXY")]
fn test_two_qubitgates_debug(message: &'static str, gate: Operation) {
    assert_eq!(format!("{:?}", gate), message);
}

/// Test PartialEq for TwoQubitGate Operations
#[test_case(
    Operation::from(CNOT::new(0, 1)),
    Operation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    Operation::from(SWAP::new(0, 1)),
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    Operation::from(ISwap::new(0, 1)),
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    Operation::from(FSwap::new(0, 1)),
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    Operation::from(SqrtISwap::new(0, 1)),
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    Operation::from(InvSqrtISwap::new(0, 1)),
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    Operation::from(XY::new(0, 1, CalculatorFloat::PI)),
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)),
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    Operation::from(ControlledPauliY::new(0, 1)),
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    Operation::from(ControlledPauliZ::new(0, 1)),
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    Operation::from(MolmerSorensenXX::new(0, 1)),
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)),
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(
    Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(
    Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)),
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)),
    Operation::from(PhaseShiftedControlledZ::new(1, 0, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(
    Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)),
    Operation::from(PhaseShiftedControlledPhase::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledPhase")]
#[test_case(
    Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::PI)),
    Operation::from(ControlledRotateX::new(1, 0, CalculatorFloat::PI)); "ControlledRotateX")]
#[test_case(
    Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)),
    Operation::from(ControlledRotateXY::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_twoqubitgates_partialeq(gate1: Operation, gate2: Operation) {
    assert!(gate1 == gate1.clone());
    assert_eq!(gate1, gate1.clone());
    assert_ne!(gate2, gate1);
    assert_ne!(gate1, gate2);
}

/// Test powerfc function for Rotate gates
#[test_case(Rotation::from(XY::new(0, 1, CalculatorFloat::PI)),
            Rotation::from(XY::new(0, 1, CalculatorFloat::PI * 1.5)); "XY")]
#[test_case(Rotation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)),
            Rotation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4 * 1.5)); "ControlledPhaseShift")]
#[test_case(Rotation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)),
            Rotation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI * 1.5)); "VariableMSXX")]
#[test_case(Rotation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
            Rotation::from(GivensRotation::new(0, 1, CalculatorFloat::PI * 1.5, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Rotation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
            Rotation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI * 1.5, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Rotation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)),
            Rotation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2 * 1.5)); "ControlledRotateX")]
#[test_case(Rotation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)),
            Rotation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2 * 1.5, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_rotate_powercf(gate: Rotation, gate2: Rotation) {
    let power_gate = gate.powercf(CalculatorFloat::from(1.5));
    assert_eq!(power_gate, gate2);
    assert_eq!(power_gate.theta(), gate2.theta());
}

/// Test substitute parameters function for TwoQubitGate Operations where it has no effect
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "Swap")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "VariableMSXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX_pi")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::PI, CalculatorFloat::PI, CalculatorFloat::PI)); "Fsim_PI")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledRotateXY")]
fn test_ineffective_substitute_parameters(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate);
}

/// Test substitute parameters function for TwoQubitGate Operations
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(XY::new(0, 1, CalculatorFloat::ZERO)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::ZERO)); "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::ZERO)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_4)),
            Operation::from(GivensRotation::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_4)),
            Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
            Operation::from(Qsim::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
            Operation::from(Fsim::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
            Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(PMInteraction::new(0, 1, CalculatorFloat::ZERO)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(-1.0))),
            Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(-1.0))),
            Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::ZERO)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_2)),
            Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::from("theta"))),
            Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::ZERO)); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_2)),
            Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::ZERO, CalculatorFloat::FRAC_PI_2)); "ControlledRotateXY")]
fn test_substitute_parameters(gate: Operation, gate2: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate2);
}

/// Test substitute parameters function for TwoQubitGate Operations with error
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("theta"))); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("theta"))); "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("theta"))); "VariableMSXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("theta"))); "VariableMSXX_pi")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::PI, CalculatorFloat::PI)); "Fsim_PI")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("theta"))); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("theta"))); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, CalculatorFloat::from("theta"))); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::FRAC_PI_2)); "ControlledRotateXY")]
fn test_substitute_parameters_error(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("error", 0.0);
    let result = gate.substitute_parameters(&substitution_dict);
    assert!(result.is_err());
}

#[test]
fn test_inputs_givens() {
    let gate = GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4);
    assert_eq!(gate.phi(), &CalculatorFloat::FRAC_PI_4);
    let gate =
        GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4);
    assert_eq!(gate.phi(), &CalculatorFloat::FRAC_PI_4);
}

#[test]
fn test_inputs_qsim_spininteraction() {
    let gate = Qsim::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(-1.0),
    );
    assert_eq!(gate.x(), &CalculatorFloat::PI);
    assert_eq!(gate.y(), &CalculatorFloat::from(1.0));
    assert_eq!(gate.z(), &CalculatorFloat::from(-1.0));
    let gate = SpinInteraction::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(-1.0),
    );
    assert_eq!(gate.x(), &CalculatorFloat::PI);
    assert_eq!(gate.y(), &CalculatorFloat::from(1.0));
    assert_eq!(gate.z(), &CalculatorFloat::from(-1.0));
}

#[test]
fn test_inputs_pminteraction_complexpminteraction() {
    let gate = PMInteraction::new(0, 1, CalculatorFloat::PI);
    assert_eq!(gate.t(), &CalculatorFloat::PI);
    let gate = ComplexPMInteraction::new(0, 1, CalculatorFloat::PI, CalculatorFloat::ZERO);
    assert_eq!(gate.t_real(), &CalculatorFloat::PI);
    assert_eq!(gate.t_imag(), &CalculatorFloat::ZERO);
}

#[test]
fn test_inputs_phaseshiftedcontrolledz() {
    let gate = PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI);
    assert_eq!(gate.phi(), &CalculatorFloat::PI);
}

#[test]
fn test_inputs_phaseshiftedcontrolledphase() {
    let gate = PhaseShiftedControlledPhase::new(
        0,
        1,
        CalculatorFloat::FRAC_PI_4,
        CalculatorFloat::FRAC_PI_2,
    );
    assert_eq!(gate.theta(), &CalculatorFloat::FRAC_PI_4);
    assert_eq!(gate.phi(), &CalculatorFloat::FRAC_PI_2);
}

#[test]
fn test_inputs_bogoliubov() {
    let gate = Bogoliubov::new(0, 1, CalculatorFloat::PI, CalculatorFloat::from(0.0));
    assert_eq!(gate.delta_real(), &CalculatorFloat::PI);
    assert_eq!(gate.delta_imag(), &CalculatorFloat::ZERO);
}

#[test]
fn test_inputs_controlledrotatex() {
    let gate = ControlledRotateX::new(0, 1, CalculatorFloat::PI);
    assert_eq!(gate.theta(), &CalculatorFloat::PI);
}

#[test]
fn test_inputs_controlledrotatexy() {
    let gate = ControlledRotateXY::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2);
    assert_eq!(gate.theta(), &CalculatorFloat::PI);
    assert_eq!(gate.phi(), &CalculatorFloat::FRAC_PI_2);
}

#[test]
fn test_inputs_fsim() {
    let gate = Fsim::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::from(2.0),
        CalculatorFloat::from(-1.0),
    );
    assert_eq!(gate.t(), &CalculatorFloat::PI);
    assert_eq!(gate.u(), &CalculatorFloat::from(2.0));
    assert_eq!(gate.delta(), &CalculatorFloat::from(-1.0));
}

#[test]
fn test_kakdecomposition_partialeq() {
    // CNOT-like KakDecomposition
    let mut circuit_b = Circuit::new();
    circuit_b += RotateZ::new(1, CalculatorFloat::FRAC_PI_2);
    circuit_b += RotateY::new(1, CalculatorFloat::FRAC_PI_2);
    circuit_b += RotateX::new(0, CalculatorFloat::FRAC_PI_2);

    let mut circuit_a = Circuit::new();
    circuit_a += RotateY::new(0, CalculatorFloat::FRAC_PI_2 * (-1.0));

    let gate1 = KakDecomposition {
        global_phase: CalculatorFloat::FRAC_PI_4,
        k_vector: [
            CalculatorFloat::FRAC_PI_4,
            CalculatorFloat::ZERO,
            CalculatorFloat::ZERO,
        ],
        circuit_before: Some(circuit_b),
        circuit_after: Some(circuit_a),
    };

    // SWAP-like KakDecomposition
    let gate2 = KakDecomposition {
        global_phase: CalculatorFloat::from((-1.0) * PI / 4.0),
        k_vector: [
            CalculatorFloat::FRAC_PI_4,
            CalculatorFloat::FRAC_PI_4,
            CalculatorFloat::FRAC_PI_4,
        ],
        circuit_before: None,
        circuit_after: None,
    };

    // comparison
    assert!(gate1 == gate1);
    assert_eq!(gate1, gate1.clone());
    assert_ne!(gate2, gate1);
    assert_ne!(gate1, gate2);
}

#[test]
fn test_kakdecomposition_debug() {
    let gate = KakDecomposition {
        global_phase: CalculatorFloat::from(1.0),
        k_vector: [
            CalculatorFloat::ZERO,
            CalculatorFloat::ZERO,
            CalculatorFloat::ZERO,
        ],
        circuit_before: None,
        circuit_after: None,
    };
    let message = "KakDecomposition { global_phase: Float(1.0), k_vector: [Float(0.0), Float(0.0), Float(0.0)], circuit_before: None, circuit_after: None }";
    assert_eq!(format!("{:?}", gate), message);
}
