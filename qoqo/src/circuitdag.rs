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

///! Module containing the CircuitDag class that represents the Directed Acyclic Graph (DAG)
///! of a quantum circuit in qoqo.
///!

use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;
use roqoqo::CircuitDag;

use crate::operations::{convert_operation_to_pyobject, convert_pyany_to_operation};

/// Module containing the CircuitDag class that represents the Directed Acyclic Graph (DAG)
/// of a quantum circuit in qoqo.
///
#[pymodule]
fn circuitdag(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<CircuitDagWrapper>()?;
    Ok(())
}

/// Represents the Direct Acyclic Graph (DAG) of a Circuit.
///
#[pyclass(name = "CircuitDag", module = "qoqo")]
#[derive(Clone, Debug)]
pub struct CircuitDagWrapper {
    /// Internal storage of [roqoqo:CircuitDag]
    pub internal: CircuitDag,
}

impl Default for CircuitDagWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl CircuitDagWrapper {
    /// Create an empty CircuitDag.
    ///
    /// Returns:
    ///     self: The new, empty CircuitDag.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: CircuitDag::new(),
        }
    }

    /// Add an Operation to the back of the CircuitDag, if necessary.
    ///
    /// Args:
    ///     op (Operation): The Operation to add to the back of the CircuitDag.
    /// 
    /// Raises:
    ///     TypeError: The Python Object cannot be converted to Operation.
    pub fn add_to_back(&mut self, op: &PyAny) -> PyResult<Option<usize>> {
        let operation = convert_pyany_to_operation(op).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Operation {:?}", x))
        })?;
        Ok(self.internal.add_to_back(operation))
    }

    /// Add an Operation to the front of the CircuitDag, if necessary.
    ///
    /// Args:
    ///     op (Operation): The Operation to add to the front of the CircuitDag.
    /// 
    /// Raises:
    ///     TypeError: The Python Object cannot be converted to Operation.
    pub fn add_to_front(&mut self, op: &PyAny) -> PyResult<Option<usize>> {
        let operation = convert_pyany_to_operation(op).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Operation {:?}", x))
        })?;
        Ok(self.internal.add_to_front(operation))
    }

    /// Given a NodeIndex, returns the Operation contained in the node of
    /// the CircuitDag.
    ///
    /// Args:
    ///     index (usize): The index of the node to get from the CircuitDag.
    /// 
    /// Returns:
    ///     Operation: The Operation at the given index (if it exists).
    /// 
    /// Raises:
    ///     IndexError: Index out of range.
    pub fn get(&self, index: usize) -> PyResult<PyObject> {
        let operation = self
            .internal
            .get(index)
            .ok_or_else(|| PyIndexError::new_err(format!("Index {} out of range", index)))?
            .clone();
        convert_operation_to_pyobject(operation)
    }

    /*
    #[allow(unused_variables)]
    #[classmethod]
    /// Convert the bincode representation of the CircuitDag to a CircuitDag using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized CircuitDag (in [bincode] form).
    ///
    /// Returns:
    ///     Circuit: The deserialized CircuitDag.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to Circuit.
    pub fn from_bincode(cls: &PyType, input: &PyAny) -> PyResult<Self> {

    }
    */
}
/*
/// Convert generic python object to [roqoqo::CircuitDag].
///
/// Fallible conversion of generic python object to [roqoqo::CircuitDag].
pub fn convert_into_circuitdag(input: &PyAny) -> Result<CircuitDag, QoqoError> {
    if let Ok(try_downcast) = input.extract::<CircuitDagWrapper>() {
        return Ok(try_downcast.internal);
    }
    // Everything that follows tries to extract the circuitdag when two separately
    // compiled python packages are involved
    let get_version = input
        .call_method0("_qoqo_versions")
        .map_err(|_| QoqoError::CannotExtractObject)?;
    let version = get_version
        .extract::<(&str, &str)>()
        .map_err(|_| QoqoError::CannotExtractObject)?;
    let mut rsplit = ROQOQO_VERSION.split('.').take(2);
    let mut qsplit = QOQO_VERSION.split('.').take(2);
    let rver = format!(
        "{}.{}",
        rsplit.next().expect("ROQOQO_VERSION badly formatted"),
        rsplit.next().expect("ROQOQO_VERSION badly formatted")
    );
    let qver = format!(
        "{}.{}",
        qsplit.next().expect("QOQO_VERSION badly formatted"),
        qsplit.next().expect("QOQO_VERSION badly formatted")
    );
    let test_version: (&str, &str) = (rver.as_str(), qver.as_str());
    if version == test_version {
        let get_bytes = input
            .call_method0("to_bincode")
            .map_err(|_| QoqoError::CannotExtractObject)?;
        let bytes = get_bytes
            .extract::<Vec<u8>>()
            .map_err(|_| QoqoError::CannotExtractObject)?;
        deserialize(&bytes[..]).map_err(|_| QoqoError::CannotExtractObject)
    } else {
        Err(QoqoError::VersionMismatch)
    }
}
*/
