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
//

//! Qoqo devices

use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyType};
use roqoqo::devices::{Device, GenericGrid};
// use ndarray::Array2;

/// A generic 2D Grid Device with only next-neighbours-connectivity.
///
/// Args:
///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
///     two_qubit_gate (str): The 'hqslang' name of the basic two-qubit-gate supported by the device.
///
#[pyclass(name = "GenericGrid", module = "qoqo")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericGridWrapper {
    /// Internal storage of [roqoqo::devices::GenericGrid]
    pub internal: GenericGrid,
}

#[pymethods]
impl GenericGridWrapper {
    /// Create new GenericGrid device
    ///
    /// Args:
    ///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
    ///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gate (str): The 'hqslang' name of the basic two-qubit-gate supported by the device.
    ///
    #[new]
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gate: String,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: GenericGrid::new(
                number_rows,
                number_columns,
                &single_qubit_gates,
                two_qubit_gate,
            ),
        })
    }

    /// Return a copy of the GenericGrid (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     GenericGrid: A deep copy of self.
    pub fn __copy__(&self) -> GenericGridWrapper {
        self.clone()
    }

    /// Return a deep copy of the GenericGrid.
    ///
    /// Returns:
    ///     GenericGrid: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> GenericGridWrapper {
        self.clone()
    }

    /// Return the bincode representation of the GenericGrid using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized GenericGrid (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize GenericGrid to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize GenericGrid to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the GenericGrid to a GenericGrid using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized GenericGrid (in bincode form).
    ///
    /// Returns:
    ///     GenericGrid: The deserialized GenericGrid.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to GenericGrid.
    #[classmethod]
    pub fn from_bincode(_cls: &PyType, input: &PyAny) -> PyResult<GenericGridWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(GenericGridWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to GenericGrid")
            })?,
        })
    }

    /// Return the json representation of the GenericGrid.
    ///
    /// Returns:
    ///     str: The serialized form of GenericGrid.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize GenericGrid to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize GenericGrid to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a GenericGrid to a GenericGrid.
    ///
    /// Args:
    ///     input (str): The serialized GenericGrid in json form.
    ///
    /// Returns:
    ///     GenericGrid: The deserialized GenericGrid.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to GenericGrid.
    #[classmethod]
    fn from_json(_cls: &PyType, input: &str) -> PyResult<GenericGridWrapper> {
        Ok(GenericGridWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to GenericGrid")
            })?,
        })
    }

    /// Return number of qubits in device.
    ///
    /// Returns:
    ///     int: The number of qubits.
    ///
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Return the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.
    ///
    /// Returns:
    ///     int: The number of rows.
    ///
    pub fn number_rows(&self) -> usize {
        self.internal.number_rows()
    }

    /// Return number of columns in device.
    ///
    /// Returns:
    ///     int: The number of columns.
    ///
    pub fn number_columns(&self) -> usize {
        self.internal.number_columns()
    }

    /// Return the list of pairs of qubits linked by a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the user's
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// Returns:
    ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_qubit_edges()
    }

    // DRAFT: trait IntoPyCallbackOutput is not implemented for Option<Array2>.
    // /// Returns the matrix of the decoherence rates of the Lindblad equation.
    // ///
    // /// Args:
    // ///     qubit[int]: The qubit for which the rate matrix M is returned
    // ///
    // /// Returns:
    // ///
    // /// Option: Some<Array2<f64>> for the decoherence rates.
    // ///                 or None if the qubit is not part of the device.
    // ///
    // fn qubit_decoherence_rates(&self, qubit: usize) -> Option<Array2<f64>> {
    //     self.internal.qubit_decoherence_rates(&qubit)
    // }

    /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
    ///
    /// Args:
    ///     hqslang[str]: The hqslang name of a single qubit gate.
    ///     qubit[int]: The qubit the gate acts on
    ///
    /// Returns:
    ///     Option: Some<f64> for the gate time.
    ///                      Or None if the gate is not available on the device.
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> Option<f64> {
        self.internal.single_qubit_gate_time(hqslang, &qubit)
    }

    /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
    ///
    /// Args:
    ///     hqslang[str]: The hqslang name of a single qubit gate.
    /// control[int]: The control qubit the gate acts on.
    /// target[int]: The target qubit the gate acts on.
    ///
    /// Returns:
    ///     Option: Some<f64> for the gate time.
    ///                      Or None if the gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: usize, target: usize) -> Option<f64> {
        self.internal
            .two_qubit_gate_time(hqslang, &control, &target)
    }

    /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
    ///
    /// Args:
    ///     hqslang[str]: The hqslang name of a multi qubit gate.
    ///     qubits[List[int]]: The qubits the gate acts on.
    ///
    /// Returns:
    ///     Option: Some<f64> for the gate time.
    ///                      Or None if the gate is not available on the device.
    ///
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> Option<f64> {
        self.internal.multi_qubit_gate_time(hqslang, &qubits)
    }
}
