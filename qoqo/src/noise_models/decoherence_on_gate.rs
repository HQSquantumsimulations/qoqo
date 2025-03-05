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

use pyo3::{exceptions::PyValueError, prelude::*};
use qoqo_macros::noise_model_wrapper;
use roqoqo::noise_models::{DecoherenceOnGateModel, NoiseModel};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};
use struqture_py;

/// Error model for noise that is only present on gate executions.
///
/// Adds additional noise when specific gates (identified by hqslang name and qubits acted on) are executed.
/// The noise is given in the form of a struqture.spins.PlusMinusLindbladNoiseOperator the same way it
/// is for the ContinuousDecoherence model.
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
#[pyclass(frozen, name = "DecoherenceOnGateModel")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DecoherenceOnGateModelWrapper {
    internal: DecoherenceOnGateModel,
}

#[noise_model_wrapper]
impl DecoherenceOnGateModelWrapper {
    /// Create a new DecoherenceOnGateModel.
    #[new]
    pub fn new() -> DecoherenceOnGateModelWrapper {
        DecoherenceOnGateModelWrapper {
            internal: DecoherenceOnGateModel::new(),
        }
    }

    /// Set extra noise for a single qubit gate.
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
    pub fn set_single_qubit_gate_error(
        &self,
        gate: &str,
        qubit: usize,
        noise_operator: &Bound<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator =
            match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(
                noise_operator,
            ) {
                Ok(x) => x,
                Err(_) => match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany_struqture_1(noise_operator) {
                    Ok(x) => x,
                    Err(err) => return Err(PyValueError::new_err(format!("Could not convert input noise_operator from either struqture 1.x or struqture 2.x: {:?}", err))),
                }
            };
        Ok(Self {
            internal: self.internal.clone().set_single_qubit_gate_error(
                gate,
                qubit,
                noise_operator,
            ),
        })
    }

    /// Return the extra noise for a single qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubit (int): The qubit the gate acts on.
    ///
    /// Returns:
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_single_qubit_gate_error(&self, gate: &str, qubit: usize) -> Option<Py<PyAny>> {
        match self.internal.get_single_qubit_gate_error(gate, qubit) {
            Some(struqture_obj) => {
                Python::with_gil(|py| Some(crate::get_operator(py, struqture_obj)))
            }
            None => None,
        }
    }

    /// Set extra noise for a two qubit gate.
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
    pub fn set_two_qubit_gate_error(
        &self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: &Bound<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator =
            match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(
                noise_operator,
            ) {
                Ok(x) => x,
                Err(_) => match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany_struqture_1(noise_operator) {
                    Ok(x) => x,
                    Err(err) => return Err(PyValueError::new_err(format!("Could not convert input noise_operator from either struqture 1.x or struqture 2.x: {:?}", err))),
                }
            };
        Ok(Self {
            internal: self.internal.clone().set_two_qubit_gate_error(
                gate,
                control,
                target,
                noise_operator,
            ),
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
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_two_qubit_gate_error(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<Py<PyAny>> {
        match self
            .internal
            .get_two_qubit_gate_error(gate, control, target)
        {
            Some(struqture_obj) => {
                Python::with_gil(|py| Some(crate::get_operator(py, struqture_obj)))
            }
            None => None,
        }
    }

    /// Set extra noise for a single qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control0 (int): The first control qubit the gate acts on.
    ///     control1 (int): The second control qubit the gate acts on.
    ///     target (int): The target qubit the gate acts on.
    ///     noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.
    ///
    /// Returns:
    ///     Self: The error model with the new noise on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
    pub fn set_three_qubit_gate_error(
        &self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
        noise_operator: &Bound<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator =
            match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(
                noise_operator,
            ) {
                Ok(x) => x,
                Err(_) => match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany_struqture_1(noise_operator) {
                    Ok(x) => x,
                    Err(err) => return Err(PyValueError::new_err(format!("Could not convert input noise_operator from either struqture 1.x or struqture 2.x: {:?}", err))),
                }
            };
        Ok(Self {
            internal: self.internal.clone().set_three_qubit_gate_error(
                gate,
                control0,
                control1,
                target,
                noise_operator,
            ),
        })
    }

    /// Return the extra noise for a three qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     control0 (int): The first control qubit the gate acts on.
    ///     control1 (int): The second control qubit the gate acts on.
    ///     target (int): The target qubit the gate acts on.
    ///
    /// Returns:
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_three_qubit_gate_error(
        &self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
    ) -> Option<Py<PyAny>> {
        match self
            .internal
            .get_three_qubit_gate_error(gate, control0, control1, target)
        {
            Some(struqture_obj) => {
                Python::with_gil(|py| Some(crate::get_operator(py, struqture_obj)))
            }
            None => None,
        }
    }

    /// Set extra noise for a multi qubit gate.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubits (list): The qubits the gate acts on.
    ///     noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.
    ///
    /// Returns:
    ///     Self: The error model with the new noise on gate set.
    ///
    /// Raises:
    ///     PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
    pub fn set_multi_qubit_gate_error(
        &self,
        gate: &str,
        qubits: Vec<usize>,
        noise_operator: &Bound<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator =
            match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(
                noise_operator,
            ) {
                Ok(x) => x,
                Err(_) => match struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany_struqture_1(noise_operator) {
                    Ok(x) => x,
                    Err(err) => return Err(PyValueError::new_err(format!("Could not convert input noise_operator from either struqture 1.x or struqture 2.x: {:?}", err))),
                }
            };
        Ok(Self {
            internal: self.internal.clone().set_multi_qubit_gate_error(
                gate,
                qubits,
                noise_operator,
            ),
        })
    }

    /// Return the extra noise for a multi qubit gate, if it exists.
    ///
    /// Args:
    ///     gate (str): The name of the gate.
    ///     qubits (List[int]): The qubits the gate acts on.
    ///
    /// Returns:
    ///     Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
    pub fn get_multi_qubit_gate_error(&self, gate: &str, qubits: Vec<usize>) -> Option<Py<PyAny>> {
        match self.internal.get_multi_qubit_gate_error(gate, qubits) {
            Some(struqture_obj) => {
                Python::with_gil(|py| Some(crate::get_operator(py, struqture_obj)))
            }
            None => None,
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
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<DecoherenceOnGateModelWrapper> {
        let bytes = input.as_ref().extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_model: NoiseModel = bincode::deserialize(&bytes[..]).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::DecoherenceOnGateModel(internal) => {
                Ok(DecoherenceOnGateModelWrapper { internal })
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
    pub fn from_json(input: &str) -> PyResult<DecoherenceOnGateModelWrapper> {
        let noise_model: NoiseModel = serde_json::from_str(input).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::DecoherenceOnGateModel(internal) => {
                Ok(DecoherenceOnGateModelWrapper { internal })
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
        let schema = schemars::schema_for!(DecoherenceOnGateModel);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }
}
