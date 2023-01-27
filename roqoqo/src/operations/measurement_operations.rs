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
//
//! Collection of roqoqo measurement operations.

use qoqo_calculator::Calculator;
use std::collections::{HashMap, HashSet};

use super::InvolvedClassical;
use super::SupportedVersion;
use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperatePragma, OperateSingleQubit, RoqoqoError,
    Substitute,
};
use crate::Circuit;

/// Measurement gate operation.
///
/// This Operation acts on one qubit writing the result of the measurement into a readout.
/// The classical register for the readout needs to be defined in advance by using a Definition operation.
///
/// # Note
///
/// Here, it is a measurement in terms of quantum mechanics. The obtained result of a single measurement will be either a `0` or a `1`.  
/// In order to be able to derive probabilities in the post-processing, the actual measurement needs to be repeated lots of times.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct MeasureQubit {
    /// The measured qubit.
    qubit: usize,
    /// The register for the readout.
    readout: String,
    /// The index in the readout the result is saved to.
    readout_index: usize,
}

impl InvolveQubits for MeasureQubit {
    fn involved_qubits(&self) -> InvolvedQubits {
        let mut a: HashSet<usize> = HashSet::new();
        a.insert(self.qubit);
        InvolvedQubits::Set(a)
    }

    fn involved_classical(&self) -> super::InvolvedClassical {
        let mut a: HashSet<(String, usize)> = HashSet::new();
        a.insert((self.readout.clone(), self.readout_index));
        InvolvedClassical::Set(a)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_MeasureQubit: &[&str; 3] = &["Operation", "Measurement", "MeasureQubit"];

/// This PRAGMA measurement operation returns the statevector of a quantum register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaGetStateVector {
    /// The name of the classical readout register.
    readout: String,
    /// The measurement preparation Circuit, applied on a copy of the register before measurement (None if not defined, Some(Circuit) otherwise).
    circuit: Option<Circuit>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGetStateVector: &[&str; 4] = &[
    "Operation",
    "Measurement",
    "PragmaOperation",
    "PragmaGetStateVector",
];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaGetStateVector {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.remap_qubits(mapping)?),
            _ => None,
        };
        Ok(PragmaGetStateVector::new(self.readout.clone(), new_circuit))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.substitute_parameters(calculator)?),
            _ => None,
        };
        Ok(PragmaGetStateVector::new(self.readout.clone(), new_circuit))
    }
}

// Implements the InvolveQubits trait for PragmaGetStateVector.
impl InvolveQubits for PragmaGetStateVector {
    /// Lists all involved qubits (here: All).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }

    fn involved_classical(&self) -> InvolvedClassical {
        InvolvedClassical::All(self.readout.clone())
    }
}

/// This PRAGMA measurement operation returns the density matrix of a quantum register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaGetDensityMatrix {
    /// The name of the classical readout register.
    readout: String,
    /// The measurement preparation Circuit, applied on a copy of the register before measurement (None if not defined, Some(Circuit) otherwise).
    circuit: Option<Circuit>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGetDensityMatrix: &[&str; 4] = &[
    "Operation",
    "Measurement",
    "PragmaOperation",
    "PragmaGetDensityMatrix",
];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaGetDensityMatrix {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.remap_qubits(mapping)?),
            _ => None,
        };
        Ok(PragmaGetDensityMatrix::new(
            self.readout.clone(),
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.substitute_parameters(calculator)?),
            _ => None,
        };
        Ok(PragmaGetDensityMatrix::new(
            self.readout.clone(),
            new_circuit,
        ))
    }
}

// Implements the InvolveQubits trait for PragmaGetDensityMatrix.
impl InvolveQubits for PragmaGetDensityMatrix {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }

    fn involved_classical(&self) -> InvolvedClassical {
        InvolvedClassical::All(self.readout.clone())
    }
}

/// This PRAGMA measurement operation returns the vector of the occupation probabilities.
///
/// Occupation probabilities in the context of this PRAGMA Operation are probabilities of finding the quantum
/// register in each σ^z basis state. The quantum register remains unchanged by this PRAGMA measurement operation.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaGetOccupationProbability {
    /// The name of the classical readout register.
    readout: String,
    /// The measurement preparation Circuit, applied on a copy of the register before measurement (None if not defined, Some(Circuit) otherwise).
    circuit: Option<Circuit>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGetOccupationProbability: &[&str; 4] = &[
    "Operation",
    "Measurement",
    "PragmaOperation",
    "PragmaGetOccupationProbability",
];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaGetOccupationProbability {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.remap_qubits(mapping)?),
            _ => None,
        };
        Ok(PragmaGetOccupationProbability::new(
            self.readout.clone(),
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = match self.circuit.as_ref() {
            Some(x) => Some(x.substitute_parameters(calculator)?),
            _ => None,
        };
        Ok(PragmaGetOccupationProbability::new(
            self.readout.clone(),
            new_circuit,
        ))
    }
}

