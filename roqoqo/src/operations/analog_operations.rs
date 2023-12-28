// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperateSpinsAnalog, Substitute, SubstituteModes,
    SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;
use std::collections::{HashMap, HashSet};
use struqture::spins::{PauliProduct, SpinHamiltonian};
use struqture::OperateOnDensityMatrix;
use struqture::SpinIndex;

/// Implements the continuous time spin Hamiltonian
///
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ApplyConstantSpinHamiltonian {
    pub hamiltonian: SpinHamiltonian,
    pub time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_ApplyConstantSpinHamiltonian: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "OperateSpinsAnalog",
    "ApplyConstantSpinHamiltonian",
];

impl OperateSpinsAnalog for ApplyConstantSpinHamiltonian {
    fn spin(&self) -> Vec<usize> {
        let mut qubit_set = HashSet::new();
        println!("{:?}", self.hamiltonian.keys());
        for pps in self.hamiltonian.keys() {
            for (index, _) in PauliProduct::iter(pps) {
                qubit_set.insert(*index);
            }
        }
        let mut qubits = Vec::from_iter(qubit_set);
        qubits.sort();
        qubits
    }
}
impl SupportedVersion for ApplyConstantSpinHamiltonian {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

impl InvolveQubits for ApplyConstantSpinHamiltonian {
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ApplyTimeDependentSpinHamiltonian {
    hamiltonian: SpinHamiltonian,
    time: Vec<f64>,
    values: HashMap<String, Vec<f64>>,
}

#[allow(non_upper_case_globals)]
const TAGS_ApplyTimeDependentSpinHamiltonian: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "OperateSpinsAnalog",
    "ApplyTimeDependentSpinHamiltonian",
];

impl OperateSpinsAnalog for ApplyTimeDependentSpinHamiltonian {
    fn spin(&self) -> Vec<usize> {
        let mut qubit_set = HashSet::new();
        println!("{:?}", self.hamiltonian.keys());
        for pps in self.hamiltonian.keys() {
            for (index, _) in PauliProduct::iter(pps) {
                qubit_set.insert(*index);
            }
        }
        let mut qubits = Vec::from_iter(qubit_set);
        qubits.sort();
        qubits
    }
}

impl SupportedVersion for ApplyTimeDependentSpinHamiltonian {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

impl InvolveQubits for ApplyTimeDependentSpinHamiltonian {
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}
