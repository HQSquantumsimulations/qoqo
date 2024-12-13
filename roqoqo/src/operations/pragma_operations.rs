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
//
//! Collection of roqoqo PRAGMA operations.
//!

use crate::operations::Operation;
use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperateMultiQubit, OperatePragma, OperatePragmaNoise,
    OperatePragmaNoiseProba, OperateSingleQubit, RoqoqoError, Substitute, SupportedVersion,
};
use crate::Circuit;
#[cfg(feature = "json_schema")]
use crate::{Array1C64Def, Array2C64Def, Array2f64Def};
#[cfg(feature = "serialize")]
use bincode::serialize;
use nalgebra::{matrix, Matrix4};
use ndarray::{array, Array, Array1, Array2};
use num_complex::Complex64;
use qoqo_calculator::{Calculator, CalculatorFloat};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use super::InvolvedClassical;

/// This PRAGMA Operation sets the number of measurements of the circuit.
///
/// This is used for backends that allow setting the number of tries. However, setting the number of
/// measurements does not allow access to the underlying wavefunction or density matrix.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaSetNumberOfMeasurements {
    /// The number of measurements.
    number_measurements: usize,
    /// The register for the readout.
    readout: String,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetNumberOfMeasurements: &[&str; 3] = &[
    "Operation",
    "PragmaOperation",
    "PragmaSetNumberOfMeasurements",
];

