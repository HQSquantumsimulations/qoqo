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
use roqoqo::noise_models::{ErrorOnGateModel, NoiseModel};
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
/// from qoqo.noise_models import ErrorOnGateModel
/// from struqture_py.spins import (PlusMinusLindbladNoiseOperator, PlusMinusProduct)
///
/// noise_model = ErrorOnGateModel()
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
/// );
/// ```
#[pyclass(frozen, name = "ErrorOnGateModel")]
#[derive(Debug, Clone)]
pub struct ErrorOnGateModelWrapper {
    internal: ErrorOnGateModel,
}

#[noise_model_wrapper]
impl ErrorOnGateModelWrapper {
    /// Creates a new ErrorOnGateModel.
    #[new]
    pub fn new() -> ErrorOnGateModelWrapper {
        ErrorOnGateModelWrapper {
            internal: ErrorOnGateModel::new(),
        }
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_single_qubit_gate_error(
        &self,
        gate: &str,
        qubit: usize,
        noise_operator: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator =
            struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(noise_operator)?;
        Ok(Self {
            internal: self.internal.clone().set_single_qubit_gate_error(
                gate,
                qubit,
                noise_operator,
            ),
        })
    }

    /// Returns the extra noise for a single qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_single_qubit_gate_error(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper> {
        if let Some(noise_operator) = self.internal.get_single_qubit_gate_error(gate, qubit) {
            Some(struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper {
                internal: noise_operator.clone(),
            })
        } else {
            None
        }
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_two_qubit_gate_error(
        &self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator =
            struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(noise_operator)?;
        Ok(Self {
            internal: self.internal.clone().set_two_qubit_gate_error(
                gate,
                control,
                target,
                noise_operator,
            ),
        })
    }

    /// Returns the extra noise for a single qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_two_qubit_gate_error(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper> {
        if let Some(noise_operator) = self
            .internal
            .get_two_qubit_gate_error(gate, control, target)
        {
            Some(struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper {
                internal: noise_operator.clone(),
            })
        } else {
            None
        }
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control0` - The first control qubit the gate acts on.
    /// * `control1` - The second control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_three_qubit_gate_error(
        &self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
        noise_operator: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator =
            struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(noise_operator)?;
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

    /// Returns the extra noise for a three qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control0` - The first control qubit the gate acts on.
    /// * `control1` - The second control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_three_qubit_gate_error(
        &self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
    ) -> Option<struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper> {
        if let Some(noise_operator) = self
            .internal
            .get_three_qubit_gate_error(gate, control0, control1, target)
        {
            Some(struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper {
                internal: noise_operator.clone(),
            })
        } else {
            None
        }
    }

    /// Sets extra noise for a multi qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubits` - The qubits the gate acts on.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_multi_qubit_gate_error(
        &self,
        gate: &str,
        qubits: Vec<usize>,
        noise_operator: Py<PyAny>,
    ) -> PyResult<Self> {
        let noise_operator =
            struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper::from_pyany(noise_operator)?;
        Ok(Self {
            internal: self.internal.clone().set_multi_qubit_gate_error(
                gate,
                qubits,
                noise_operator,
            ),
        })
    }

    /// Returns the extra noise for a multi qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubits` - The qubits the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_multi_qubit_gate_error(
        &self,
        gate: &str,
        qubits: Vec<usize>,
    ) -> Option<struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper> {
        if let Some(noise_operator) = self.internal.get_multi_qubit_gate_error(gate, qubits) {
            Some(struqture_py::spins::PlusMinusLindbladNoiseOperatorWrapper {
                internal: noise_operator.clone(),
            })
        } else {
            None
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
    pub fn from_bincode(input: &PyAny) -> PyResult<ErrorOnGateModelWrapper> {
        let bytes = input.extract::<Vec<u8>>().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Input cannot be converted to byte array")
        })?;
        let noise_model: NoiseModel = bincode::deserialize(&bytes[..]).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ErrorOnGateModel(internal) => Ok(ErrorOnGateModelWrapper { internal }),
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
    pub fn from_json(input: &str) -> PyResult<ErrorOnGateModelWrapper> {
        let noise_model: NoiseModel = serde_json::from_str(input).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Input cannot be deserialized to Noise-Model.")
        })?;
        match noise_model {
            NoiseModel::ErrorOnGateModel(internal) => Ok(ErrorOnGateModelWrapper { internal }),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Input cannot be deserialized to selected Noise-Model.",
            )),
        }
    }
}

// impl ErrorOnGateModelWrapper {
//     /// Fallible conversion of generic python object..
//     pub fn from_pyany(input: Py<PyAny>) -> PyResult<NoiseModel> {
//         Python::with_gil(|py| -> PyResult<NoiseModel> {
//             let input = input.as_ref(py);
//             if let Ok(try_downcast) = input.extract::<ErrorOnGateModelWrapper>() {
//                 Ok(try_downcast.internal.into())
//             } else {
//                 // This allows all devices to be imported as generic device
//                 let generic_device_candidate = input.call_method0("generic_device")?;
//                 let get_bytes = generic_device_candidate.call_method0("to_bincode")?;
//                 let bytes = get_bytes.extract::<Vec<u8>>()?;
//                 bincode::deserialize(&bytes[..]).map_err(|err| {
//                     pyo3::exceptions::PyValueError::new_err(format!(
//                         "Cannot treat input as NoiseModel: {}",
//                         err
//                     ))
//                 })
//             }
//         })
//     }
// }
