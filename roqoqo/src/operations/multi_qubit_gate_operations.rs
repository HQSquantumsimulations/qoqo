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

use std::panic;

use crate::operations;
use crate::prelude::*;
use crate::Circuit;
use crate::RoqoqoError;
use ndarray::Array2;
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
#[cfg(feature = "overrotate")]
use rand_distr::{Distribution, Normal};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// The Molmer-Sorensen gate between multiple qubits.
///
/// The gate applies the rotation under the product of Pauli X operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * θ/2 * X_i0 * X_i1 * ... * X_in).
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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

/// The multi qubit Pauli-Z-Product gate.
///
/// The gate applies the rotation under the product of Pauli Z operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * θ/2 * Z_i0 * Z_i1 * ... * Z_in).
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct MultiQubitZZ {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_MultiQubitZZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "MultiQubitZZ",
];

impl OperateGate for MultiQubitZZ {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 2_usize.pow(self.qubits.len() as u32);
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        let cos: Complex64 = Complex64::new((self.theta.float()? / 2.0).cos(), 0.0);
        let sin: Complex64 = Complex64::new(0.0, -(self.theta.float()? / 2.0).sin());
        for i in 0..dim {
            // Fix the signs of the imaginary part due to the ZZZ..ZZ product
            let prefactor: f64 = (0..self.qubits.len())
                .map(|q| match i.div_euclid(2usize.pow(q as u32)) % 2 {
                    0 => 1.0,
                    1 => -1.0,
                    _ => panic!("Internal division error MuliQubitZZ"),
                })
                .product();
            array[(i, i)] = cos + prefactor * sin;
        }
        Ok(array)
    }
}

impl OperateMultiQubitGate for MultiQubitZZ {
    // Todo fill out circuit
    fn circuit(&self) -> Circuit {
        let dim = self.qubits.len();
        let mut circuit = Circuit::new();
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(*q - 1, *q);
        }
        circuit += operations::RotateZ::new(dim - 1, self.theta.clone() / 2);
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(dim - *q - 1, dim - *q);
        }
        circuit
    }
}
