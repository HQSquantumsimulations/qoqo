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

use crate::operations::*;
use crate::Circuit;
use ndarray::Array2;
use num_complex::Complex64;

/// The triple-controlled PauliX gate.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateFourQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TripleControlledPauliX {
    /// The first control qubit involved in the triple-controlled PauliX gate.
    control_0: usize,
    /// The second control qubit involved in the triple-controlled PauliX gate.
    control_1: usize,
    /// The third control qubit involved in the triple-controlled PauliX gate.
    control_2: usize,
    /// The target qubit to apply the PauliX gate to.
    target: usize,
}

impl super::ImplementedIn1point16 for TripleControlledPauliX {}

impl SupportedVersion for TripleControlledPauliX {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_TripleControlledPauliX: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "FourQubitGateOperation",
    "TripleControlledPauliX",
];

impl OperateGate for TripleControlledPauliX {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 16;
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        for i in 0..dim - 2 {
            array[(i, i)] = Complex64::new(1.0, 0.0);
        }
        array[(dim - 2, dim - 1)] = Complex64::new(1.0, 0.0);
        array[(dim - 1, dim - 2)] = Complex64::new(1.0, 0.0);
        Ok(array)
    }
}

impl OperateFourQubitGate for TripleControlledPauliX {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += CNOT::new(self.control_0, self.target);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += CNOT::new(self.control_1, self.target);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += CNOT::new(self.control_1, self.target);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit += CNOT::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += CNOT::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit += CNOT::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += CNOT::new(self.control_2, self.target);
        circuit
    }
}

/// The triple-controlled PauliZ gate.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateFourQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TripleControlledPauliZ {
    /// The first control qubit involved in the triple-controlled PauliZ gate.
    control_0: usize,
    /// The second control qubit involved in the triple-controlled PauliZ gate.
    control_1: usize,
    /// The third control qubit involved in the triple-controlled PauliZ gate.
    control_2: usize,
    /// The target qubit to apply the PauliZ gate to.
    target: usize,
}

impl super::ImplementedIn1point16 for TripleControlledPauliZ {}

impl SupportedVersion for TripleControlledPauliZ {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_TripleControlledPauliZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "FourQubitGateOperation",
    "TripleControlledPauliZ",
];

impl OperateGate for TripleControlledPauliZ {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 16;
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        for i in 0..dim - 1 {
            array[(i, i)] = Complex64::new(1.0, 0.0);
        }
        array[(dim - 1, dim - 1)] = Complex64::new(-1.0, 0.0);
        Ok(array)
    }
}

impl OperateFourQubitGate for TripleControlledPauliZ {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += ControlledPauliZ::new(self.control_0, self.target);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += ControlledPauliZ::new(self.control_1, self.target);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += ControlledPauliZ::new(self.control_1, self.target);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit += ControlledPauliZ::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += ControlledPauliZ::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit += ControlledPauliZ::new(self.control_2, self.target);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += ControlledPauliZ::new(self.control_2, self.target);
        circuit
    }
}

/// The triple-controlled PhaseShift gate.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateFourQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TripleControlledPhaseShift {
    /// The first control qubit involved in the triple-controlled PhaseShift gate.
    control_0: usize,
    /// The second control qubit involved in the triple-controlled PhaseShift gate.
    control_1: usize,
    /// The third control qubit involved in the triple-controlled PhaseShift gate.
    control_2: usize,
    /// The target qubit to apply the PhaseShift gate to.
    target: usize,
    /// The rotation angle θ.
    theta: CalculatorFloat,
}

impl super::ImplementedIn1point16 for TripleControlledPhaseShift {}

impl SupportedVersion for TripleControlledPhaseShift {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 16, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_TripleControlledPhaseShift: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "FourQubitGateOperation",
    "TripleControlledPhaseShift",
];

impl OperateGate for TripleControlledPhaseShift {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())?).cos();
        let s: f64 = (f64::try_from(self.theta.clone())?).sin();
        let dim = 16;
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        for i in 0..dim - 1 {
            array[(i, i)] = Complex64::new(1.0, 0.0);
        }
        array[(dim - 1, dim - 1)] = Complex64::new(c, s);
        Ok(array)
    }
}

impl OperateFourQubitGate for TripleControlledPhaseShift {
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += ControlledPhaseShift::new(self.control_0, self.target, self.theta().clone() / 2);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit +=
            ControlledPhaseShift::new(self.control_1, self.target, -self.theta().clone() / 2);
        circuit += CNOT::new(self.control_0, self.control_1);
        circuit += ControlledPhaseShift::new(self.control_1, self.target, self.theta().clone() / 2);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit +=
            ControlledPhaseShift::new(self.control_2, self.target, -self.theta().clone() / 2);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += ControlledPhaseShift::new(self.control_2, self.target, self.theta().clone() / 2);
        circuit += CNOT::new(self.control_1, self.control_2);
        circuit +=
            ControlledPhaseShift::new(self.control_2, self.target, -self.theta().clone() / 2);
        circuit += CNOT::new(self.control_0, self.control_2);
        circuit += ControlledPhaseShift::new(self.control_2, self.target, self.theta().clone() / 2);
        circuit
    }
}
