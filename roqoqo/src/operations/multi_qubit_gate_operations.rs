// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::operations;
use crate::prelude::*;
use crate::Circuit;
use crate::RoqoqoError;
use ndarray::prelude::*;
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
#[cfg(feature = "overrotate")]
use rand_distr::{Distribution, Normal};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// The Molmer-Sorensen gate between multiple qubits.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MultiQubitMS {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_MultiQubitMS: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "MultiQubitMS",
];

impl OperateGate for MultiQubitMS {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 2_usize.pow(self.qubits.len() as u32);
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        let cos: Complex64 = Complex64::new((self.theta.float()? / 2.0).cos(), 0.0);
        let sin: Complex64 = Complex64::new(0.0, -(self.theta.float()? / 2.0).sin());
        for i in 0..dim {
            array[(i, i)] = cos;
            array[(i, dim - i - 1)] = sin;
        }
        Ok(array)
    }
}

impl OperateMultiQubitGate for MultiQubitMS {
    // Todo fill out circuit
    fn circuit(&self) -> Circuit {
        let dim = self.qubits.len();
        let mut circuit = Circuit::new();
        for q in self.qubits.iter() {
            circuit += operations::Hadamard::new(*q);
        }
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(*q - 1, *q);
        }
        circuit += operations::RotateZ::new(dim - 1, self.theta.clone() / 2);
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(dim - *q - 1, dim - *q);
        }
        for q in self.qubits.iter() {
            circuit += operations::Hadamard::new(*q);
        }
        circuit
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// The CNOT gate with multiple controls
pub struct MultiCNOT {
    qubits: Vec<usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_MultiCNOT: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "MultiCNOT",
];

impl OperateGate for MultiCNOT {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 2_usize.pow(self.qubits.len() as u32);
        let mut array = Array2::eye(dim);
        array
            .slice_mut(s![dim - 2.., dim - 2..])
            .assign(&array![[0., 1.], [1., 0.]]);
        Ok(array.map(|x| x.into()))
    }
}

impl OperateMultiQubitGate for MultiCNOT {
    // https://en.wikipedia.org/wiki/Toffoli_gate#/media/File:Qcircuit_ToffolifromCNOT.svg
    fn circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        match self.qubits().len() {
            2 => {
                circuit += operations::CNOT::new(self.qubits[0], self.qubits[1]);
            }
            3 => {
                circuit += operations::Hadamard::new(self.qubits[2]);
                circuit += operations::CNOT::new(self.qubits[1], self.qubits[2]);
                circuit +=
                    operations::PhaseShiftState1::new(self.qubits[2], -CalculatorFloat::FRAC_PI_4);
                circuit += operations::CNOT::new(self.qubits[0], self.qubits[2]);
                circuit += operations::TGate::new(self.qubits[2]);
                circuit += operations::CNOT::new(self.qubits[1], self.qubits[2]);
                circuit +=
                    operations::PhaseShiftState1::new(self.qubits[2], -CalculatorFloat::FRAC_PI_4);
                circuit += operations::CNOT::new(self.qubits[0], self.qubits[2]);
                circuit += operations::TGate::new(self.qubits[1]);
                circuit += operations::TGate::new(self.qubits[2]);
                circuit += operations::Hadamard::new(self.qubits[2]);
                circuit += operations::CNOT::new(self.qubits[0], self.qubits[1]);
                circuit += operations::TGate::new(self.qubits[0]);
                circuit +=
                    operations::PhaseShiftState1::new(self.qubits[1], -CalculatorFloat::FRAC_PI_4);
                circuit += operations::CNOT::new(self.qubits[0], self.qubits[1]);
            }
            _ => panic!("Only MultiCNOT gates with 2 or 3 controls can be turned into a circuit."),
        }
        circuit
    }
}