// Implementing the InvolveQubits trait for PragmaSetNumberOfMeasurements.
impl InvolveQubits for PragmaSetNumberOfMeasurements {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation sets the statevector of a quantum register.
///
/// The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
/// operation allows you to set the state of the qubits to a state of your choosing.
///
/// # Example
///
/// For instance, to initialize the | Ψ- > Bell state, we pass the following `statevec` to
/// the PragmaSetStateVector operation.
///
/// ```
/// use ndarray::{array, Array1};
/// use num_complex::Complex64;
/// use roqoqo::operations::PragmaSetStateVector;
///
/// let statevec: Array1<Complex64> = array![
///     Complex64::new(0.0, 0.0),
///     Complex64::new(1.0 / (2.0_f64).sqrt(), 0.0),
///     Complex64::new(-1.0 / (2.0_f64).sqrt(), 0.0),
///     Complex64::new(0.0, 0.0)
/// ];
///
/// let pragma = PragmaSetStateVector::new(statevec.clone());
/// ```
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaSetStateVector {
    /// The statevector that is initialized.
    statevector: Array1<Complex64>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PragmaSetStateVector {
    fn schema_name() -> String {
        "PragmaSetStateVector".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperPragmaSetStateVector>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[allow(dead_code)]
struct SchemaHelperPragmaSetStateVector {
    /// The statevector that is initialized.
    #[serde(with = "Array1C64Def")]
    statevector: Array1<Complex64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetStateVector: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaSetStateVector"];

// Implementing the InvolveQubits trait for PragmaSetStateVector.
impl InvolveQubits for PragmaSetStateVector {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// This PRAGMA Operation sets the density matrix of a quantum register.
///
/// The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
/// operation allows you to set the state of the qubits by setting a density matrix of your choosing.
///
/// # Example
///
/// ```
/// use ndarray::{array, Array2};
/// use num_complex::Complex64;
/// use roqoqo::operations::PragmaSetDensityMatrix;
///
/// let matrix: Array2<Complex64> = array![
///    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
///    [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
/// ];
///
/// let pragma = PragmaSetDensityMatrix::new(matrix.clone());
/// ```
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaSetDensityMatrix {
    /// The density matrix that is initialized.
    density_matrix: Array2<Complex64>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PragmaSetDensityMatrix {
    fn schema_name() -> String {
        "PragmaSetDensityMatrix".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperPragmaSetDensityMatrix>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[allow(dead_code)]
struct SchemaHelperPragmaSetDensityMatrix {
    /// The density matrix that is initialized.
    #[serde(with = "Array2C64Def")]
    density_matrix: Array2<Complex64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSetDensityMatrix: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaSetDensityMatrix"];

// Implementing the InvolveQubits trait for PragmaSetDensityMatrix.
impl InvolveQubits for PragmaSetDensityMatrix {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// The repeated gate PRAGMA operation.
///
/// This PRAGMA Operation repeats the next gate in the circuit the given number of times to increase the rate for error mitigation.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaRepeatGate {
    /// The number of times the following gate is repeated.
    repetition_coefficient: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaRepeatGate: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaRepeatGate"];

// Implementing the InvolveQubits trait for PragmaRepeatGate.
impl InvolveQubits for PragmaRepeatGate {
    /// Lists all involved qubits (here, all).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// The statistical overrotation PRAGMA operation.
///
/// This PRAGMA applies a statistical overrotation to the next rotation gate in the circuit, which
/// matches the hqslang name in the `gate` parameter of PragmaOverrotation and the involved qubits in `qubits`.
///
/// The applied overrotation corresponds to adding a random number to the rotation angle.
/// The random number is drawn from a normal distribution with mean `0`
/// and standard deviation `variance` and is multiplied by the `amplitude`.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::OperatePragma,
    roqoqo_derive::OperateMultiQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
// #[cfg_attr(feature = "overrotate")]
pub struct PragmaOverrotation {
    /// The unique hqslang name of the gate to overrotate.
    gate_hqslang: String,
    /// The qubits of the gate to overrotate.
    qubits: Vec<usize>,
    /// The amplitude the random number is multiplied by.
    amplitude: f64,
    /// The standard deviation of the normal distribution the random number is drawn from.
    variance: f64,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaOverrotation: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaOverrotation",
];

/// This PRAGMA Operation boosts noise and overrotations in the circuit.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaBoostNoise {
    /// The coefficient by which the noise is boosted, i.e. the number by which the gate time is multiplied.
    noise_coefficient: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaBoostNoise: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaBoostNoise"];

// Implementing the InvolveQubits trait for PragmaBoostNoise.
impl InvolveQubits for PragmaBoostNoise {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation signals the STOP of a parallel execution block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaStopParallelBlock {
    /// The qubits involved in parallel execution block.
    qubits: Vec<usize>,
    /// The time for the execution of the block in seconds.
    execution_time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStopParallelBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStopParallelBlock",
];

/// The global phase PRAGMA operation.
///
/// This PRAGMA Operation signals that the quantum register picks up a global phase,
/// i.e. it provides information that there is a global phase to be considered.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaGlobalPhase {
    /// The picked up global phase.
    phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGlobalPhase: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaGlobalPhase"];

// Implementing the InvolveQubits trait for PragmaGlobalPhase.
impl InvolveQubits for PragmaGlobalPhase {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// This PRAGMA Operation makes the quantum hardware wait a given amount of time.
///
/// This PRAGMA Operation is used for error mitigation reasons, for instance.
/// It can be used to boost the noise on the qubits since it gets worse with time.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaSleep {
    /// The qubits involved in the sleep block.
    qubits: Vec<usize>,
    /// Time for the execution of the operation in seconds.
    sleep_time: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaSleep: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaSleep",
];

/// This PRAGMA Operation resets the chosen qubit to the zero state.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaActiveReset {
    /// The qubit to be reset.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaActiveReset: &[&str; 4] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaActiveReset",
];

/// This PRAGMA Operation signals the START of a decomposition block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaStartDecompositionBlock {
    /// The qubits involved in the decomposition block.
    qubits: Vec<usize>,
    /// The reordering dictionary of the block.
    reordering_dictionary: HashMap<usize, usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStartDecompositionBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStartDecompositionBlock",
];

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaStartDecompositionBlock {
    /// Remaps qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        let mut new_qubits: Vec<usize> = Vec::new();
        for q in &self.qubits {
            new_qubits.push(*mapping.get(q).ok_or(Err("")).map_err(
                |_x: std::result::Result<&usize, &str>| RoqoqoError::QubitMappingError {
                    qubit: *q,
                },
            )?)
        }

        let mut mutable_reordering: HashMap<usize, usize> = HashMap::new();
        for (old_qubit, new_qubit) in self.reordering_dictionary.clone() {
            let old_remapped = *mapping.get(&old_qubit).unwrap_or(&old_qubit);
            let new_remapped = *mapping.get(&new_qubit).unwrap_or(&new_qubit);
            mutable_reordering.insert(old_remapped, new_remapped);
        }

        Ok(PragmaStartDecompositionBlock::new(
            new_qubits,
            mutable_reordering,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, _calculator: &Calculator) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }
}

/// This PRAGMA Operation signals the STOP of a decomposition block.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateMultiQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaStopDecompositionBlock {
    /// The qubits involved in the decomposition block.
    qubits: Vec<usize>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaStopDecompositionBlock: &[&str; 4] = &[
    "Operation",
    "MultiQubitOperation",
    "PragmaOperation",
    "PragmaStopDecompositionBlock",
];

/// The damping PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a pure damping error corresponding to zero temperature environments.
///
/// # Note
///
/// Damping means going from state `|1>` to `|0>` and corresponds to zero-temperature in a physical
/// device where `|0>` is the ground state.
/// With respect to the definition of the Pauli operator `Z`, `|0>` is the excited state and damping leads to
/// an increase in energy.
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaDamping {
    /// The qubit on which to apply the damping.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the damping (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDamping: &[&str; 6] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaNoiseProbaOperation",
    "PragmaDamping",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDamping {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        // let prob: f64 = f64::try_from(self.probability())?;
        let t1_decay: f64 = f64::try_from(-self.gate_time.clone() * self.rate.clone())?.exp();
        let t2_decay: f64 = f64::try_from(-self.gate_time.clone() * self.rate.clone() * 0.5)?.exp();
        // let sqrt: f64 = (1.0 - prob).sqrt();

        Ok(array![
            [1.0, 0.0, 0.0, 1.0 - t1_decay],
            [0.0, t2_decay, 0.0, 0.0],
            [0.0, 0.0, t2_decay, 0.0],
            [0.0, 0.0, 0.0, t1_decay],
        ])
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// OperatePragmaNoiseProba trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoiseProba for PragmaDamping {
    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            (self.gate_time.clone() * self.rate.clone() * (-1.0)).exp() * (-1.0) + 1.0;
        prob
    }
}

/// The depolarising PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a depolarising error corresponding to infinite temperature environments.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaDepolarising {
    /// The qubit on which to apply the depolarising.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the depolarisation (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDepolarising: &[&str; 6] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaNoiseProbaOperation",
    "PragmaDepolarising",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDepolarising {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let t1_decay: f64 = f64::try_from(-self.gate_time.clone() * self.rate.clone() * 1.0)?.exp();
        let t2_decay: f64 = f64::try_from(-self.gate_time.clone() * self.rate.clone() * 1.0)?.exp();
        Ok(array![
            [0.5 + 0.5 * t1_decay, 0.0, 0.0, 0.5 - 0.5 * t1_decay],
            [0.0, t2_decay, 0.0, 0.0],
            [0.0, 0.0, t2_decay, 0.0],
            [0.5 - 0.5 * t1_decay, 0.0, 0.0, 0.5 + 0.5 * t1_decay],
        ])
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// OperatePragmaNoiseProba trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoiseProba for PragmaDepolarising {
    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            ((self.gate_time.clone() * self.rate.clone() * (-1.0)).exp() * (-1.0) + 1.0) * 0.75;
        prob
    }
}

/// The dephasing PRAGMA noise Operation.
///
/// This PRAGMA Operation applies a pure dephasing error.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaDephasing {
    /// The qubit on which to apply the dephasing.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the dephasing (in 1/second).
    rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaDephasing: &[&str; 6] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaNoiseProbaOperation",
    "PragmaDephasing",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaDephasing {
    /// Returns the superoperator matrix of the operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.rate.clone())?;

        let pre_exp: f64 = -2.0 * gate_time * rate;
        let prob: f64 = (1.0 / 2.0) * (1.0 - pre_exp.exp());

        Ok(array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0 - 2.0 * prob, 0.0, 0.0],
            [0.0, 0.0, 1.0 - 2.0 * prob, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// OperatePragmaNoiseProba trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoiseProba for PragmaDephasing {
    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time` and `rate`.
    fn probability(&self) -> CalculatorFloat {
        let prob: CalculatorFloat =
            ((self.gate_time.clone() * self.rate.clone() * (-2.0)).exp() * (-1.0) + 1.0) * 0.5;
        prob
    }
}

/// The random noise PRAGMA operation.
///
/// This PRAGMA Operation applies a stochastically unravelled combination of dephasing and depolarising.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaRandomNoise {
    /// The qubit the PRAGMA Operation is applied to.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The error rate of the depolarisation (in 1/second).
    depolarising_rate: CalculatorFloat,
    /// The error rate of the dephasing (in 1/second).
    dephasing_rate: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaRandomNoise: &[&str; 6] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaNoiseProbaOperation",
    "PragmaRandomNoise",
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaRandomNoise {
    /// Returns the superoperator matrix of the operation. For the RandomNoise pragma, the superoperator
    /// is the effective superoperator after averaging over many trajectories: the dephasing superoperator.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        let rate: f64 = f64::try_from(self.dephasing_rate.clone())?;

        let pre_exp: f64 = -2.0 * gate_time * rate;
        let prob: f64 = (1.0 / 2.0) * (1.0 - pre_exp.exp());

        Ok(array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0 - 2.0 * prob, 0.0, 0.0],
            [0.0, 0.0, 1.0 - 2.0 * prob, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// OperatePragmaNoiseProba trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoiseProba for PragmaRandomNoise {
    /// Returns the probability of the noise gate affecting the qubit, based on its `gate_time`, `depolarising_rate` and `dephasing_rate`.
    fn probability(&self) -> CalculatorFloat {
        let rates = [
            self.depolarising_rate.clone() / 4.0,
            self.depolarising_rate.clone() / 4.0,
            (self.depolarising_rate.clone() / 4.0) + self.dephasing_rate.clone(),
        ];
        (rates[0].clone() + &rates[1] + &rates[2]) * &self.gate_time
    }
}

/// The general noise PRAGMA operation.
///
/// This PRAGMA operation applies a noise term according to the given rates.
/// The rates are represented by a 3x3 matrix,  where the coefficients correspond to the following summands
/// expanded from the first term of the non-coherent part of the Lindblad equation:
///
/// d/dt * ρ = Σ Mij * Li * ρ * Lj† - 1/2 * ( Lj† * Li * ρ + ρ * Lj† * Li),
///
/// where the indices i and j run from 0 to 2
///
/// with L0 = σ+, L1 = σ- and L3 = σz.
/// Applying the Pragma with a given `gate_time` corresponds to applying the full time-evolution under the Lindblad equation for `gate_time` time.
///
///  Note: as long as gate times and decoherence rates are scaled inversely
///  any kind of units can be used. However, we recommend using nanoseconds
///  and inverse nanosecconds as units for gate times and decoherence rates.
///
/// # Example
///
/// ```
/// use ndarray::{array, Array2};
/// use roqoqo::operations::PragmaGeneralNoise;
/// use qoqo_calculator::CalculatorFloat;
///
/// let rates: Array2<f64> = array![
///    [
///         1.0,
///         0.0,
///         0.0
///     ],
///     [
///         0.0,
///         1.0,
///         0.0
///     ],
///     [
///         0.0,
///         0.0,
///         1.0
///     ],
/// ];
/// let pragma = PragmaGeneralNoise::new(
///     0,
///     CalculatorFloat::from(0.005),
///     rates.clone(),
/// );
/// ```
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PragmaGeneralNoise {
    /// The qubit the PRAGMA Operation is applied to.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The rates representing the general noise matrix M (a 3x3 matrix).
    rates: Array2<f64>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PragmaGeneralNoise {
    fn schema_name() -> String {
        "PragmaGeneralNoise".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperPragmaGeneralNoise>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[allow(dead_code)]
struct SchemaHelperPragmaGeneralNoise {
    /// The qubit the PRAGMA Operation is applied to.
    qubit: usize,
    /// The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    gate_time: CalculatorFloat,
    /// The rates representing the general noise matrix M (a 3x3 matrix).
    #[serde(with = "Array2f64Def")]
    rates: Array2<f64>,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaGeneralNoise: &[&str; 5] = &[
    "Operation",
    "SingleQubitOperation",
    "PragmaOperation",
    "PragmaNoiseOperation",
    "PragmaGeneralNoise",
];

// Collection of superoperators that appear in the Lindblad equation for a single qubit/spin with
// a basis of the form 0: sigma+ 1:sigma- 2: sigmaz
const PGN_SUPEROP: [[Matrix4<f64>; 3]; 3] = [
    [
        // sigma+ sigma+
        matrix![
            0., 0., 0., 1.;
            0., -0.5, 0., 0.;
            0., 0., -0.5, 0.;
            0., 0., 0., -1.;
        ],
        // sigma+ sigma-
        matrix![
            0., 0., 0., 0.;
            0., 0., 1., 0.;
            0., 0., 0., 0.;
            0., 0., 0., 0.;
        ],
        // sigma+ sigmaz
        matrix![
            0., 0., 0.5, 0.;
            -0.5, 0., 0., -1.5;
            0., 0., 0., 0.;
            0., 0., -0.5, 0.;
        ],
    ],
    [
        // sigma- sigma+
        matrix![
            0., 0., 0., 0.;
            0., 0., 0., 0.;
            0., 1., 0., 0.;
            0., 0., 0., 0.;
        ],
        // sigma- sigma-
        matrix![
        -1., 0., 0., 0.;
        0., -0.5, 0., 0.;
        0., 0., -0.5, 0.;
        1., 0., 0., 0.;
                ],
        // sigma- sigmaz
        matrix![
            0., 0.5, 0., 0.;
            0., 0., 0., 0.;
            1.5, 0., 0., 0.5;
            0., -0.5, 0., 0.;
        ],
    ],
    [
        //  sigmaz sigma+
        matrix![
            0., 0.5, 0., 0.;
            0., 0., 0., 0.;
            -0.5, 0., 0., -1.5;
            0., -0.5, 0., 0.;
        ],
        // sigmaz sigma-
        matrix![
            0., 0., 0.5, 0.;
            1.5, 0., 0., 0.5;
            0., 0., 0., 0.;
            0., 0., -0.5, 0.;
        ],
        // sigmaz sigmaz
        matrix![
            0., 0., 0., 0.;
            0., -2., 0., 0.;
            0., 0., -2., 0.;
            0., 0., 0., 0.;
        ],
    ],
];

/// OperatePragmaNoise trait creating necessary functions for a PRAGMA noise Operation.
impl OperatePragmaNoise for PragmaGeneralNoise {
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
        let gate_time: f64 = f64::try_from(self.gate_time.clone())?;
        // Creating the superoperator that propagates the density matrix in vector form scaled by rate and time
        let mut superop = Matrix4::<f64>::default();
        for (i, row) in PGN_SUPEROP.iter().enumerate() {
            for (j, op) in row.iter().clone().enumerate() {
                let tmp_superop: Matrix4<f64> = *op;
                superop += gate_time * self.rates[(i, j)] * tmp_superop;
            }
        }
        // Integrate superoperator for infinitesimal time to get superoperator for given rate and gate-time
        // Use exponential
        let mut exp_superop: Matrix4<f64> = superop.exp();
        // transpose because NAlgebra matrix iter is column major
        exp_superop.transpose_mut();
        let mut tmp_iter = exp_superop.iter();
        // convert to ndarray.
        let array: Array2<f64> = Array::from_shape_simple_fn((4, 4), || *tmp_iter.next().unwrap());

        Ok(array)
    }

    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: CalculatorFloat) -> Self {
        let mut new = self.clone();
        new.gate_time = power * self.gate_time.clone();
        new
    }
}

/// The conditional PRAGMA operation.
///
/// This PRAGMA executes a circuit when the condition bit/bool stored in a [crate::registers::BitRegister] is true.
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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaConditional {
    /// The name of the [crate::registers::BitRegister] containting the condition bool value.
    condition_register: String,
    /// The index in the [crate::registers::BitRegister] containting the condition bool value.
    condition_index: usize,
    /// The circuit executed if the condition is met.
    circuit: Circuit,
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaConditional: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaConditional"];

// Implementing the InvolveQubits trait for PragmaConditional.
impl InvolveQubits for PragmaConditional {
    /// Lists all involved qubits.
    fn involved_qubits(&self) -> InvolvedQubits {
        self.circuit.involved_qubits()
    }

    fn involved_classical(&self) -> super::InvolvedClassical {
        let mut new_set: HashSet<(String, usize)> = HashSet::new();
        new_set.insert((self.condition_register.clone(), self.condition_index));
        super::InvolvedClassical::Set(new_set)
    }
}

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaConditional {
    /// Remaps qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.remap_qubits(mapping).unwrap();
        Ok(PragmaConditional::new(
            self.condition_register.clone(),
            self.condition_index,
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.substitute_parameters(calculator).unwrap();
        Ok(PragmaConditional::new(
            self.condition_register.clone(),
            self.condition_index,
            new_circuit,
        ))
    }
}

/// A circuit controlled by a qubit.
///
/// The circuit is applied when the qubit is in state 1.
/// Note that this is a unitary operation (for example a CNOT(0,1)
/// is equvalent to a PragmaControlledCircuit(0, [PauliX(1)]) but it cannot be represented
/// by a unitary operation in qoqo for arbitraty circuits.
///
#[derive(Debug, Clone, PartialEq, roqoqo_derive::Operate, roqoqo_derive::OperatePragma)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaControlledCircuit {
    /// The qubit controlling if the circuit is applied. Circuit is applied for qubit in state 1.
    controlling_qubit: usize,
    /// The circuit executed if the condition is met.
    circuit: Circuit,
}

impl SupportedVersion for PragmaControlledCircuit {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        if self.circuit.minimum_supported_roqoqo_version() > (1, 5, 0) {
            return self.circuit.minimum_supported_roqoqo_version();
        }
        (1, 5, 0)
    }
}

impl super::ImplementedIn1point5 for PragmaControlledCircuit {}

#[allow(non_upper_case_globals)]
const TAGS_PragmaControlledCircuit: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaControlledCircuit"];

// Implementing the InvolveQubits trait for PragmaControlledCircuit.
impl InvolveQubits for PragmaControlledCircuit {
    /// Lists all involved qubits.
    fn involved_qubits(&self) -> InvolvedQubits {
        match self.circuit.involved_qubits() {
            InvolvedQubits::All => InvolvedQubits::All,
            InvolvedQubits::None => {
                let set: HashSet<usize> = [self.controlling_qubit].into_iter().collect();
                InvolvedQubits::Set(set)
            }
            InvolvedQubits::Set(s) => {
                let mut set = s;
                set.insert(self.controlling_qubit);
                InvolvedQubits::Set(set)
            }
        }
    }

    fn involved_classical(&self) -> super::InvolvedClassical {
        super::InvolvedClassical::None
    }
}

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaControlledCircuit {
    /// Remaps qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.remap_qubits(mapping).unwrap();
        crate::operations::check_valid_mapping(mapping)?;
        let new_controlling_qubit = mapping
            .get(&self.controlling_qubit)
            .unwrap_or(&self.controlling_qubit);
        Ok(PragmaControlledCircuit::new(
            *new_controlling_qubit,
            new_circuit,
        ))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.substitute_parameters(calculator).unwrap();
        Ok(PragmaControlledCircuit::new(
            self.controlling_qubit,
            new_circuit,
        ))
    }
}

/// A wrapper around backend specific PRAGMA operations capable of changing a device.
///
/// This PRAGMA is a thin wrapper around device specific operations that can change
/// device properties.
///
/// # NOTE
///
/// Since this PRAGMA uses serde and bincode to store a representation of the wrapped
/// operation internally it is only available when roqoqo is built with the `serialize` feature
#[derive(
    Debug, Clone, PartialEq, Eq, roqoqo_derive::SupportedVersion, roqoqo_derive::OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaChangeDevice {
    /// The tags of the wrapped operation.
    pub wrapped_tags: Vec<String>,
    /// The hqslang name of the wrapped operation.
    pub wrapped_hqslang: String,
    /// Binary representation of the wrapped operation using serde and bincode.
    pub wrapped_operation: Vec<u8>,
}
#[cfg_attr(feature = "dynamic", typetag::serde)]
impl Operate for PragmaChangeDevice {
    fn tags(&self) -> &'static [&'static str] {
        TAGS_PragmaChangeDevice
    }
    fn hqslang(&self) -> &'static str {
        "PragmaChangeDevice"
    }
    fn is_parametrized(&self) -> bool {
        false
    }
}
impl PragmaChangeDevice {
    #[cfg(feature = "serialize")]
    pub fn new<T>(wrapped_pragma: &T) -> Result<Self, RoqoqoError>
    where
        T: Operate,
        T: Serialize,
    {
        Ok(Self {
            wrapped_tags: wrapped_pragma
                .tags()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            wrapped_hqslang: wrapped_pragma.hqslang().to_string(),
            wrapped_operation: serialize(wrapped_pragma).map_err(|err| {
                RoqoqoError::SerializationError {
                    msg: format!("{:?}", err),
                }
            })?,
        })
    }
}
#[allow(non_upper_case_globals)]
const TAGS_PragmaChangeDevice: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaChangeDevice"];

// Implementing the InvolveQubits trait for PragmaConditional.
impl InvolveQubits for PragmaChangeDevice {
    /// Lists all involved qubits.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaChangeDevice {
    /// Remaps qubits in clone of the operation.
    /// This is not supported  for PragmaChangeDevice and should throw and error when a non-trivial remapping
    /// is used
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        crate::operations::check_valid_mapping(mapping)?;
        match mapping.iter().find(|(x, y)| x != y) {
            Some((x, _)) => Err(RoqoqoError::QubitMappingError { qubit: *x }),
            None => Ok(self.clone()),
        }
    }

    #[allow(unused_variables)]
    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }
}

/// This PRAGMA repeats a circuit .
///
#[derive(Debug, Clone, PartialEq, roqoqo_derive::Operate, roqoqo_derive::OperatePragma)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaLoop {
    /// The name of the classical readout register.
    repetitions: CalculatorFloat,
    /// The Circuit that is looped.
    circuit: Circuit,
}
impl super::ImplementedIn1point1 for PragmaLoop {}

impl SupportedVersion for PragmaLoop {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 1, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaLoop: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaLoop"];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaLoop {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_circuit = self.circuit.remap_qubits(mapping)?;
        Ok(PragmaLoop::new(self.repetitions.clone(), new_circuit))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_repetitions = calculator.parse_get(self.repetitions.clone())?;
        let new_circuit = self.circuit.substitute_parameters(calculator)?;
        Ok(PragmaLoop::new(new_repetitions.into(), new_circuit))
    }
}

// Implements the InvolveQubits trait for PragmaLoop.
impl InvolveQubits for PragmaLoop {
    /// Lists all involved qubits (here: All).
    fn involved_qubits(&self) -> InvolvedQubits {
        self.circuit.involved_qubits()
    }

    fn involved_classical(&self) -> InvolvedClassical {
        let mut involved = InvolvedClassical::None;
        for op in self.circuit.iter() {
            let tmp_involved = op.involved_classical();
            match &tmp_involved {
                InvolvedClassical::All(x) => {
                    return InvolvedClassical::All(x.clone());
                }
                InvolvedClassical::AllQubits(x) => {
                    return InvolvedClassical::AllQubits(x.clone());
                }
                InvolvedClassical::None => (),
                InvolvedClassical::Set(x) => match involved {
                    InvolvedClassical::All(y) => {
                        return InvolvedClassical::All(y);
                    }
                    InvolvedClassical::AllQubits(y) => {
                        return InvolvedClassical::AllQubits(y);
                    }
                    InvolvedClassical::None => involved = tmp_involved,
                    InvolvedClassical::Set(y) => {
                        let mut combined = x.clone();
                        combined.extend(y);
                        involved = InvolvedClassical::Set(combined)
                    }
                },
            }
        }
        involved
    }
}

/// This PRAGMA annotates an Operation.
///
#[derive(Debug, Clone, PartialEq, roqoqo_derive::OperatePragma)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaAnnotatedOp {
    /// The Operation to be annotated.
    pub operation: Box<Operation>,
    /// The annotation.
    pub annotation: String,
}

#[cfg_attr(feature = "dynamic", typetag::serde)]
impl Operate for PragmaAnnotatedOp {
    fn tags(&self) -> &'static [&'static str] {
        TAGS_PragmaAnnotatedOp
    }
    fn hqslang(&self) -> &'static str {
        "PragmaAnnotatedOp"
    }
    fn is_parametrized(&self) -> bool {
        self.operation.is_parametrized()
    }
}

impl PragmaAnnotatedOp {
    /// Create a PragmaAnnotatedOp instance.
    ///
    /// # Arguments
    ///
    /// * `operation` - The Operation to be annotated.
    /// * `annotation`` - The annotation.
    pub fn new(operation: Operation, annotation: String) -> Self {
        Self {
            operation: Box::new(operation),
            annotation,
        }
    }
}

impl super::ImplementedIn1point8 for PragmaAnnotatedOp {}

impl SupportedVersion for PragmaAnnotatedOp {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 8, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaAnnotatedOp: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaAnnotatedOp"];

/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for PragmaAnnotatedOp {
    /// Remaps qubits in operations in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let new_op = self.operation.remap_qubits(mapping)?;
        Ok(PragmaAnnotatedOp::new(new_op, self.annotation.clone()))
    }

    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let new_op = self.operation.substitute_parameters(calculator)?;
        Ok(PragmaAnnotatedOp::new(new_op, self.annotation.clone()))
    }
}

// Implements the InvolveQubits trait for PragmaAnnotatedOp.
impl InvolveQubits for PragmaAnnotatedOp {
    fn involved_qubits(&self) -> InvolvedQubits {
        self.operation.involved_qubits()
    }
}

/// This PRAGMA sets the number of repetitions for stochastic simulations of the quantum circuit.
///
/// This is different from the number of measurements, which is set either with
/// PragmaSetNumberOfMeasurements or with PragmaRepeatedMeasurement. PragmaSimulationRepetitions
/// only applies to stochastic simulations, i.e. simulations of quantum circuits that involve either
/// multiple subsequent measurements on the same qubits, or operations on qubits that have already
/// been measured, and sets the number of times that the whole circuit is simulated in order to obtain
/// sufficient statistics.
///
#[cfg(feature = "unstable_simulation_repetitions")]
#[derive(Debug, Clone, PartialEq, Eq, roqoqo_derive::Substitute, roqoqo_derive::OperatePragma)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PragmaSimulationRepetitions {
    repetitions: usize,
}

#[cfg(feature = "unstable_simulation_repetitions")]
#[allow(non_upper_case_globals)]
const TAGS_PragmaSimulationRepetitions: &[&str; 3] = &[
    "Operation",
    "PragmaOperation",
    "PragmaSimulationRepetitions",
];

// Implementing the InvolveQubits trait for PragmaSimulationRepetitions.
#[cfg(feature = "unstable_simulation_repetitions")]
impl InvolveQubits for PragmaSimulationRepetitions {
    /// Lists all involved qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

#[cfg(feature = "unstable_simulation_repetitions")]
#[cfg_attr(feature = "dynamic", typetag::serde)]
impl Operate for PragmaSimulationRepetitions {
    fn tags(&self) -> &'static [&'static str] {
        TAGS_PragmaSimulationRepetitions
    }
    fn hqslang(&self) -> &'static str {
        "PragmaSimulationRepetitions"
    }
    fn is_parametrized(&self) -> bool {
        false
    }
}

#[cfg(feature = "unstable_simulation_repetitions")]
impl PragmaSimulationRepetitions {
    /// Create a PragmaSimulationRepetitions instance.
    ///
    /// # Arguments
    ///
    /// * `repetitions` - The number of simulation repetitions.
    pub fn new(repetitions: usize) -> Self {
        Self { repetitions }
    }

    /// Getter for the number of repetitions.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of simulation repetitions
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

#[cfg(feature = "unstable_simulation_repetitions")]
impl super::ImplementedIn1point17 for PragmaSimulationRepetitions {}

#[cfg(feature = "unstable_simulation_repetitions")]
impl SupportedVersion for PragmaSimulationRepetitions {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 17, 0)
    }
}
