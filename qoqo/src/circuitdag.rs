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
use crate::QoqoError;
use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;
use roqoqo::{Circuit, CircuitDag};

use crate::operations::{convert_operation_to_pyobject, convert_pyany_to_operation};
use crate::CircuitWrapper;

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
#[derive(Clone, Debug, PartialEq)]
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

    /// Create a CircuitDag from a given Circuit;
    ///
    /// Returns:
    ///     self: The new CircuitDag.
    pub fn from_circuit(&self, circuit: Py<PyAny>) -> PyResult<Self> {
        let circuit = Python::with_gil(|py| -> Result<Circuit, QoqoError> {
            let circ_ref = circuit.as_ref(py);
            crate::convert_into_circuit(circ_ref)
        })
        .unwrap();

        Ok(Self {
            internal: CircuitDag::from(circuit),
        })
    }

    /// Transforms the CircuitDag into a Circuit.
    ///
    pub fn to_circuit(&self) -> PyResult<CircuitWrapper> {
        Ok(CircuitWrapper {
            internal: Circuit::from(self.internal.clone()),
        })
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

    /// Returns a copy of the CircuitDag (produces a deepcopy).
    ///
    /// Returns:
    ///     CircuitDag: A copy of self.
    pub fn __copy__(&self) -> CircuitDagWrapper {
        self.clone()
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on Circuit.
    ///
    /// Args:
    ///     self: The CircuitDag object.
    ///     other: The object to compare self to.
    ///     op: Type of comparison.
    ///
    /// Returns:
    ///     Whether the two operations compared evaluated to True or False.
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented.
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other = Python::with_gil(|py| -> Result<CircuitDag, QoqoError> {
            let other_ref = other.as_ref(py);
            crate::convert_into_circuitdag(other_ref)
        });
        match op {
            pyo3::class::basic::CompareOp::Eq => match other {
                Ok(dag) => Ok(self.internal == dag),
                _ => Ok(false),
            },
            pyo3::class::basic::CompareOp::Ne => match other {
                Ok(dag) => Ok(self.internal != dag),
                _ => Ok(true),
            },
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }
}

/// Convert generic python object to [roqoqo::CircuitDag].
///
/// Fallible conversion of generic python object to [roqoqo::CircuitDag].
pub fn convert_into_circuitdag(input: &PyAny) -> Result<CircuitDag, QoqoError> {
    if let Ok(try_downcast) = input.extract::<CircuitDagWrapper>() {
        return Ok(try_downcast.internal);
    }
    /*
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
    } */
    else {
        Err(QoqoError::VersionMismatch)
    }
}
