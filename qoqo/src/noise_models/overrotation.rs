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
use roqoqo::noise_models::{
    NoiseModel, SingleQubitOverrotationDescription, SingleQubitOverrotationOnGate,
};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};

/// Description of single qubit overrotation noise model, [roqoqo::noise_models::SingleQubitOverrotationOnGate].
///
/// Consists of the raw data needed to construct a rotation gate that adds
/// overrotation: gate name and statistics (mean and standard deviation) of a Gaussian distribution
/// from which the overrotation angle is sampled.
/// Example:
///
/// ```
/// from qoqo.noise_models import SingleQubitOverrotationDescription;
/// gate = "RotateX";
/// theta_mean = 0.0;
/// theta_std = 1.0;
/// noise_desc = SingleQubitOverrotationDescription(gate, theta_mean, theta_std);
/// ```
#[pyclass(frozen, name = "SingleQubitOverrotationDescription")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SingleQubitOverrotationDescriptionWrapper {
    /// Single qubit overrotation description
    internal: SingleQubitOverrotationDescription,
}

/// Create a new SingleQubitOverrotationDescription.
///
/// # Arguments
///
/// * `gate` - The name of the gate.
/// * `theta_mean` - The mean of Gaussian distrbution from which overrotation angle is sampled.
/// * `theta_std` - The standard deviation of Gaussian distrbution from which overrotation angle is sampled.
///
/// # Returns
///
/// `Self` - New description for overrotation noise model.

#[pymethods]
impl SingleQubitOverrotationDescriptionWrapper {
    /// Return description to generate single qubit overrotation noise
    ///
    /// Args:
    ///     gate: The name qubit gate.
    ///     theta_mean: The mean of Gaussian distrbution from which overrotation angle is sampled.
    ///     theta_std: The standard deviation of Gaussian distrbution from which overrotation angle is sampled.
    ///
    /// Returns:
    ///     `self`.
    #[new]
    pub fn new(
        gate: &str,
        theta_mean: f64,
        theta_std: f64,
    ) -> SingleQubitOverrotationDescriptionWrapper {
        SingleQubitOverrotationDescriptionWrapper {
            internal: SingleQubitOverrotationDescription::new(gate, theta_mean, theta_std),
        }
    }

