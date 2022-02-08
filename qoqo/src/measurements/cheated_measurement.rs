// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Qoqo cheated measurement

use super::CheatedInputWrapper;
use crate::CircuitWrapper;
use bincode::serialize;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyType;
use roqoqo::measurements::Cheated;
use roqoqo::prelude::*;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::Circuit;
use std::collections::HashMap;
#[pyclass(name = "Cheated", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Collected information for executing a cheated measurement.
pub struct CheatedWrapper {
    /// Internal storage of [roqoqo::Cheated]
    pub internal: Cheated,
}

#[pymethods]
impl CheatedWrapper {
    /// Create an new Cheated measurement
    ///
    /// Args:
    ///     constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    ///     circuits (list[Circuit]): The collection of quantum circuits executed for the measurement.
    ///     input (CheatedInput): The additional input information required for measurement.
    ///
    /// Returns:
    ///     Cheated: The new measurement.
    #[new]
    pub fn new(
        constant_circuit: Option<CircuitWrapper>,
        circuits: Vec<CircuitWrapper>,
        input: CheatedInputWrapper,
    ) -> Self {
        let new_circuits: Vec<Circuit> = circuits.into_iter().map(|c| c.internal).collect();
        let new_constant: Option<Circuit> = match constant_circuit {
            None => None,
            Some(c) => Some(c.internal),
        };
        Self {
            internal: Cheated {
                constant_circuit: new_constant,
                circuits: new_circuits,
                input: input.internal,
            },
        }
    }

    /// Execute the cheated measurement.
    ///
    /// Args:
    ///     input_bit_registers (dict[str, Union[list[list[int]], list[list[bool]]]]): The classical bit registers with the register name as key.
    ///     float_registers (dict[str, list[list[float]]): The classical float registers as a dictionary with the register name as key.
    ///     complex_registers (dict[str, list[list[complex]]): The classical complex registers as a dictionary with the register name as key.
    ///
    /// Returns:
    ///     Optional[dict[str, float]: The evaluated expectation values.
    ///
    /// Raises:
    ///     RuntimeError: Unexpected repetition of key in bit_register.
    ///     RuntimeError: Error evaluating cheated measurement.
    pub fn evaluate(
        &mut self,
        input_bit_registers: Py<PyAny>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> PyResult<Option<HashMap<String, f64>>> {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let bit_registers_bool: PyResult<HashMap<String, Vec<Vec<bool>>>> =
            Python::with_gil(|py| -> PyResult<HashMap<String, Vec<Vec<bool>>>> {
                input_bit_registers
                    .as_ref(py)
                    .extract::<HashMap<String, BitOutputRegister>>()
            });
        if let Ok(try_downcast) = bit_registers_bool {
            bit_registers = try_downcast
        } else {
            let tmp_bit_registers =
                Python::with_gil(|py| -> PyResult<HashMap<String, Vec<Vec<usize>>>> {
                    input_bit_registers
                        .as_ref(py)
                        .extract::<HashMap<String, Vec<Vec<usize>>>>()
                })?;
            for (name, output_reg) in tmp_bit_registers {
                let mut tmp_output_reg: Vec<Vec<bool>> = Vec::with_capacity(output_reg.len());
                for reg in output_reg {
                    tmp_output_reg.push(reg.into_iter().map(|x| !matches!(x, 0)).collect());
                }
                bit_registers.insert(name, tmp_output_reg);
                // if t.is_some() {
                //     return Err(PyRuntimeError::new_err(
                //         "Unexpected repetition of key in bit_registers",
                //     ));
                // }
            }
        }
        self.internal
            .evaluate(bit_registers, float_registers, complex_registers)
            .map_err(|x| {
                PyRuntimeError::new_err(format!("Error evaluating cheated measurement {:?}", x))
            })
    }

    /// Return the collection of quantum circuits for the separate cheated measurements.
    ///
    /// Returns:
    ///     list[Circuit]: The quantum circuits.
    pub fn circuits(&self) -> Vec<CircuitWrapper> {
        self.internal
            .circuits()
            .map(|c| CircuitWrapper {
                internal: c.clone(),
            })
            .collect()
    }

    /// Returns constant circuit that is executed before any Circuit in circuits.
    ///
    /// Returns:
    ///     Optional[Circuit]: The constant Circuit (None if not defined).
    pub fn constant_circuit(&self) -> Option<CircuitWrapper> {
        self.internal
            .constant_circuit()
            .clone()
            .map(|c| CircuitWrapper { internal: c })
    }

    /// Returns the measurement input data defining how to construct expectation values from measurements.
    ///
    /// Returns:
    ///     CheatedInput: The input of Cheated measurement
    pub fn input(&self) -> CheatedInputWrapper {
        let input = self.internal.input.clone();
        CheatedInputWrapper { internal: input }
    }

    /// Returns the type of the measurement in string form.
    ///
    /// Returns:
    ///    str: The type of the measurement.
    pub fn measurement_type(&self) -> &'static str {
        "Cheated"
    }

    /// Return copy of Measurement with symbolic parameters replaced.
    ///
    /// Arguments:
    ///     substituted_parameters (dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
    ///
    /// Raises:
    ///     RuntimeError: Error substituting symbolic parameters.
    pub fn substitute_parameters(
        &self,
        substituted_parameters: HashMap<String, f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(substituted_parameters)
                .map_err(|x| {
                    PyRuntimeError::new_err(format!(
                        "Error substituting symbolic parameters {:?}",
                        x
                    ))
                })?,
        })
    }

    /// Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.
    ///
    /// Returns:
    ///     (str, ByteArray): Name and serialized measurement (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Measurement to bytes.
    pub fn _internal_to_bincode(&self) -> PyResult<(&'static str, Py<PyByteArray>)> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize CheatedMeasurement to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(("Cheated", b))
    }

    /// Serialize the Cheated measurement to json form.
    ///
    /// Returns:
    ///     str: The serialized Cheated measurement.
    ///
    /// Raises:
    ///     RuntimeError: Unexpected error serializing Cheated.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.internal)
            .map_err(|_| PyRuntimeError::new_err("Unexpected error serializing Cheated"))
    }

    /// Deserialize the Cheated measurement from json form.
    ///
    /// Returns:
    ///     Cheated: The deserialized Cheated Measurement.
    ///
    /// Raises:
    ///     RuntimeError: Cannot deserialize string to Cheated.
    #[allow(unused_variables)]
    #[classmethod]
    pub fn from_json(cls: &PyType, json_string: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(json_string)
                .map_err(|_| PyValueError::new_err("Cannot deserialize string to Cheated"))?,
        })
    }
}
