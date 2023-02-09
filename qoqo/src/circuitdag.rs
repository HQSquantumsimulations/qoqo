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

///! Module containing the CircuitDag class that represents the Directed Acyclic Graph (DAG)
///! of a quantum circuit in qoqo.
///!
use std::collections::HashSet;

use crate::{QoqoError, QOQO_VERSION};
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use roqoqo::{Circuit, CircuitDag, ROQOQO_VERSION};

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
#[pyo3(text_signature = "(node_number, edge_number, /)")]
pub struct CircuitDagWrapper {
    /// Internal storage of [roqoqo:CircuitDag]
    pub internal: CircuitDag,
}

impl Default for CircuitDagWrapper {
    fn default() -> Self {
        Self::new(100, 300)
    }
}

impl CircuitDagWrapper {
    /// Extracts a CircuitDag from a CircuitDagWrapper python object.
    ///
    /// When working with qoqo and other rust based python packages compiled separately
    /// a downcast will not detect that two CircuitDagWrapper objects are compatible.
    /// Provides a custom function to convert qoqo CircuitDags between different Python packages.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo::Circuit]
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<CircuitDag> {
        Python::with_gil(|py| -> PyResult<CircuitDag> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<CircuitDagWrapper>() {
                Ok(try_downcast.internal)
            } else {
                let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo CircuitDag: Cast to binary representation failed".to_string())
            })?;
                let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo CircuitDag: Cast to binary representation failed".to_string())
            })?;
                deserialize(&bytes[..]).map_err(|err| {
                PyTypeError::new_err(format!(
                    "Python object cannot be converted to qoqo CircuitDag: Deserialization failed: {}",
                    err
                ))}
            )
            }
        })
    }
}

#[pymethods]
impl CircuitDagWrapper {
    /// Create an empty CircuitDag.
    ///
    /// Args:
    ///     node_number (int): The node max capacity of the new CircuitDag.
    ///     edge_number (int): The edge max capacity of the new CircuitDag.
    ///
    /// Returns:
    ///     self: The new, empty CircuitDag.
    #[new]
    #[pyo3(signature=(node_number = 100, edge_number = 300))]
    pub fn new(node_number: usize, edge_number: usize) -> Self {
        Self {
            internal: CircuitDag::with_capacity(node_number, edge_number),
        }
    }

