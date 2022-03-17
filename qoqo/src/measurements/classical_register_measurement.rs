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

//! Qoqo classical registers

use crate::CircuitWrapper;
use bincode::serialize;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyType;
use roqoqo::measurements::ClassicalRegister;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use std::collections::HashMap;
#[pyclass(name = "ClassicalRegister", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Collected information for executing a classical register.
pub struct ClassicalRegisterWrapper {
    /// Internal storage of [roqoqo::ClassicalRegister].
    pub internal: ClassicalRegister,
}

#[pymethods]
impl ClassicalRegisterWrapper {
    /// Create an new ClassicalRegister measurement.
    ///
    /// Args:
    ///     constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    ///     circuits (list[Circuit]): The collection of quantum circuits executed for the measurement.
    ///
    /// Returns:
    ///     ClassicalRegister: The new register.
    #[new]
    pub fn new(constant_circuit: Option<CircuitWrapper>, circuits: Vec<CircuitWrapper>) -> Self {
        let new_circuits: Vec<Circuit> = circuits.into_iter().map(|c| c.internal).collect();
        let new_constant: Option<Circuit> = match constant_circuit {
            None => None,
            Some(c) => Some(c.internal),
        };
        Self {
            internal: ClassicalRegister {
                constant_circuit: new_constant,
                circuits: new_circuits,
            },
        }
    }

    /// Return the collection of quantum circuits that make up the total measurement.
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

    /// Returns the type of the measurement in string form.
    ///
    /// Returns:
    ///    str: The type of the measurement.
    pub fn measurement_type(&self) -> &'static str {
        "ClassicalRegister"
    }

    /// Return copy of Measurement with symbolic parameters replaced.
    ///
    /// Args:
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
            .map_err(|_| PyValueError::new_err("Cannot serialize ClassicalRegister to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(("ClassicalRegister", b))
    }

    /// Serialize the ClassicalRegister measurement to json form.
    ///
    /// Returns:
    ///     str: The serialized ClassicalRegister measurement.
    ///
    /// Raises:
    ///     PyRuntimeError: Unexpected error serializing ClassicalRegister.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.internal)
            .map_err(|_| PyRuntimeError::new_err("Unexpected error serializing ClassicalRegister"))
    }

    /// Deserialize the ClassicalRegister measurement from json form.
    ///
    /// Returns:
    ///     ClassicalRegister: The deserialized ClassicalRegister Measurement.
    ///
    /// Raises:
    ///     PyRuntimeError: Cannot deserialize string to ClassicalRegister.
    #[allow(unused_variables)]
    #[classmethod]
    pub fn from_json(cls: &PyType, json_string: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(json_string).map_err(|_| {
                PyValueError::new_err("Cannot deserialize string to ClassicalRegister")
            })?,
        })
    }
}
