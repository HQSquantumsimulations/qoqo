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
use roqoqo::noise_models::{SingleQubitOverrotationDescription, SingleQubitOverrotationOnGate, NoiseModel};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};
use struqture_py;

///
#[pyclass(frozen, name = "SingleQubitOverrotationDescription")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SingleQubitOverrotationDescriptionWrapper {
    ///
    pub internal: SingleQubitOverrotationDescription,
}

impl SingleQubitOverrotationDescriptionWrapper {
    ///
    pub fn new(gate:&str , theta_mean:f64, theta_std:f64) -> SingleQubitOverrotationDescriptionWrapper {
        SingleQubitOverrotationDescriptionWrapper{
            internal: SingleQubitOverrotationDescription::new(gate, theta_mean, theta_std),
        }
    }

    /// Returns a copy of the device (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     A deep copy of self.
    ///
    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    /// Creates deep copy of Noise-Model.
    ///
    /// Returns:
    ///     A deep copy of self.
    ///
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> Self {
        self.clone()
    }
    ///
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<SingleQubitOverrotationDescription> {
        Python::with_gil(|py| -> PyResult<SingleQubitOverrotationDescription> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<SingleQubitOverrotationDescriptionWrapper>() {
                Ok(try_downcast.internal.into())
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
        })
    }
    ///
    pub fn to_bincode(&self) -> PyResult<Py<pyo3::types::PyByteArray>> {
        let noise_descp = SingleQubitOverrotationDescription::from(self.internal.clone());
        let serialized = bincode::serialize(&noise_descp)
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("Cannot serialize Noise-Overrotation description to bytes"))?;
        let b: Py<pyo3::types::PyByteArray> = Python::with_gil(|py| -> Py<pyo3::types::PyByteArray> {
            pyo3::types::PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Return the json representation of the single qubit overrotation description.
    ///
    /// Returns:
    ///     str: The serialized form of single qubit overrotation description.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize single qubit overrotation description to json.
    ///
    pub fn to_json(&self) -> PyResult<String> {
        let noise_descp = SingleQubitOverrotationDescription::from(self.internal.clone());
        let serialized = serde_json::to_string(&noise_descp)
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("Cannot serialize single qubit overrotation description to json."))?;
        Ok(serialized)
    }

    #[cfg(feature = "json_schema")]
    /// Return the minimum version of qoqo that supports this object.
    ///
    /// Returns:
    ///     str: The minimum version of the qoqo library to deserialize this object.
    pub fn min_supported_version(&self) -> String {
        let min_version: (u32, u32, u32) =
            SingleQubitOverrotationDescription::minimum_supported_roqoqo_version(&SingleQubitOverrotationDescription::from(self.internal.clone()));
        format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on mixed system.
    ///
    /// Args:
    ///     other: The object to compare self to.
    ///     op: Whether they should be equal or not.
    ///
    /// Returns:
    ///     bool
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented.
    ///
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other = SingleQubitOverrotationDescriptionWrapper::from_pyany(other);

        match op {
            pyo3::class::basic::CompareOp::Eq => match other {
                Ok(osystem) => Ok(SingleQubitOverrotationDescription::from(self.internal.clone()) == osystem),
                _ => Ok(false),
            },
            pyo3::class::basic::CompareOp::Ne => match other {
                Ok(osystem) => Ok(SingleQubitOverrotationDescription::from(self.internal.clone()) != osystem),
                _ => Ok(true),
            },
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }
}

/// .
///
/// Example:
///
/// ```
/// from qoqo.noise_models import DecoherenceOnGateModel
/// from struqture_py.spins import (PlusMinusLindbladNoiseOperator, PlusMinusProduct)
///
/// noise_model = DecoherenceOnGateModel()
/// lindblad_noise = PlusMinusLindbladNoiseOperator()
/// lindblad_noise.add_operator_product(
///    (PlusMinusProduct().z(0), PlusMinusProduct().z(0)),
///    0.9)
/// lindblad_noise.add_operator_product(
///    (PlusMinusProduct().z(1), PlusMinusProduct().z(1)),
///    0.9)
///
/// noise_model = noise_model.set_two_qubit_term(
/// "CNOT", 0,1,
/// lindblad_noise
/// )
/// ```
#[pyclass(frozen, name = "SingleQubitOverrotationOnGate")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SingleQubitOverrotationOnGateWrapper {
    internal: SingleQubitOverrotationOnGate,
}

#[noise_model_wrapper]
impl SingleQubitOverrotationOnGateWrapper {
    /// Creates a new DecoherenceOnGateModel.
    #[new]
    pub fn new() -> SingleQubitOverrotationOnGateWrapper {
        SingleQubitOverrotationOnGateWrapper {
            internal: SingleQubitOverrotationOnGate::new(),
        }
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubit (int): The qubit the gate acts on.
    ///     noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.
    ///
    /// Returns:
    ///     Self: The error model with the new noise on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
    pub fn set_single_qubit_overrotations(
        &self,
        gate: &str,
        qubit: usize,
        noise_descp: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_descp = SingleQubitOverrotationDescriptionWrapper::from_pyany(noise_descp)?;
        Ok(Self {
            internal: self.internal.clone().set_single_qubit_overrotations(
                gate,
                qubit,
                noise_descp
            ),
        })
    }

    /// Returns the extra noise for a single qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubit (int): The qubit the gate acts on.
    ///
    /// Returns
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_single_qubit_overrotations(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<SingleQubitOverrotationDescriptionWrapper> {
        self.internal
            .get_single_qubit_overrotations(gate, qubit)
            .map(
                |noise_descp| SingleQubitOverrotationDescriptionWrapper {
                    internal: noise_descp.clone(),
                },
            )
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control (int): The control qubit the gate acts on.
    ///     target (int): The target qubit the gate acts on.
    ///     noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.
    ///
    /// Returns:
    ///     Self: The error model with the new noise on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
    pub fn set_two_qubit_overrotations(
        &self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator =
            struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(noise_operator)?;
        Ok(Self {
            internal: self.internal.clone().set_two_qubit_overrotations(
                gate,
                control,
                target,
                noise_operator,
            ),
        })
    }

    /// Returns the extra noise for a single qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control (int) - The control qubit the gate acts on.
    ///     target (int) - The target qubit the gate acts on.
    ///
    /// Returns
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_two_qubit_overrotations(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper> {
        self.internal
            .get_two_qubit_overrotations(gate, control, target)
            .map(
                |noise_operator| struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper {
                    internal: noise_operator.clone(),
                },
            )
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
    pub fn from_bincode(input: &PyAny) -> PyResult<SingleQubitOverrotationOnGateWrapper> {
        let bytes = input.extract::<Vec<u8>>().map_err(|_| {
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
