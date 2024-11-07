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

use super::{ControlledPhaseShift, Hadamard, PhaseShiftedControlledPhase, CNOT};
use super::{RotateZ, TGate};
use crate::prelude::*;
use crate::Circuit;
use ndarray::{array, Array2};
use num_complex::Complex64;

use qoqo_calculator::CalculatorFloat;
#[cfg(feature = "overrotate")]
use rand_distr::{Distribution, Normal};
use std::f64::consts::PI;

/// Implements the double-controlled PauliZ gate.
///
/// The double-controlled PauliZ applies a PauliZ unitary to the `target` qubit
/// depending on the states of both `control_0` and `control_1` qubits.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ControlledControlledPauliZ {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
    target: usize,
}
impl super::ImplementedIn1point3 for ControlledControlledPauliZ {}

impl SupportedVersion for ControlledControlledPauliZ {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 3, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_ControlledControlledPauliZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "ControlledControlledPauliZ",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for ControlledControlledPauliZ {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for ControlledControlledPauliZ {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit +=
            ControlledPhaseShift::new(self.control_1, self.target, CalculatorFloat::FRAC_PI_2);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit +=
            ControlledPhaseShift::new(self.control_1, self.target, -CalculatorFloat::FRAC_PI_2);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit +=
            ControlledPhaseShift::new(self.control_0, self.target, CalculatorFloat::FRAC_PI_2);
        circuit
    }
}

/// Implements the double-controlled PhaseShift gate.
///
/// The double-controlled PhaseShift applies a phase shift to the `target` qubit
/// depending on the states of both `control_0` and `control_1` qubits.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ControlledControlledPhaseShift {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
    target: usize,
    /// The rotation angle θ.
    theta: CalculatorFloat,
}
impl super::ImplementedIn1point3 for ControlledControlledPhaseShift {}

impl SupportedVersion for ControlledControlledPhaseShift {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 3, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_ControlledControlledPhaseShift: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "Rotation",
    "ControlledControlledPhaseShift",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for ControlledControlledPhaseShift {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())?).cos();
        let s: f64 = (f64::try_from(self.theta.clone())?).sin();
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(c, s)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for ControlledControlledPhaseShift {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += ControlledPhaseShift::new(self.control_1, self.target, self.theta.clone() / 2.0);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit +=
            ControlledPhaseShift::new(self.control_1, self.target, -self.theta.clone() / 2.0);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += ControlledPhaseShift::new(self.control_0, self.target, self.theta.clone() / 2.0);
        circuit
    }
}

/// Implements the Toffoli gate.
///
/// The Toffoli gate applies a PauliX gate to the `target` qubit
/// depending on the states of both `control_0` and `control_1` qubits.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Toffoli {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
    target: usize,
}

impl super::ImplementedIn1point3 for Toffoli {}

impl SupportedVersion for Toffoli {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 3, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_Toffoli: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "Toffoli",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for Toffoli {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for Toffoli {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += Hadamard::new(self.target);
        circuit += CNOT::new(self.control_1, self.target);
        circuit += RotateZ::new(self.target, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control_0, self.target);
        circuit += TGate::new(self.target);
        circuit += CNOT::new(self.control_1, self.target);
        circuit += RotateZ::new(self.target, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control_0, self.target);
        circuit += TGate::new(self.control_1);
        circuit += TGate::new(self.target);
        circuit += Hadamard::new(self.target);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += TGate::new(self.control_0);
        circuit += RotateZ::new(self.control_1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit
    }
}

/// Implements the controlled SWAP gate.
///
/// NOTE: for compatibility reasons, the OperateThreeQubit trait is implemented, but
/// the "control" qubit of the operation can be accessed via the "control_0()" method,
/// the "target_0" qubit of the operation can be accessed via the "control_1()" method and
/// the "target_1" qubit of the operation can be accessed via the "target()" method.
#[derive(Debug, Clone, PartialEq, Eq, roqoqo_derive::Operate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ControlledSWAP {
    /// The index of the most significant qubit in the unitary representation. Here, the controlling qubit of the operation.
    control: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the first targeting qubit of the operation.
    target_0: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the second targeting qubit of the operation.
    target_1: usize,
}

impl super::ImplementedIn1point16 for ControlledSWAP {}

impl SupportedVersion for ControlledSWAP {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_ControlledSWAP: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "ControlledSWAP",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for ControlledSWAP {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for ControlledSWAP {
    fn circuit(&self) -> Circuit {
        // Based on CNOT(2, 1) -> Toffoli(0, 1, 2) -> CNOT(2, 1)
        let mut circuit = Circuit::new();
        circuit += CNOT::new(self.target_1, self.target_0);
        circuit += Hadamard::new(self.target_1);
        circuit += CNOT::new(self.target_0, self.target_1);
        circuit += RotateZ::new(self.target_1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control, self.target_1);
        circuit += TGate::new(self.target_1);
        circuit += CNOT::new(self.target_0, self.target_1);
        circuit += RotateZ::new(self.target_1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control, self.target_1);
        circuit += TGate::new(self.target_0);
        circuit += TGate::new(self.target_1);
        circuit += Hadamard::new(self.target_1);
        circuit += CNOT::new(self.control, self.target_0);
        circuit += TGate::new(self.control);
        circuit += RotateZ::new(self.target_0, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(self.control, self.target_0);
        circuit += CNOT::new(self.target_1, self.target_0);
        circuit
    }
}

impl OperateThreeQubit for ControlledSWAP {
    /// Returns `target_1` qubit of the three qubit Operation.
    fn target(&self) -> &usize {
        &self.target_1
    }