// Implements the InvolveQubits trait for PragmaGetOccupationProbability.
impl InvolveQubits for PragmaGetOccupationProbability {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }

    fn involved_classical(&self) -> InvolvedClassical {
        InvolvedClassical::All(self.readout.clone())
    }
}

/// This PRAGMA measurement operation returns a Pauli product expectation value.
///
/// This PRAGMA Operation returns a Pauli product expectation value after applying
/// a Rotate to another basis. It performs all of the operation on a clone of the quantum register,
/// so that the actual quantum register remains unchanged.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaGetPauliProduct {
    /// The HashMap of the pauli matrix to apply to each qubit in the form {qubit: pauli}. Allowed values to be provided for 'pauli' are: `0` = identity, `1` = PauliX, `2` = PauliY, `3` = PauliZ.
    qubit_paulis: HashMap<usize, usize>,
    /// The name of the classical readout register.
    readout: String,
    /// The measurement preparation Circuit, applied on a copy of the register before measurement.
    circuit: Circuit,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGetPauliProduct: &[&str; 4] = &[
    "Operation",
    "Measurement",
    "PragmaOperation",
    "PragmaGetPauliProduct",
];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaGetPauliProduct {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;

        let mut mutable_mapping: HashMap<usize, usize> = HashMap::new();
        for (key, val) in self.qubit_paulis.iter() {
            let new_key = mapping.get(key).unwrap_or(key);

            mutable_mapping.insert(*new_key, *val);
        }

        let new_circuit = self.circuit.remap_qubits(mapping).unwrap();
        Ok(PragmaGetPauliProduct::new(
            mutable_mapping,
            self.readout.clone(),
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.substitute_parameters(calculator).unwrap();
        Ok(PragmaGetPauliProduct::new(
            self.qubit_paulis.clone(),
            self.readout.clone(),
            new_circuit,
        ))
    }
}

// Implements the InvolveQubits trait for PragmaGetPauliProduct.
impl InvolveQubits for PragmaGetPauliProduct {
    /// Lists all involved qubits.
    fn involved_qubits(&self) -> InvolvedQubits {
        let mut new_hash_set: HashSet<usize> = HashSet::new();
        for qubit in self.qubit_paulis.keys() {
            new_hash_set.insert(*qubit);
        }
        if let InvolvedQubits::Set(tmp_set) = &self.circuit.involved_qubits() {
            for qubit in tmp_set {
                new_hash_set.insert(*qubit);
            }
        }
        InvolvedQubits::Set(new_hash_set)
    }

    fn involved_classical(&self) -> InvolvedClassical {
        InvolvedClassical::All(self.readout.clone())
    }
}

/// This PRAGMA measurement operation returns a measurement record for N repeated measurements.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaRepeatedMeasurement {
    /// The name of the classical readout register.
    readout: String,
    /// The number of times N to repeat the measurement.
    number_measurements: usize,
    /// The mapping of qubits to indices in the readout register.
    qubit_mapping: Option<HashMap<usize, usize>>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaRepeatedMeasurement: &[&str; 4] = &[
    "Operation",
    "Measurement",
    "PragmaOperation",
    "PragmaRepeatedMeasurement",
];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaRepeatedMeasurement {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        let new_mapping = match &self.qubit_mapping {
            Some(hm) => {
                let mut mutable_mapping: HashMap<usize, usize> = HashMap::new();
                for (key, val) in hm {
                    let new_key = mapping.get(key).unwrap_or(key);

                    mutable_mapping.insert(*new_key, *val);
                }
                for (key, val) in mapping.iter() {
                    if mutable_mapping.get(key).is_none() {
                        mutable_mapping.insert(*key, *val);
                    }
                }
                Some(mutable_mapping)
            }
            None => Some(mapping.clone()),
        };
        Ok(PragmaRepeatedMeasurement::new(
            self.readout.clone(),
            self.number_measurements,
            new_mapping,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, _calculator: &Calculator) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }
}

// Implements the InvolveQubits trait for PragmaRepeatedMeasurement.
impl InvolveQubits for PragmaRepeatedMeasurement {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }

    fn involved_classical(&self) -> InvolvedClassical {
        match &self.qubit_mapping {
            None => InvolvedClassical::AllQubits(self.readout.clone()),
            Some(x) => {
                let new_set: HashSet<(String, usize)> =
                    x.values().map(|v| (self.readout.clone(), *v)).collect();
                InvolvedClassical::Set(new_set)
            }
        }
    }
}
