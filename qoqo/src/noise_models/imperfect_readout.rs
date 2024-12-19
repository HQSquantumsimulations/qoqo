// Copyright © 2023-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use roqoqo::noise_models::{ImperfectReadoutModel, NoiseModel};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};

/// Noise model representing readout errors.
///
/// Readout errors are modeled by two probabilities in this simple model.
/// One probability to detect a 1 instead of a 0 when the quantum measurement gives 0 and
/// one probability to detect a 0 instead of a 1 when the quantum measurement gives 1.
///
/// # Example
///
/// ```Python
/// form qoqo.noise_models import ImperfectReadoutModel
///
/// model = ImperfectReadoutModel.new_with_uniform_error(3, 0.5, 0.5);
/// model = model.set_error_probabilites(2, 0.3, 0.7)
/// uniform_prob = model.prob_detect_0_as_1(0)
/// assert uniform_prob == 0.5
/// lower_prob = model.prob_detect_0_as_1(2)
/// assert lower_prob == 0.3
/// higher_prob = model.prob_detect_1_as_0(2)
/// assert higher_prob == 0.7
/// ```
#[pyclass(frozen, name = "ImperfectReadoutModel")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImperfectReadoutModelWrapper {
    internal: ImperfectReadoutModel,
}

#[noise_model_wrapper]
impl ImperfectReadoutModelWrapper {
    /// Create a new ContinuousDecoherenceModel
    #[new]
    pub fn new() -> Self {
        ImperfectReadoutModelWrapper {
            internal: ImperfectReadoutModel::new(),
        }
    }

    /// Convenience function to create uniform error probabilities
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits the uniform error probabilites should be initialized for.
    ///     prob_detect_0_as_1 (float): The error probability to detect a 1 instead of a 0 when measuring 0
    ///     prob_detect_1_as_0 (float): The error probability to detect a 0 instead of a 1 when measuring 1
    ///
    /// Returns:
    ///     ImperfectReadoutModel: The new error model
    ///
    /// Raises:
    ///     ValueError: Raised if the error probabilities are not valid (< 0 or > 1)
    #[staticmethod]
    pub fn new_with_uniform_error(
        number_qubits: usize,
        prob_detect_0_as_1: f64,
        prob_detect_1_as_0: f64,
    ) -> PyResult<Self> {
        let internal = ImperfectReadoutModel::new_with_uniform_error(
            number_qubits,
            prob_detect_0_as_1,
            prob_detect_1_as_0,
        )
        .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
        Ok(ImperfectReadoutModelWrapper { internal })
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
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<ImperfectReadoutModelWrapper> {
        let bytes = input.as_ref().extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_model: NoiseModel = bincode::deserialize(&bytes[..]).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ImperfectReadoutModel(internal) => {
                Ok(ImperfectReadoutModelWrapper { internal })
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
    pub fn from_json(input: &str) -> PyResult<ImperfectReadoutModelWrapper> {
        let noise_model: NoiseModel = serde_json::from_str(input).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ImperfectReadoutModel(internal) => {
                Ok(ImperfectReadoutModelWrapper { internal })
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
        let schema = schemars::schema_for!(ImperfectReadoutModel);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    /// Set and overwrite the measurement error probabilities
    ///
    /// Args:
    ///     qubit (int): The qubitsfor which error rates is set.
    ///     prob_detect_0_as_1 (float): The error probability to detect a 1 instead of a 0 when measuring 0
    ///     prob_detect_1_as_0 (float): The error probability to detect a 0 instead of a 1 when measuring 1
    ///
    /// Returns:
    ///     Self: The updated error model
    ///
    /// Raises:
    ///     ValueError: Raised if the error probabilities are not valid (< 0 or > 1).
    pub fn set_error_probabilites(
        &self,
        qubit: usize,
        prob_detect_0_as_1: f64,
        prob_detect_1_as_0: f64,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: self
                .internal
                .clone()
                .set_error_probabilites(qubit, prob_detect_0_as_1, prob_detect_1_as_0)
                .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?,
        })
    }

    /// Return probability to detect 0 as 1 for a qubit
    ///
    /// Args:
    ///     qubit (int): The qubit for which the probability is returned.
    ///
    /// Returns:
    ///     float: The probability to detect 0 as 1 for the qubit
    pub fn prob_detect_0_as_1(&self, qubit: usize) -> f64 {
        self.internal.prob_detect_0_as_1(&qubit)
    }

    /// Return probability to detect 1 as 0 for a qubit
    ///
    /// Args:
    ///     qubit (int): The qubit for which the probability is returned.
    ///
    /// Returns:
    ///     float: The probability to detect 1 as 0 for the qubit
    pub fn prob_detect_1_as_0(&self, qubit: usize) -> f64 {
        self.internal.prob_detect_1_as_0(&qubit)
    }
}