    /// Return a copy of the device (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     A deep copy of self.
    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    /// Create deep copy of Noise-Model.
    ///
    /// Returns:
    ///     A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: &Bound<PyAny>) -> Self {
        self.clone()
    }

    /// Return the bincode representation of SingleQubitOverrotationDescription using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized SingleQubitOverrotationDescription (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize SingleQubitOverrotationDescription to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<pyo3::types::PyByteArray>> {
        let noise_descp = self.internal.clone();
        let serialized = bincode::serialize(&noise_descp).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err(
                "Cannot serialize Noise-Overrotation description to bytes",
            )
        })?;
        let b: Py<pyo3::types::PyByteArray> =
            Python::with_gil(|py| -> Py<pyo3::types::PyByteArray> {
                pyo3::types::PyByteArray::new(py, &serialized[..]).into()
            });
        Ok(b)
    }

    /// Return the json representation of the SingleQubitOverrotationDescription.
    ///
    /// Returns:
    ///     str: The serialized form of SingleQubitOverrotationDescription.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize SingleQubitOverrotationDescription.
    pub fn to_json(&self) -> PyResult<String> {
        let noise_descp = self.internal.clone();
        let serialized = serde_json::to_string(&noise_descp).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err(
                "Cannot serialize single qubit overrotation description to json.",
            )
        })?;
        Ok(serialized)
    }

    /// Convert the bincode representation of the overotation description to a device using the bincode crate.
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
    pub fn from_bincode(
        input: &Bound<PyAny>,
    ) -> PyResult<SingleQubitOverrotationDescriptionWrapper> {
        let bytes = input.as_ref().extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_description: SingleQubitOverrotationDescription =
            bincode::deserialize(&bytes[..]).map_err(|_| {
                pyo3::exceptions::PyValueError::new_err(
                    "Input cannot be deserialized to overrotation description.",
                )
            })?;

        Ok(SingleQubitOverrotationDescriptionWrapper {
            internal: noise_description,
        })
    }

    /// Convert the json representation of a device to a overotation description.
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
    pub fn from_json(input: &str) -> PyResult<SingleQubitOverrotationDescriptionWrapper> {
        let noise_description: SingleQubitOverrotationDescription = serde_json::from_str(input)
            .map_err(|_| {
                pyo3::exceptions::PyValueError::new_err(
                    "Input cannot be deserialized to overrotation description.",
                )
            })?;
        Ok(SingleQubitOverrotationDescriptionWrapper {
            internal: noise_description,
        })
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on mixed system.
    ///
    /// Args:
    ///     other: The object to compare self to.
    ///     op: Whether they should be equal or not.
    ///
    /// Returns:
    ///     bool: Whether they are equal or not.
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented.
    ///
    fn __richcmp__(
        &self,
        other: &Bound<PyAny>,
        op: pyo3::class::basic::CompareOp,
    ) -> PyResult<bool> {
        let other = SingleQubitOverrotationDescriptionWrapper::from_pyany(other);

        match op {
            pyo3::class::basic::CompareOp::Eq => match other {
                Ok(osystem) => Ok(self.internal.clone() == osystem),
                _ => Ok(false),
            },
            pyo3::class::basic::CompareOp::Ne => match other {
                Ok(osystem) => Ok(self.internal.clone() != osystem),
                _ => Ok(true),
            },
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }

    #[cfg(feature = "json_schema")]
    /// Return the minimum version of qoqo that supports this object.
    ///
    /// Returns:
    ///     str: The minimum version of the qoqo library to deserialize this object.
    pub fn min_supported_version(&self) -> String {
        let min_version: (u32, u32, u32) =
            SingleQubitOverrotationDescription::minimum_supported_roqoqo_version(
                &self.internal.clone(),
            );
        format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
    }

    #[cfg(feature = "json_schema")]
    #[staticmethod]
    /// Return the current version of the qoqo library.
    ///
    /// Returns:
    ///     str: The current version of the library.
    pub fn current_version() -> String {
        ROQOQO_VERSION.to_string()
    }

    #[cfg(feature = "json_schema")]
    #[staticmethod]
    /// Return the JsonSchema for the json serialisation of the class.
    ///
    /// Returns:
    ///     str: The json schema serialized to json
    pub fn json_schema() -> String {
        let schema = schemars::schema_for!(SingleQubitOverrotationDescription);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    /// Return a string containing a printable representation of the object.
    ///
    /// Returns:
    ///     str: The SingleQubitOverrotationDescription, represented as a string.
    fn __repr__(&self) -> String {
        format!("{:?}", self.internal)
    }
}

impl SingleQubitOverrotationDescriptionWrapper {
    /// Fallible conversion of generic python object..
    pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<SingleQubitOverrotationDescription> {
        if let Ok(try_downcast) = input.extract::<SingleQubitOverrotationDescriptionWrapper>() {
            Ok(try_downcast.internal)
        } else {
            let get_bytes = input.call_method0("to_bincode")?;
            let bytes = get_bytes.extract::<Vec<u8>>()?;
            bincode::deserialize(&bytes[..]).map_err(|err| {
                pyo3::exceptions::PyValueError::new_err(format!(
                    "Cannot treat input as Overrotation Description: {}",
                    err
                ))
            })
        }
    }
}

/// Single qubit overrotation noise model on gate.
///
/// Adds a rotation gate with a randomly distributed rotation angle after specified gates in a quantum circuit.
/// Example:
///
/// ```
/// from qoqo.noise_models import SingleQubitOverrotationDescription
/// from qoqo.noise_models import SingleQubitOverrotationOnGate
/// gate = "RotateX"
/// theta_mean = 0.0
/// theta_std = 1.0
/// noise_desc = SingleQubitOverrotationDescription(gate, theta_mean, theta_std)
///
/// noise = SingleQubitOverrotationOnGate();
/// circuit_gate_with_noise = "RotateZ";
/// qubit = 0;
/// noise.set_single_qubit_overrotation(circuit_gate_with_noise, qubit, noise_desc);
/// ```
#[pyclass(frozen, name = "SingleQubitOverrotationOnGate")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SingleQubitOverrotationOnGateWrapper {
    internal: SingleQubitOverrotationOnGate,
}

#[noise_model_wrapper]
impl SingleQubitOverrotationOnGateWrapper {
    /// Create a new SingleQubitOverrotationOnGate.
    #[new]
    pub fn new() -> SingleQubitOverrotationOnGateWrapper {
        SingleQubitOverrotationOnGateWrapper {
            internal: SingleQubitOverrotationOnGate::new(),
        }
    }

    /// Set overrotation for a single qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubit (int): The qubit the gate acts on.
    ///     noise_description (SingleQubitOverrotationDescription) - overrotation description for gate.
    ///
    /// Returns:
    ///     Self: The overotation model with the new overrotation on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise description is not a SingleQubitOverrotationDescription.
    pub fn set_single_qubit_overrotation(
        &self,
        gate: &str,
        qubit: usize,
        noise_description: &Bound<PyAny>,
    ) -> PyResult<Self> {
        let noise_description =
            SingleQubitOverrotationDescriptionWrapper::from_pyany(noise_description)?;
        Ok(Self {
            internal: self.internal.clone().set_single_qubit_overrotation(
                gate,
                qubit,
                noise_description,
            ),
        })
    }

    /// Return the overrotation description for a single qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubit (int): The qubit the gate acts on.
    ///
    /// Returns:
    ///     Optional[SingleQubitOverrotationDescription]: The overrotation applied when gate is applied.
    pub fn get_single_qubit_overrotation(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<SingleQubitOverrotationDescriptionWrapper> {
        self.internal
            .get_single_qubit_overrotation(gate, qubit)
            .map(|noise_descp| SingleQubitOverrotationDescriptionWrapper {
                internal: noise_descp.clone(),
            })
    }

    /// Set extra noise for a two qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control (int): The control qubit the gate acts on.
    ///     target (int): The target qubit the gate acts on.
    ///     noise_description ((SingleQubitOverrotationDescription, SingleQubitOverrotationDescription)) - overrotation description for gate.
    ///
    /// Returns:
    ///     Self: The overrotation model with the new overrotation on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise description is not a (SingleQubitOverrotationDescription, SingleQubitOverrotationDescription).
    pub fn set_two_qubit_overrotation(
        &self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: (Py<PyAny>, Py<PyAny>),
    ) -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<Self> {
            let noise1 =
                SingleQubitOverrotationDescriptionWrapper::from_pyany(noise_operator.0.bind(py))?;
            let noise2 =
                SingleQubitOverrotationDescriptionWrapper::from_pyany(noise_operator.1.bind(py))?;

            Ok(Self {
                internal: self.internal.clone().set_two_qubit_overrotation(
                    gate,
                    control,
                    target,
                    (noise1, noise2),
                ),
            })
        })
    }

    /// Return the extra noise for a single qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control (int): The control qubit the gate acts on.
    ///     target (int): The target qubit the gate acts on.
    ///
    /// Returns:
    ///     Optional[Tuple[SingleQubitOverrotationDescription, SingleQubitOverrotationDescription]]: The overrotation applied when gate is applied.
    pub fn get_two_qubit_overrotation(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<(
        SingleQubitOverrotationDescriptionWrapper,
        SingleQubitOverrotationDescriptionWrapper,
    )> {
        self.internal
            .get_two_qubit_overrotation(gate, control, target)
            .map(|noise| {
                (
                    SingleQubitOverrotationDescriptionWrapper {
                        internal: noise.0.clone(),
                    },
                    SingleQubitOverrotationDescriptionWrapper {
                        internal: noise.1.clone(),
                    },
                )
            })
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
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<SingleQubitOverrotationOnGateWrapper> {
        let bytes = input.as_ref().extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_model: NoiseModel = bincode::deserialize(&bytes[..]).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::SingleQubitOverrotationOnGate(internal) => {
                Ok(SingleQubitOverrotationOnGateWrapper { internal })
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
    pub fn from_json(input: &str) -> PyResult<SingleQubitOverrotationOnGateWrapper> {
        let noise_model: NoiseModel = serde_json::from_str(input).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::SingleQubitOverrotationOnGate(internal) => {
                Ok(SingleQubitOverrotationOnGateWrapper { internal })
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
        let schema = schemars::schema_for!(SingleQubitOverrotationOnGate);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }
}
