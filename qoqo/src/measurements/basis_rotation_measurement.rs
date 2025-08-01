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

//! Qoqo PauliZ product measurement.

use super::PauliZProductInputWrapper;
use crate::CircuitWrapper;
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use roqoqo::measurements::PauliZProduct;
use roqoqo::prelude::*;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::Circuit;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::HashMap;

#[pyclass(name = "PauliZProduct", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Collected information for executing a measurement of PauliZ product.
///
/// Args:
///     constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
///     circuits (List[Circuit]): The collection of quantum circuits for the separate basis rotations.
///     input (PauliZProductInput): The additional input information required for measurement.
///
/// Returns:
///     PauliZProduct: The PauliZProduct containing the new PauliZ product measurement.
pub struct PauliZProductWrapper {
    /// Internal storage of [roqoqo::PauliZProduct].
    pub internal: PauliZProduct,
}

#[pymethods]
impl PauliZProductWrapper {
    /// Create a new PauliZProduct measurement.
    ///
    /// Args:
    ///     constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    ///     circuits (List[Circuit]): The collection of quantum circuits for the separate basis rotations.
    ///     input (PauliZProductInput): The additional input information required for measurement.
    ///
    /// Returns:
    ///     PauliZProduct: The PauliZProduct containing the new PauliZ product measurement.
    #[new]
    #[pyo3(signature=(constant_circuit, circuits, input))]
    pub fn new(
        constant_circuit: Option<Py<PyAny>>,
        circuits: Vec<Py<PyAny>>,
        input: Py<PyAny>,
    ) -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<Self> {
            let mut new_circuits: Vec<Circuit> = Vec::new();
            for c in circuits.into_iter() {
                let tmp_c = CircuitWrapper::from_pyany(c.bind(py)).map_err(|err| {
                    PyTypeError::new_err(format!(
                        "`circuits` argument is not a list of qoqo Circuits: {err}"
                    ))
                })?;
                new_circuits.push(tmp_c)
            }
            let new_constant: Option<Circuit> = match constant_circuit {
                None => None,
                Some(c) => {
                    let tmp_c = CircuitWrapper::from_pyany(c.bind(py)).map_err(|err| {
                        PyTypeError::new_err(format!(
                            "`constant_circuit` argument is not None or a qoqo Circuit: {err}"
                        ))
                    })?;
                    Some(tmp_c)
                }
            };
            let input = PauliZProductInputWrapper::from_pyany(input.bind(py)).map_err(|err| {
                PyTypeError::new_err(format!(
                    "`input` argument is not a qoqo CheatedInput: {err}"
                ))
            })?;
            Ok(Self {
                internal: PauliZProduct {
                    input,
                    constant_circuit: new_constant,
                    circuits: new_circuits,
                },
            })
        })
    }

    /// Execute the PauliZ product measurement.
    ///
    /// Args:
    ///     input_bit_registers (Dict[str, Union[List[List[int]], List[List[bool]]]]): The classical bit registers with the register name as key
    ///     float_registers (Dict[str, List[List[float]]]): The classical float registers as a dictionary with the register name as key
    ///     complex_registers (Dict[str, List[List[complex]]]): The classical complex registers as a dictionary with the register name as key
    ///
    /// Returns:
    ///     Optional[Dict[str, float]]: The evaluated measurement.
    ///
    /// Raises:
    ///     RuntimeError: Unexpected repetition of key in bit_register.
    ///     RuntimeError: Error evaluating PauliZ product measurement.
    pub fn evaluate(
        &mut self,
        input_bit_registers: &Bound<PyAny>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> PyResult<Option<HashMap<String, f64>>> {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let bit_registers_bool: PyResult<HashMap<String, Vec<Vec<bool>>>> =
            input_bit_registers.extract::<HashMap<String, BitOutputRegister>>();
        if let Ok(try_downcast) = bit_registers_bool {
            bit_registers = try_downcast
        } else {
            let tmp_bit_registers =
                input_bit_registers.extract::<HashMap<String, Vec<Vec<usize>>>>()?;
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
                PyRuntimeError::new_err(format!(
                    "Error evaluating PauliZ product measurement {x:?}"
                ))
            })
    }

    /// Return the collection of quantum circuits for the separate basis rotations.
    ///
    /// Returns:
    ///     List[Circuit]: The quantum circuits.
    pub fn circuits(&self) -> Vec<CircuitWrapper> {
        self.internal
            .circuits()
            .map(|c| CircuitWrapper {
                internal: c.clone(),
            })
            .collect()
    }

    /// Return constant circuit that is executed before any Circuit in circuits.
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
    ///     PauliZProductInput: The measurment input of PauliZProduct.
    pub fn input(&self) -> PauliZProductInputWrapper {
        let input = self.internal.input.clone();
        PauliZProductInputWrapper { internal: input }
    }

    /// Returns the type of the measurement in string form.
    ///
    /// Returns:
    ///    str: The type of the measurement.
    pub fn measurement_type(&self) -> &'static str {
        "PauliZProduct"
    }

    /// Return clone of Measurement with symbolic parameters replaced.
    ///
    /// Args:
    ///     substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
    pub fn substitute_parameters(
        &self,
        substituted_parameters: HashMap<String, f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(substituted_parameters)
                .map_err(|x| {
                    PyRuntimeError::new_err(format!("Error substituting symbolic parameters {x:?}"))
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
        let serialized = serialize(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize PauliZProductMeasurement to bytes")
        })?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(("PauliZProduct", b))
    }

    /// Return the bincode representation of the PauliZProduct using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized PauliZProduct (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize PauliZProduct to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize PauliZProduct to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the PauliZProduct to a PauliZProduct using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized PauliZProduct (in [bincode] form).
    ///
    /// Returns:
    ///     PauliZProduct: The deserialized PauliZProduct.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to PauliZProduct.
    #[staticmethod]
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<Self> {
        let bytes = input
            .as_ref()
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(Self {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to PauliZProduct")
            })?,
        })
    }

    /// Serialize the PauliZProduct to json form using the [serde_json] crate.
    ///
    /// Returns:
    ///     str: The serialized PauliZProduct.
    ///
    /// Raises:
    ///     RuntimeError: Unexpected error serializing PauliZProduct.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.internal)
            .map_err(|_| PyRuntimeError::new_err("Unexpected error serializing PauliZProduct"))
    }

    /// Deserialize the PauliZProduct from json form using the [serde_json] crate.
    ///
    /// Returns:
    ///     PauliZProduct: The deserialized PauliZProduct.
    ///
    /// Raises:
    ///     RuntimeError: Cannot deserialize string to PauliZProduct.
    #[staticmethod]
    pub fn from_json(json_string: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(json_string)
                .map_err(|_| PyValueError::new_err("Cannot deserialize string to PauliZProduct"))?,
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
    pub fn __deepcopy__(&self, _memodict: &Bound<PyAny>) -> Self {
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
        other: PauliZProductWrapper,
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

    #[cfg(feature = "json_schema")]
    /// Return the JsonSchema for the json serialisation of the class.
    ///
    /// Returns:
    ///     str: The json schema serialized to json
    #[staticmethod]
    pub fn json_schema() -> String {
        let schema = schemars::schema_for!(PauliZProduct);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    #[cfg(feature = "json_schema")]
    /// Returns the current version of the qoqo library .
    ///
    /// Returns:
    ///     str: The current version of the library.
    #[staticmethod]
    pub fn current_version() -> String {
        ROQOQO_VERSION.to_string()
    }

    #[cfg(feature = "json_schema")]
    /// Return the minimum version of qoqo that supports this object.
    ///
    /// Returns:
    ///     str: The minimum version of the qoqo library to deserialize this object.
    pub fn min_supported_version(&self) -> String {
        let min_version: (u32, u32, u32) =
            PauliZProduct::minimum_supported_roqoqo_version(&self.internal);
        format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
    }
}

impl PauliZProductWrapper {
    /// Extracts a PauliZProduct from a PauliZProductWrapper python  object.
    ///
    /// When working with qoqo and other rust based python packages compiled separately
    /// a downcast will not detect that two PauliZProductWrapper objects are compatible.
    /// Provides a custom function to convert qoqo PauliZProducts between different Python packages.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo::PauliZProduct]
    pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<PauliZProduct> {
        if let Ok(try_downcast) = input.extract::<PauliZProductWrapper>() {
            Ok(try_downcast.internal)
        } else {
            let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo PauliZProduct: Cast to binary representation failed".to_string())
            })?;
            let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo PauliZProduct: Cast to binary representation failed".to_string())
            })?;
            deserialize(&bytes[..]).map_err(|err| {
                    PyTypeError::new_err(format!(
                    "Python object cannot be converted to qoqo PauliZProduct: Deserialization failed: {err}"
                ))
                })
        }
    }
}
