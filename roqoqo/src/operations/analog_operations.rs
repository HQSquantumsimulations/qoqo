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
    ImplementedIn1point11, InvolveQubits, InvolvedQubits, Operate, OperateSpinsAnalog, Substitute,
    SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::{Calculator, CalculatorFloat};
use std::collections::{HashMap, HashSet};
use struqture::spins::SpinHamiltonian;
use struqture::OperateOnDensityMatrix;
use struqture::SpinIndex;

/// Implements the continuous time, constant spin Hamiltonian
#[derive(Debug, Clone, PartialEq, roqoqo_derive::Operate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ApplyConstantSpinHamiltonian {
    /// Hamiltonian to be simulated.
    pub hamiltonian: SpinHamiltonian,
    /// The duration for which the state evolution takes place.
    pub time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_ApplyConstantSpinHamiltonian: &[&str; 3] = &[
    "Operation",
    "SpinsAnalogOperation",
    "ApplyConstantSpinHamiltonian",
];

impl ImplementedIn1point11 for ApplyConstantSpinHamiltonian {}

impl OperateSpinsAnalog for ApplyConstantSpinHamiltonian {
    fn spin(&self) -> Result<Vec<usize>, RoqoqoError> {
        let mut qubit_set = HashSet::new();
        for pps in self.hamiltonian.keys() {
            for (index, _) in pps.iter() {
                qubit_set.insert(*index);
            }
        }
        let mut qubits = Vec::from_iter(qubit_set);
        qubits.sort();
        Ok(qubits)
    }
}
impl SupportedVersion for ApplyConstantSpinHamiltonian {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

impl InvolveQubits for ApplyConstantSpinHamiltonian {
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

impl Substitute for ApplyConstantSpinHamiltonian {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        let mut new_hamiltonian = SpinHamiltonian::new();
        for (pp, value) in &self.hamiltonian {
            let new_pp = pp.remap_qubits(mapping);
            new_hamiltonian.add_operator_product(new_pp, value.clone())?;
        }

        Ok(ApplyConstantSpinHamiltonian::new(
            new_hamiltonian,
            self.time.clone(),
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let mut new_hamiltonian = self.hamiltonian.clone();
        for (key, value) in &self.hamiltonian {
            let new_value = calculator.parse_get(value.clone())?;
            new_hamiltonian.set(key.clone(), new_value.into())?;
        }
        let new_time = calculator.parse_get(self.time.clone())?;
        Ok(ApplyConstantSpinHamiltonian::new(
            new_hamiltonian,
            new_time.into(),
        ))
    }
}

/// Implements the continuous time, time-dependent spin Hamiltonian
#[derive(Debug, Clone, PartialEq, roqoqo_derive::Operate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ApplyTimeDependentSpinHamiltonian {
    /// Hamiltonian to be simulated.
    hamiltonian: SpinHamiltonian,
    /// Range of time stored as a vector. The total duration of the simulations is given by the last value in the range.
    time: Vec<f64>,
    /// Values of time-dependent parameters, appearing in `hamiltonian`, at instances given by the vector `time`.
    values: HashMap<String, Vec<f64>>,
}

#[allow(non_upper_case_globals)]
const TAGS_ApplyTimeDependentSpinHamiltonian: &[&str; 3] = &[
    "Operation",
    "SpinsAnalogOperation",
    "ApplyTimeDependentSpinHamiltonian",
];

impl ImplementedIn1point11 for ApplyTimeDependentSpinHamiltonian {}

impl OperateSpinsAnalog for ApplyTimeDependentSpinHamiltonian {
    fn spin(&self) -> Result<Vec<usize>, RoqoqoError> {
        let mut qubit_set = HashSet::new();
        for pps in self.hamiltonian.keys() {
            for (index, _) in pps.iter() {
                qubit_set.insert(*index);
            }
        }
        let mut qubits = Vec::from_iter(qubit_set);
        qubits.sort();
        Ok(qubits)
    }
}

impl SupportedVersion for ApplyTimeDependentSpinHamiltonian {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

impl InvolveQubits for ApplyTimeDependentSpinHamiltonian {
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

impl Substitute for ApplyTimeDependentSpinHamiltonian {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        let mut new_hamiltonian = SpinHamiltonian::new();
        for (pp, value) in &self.hamiltonian {
            let new_pp = pp.remap_qubits(mapping);
            new_hamiltonian.add_operator_product(new_pp, value.clone())?;
        }

        Ok(ApplyTimeDependentSpinHamiltonian::new(
            new_hamiltonian,
            self.time.clone(),
            self.values.clone(),
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let mut new_hamiltonian = self.hamiltonian.clone();
        for (key, value) in &self.hamiltonian {
            let new_value = calculator.parse_get(value.clone())?;
            new_hamiltonian.set(key.clone(), new_value.into())?;
        }
        Ok(ApplyTimeDependentSpinHamiltonian::new(
            new_hamiltonian,
            self.time.clone(),
            self.values.clone(),
        ))
    }
}
