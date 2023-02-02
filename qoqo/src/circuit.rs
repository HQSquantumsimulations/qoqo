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

//! Module containing the Circuit class that represents a quantum circuit in qoqo.
//!
//! In qoqo, single operations are collected in a circuit to build up a quantum program.
//! Qoqo circuits are strictly linear sequences of operations.
//! The circuit struct behaves similar to a list and provides several standard
//! functions of a Vec<Operation>, such as len(), is_empty(), get(), iter() and into_iter().
//!

use crate::{QoqoError, QOQO_VERSION};
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyIndexError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use roqoqo::prelude::*;
use roqoqo::{Circuit, OperationIterator, ROQOQO_VERSION};
use std::collections::HashSet;

use crate::operations::{convert_operation_to_pyobject, convert_pyany_to_operation};

/// Module containing the Circuit class that represents a quantum circuit in qoqo.
///
/// In qoqo, single operations are collected in a circuit to build up a quantum program.
/// Qoqo circuits are strictly linear sequences of operations.
/// The circuit struct behaves similar to a list and provides several standard
/// functions of a Vec<Operation>, such as len(), is_empty(), get(), iter() and into_iter().
///

#[pymodule]
fn circuit(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<CircuitWrapper>()?;
    Ok(())
}
/// Circuit of Operations.
///
/// A quantum program is represented as a linear sequence of Operations.
#[pyclass(name = "Circuit", module = "qoqo")]
#[derive(Clone, Debug, PartialEq)]
pub struct CircuitWrapper {
    /// Internal storage of [roqoqo::Circuit]
    pub internal: Circuit,
}

impl Default for CircuitWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl CircuitWrapper {
    /// Extracts a Circuit from a CircuitWrapper python object.
    ///
    /// When working with qoqo and other rust based python packages compiled separately
    /// a downcast will not detect that two CircuitWrapper objects are compatible.
    /// Provides a custom function to convert qoqo Circuits between different Python packages.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo::Circuit]
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<Circuit> {
        Python::with_gil(|py| -> PyResult<Circuit> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<CircuitWrapper>() {
                Ok(try_downcast.internal)
            } else {
                let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo Circuit: Cast to binary representation failed".to_string())
            })?;
                let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo Circuit: Cast to binary representation failed".to_string())
            })?;
                deserialize(&bytes[..]).map_err(|err| {
                    PyTypeError::new_err(format!(
                    "Python object cannot be converted to qoqo Circuit: Deserialization failed: {}",
                    err
                ))
                })
            }
        })
    }
}

