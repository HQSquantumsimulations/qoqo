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

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperateSpinsAnalog, Substitute, SubstituteModes,
    SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;
use struqture::spins::SpinHamiltonian;
use struqture::OperateOnDensityMatrix;

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
pub struct ApplyConstantSpinHamiltonian {
    hamiltonian: SpinHamiltonian,
    time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_ApplyConstantSpinHamiltonian: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "OperateSpinsAnalog",
    "ApplyConstantSpinHamiltonian",
];

impl OperateSpinsAnalog for ApplyConstantSpinHamiltonian {
    fn spin(&self) -> &usize {
        &1 // Not sure what goes in here yet
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
    fn spin(&self) -> &usize {
        &1 // Not sure what goes in here yet
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
