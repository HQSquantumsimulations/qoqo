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
use std::panic;

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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    fn circuit(&self) -> Circuit {
        let dim = self.qubits.len();
        let mut circuit = Circuit::new();
        for q in self.qubits.iter() {
            circuit += operations::Hadamard::new(*q);
        }
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(*q - 1, *q);
        }
        circuit += operations::RotateZ::new(dim - 1, self.theta.clone());
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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
/// The CNOT gate with multiple controls
pub struct MultiQubitCNOT {
    qubits: Vec<usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_MultiQubitCNOT: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "MultiQubitCNOT",
];

impl operations::ImplementedIn1point19 for MultiQubitCNOT {}

impl SupportedVersion for MultiQubitCNOT {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 19, 0)
    }
}

impl OperateGate for MultiQubitCNOT {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 2_usize.pow(self.qubits.len() as u32);
        let mut array = Array2::eye(dim);
        array
            .slice_mut(s![dim - 2.., dim - 2..])
            .assign(&array![[0., 1.], [1., 0.]]);
        Ok(array.map(|x| x.into()))
    }
}

impl OperateMultiQubitGate for MultiQubitCNOT {
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
            _ => panic!("Only MultiQubitCNOT gates with 2 or 3 controls can be turned into a circuit."),
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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    fn circuit(&self) -> Circuit {
        let dim = self.qubits.len();
        let mut circuit = Circuit::new();
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(*q - 1, *q);
        }
        circuit += operations::RotateZ::new(dim - 1, self.theta.clone());
        for q in self.qubits[1..].iter() {
            circuit += operations::CNOT::new(dim - *q - 1, dim - *q);
        }
        circuit
    }
}

/// The gate to be replaced by a gate defined with GateDefinition gate.
///
/// The gate applies a gate previously defined by GateDefinition with the name gate_name.
#[cfg(feature = "unstable_operation_definition")]
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::Operate,
    roqoqo_derive::InvolveQubits,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct CallDefinedGate {
    /// The name of the called defined operations.
    gate_name: String,
    /// The qubits that for this call replace the qubits in the internal definition of the called gate
    /// (get replaced in order of apppearance in gate defintion).
    qubits: Vec<usize>,
    /// List of float values that replace the free parameters in the internal defintion of the called gate
    /// (get replaced in order of apppearance in gate defintion).
    free_parameters: Vec<CalculatorFloat>,
}

#[cfg(feature = "unstable_operation_definition")]
impl Substitute for CallDefinedGate {
    fn substitute_parameters(
        &self,
        calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError> {
        let mut new_params: Vec<CalculatorFloat> = vec![];
        for param in &self.free_parameters.clone() {
            new_params.push(CalculatorFloat::from(
                calculator
                    .parse_get(param.clone())
                    .map_err(RoqoqoError::CalculatorError)?,
            ));
        }

        Ok(CallDefinedGate::new(
            self.gate_name.clone(),
            self.qubits.clone(),
            new_params,
        ))
    }

    fn remap_qubits(
        &self,
        mapping: &std::collections::HashMap<usize, usize>,
    ) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        let mut new_qubits: Vec<usize> = Vec::new();
        for q in &self.qubits {
            new_qubits.push(*mapping.get(q).ok_or(Err("")).map_err(
                |_x: std::result::Result<&usize, &str>| RoqoqoError::QubitMappingError {
                    qubit: *q,
                },
            )?)
        }

        Ok(CallDefinedGate::new(
            self.gate_name.clone(),
            new_qubits,
            self.free_parameters.clone(),
        ))
    }
}

#[cfg(feature = "unstable_operation_definition")]
impl super::ImplementedIn1point13 for CallDefinedGate {}

#[cfg(feature = "unstable_operation_definition")]
impl SupportedVersion for CallDefinedGate {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 13, 0)
    }
}

#[cfg(feature = "unstable_operation_definition")]
#[allow(non_upper_case_globals)]
const TAGS_CallDefinedGate: &[&str; 3] =
    &["Operation", "MultiQubitGateOperation", "CallDefinedGate"];