#[pymethods]
impl CircuitWrapper {
    /// Create an empty quantum Circuit.
    ///
    /// Returns:
    ///     self: The new, empty Circuit.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: Circuit::new(),
        }
    }

    /// Substitute the symbolic parameters in a clone of the Circuit according to the substitution_parameters input.
    ///
    /// Args:
    ///     substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
    ///
    /// Returns:
    ///     self: The Circuit with the parameters substituted.
    ///
    /// Raises:
    ///     RuntimeError: The parameter substitution failed.
    pub fn substitute_parameters(
        &self,
        substitution_parameters: std::collections::HashMap<&str, f64>,
    ) -> PyResult<Self> {
        let mut calculator = qoqo_calculator::Calculator::new();
        for (key, val) in substitution_parameters.iter() {
            calculator.set_variable(key, *val);
        }
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(&calculator)
                .map_err(|x| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Parameter Substitution failed: {:?}",
                        x
                    ))
                })?,
        })
    }

    /// Remap qubits in operations in clone of Circuit.
    ///
    /// Args:
    ///     mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the Circuit.
    ///
    /// Returns:
    ///     self: The Circuit with the qubits remapped.
    ///
    /// Raises:
    ///     RuntimeError: The qubit remapping failed.
    pub fn remap_qubits(&self, mapping: std::collections::HashMap<usize, usize>) -> PyResult<Self> {
        let new_internal = self.internal.remap_qubits(&mapping).map_err(|err| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Qubit remapping failed: {:?}", err))
        })?;
        Ok(Self {
            internal: new_internal,
        })
    }

    /// Return clone of the circuit with all overrotation Pragmas applied.
    ///
    /// Returns:
    ///     Circuit: Circuit with the overrotation applied
    ///
    /// Raises:
    ///     RuntimeError: Error applying PragmaOverrotation in circuit.
    ////
    /// Example:
    ///
    /// >>> circuit = Circuit()
    /// >>> circuit += PragmaOverrotation("RotateY", [1,], 20.0, 30.0)
    /// >>> circuit += RotateX(0, 0.0)
    /// >>> circuit += RotateY(0, 1.0)
    /// >>> circuit += RotateY(1, 2.0)
    /// >>> circuit += RotateY(1, 3.0)
    /// >>> circuit_overrotated = circuit.overrotate()
    /// print(circuit)
    /// print(circuit_overrotated)
    ///
    pub fn overrotate(&self) -> PyResult<Self> {
        Ok(Self {
            internal: self.internal.overrotate().map_err(|_| {
                PyRuntimeError::new_err("Error applying PragmaOverrotation in circuit")
            })?,
        })
    }

    /// Count the number of occurences of a set of operation tags in the circuit.
    ///
    /// Args:
    ///     operations (list[str]): List of operation tags that should be counted.
    ///
    /// Returns:
    ///     int: The number of occurences of these operation tags.
    pub fn count_occurences(&self, operations: Vec<&str>) -> usize {
        let mut counter: usize = 0;
        for op in self.internal.iter() {
            if operations.iter().any(|x| op.tags().contains(x)) {
                counter += 1
            }
        }
        counter
    }

    /// Return a list of the hqslang names of all operations occuring in the circuit.
    ///
    /// Returns:
    ///     set[str]: The operation types in the Circuit.
    pub fn get_operation_types(&self) -> HashSet<&str> {
        let mut operations: HashSet<&str> = HashSet::new();
        for op in self.internal.iter() {
            let _ = operations.insert(op.hqslang());
        }
        operations
    }

    /// Return a copy of the Circuit (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     Circuit: A deep copy of self.
    pub fn __copy__(&self) -> CircuitWrapper {
        self.clone()
    }

    /// Return a deep copy of the Circuit.
    ///
    /// Returns:
    ///     Circuit: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> CircuitWrapper {
        self.clone()
    }

    /// Return the roqoqo and qoqo versions from when the code was compiled.
    ///
    /// Returns:
    ///     tuple[str, str]: The roqoqo and qoqo versions.
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

    /// Return the bincode representation of the Circuit using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized Circuit (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Circuit to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize Circuit to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    #[staticmethod]
    /// Convert the bincode representation of the Circuit to a Circuit using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized Circuit (in [bincode] form).
    ///
    /// Returns:
    ///     Circuit: The deserialized Circuit.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to Circuit.
    pub fn from_bincode(input: &PyAny) -> PyResult<Self> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(Self {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to Circuit"))?,
        })
    }

    /// Return the json representation of the Circuit.
    ///
    /// Returns:
    ///     str: The serialized form of Circuit.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Circuit to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize Circuit to json"))?;
        Ok(serialized)
    }

    #[staticmethod]
    /// Convert the json representation of a Circuit to a Circuit.
    ///
    /// Args:
    ///     input (str): The serialized Circuit in json form.
    ///
    /// Returns:
    ///     Circuit: The deserialized Circuit.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to Circuit.
    pub fn from_json(json_string: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(json_string)
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to Circuit"))?,
        })
    }

    /// Return a copy of the Operation at a certain index of the Circuit.
    ///
    /// Args:
    ///     index (int): The index of the Operation to get in the Circuit.
    ///
    /// Returns:
    ///     Operation: The operation at the given index (if it exists).
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

    /// Return the copy of a slice of the Circuit.
    ///
    /// Args:
    ///     start (Optional[int]): The starting index of the slice (inclusive).
    ///     stop (Optional[int]): The stopping index of the slice (exclusive).
    ///
    /// Returns:
    ///     Circuit: The slice of the operations in the Circuit with the specified indices.
    ///
    /// Raises:
    ///     IndexError: Stop index smaller than start index.
    ///     IndexError: Stop index out of range.
    ///     IndexError: Start index out of range.
    pub fn get_slice(&self, start: Option<usize>, stop: Option<usize>) -> PyResult<CircuitWrapper> {
        let start = match start {
            Some(x) => x,
            _ => 0,
        };
        let stop = match stop {
            Some(x) => x,
            _ => self.internal.len(),
        };
        if start >= stop {
            return Err(PyIndexError::new_err(format!(
                "Stop index {} smaller than start index {}",
                stop, start
            )));
        }
        if start >= self.internal.len() {
            return Err(PyIndexError::new_err(format!(
                "Start index {} out of range",
                start
            )));
        }
        if stop > self.internal.len() {
            return Err(PyIndexError::new_err(format!(
                "Stop index {} out of range",
                stop
            )));
        }

        // This is the preferred way once advance_by has been stabilized
        // let circuit_slice: Circuit =  self.internal.iter().advance_by(start).map_err(|| PyIndexError::new_err(format!("Start index {} out of range", start)))?.take(stop-start).collect();

        let mut tmp_iter = self.internal.iter();
        if start > 0 {
            tmp_iter.nth(start - 1);
        }
        let circuit_slice: Circuit = tmp_iter.take(stop - start + 1).cloned().collect();
        Ok(CircuitWrapper {
            internal: circuit_slice,
        })
    }

    /// Return a list of definitions in the Circuit.
    ///
    /// Definitions need to be unique.
    ///
    /// Returns:
    ///     list[Operation]: A vector of the definitions in the Circuit.
    pub fn definitions(&self) -> PyResult<Vec<PyObject>> {
        let mut defs: Vec<PyObject> = Vec::new();
        for op in self
            .internal
            .definitions()
            .iter()
            .cloned()
            .map(convert_operation_to_pyobject)
        {
            defs.push(op?)
        }
        Ok(defs)
    }

    /// Return a list of all operations in the Circuit.
    ///
    /// Returns:
    ///     list[Operation]: A vector of the operations in the Circuit.
    pub fn operations(&self) -> PyResult<Vec<PyObject>> {
        let mut ops: Vec<PyObject> = Vec::new();
        for op in self
            .internal
            .operations()
            .iter()
            .cloned()
            .map(convert_operation_to_pyobject)
        {
            ops.push(op?)
        }
        Ok(ops)
    }

    /// Return a list of operations with given tag.
    ///
    /// Args:
    ///     tag (str): tag by which to filter operations.
    ///
    /// Returns:
    ///     list[Operation]: A vector of the operations with the specified tag in the Circuit.
    pub fn filter_by_tag(&self, tag: &str) -> PyResult<Vec<PyObject>> {
        let mut tagged: Vec<PyObject> = Vec::new();
        for op in self
            .internal
            .iter()
            .filter(|x| x.tags().contains(&tag))
            .cloned()
            .map(convert_operation_to_pyobject)
        {
            tagged.push(op?)
        }
        Ok(tagged)
    }

    /// Add an Operation to Circuit.
    ///
    /// Args:
    ///     op (Operation): The Operation to add to the Circuit.
    pub fn add(&mut self, op: &PyAny) -> PyResult<()> {
        let operation = convert_pyany_to_operation(op).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Operation {:?}", x))
        })?;
        self.internal.add_operation(operation);
        Ok(())
    }

    /// Return a string containing a formatted (string) representation of the Circuit.
    ///
    /// Returns:
    ///     str: The string representation of the Circuit.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.internal))
    }

    /// Return a string containing a printable representation of the Circuit.
    ///
    /// Returns:
    ///     str: The printable string representation of the Circuit.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.internal))
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on Circuit.
    ///
    /// Args:
    ///     self: The PragmaGeneralNoiseWrapper object.
    ///     other: The object to compare self to.
    ///     op: Type of comparison.
    ///
    /// Returns:
    ///     Whether the two operations compared evaluated to True or False
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other = Self::from_pyany(other);
        match op {
            pyo3::class::basic::CompareOp::Eq => match other {
                Ok(circ) => Ok(self.internal == circ),
                _ => Ok(false),
            },
            pyo3::class::basic::CompareOp::Ne => match other {
                Ok(circ) => Ok(self.internal != circ),
                _ => Ok(true),
            },
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }

    /// Create an iterator of the Circuit.
    ///
    /// Returns:
    ///     OperationIterator: The Circuit in iterator form.
    fn __iter__(slf: PyRef<Self>) -> PyResult<OperationIteratorWrapper> {
        Ok(OperationIteratorWrapper {
            internal: slf.internal.clone().into_iter(),
        })
    }

    /// Return the length of the Circuit.
    ///
    /// Returns:
    ///     int: The length of the Circuit.
    fn __len__(&self) -> usize {
        self.internal.len()
    }

    /// Return a copy of the Operation at a certain index of the Circuit.
    ///
    /// Args:
    ///     index (int): The index of the Operation to get in the Circuit.
    ///
    /// Returns:
    ///     Operation: The operation at the given index (if it exists).
    ///
    /// Raises:
    ///     IndexError: Index out of range.
    fn __getitem__(&self, index: usize) -> PyResult<PyObject> {
        let operation = self
            .internal
            .get(index)
            .ok_or_else(|| PyIndexError::new_err(format!("Index {} out of range", index)))?
            .clone();
        convert_operation_to_pyobject(operation)
    }

    /// Set an Operation at the specified index in the Circuit.
    ///
    /// Args:
    ///     index (int): The index of the Operation to set in the Circuit.
    ///     value (Operation): The Operation to set in the Circuit.
    ///
    /// Raises:
    ///     TypeError: Cannot convert python object to Operation.
    ///     IndexError: Index out of range.
    fn __setitem__(&mut self, index: usize, value: &PyAny) -> PyResult<()> {
        let operation = convert_pyany_to_operation(value)
            .map_err(|_| PyTypeError::new_err("Cannot convert python object to Operation"))?;
        let mut_reference = self
            .internal
            .get_mut(index)
            .ok_or_else(|| PyIndexError::new_err(format!("Index {} out of range", index)))?;
        *mut_reference = operation;
        Ok(())
    }

    /// Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.
    ///
    /// Args:
    ///     other (Operation): The Operation object to be added to self.
    ///
    /// Raises:
    ///     TypeError: Right hand side cannot be converted to Operation or Circuit.
    fn __iadd__(&mut self, other: Py<PyAny>) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            let other_ref = other.as_ref(py);
            match convert_pyany_to_operation(other_ref) {
                Ok(x) => {
                    self.internal += x;
                    Ok(())
                }
                Err(_) => {
                    let other = convert_into_circuit(other_ref).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!(
                            "Right hand side cannot be converted to Operation or Circuit {:?}",
                            x
                        ))
                    });
                    match other {
                        Ok(x) => {
                            self.internal += x;
                            Ok(())
                        }
                        Err(y) => Err(y),
                    }
                }
            }
        })
    }

    /// Implement the `+` (__add__) magic method to add two Circuits.
    ///
    /// Args:
    ///     lhs (Circuit): The first Circuit object in this operation.
    ///     rhs (Circuit): The second Circuit object in this operation.
    ///
    /// Returns:
    ///     lhs + rhs (Circuit): the two Circuits added together.
    ///
    /// Raises:
    ///     TypeError: Left hand side can not be converted to Circuit.
    ///     TypeError: Right hand side cannot be converted to Operation or Circuit.
    fn __add__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CircuitWrapper> {
        Python::with_gil(|py| -> PyResult<CircuitWrapper> {
            let (lhs_ref, rhs_ref) = (lhs.as_ref(py), rhs.as_ref(py));
            let self_circ = convert_into_circuit(lhs_ref).map_err(|_| {
                PyTypeError::new_err("Left hand side can not be converted to Circuit")
            })?;
            match convert_pyany_to_operation(rhs_ref) {
                Ok(x) => Ok(CircuitWrapper {
                    internal: self_circ + x,
                }),
                Err(_) => {
                    let other = convert_into_circuit(rhs_ref).map_err(|_| {
                        pyo3::exceptions::PyTypeError::new_err(
                            "Right hand side cannot be converted to Operation or Circuit",
                        )
                    })?;
                    Ok(CircuitWrapper {
                        internal: self_circ + other,
                    })
                }
            }
        })
    }
}

/// Convert generic python object to [roqoqo::Circuit].
///
/// Fallible conversion of generic python object to [roqoqo::Circuit].
pub fn convert_into_circuit(input: &PyAny) -> Result<Circuit, QoqoError> {
    if let Ok(try_downcast) = input.extract::<CircuitWrapper>() {
        return Ok(try_downcast.internal);
    }
    // Everything that follows tries to extract the circuit when two separately
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

/// Iterator for iterating over Operations in a Circuit.
#[pyclass(name = "OperationIterator", module = "qoqo")]
#[derive(Debug)]
pub struct OperationIteratorWrapper {
    internal: OperationIterator,
}

#[pymethods]
impl OperationIteratorWrapper {
    /// Create an iterator of the Circuit.
    ///
    /// Returns:
    ///     OperationIterator: The Circuit in iterator form.
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    /// Advance the iterator and return the next value.
    ///
    /// Returns None when iteration is finished. Individual iterator implementations may choose to resume iteration,
    /// and so calling next() again may or may not eventually start returning Some(Operation) again at some point.
    ///
    /// Returns:
    ///     Optional[Operation]: Operation that is next in the Iterator.
    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.internal
            .next()
            .map(|op| convert_operation_to_pyobject(op).unwrap())
    }
}
