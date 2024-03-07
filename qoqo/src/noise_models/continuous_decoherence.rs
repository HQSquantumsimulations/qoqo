// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use pyo3::prelude::*;
use qoqo_macros::noise_model_wrapper;
use roqoqo::noise_models::{ContinuousDecoherenceModel, NoiseModel};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};
use struqture;
use struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper;

/// Noise model representing a continuous decoherence process on qubits.
///
/// This noise model assumes that all qubits are constantly experiencing
/// decoherence over time (e.g. due to coupling to the environment).
/// The noise for each qubit can be different but only single qubit noise is
/// included in the model.
///
/// Noise is given by the rates of the Lindblad equation.
/// The Lindblad equation is a so-called master equation for the time evolution of a density matrix.
/// For the example of a single qubit the non-coherent part of the Lindblad equation can take the following form:
///
/// d/dt * ρ = Σ Mij * Li * ρ * Lj† - 1/2 * ( Lj† * Li * ρ + ρ * Lj† * Li),
///
/// where the indices i and j run from 0 to 2
///
/// with L0 = σ+, L1 = σ- and L3 = σz.
///
/// Here the genreal incoherent part of the Lindblad equation is internally represented by a [struqture::spins::PlusMinusLindbladNoiseOperator].
///
/// To create a complex decoherence model first create the Lindblad noise and then turn it into a ContinuousDecoherenceModel.
/// For a simple decoherence model, use new to create an empty model and use the add_damping, add_excitation and add_dephasing methods.
/// For more fine control access the internal lindblad_noise directly and modify it.
///
/// Args:
///
///     noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): Optional initialisation of Noise Model with given Lindblad operator.
#[pyclass(frozen, name = "ContinuousDecoherenceModel")]
#[derive(Clone, Debug, PartialEq)]
pub struct ContinuousDecoherenceModelWrapper {
    internal: ContinuousDecoherenceModel,
}

#[noise_model_wrapper]
impl ContinuousDecoherenceModelWrapper {
    /// Create a new ContinuousDecoherenceModel
    #[new]
    pub fn new(noise_operator: Option<Py<PyAny>>) -> PyResult<Self> {
        if let Some(lindblad_operator) = noise_operator {
            let noise_operator =
                struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(
                    lindblad_operator,
                )?;
            Ok(Self {
                internal: ContinuousDecoherenceModel::from(noise_operator),
            })
        } else {
            Ok(ContinuousDecoherenceModelWrapper {
                internal: ContinuousDecoherenceModel::new(),
            })
        }
    }

    /// Return the internal Lindblad noise operator of the continuous noise model.
    ///
    /// Returns:
    ///     PlusMinusLindbladNoiseOperator: The internal Lindblad noise operator of the continuous noise
    pub fn get_noise_operator(&self) -> PlusMinusLindbladNoiseOperatorWrapper {
        PlusMinusLindbladNoiseOperatorWrapper {
            internal: struqture::spins::PlusMinusLindbladNoiseOperator::from(self.internal.clone()),
        }
    }

    /// Convert the bincode representation of the Noise-Model to a device using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized Noise-Model (in bincode form).
    ///
    /// Returns:
    ///     The deserialized Noise-Model.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to selected Noise-Model.
    #[staticmethod]
    #[pyo3(text_signature = "(input)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<ContinuousDecoherenceModelWrapper> {
        let bytes = input.extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_model: NoiseModel = bincode::deserialize(&bytes[..]).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ContinuousDecoherenceModel(internal) => {
                Ok(ContinuousDecoherenceModelWrapper { internal })
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Input cannot be deserialized to selected Noise-Model.",
            )),
        }
    }

    /// Convert the json representation of a device to a Noise-Model.
    ///
    /// Args:
    ///     input (str): The serialized device in json form.
    ///
    /// Returns:
    ///     The deserialized device.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to selected Noise-Model.
    #[staticmethod]
    #[pyo3(text_signature = "(input)")]
    pub fn from_json(input: &str) -> PyResult<ContinuousDecoherenceModelWrapper> {
        let noise_model: NoiseModel = serde_json::from_str(input).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ContinuousDecoherenceModel(internal) => {
                Ok(ContinuousDecoherenceModelWrapper { internal })
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Input cannot be deserialized to selected Noise-Model.",
            )),
        }
    }

    #[cfg(feature = "json_schema")]
    /// Return the JsonSchema for the json serialisation of the class.
    ///
    /// Returns:
    ///     str: The json schema serialized to json
    #[staticmethod]
    pub fn json_schema() -> String {
        let schema = schemars::schema_for!(ContinuousDecoherenceModel);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    /// Convenience function to add damping to several qubits
    ///
    /// Args:
    ///     qubits (list[int]): The qubits to add damping to.
    ///     rate (float): The damping rate.
    ///
    /// Returns:
    ///     ContinuousDecoherenceModel: The model with the damping added.
    pub fn add_damping_rate(&self, qubits: Vec<usize>, rate: f64) -> Self {
        Self {
            internal: self.internal.clone().add_damping_rate(&qubits, rate),
        }
    }

    /// Convenience function to add dephasing to several qubits
    ///
    /// Args:
    ///     qubits (list[int]): The qubits to add dephasing to.
    ///     rate (float): The dephasing rate.
    ///
    /// Returns:
    ///     ContinuousDecoherenceModel: The model with the dephasing added.
    pub fn add_dephasing_rate(&self, qubits: Vec<usize>, rate: f64) -> Self {
        Self {
            internal: self.internal.clone().add_dephasing_rate(&qubits, rate),
        }
    }

    /// Convenience function to add depolarising to several qubits
    ///
    /// Args:
    ///     qubits (list[int]): The qubits to add depolarising to.
    ///     rate (float): The depolarising rate.
    ///
    /// Returns:
    ///     ContinuousDecoherenceModel: The model with the depolarising added.
    pub fn add_depolarising_rate(&self, qubits: Vec<usize>, rate: f64) -> Self {
        Self {
            internal: self.internal.clone().add_depolarising_rate(&qubits, rate),
        }
    }

    /// Convenience function to add excitation to several qubits
    ///
    /// Args:
    ///     qubits (list[int]): The qubits to add excitation to.
    ///     rate (float): The excitation rate.
    ///
    /// Returns:
    ///     ContinuousDecoherenceModel: The model with the excitation added.
    pub fn add_excitation_rate(&self, qubits: Vec<usize>, rate: f64) -> Self {
        Self {
            internal: self.internal.clone().add_excitation_rate(&qubits, rate),
        }
    }
}