    /// Create a CircuitDag from a given Circuit;
    ///
    /// Args:
    ///     circuit (Circuit): The Circuit to build the new CircuitDag from.
    ///
    /// Returns:
    ///     self: The new CircuitDag.
    #[pyo3(text_signature = "(circuit)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, op)")]
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
    #[pyo3(text_signature = "($self, op)")]
    pub fn add_to_front(&mut self, op: &PyAny) -> PyResult<Option<usize>> {
        let operation = convert_pyany_to_operation(op).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Operation {:?}", x))
        })?;
        Ok(self.internal.add_to_front(operation))
    }

    /// Checks if executing an operation is blocked by any not-yet executed operation.
    ///
    /// Args:
    ///     already_executed (list[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
    ///     to_be_executed (int): NodeIndex of the operation that should be executed next.
    ///
    /// Returns:
    ///     list[int]: List containing the sorted blocking elements.
    #[pyo3(text_signature = "($self, already_executed, to_be_executed)")]
    pub fn execution_blocked(
        &self,
        already_executed: Vec<usize>,
        to_be_executed: usize,
    ) -> Vec<usize> {
        self.internal
            .execution_blocked(already_executed.as_slice(), &to_be_executed)
    }

    /// Checks which of the direct predecessors of an Operation in the CircuitDag blocks the execution.
    ///
    /// Warning:
    /// This method can only be used to determine if an operation can be executed when `already_executed` is consistent.
    /// When the list  `already_executed` is inconsistent (a n operation is reported as executed that could not have been executed yet)
    /// this method returning an empty vector does not imply that the `to_be_executed` operation can be executed.
    ///
    /// Args:
    ///     already_executed (list[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
    ///     to_be_executed(int): NodeIndex of the Operation that should be executed next.
    ///
    /// Returns:
    ///     list[int]: List containing the sorted blocking elements.
    #[pyo3(text_signature = "($self, already_executed, to_be_executed)")]
    pub fn blocking_predecessors(
        &self,
        already_executed: Vec<usize>,
        to_be_executed: usize,
    ) -> Vec<usize> {
        self.internal
            .blocking_predecessors(already_executed.as_slice(), &to_be_executed)
    }

    /// Returns a new front-layer after executing an operation from the current front layer.
    ///
    /// Returns an error if operation to be executed is not in the current front layer.
    ///
    /// Args:
    ///     already_executed (list[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
    ///     current_front_layer (list[int]): List of NodeIndices in the current front layer ready to be executed if physically possible.
    ///     to_be_executed (int): NodeIndex of the operation that should be executed next.
    #[pyo3(text_signature = "($self, already_executed, current_front_layer, to_be_executed)")]
    pub fn new_front_layer(
        &self,
        already_executed: Vec<usize>,
        current_front_layer: Vec<usize>,
        to_be_executed: usize,
    ) -> PyResult<Vec<usize>> {
        self.internal
            .new_front_layer(
                already_executed.as_slice(),
                current_front_layer.as_slice(),
                &to_be_executed,
            )
            .map_err(|_| {
                PyValueError::new_err(
                    "The Operation to be executed is not in the current front layer.".to_string(),
                )
            })
    }

    /// Returns an iterator over the possible parallel blocks in circuit that can be executed simultaneously
    ///
    /// Returns an Iterator over Vectors of references to the NodeIndices in the parallel block as well
    /// as references to the Operation in the blocks
    #[pyo3(text_signature = "($self)")]
    pub fn parallel_blocks(&self) -> Vec<Vec<usize>> {
        let mut par_bl_vec: Vec<Vec<usize>> = Vec::new();

        for block in self.internal.parallel_blocks() {
            par_bl_vec.push(block.clone());
        }

        par_bl_vec
    }

    /// Given a NodeIndex, returns the Operation contained in the node of
    /// the CircuitDag.
    ///
    /// Args:
    ///     index (int): The index of the node to get from the CircuitDag.
    ///
    /// Returns:
    ///     Operation: The Operation at the given index (if it exists).
    ///
    /// Raises:
    ///     IndexError: Index out of range.
    #[pyo3(text_signature = "($self, index)")]
    pub fn get(&self, index: usize) -> PyResult<PyObject> {
        let operation = self
            .internal
            .get(index)
            .ok_or_else(|| PyIndexError::new_err(format!("Index {} out of range", index)))?
            .clone();
        convert_operation_to_pyobject(operation)
    }

    /// Returns a copy of the CircuitDag (produces a deepcopy).
    ///
    /// Returns:
    ///     CircuitDag: A copy of self.
    #[pyo3(text_signature = "($self)")]
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

    /// Return the roqoqo and qoqo versions from when the code was compiled.
    ///
    /// Returns:
    ///     tuple[str, str]: The roqoqo and qoqo versions.
    #[pyo3(text_signature = "($self)")]
    fn _qoqo_versions(&self) -> (String, String) {
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
        (rver, qver)
    }

    /// Return the bincode representation of the CircuitDag using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized CircuitDag (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize CircuitDag to bytes.
    #[pyo3(text_signature = "($self)")]
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize CircuitDag to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the CircuitDag to a CircuitDag using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized CircuitDag (in [bincode] form).
    ///
    /// Returns:
    ///     CircuitDag: The deserialized CircuitDag.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to CircuitDag.
    #[staticmethod]
    #[pyo3(text_signature = "(input)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<Self> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(Self {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to CircuitDag"))?,
        })
    }

    /// Returns the list of the successors of a given node in the CircuitDag.
    ///
    pub fn successors(&self, node: usize) -> Vec<usize> {
        self.internal.successors(node).map(|n| n.index()).collect()
    }

    /// Returns the list of nodes of commuting operations in CircuitDag.
    ///
    /// Returns:
    ///     list[int]: The list of nodes of commuting operations.
    #[pyo3(text_signature = "($self)")]
    pub fn commuting_operations(&self) -> Vec<usize> {
        self.internal.commuting_operations().to_vec()
    }

    /// Returns a set containing the nodes in the first parallel block.
    ///
    /// Returns:
    ///     set[int]: The set of nodes in the first parallel block.
    #[pyo3(text_signature = "($self)")]
    pub fn first_parallel_block(&self) -> HashSet<usize> {
        self.internal.first_parallel_block().clone()
    }

    /// Returns a set containing the nodes in the last parallel block.
    ///
    /// Returns:
    ///     set[int]: The set of nodes in the last parallel block.
    #[pyo3(text_signature = "($self)")]
    pub fn last_parallel_block(&self) -> HashSet<usize> {
        self.internal.last_parallel_block().clone()
    }

    /// Returns a dictionary where a key represents a qubit and its value represents
    /// the first node that involves that qubit.
    ///
    /// Returns:
    ///     dict[int, int]: The dictionary of {qubit: node} elements.
    #[pyo3(text_signature = "($self)")]
    pub fn first_operation_involving_qubit(&self) -> PyObject {
        Python::with_gil(|py| -> PyObject {
            self.internal
                .first_operation_involving_qubit()
                .to_object(py)
        })
    }

    /// Returns a dictionary where a key represents a qubit and its value represents
    /// the last node that involves that qubit.
    ///
    /// Returns:
    ///     dict[int, int]: The dictionary of {qubit: node} elements.
    #[pyo3(text_signature = "($self)")]
    pub fn last_operation_involving_qubit(&self) -> PyObject {
        Python::with_gil(|py| -> PyObject {
            self.internal.last_operation_involving_qubit().to_object(py)
        })
    }

    /// Returns a dictionary where a key is composed by the name and the size
    /// of the classical register and its value represents the first node that involves that
    /// register.
    ///
    /// Returns:
    ///     dict[(str, int), int]: The dictionary of {(str, int), int} elements.
    #[pyo3(text_signature = "($self)")]
    pub fn first_operation_involving_classical(&self) -> PyObject {
        Python::with_gil(|py| -> PyObject {
            self.internal
                .first_operation_involving_classical()
                .to_object(py)
        })
    }

    /// Returns a dictionary where a key is composed by the name and the size
    /// of the classical register and its value represents the last node that involves that
    /// register.
    ///
    /// Returns:
    ///     dict[(str, int), int]: The dictionary of {(str, int), int} elements.
    #[pyo3(text_signature = "($self)")]
    pub fn last_operation_involving_classical(&self) -> PyObject {
        Python::with_gil(|py| -> PyObject {
            self.internal
                .last_operation_involving_classical()
                .to_object(py)
        })
    }
}

