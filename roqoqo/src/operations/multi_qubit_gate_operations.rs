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

use std::{f64::consts::PI, panic};

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
/// In mathematical terms the gate applies exp(-i * theta/2 * X_i0 * X_i1 * ... * X_in).
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

/// The multi qubit Pauli-Z-Product gate.
///
/// The gate applies the rotation under the product of Pauli Z operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * theta/2 * Z_i0 * Z_i1 * ... * Z_in).
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

/// The quantum Fourier transformation.
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
#[allow(clippy::upper_case_acronyms)]
pub struct QFT {
    /// The qubits involved in the QFT.
    qubits: Vec<usize>,
    /// Include qubit swaps at the end.
    swaps: bool,
    /// Do inverse QFT.
    inverse: bool,
}

const TAGS_QFT: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "QFT",
];

impl OperateGate for QFT {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = self.qubits.len();
        if !self.swaps && dim > 1 {
            return Err(RoqoqoError::GenericError {
                msg: "Unitary matrix output is only supported QFT with swapping.".into(),
            });
        }
        let n = 2_usize.pow(dim as u32);
        let mut array = Array2::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                let mut theta = 2. * PI * (i as f64 * j as f64) / (n as f64);
                if self.inverse {
                    theta *= -1.;
                }
                array[[i, j]] = Complex64::from_polar(1. / (n as f64).sqrt(), theta);
            }
        }
        Ok(array)
    }
}

impl OperateMultiQubitGate for QFT {
    fn circuit(&self) -> Circuit {
        let dim = self.qubits.len();
        let mut circuit = Circuit::new();

        if self.swaps && self.inverse {
            for i in 0..dim / 2 {
                circuit += operations::SWAP::new(self.qubits[i], self.qubits[dim - i - 1]);
            }
        }
        for i in 0..dim {
            circuit += operations::Hadamard::new(self.qubits[i]);
            for j in i + 1..dim {
                let mut theta = PI / 2.0_f64.powi((j - i) as i32);
                if self.inverse {
                    theta *= -1.;
                }
                circuit += operations::ControlledPhaseShift::new(
                    self.qubits[j],
                    self.qubits[i],
                    theta.into(),
                );
            }
        }
        if self.swaps && !self.inverse {
            for i in 0..dim / 2 {
                circuit += operations::SWAP::new(self.qubits[i], self.qubits[dim - i - 1]);
            }
        }
        circuit
    }
}