    /// Returns `control` qubit of the three qubit Operation.
    fn control_0(&self) -> &usize {
        &self.control
    }

    /// Returns `target_0` qubit of the three qubit Operation.
    fn control_1(&self) -> &usize {
        &self.target_0
    }
}

impl Substitute for ControlledSWAP {
    fn substitute_parameters(
        &self,
        _calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError> {
        Ok(Self::new(self.control, self.target_0, self.target_1))
    }

    fn remap_qubits(
        &self,
        mapping: &std::collections::HashMap<usize, usize>,
    ) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        Ok(Self::new(
            *mapping.get(&self.control).unwrap_or(&self.control),
            *mapping.get(&self.target_0).unwrap_or(&self.target_0),
            *mapping.get(&self.target_1).unwrap_or(&self.target_1),
        ))
    }
}

impl InvolveQubits for ControlledSWAP {
    fn involved_qubits(&self) -> InvolvedQubits {
        let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        new_hash_set.insert(self.control);
        new_hash_set.insert(self.target_0);
        new_hash_set.insert(self.target_1);
        InvolvedQubits::Set(new_hash_set)
    }
}

/// Implements the double-controlled phase-shifted PauliZ gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseShiftedControlledControlledZ {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the targeting qubit of the operation.
    target: usize,
    /// The single qubit phase φ.
    phi: CalculatorFloat,
}

impl super::ImplementedIn1point16 for PhaseShiftedControlledControlledZ {}

impl SupportedVersion for PhaseShiftedControlledControlledZ {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseShiftedControlledControlledZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "PhaseShiftedControlledControlledZ",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for PhaseShiftedControlledControlledZ {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        // exp(i*x) = cos(x)+i*sin(x)
        let phi: f64 = f64::try_from(self.phi.clone())?;
        let cos: f64 = phi.cos();
        let sin: f64 = phi.sin();
        let cos2: f64 = (2.0 * phi).cos();
        let sin2: f64 = (2.0 * phi).sin();
        let cos3: f64 = (3.0 * phi + PI).cos();
        let sin3: f64 = (3.0 * phi + PI).sin();
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos3, sin3)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for PhaseShiftedControlledControlledZ {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += PhaseShiftedControlledPhase::new(
            self.control_1,
            self.target,
            CalculatorFloat::FRAC_PI_2,
            self.phi.clone(),
        );
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += PhaseShiftedControlledPhase::new(
            self.control_1,
            self.target,
            -CalculatorFloat::FRAC_PI_2,
            self.phi.clone(),
        );
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += PhaseShiftedControlledPhase::new(
            self.control_0,
            self.target,
            CalculatorFloat::FRAC_PI_2,
            self.phi.clone(),
        );
        circuit
    }
}

/// Implements the double-controlled phase-shifted PhaseShift gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseShiftedControlledControlledPhase {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the targeting qubit of the operation.
    target: usize,
    /// The rotation angle θ.
    theta: CalculatorFloat,
    /// The single qubit phase φ.
    phi: CalculatorFloat,
}

impl super::ImplementedIn1point16 for PhaseShiftedControlledControlledPhase {}

impl SupportedVersion for PhaseShiftedControlledControlledPhase {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseShiftedControlledControlledPhase: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "PhaseShiftedControlledControlledPhase",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for PhaseShiftedControlledControlledPhase {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        // exp(i*x) = cos(x)+i*sin(x)
        let phi: f64 = f64::try_from(self.phi.clone())?;
        let theta: f64 = f64::try_from(self.theta.clone())?;
        let cos: f64 = phi.cos();
        let sin: f64 = phi.sin();
        let cos2: f64 = (2.0 * phi).cos();
        let sin2: f64 = (2.0 * phi).sin();
        let cos3: f64 = (3.0 * phi + theta).cos();
        let sin3: f64 = (3.0 * phi + theta).sin();
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos, sin),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos2, sin2),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(cos3, sin3)
            ],
        ])
    }
}

/// Trait for all gate operations acting on exactly three qubits.
impl OperateThreeQubitGate for PhaseShiftedControlledControlledPhase {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += PhaseShiftedControlledPhase::new(
            self.control_1,
            self.target,
            self.theta.clone() / 2,
            self.phi.clone(),
        );
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += PhaseShiftedControlledPhase::new(
            self.control_1,
            self.target,
            -self.theta.clone() / 2,
            self.phi.clone(),
        );
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += PhaseShiftedControlledPhase::new(
            self.control_0,
            self.target,
            self.theta.clone() / 2,
            self.phi.clone(),
        );
        circuit
    }
}