/// Convert generic python object to [roqoqo::CircuitDag].
///
/// Fallible conversion of generic python object to [roqoqo::CircuitDag].
pub fn convert_into_circuitdag(input: &PyAny) -> Result<CircuitDag, QoqoError> {
    if let Ok(try_downcast) = input.extract::<CircuitDagWrapper>() {
        return Ok(try_downcast.internal);
    }
    // Everything that follows tries to extract the circuitdag when two separately
    // compiled python packages are involved
    // let get_version = input
    //     .call_method0("_qoqo_versions")
    //     .map_err(|_| QoqoError::CannotExtractObject)?;
    // let version = get_version
    //     .extract::<(&str, &str)>()
    //     .map_err(|_| QoqoError::CannotExtractObject)?;
    // let mut rsplit = ROQOQO_VERSION.split('.').take(2);
    // let mut qsplit = QOQO_VERSION.split('.').take(2);
    // let rver = format!(
    //     "{}.{}",
    //     rsplit.next().expect("ROQOQO_VERSION badly formatted"),
    //     rsplit.next().expect("ROQOQO_VERSION badly formatted")
    // );
    // let qver = format!(
    //     "{}.{}",
    //     qsplit.next().expect("QOQO_VERSION badly formatted"),
    //     qsplit.next().expect("QOQO_VERSION badly formatted")
    // );
    // let test_version: (&str, &str) = (rver.as_str(), qver.as_str());
    // if version == test_version {
    let get_bytes = input
        .call_method0("to_bincode")
        .map_err(|_| QoqoError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoError::CannotExtractObject)?;
    deserialize(&bytes[..]).map_err(|_| QoqoError::CannotExtractObject)
    // } else {
    //     Err(QoqoError::VersionMismatch)
    // }
}
