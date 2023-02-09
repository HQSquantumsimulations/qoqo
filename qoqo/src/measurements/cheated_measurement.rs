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

//! Qoqo cheated measurement

use super::CheatedInputWrapper;
use crate::CircuitWrapper;
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
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
    #[pyo3(signature=(constant_circuit, circuits, input))]
    pub fn new(
        constant_circuit: Option<Py<PyAny>>,
        circuits: Vec<Py<PyAny>>,
        input: Py<PyAny>,
    ) -> PyResult<Self> {
        let mut new_circuits: Vec<Circuit> = Vec::new();
        for c in circuits.into_iter() {
            let tmp_c = CircuitWrapper::from_pyany(c).map_err(|err| {
                PyTypeError::new_err(format!(
                    "`circuits` argument is not a list of qoqo Circuits: {}",
                    err
                ))
            })?;
            new_circuits.push(tmp_c)
        }
        let new_constant: Option<Circuit> = match constant_circuit {
            None => None,
            Some(c) => {
                let tmp_c = CircuitWrapper::from_pyany(c).map_err(|err| {
                    PyTypeError::new_err(format!(
                        "`constant_circuit` argument is not None or a qoqo Circuit: {}",
                        err
                    ))
                })?;
                Some(tmp_c)
            }
        };
        let input = CheatedInputWrapper::from_pyany(input).map_err(|err| {
            PyTypeError::new_err(format!(
                "`input` argument is not a qoqo CheatedInput: {}",
                err
            ))
        })?;
        Ok(Self {
            internal: Cheated {
                input,
                constant_circuit: new_constant,
                circuits: new_circuits,
            },
        })
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

    /// Return the bincode representation of the Cheated using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized Cheated (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Cheated to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize Cheated to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    #[staticmethod]
    /// Convert the bincode representation of the Cheated to a Cheated using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized Cheated (in [bincode] form).
    ///
    /// Returns:
    ///     Cheated: The deserialized Cheated.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to Cheated.
    pub fn from_bincode(input: &PyAny) -> PyResult<Self> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(Self {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to Cheated"))?,
        })
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
    #[staticmethod]
    pub fn from_json(json_string: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(json_string)
                .map_err(|_| PyValueError::new_err("Cannot deserialize string to Cheated"))?,
        })
    }

    /// Implement __repr__ magic method
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.internal)
    }

    /// Return a copy of the Object (copy here produces a deepcopy).
    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    /// Return a deep copy of the Object.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> Self {
        self.clone()
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on QuantumProgram.
    ///
    /// Args:
    ///     other: The object to compare self to.
    ///     op: Type of comparison.
    ///
    /// Returns:
    ///     Whether the two operations compared evaluated to True or False
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented
    fn __richcmp__(
        &self,
        other: CheatedWrapper,
        op: pyo3::class::basic::CompareOp,
    ) -> PyResult<bool> {
        match op {
            pyo3::class::basic::CompareOp::Eq => Ok(self.internal == other.internal),
            pyo3::class::basic::CompareOp::Ne => Ok(self.internal != other.internal),
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }
}

impl CheatedWrapper {
    /// Extracts a Cheated from a CheatedWrapper python object.
    ///
    /// When working with qoqo and other rust based python packages compiled separately
    /// a downcast will not detect that two CheatedWrapper objects are compatible.
    /// Provides a custom function to convert qoqo Cheateds between different Python packages.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo::Cheated]
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<Cheated> {
        Python::with_gil(|py| -> PyResult<Cheated> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<CheatedWrapper>() {
                Ok(try_downcast.internal)
            } else {
                let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo Cheated: Cast to binary representation failed".to_string())
            })?;
                let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo Cheated: Cast to binary representation failed".to_string())
            })?;
                deserialize(&bytes[..]).map_err(|err| {
                    PyTypeError::new_err(format!(
                    "Python object cannot be converted to qoqo Cheated: Deserialization failed: {}",
                    err
                ))
                })
            }
        })
    }
}
