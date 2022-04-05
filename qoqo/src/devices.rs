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
use ndarray::Array2;
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyType};
use qoqo_macros::devicewrapper;
use roqoqo::devices::{AllToAllDevice, Device, GenericChain, GenericDevice, GenericGrid};

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

#[devicewrapper]
impl GenericGridWrapper {
    /// Create new GenericGrid device
    ///
    /// Args:
    ///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
    ///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     multi_qubit_gates (List[str]): A list of 'hqslang' names of basic multi-qubit-gate supported by the device.
    ///
    /// Returns:
    ///     An initialized GenericGrid device with empty gate times and decoherence rates set to zero.
    ///
    #[new]
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gates: Vec<String>,
        multi_qubit_gates: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: GenericGrid::new(
                number_rows,
                number_columns,
                &single_qubit_gates,
                &two_qubit_gates,
                &multi_qubit_gates,
            ),
        })
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

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// .. math::
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_2 = \sigma^{z}
    ///
    /// Args:
    ///     qubit[int]: The qubit for which the rate matrix M is returned
    ///
    /// Returns:
    ///     Decoherence rates: a 2d array of real numbers
    ///
    fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
        Python::with_gil(|py| -> Py<PyArray2<f64>> {
            match self.internal.qubit_decoherence_rates(&qubit) {
                Some(matrix) => matrix.to_pyarray(py).to_owned(),
                None => {
                    let matrix = Array2::<f64>::zeros((3, 3));
                    matrix.to_pyarray(py).to_owned()
                }
            }
        })
    }
}

/// A device assuming all-to-all connectivity between all involved qubits.
///
#[pyclass(name = "AllToAllDevice", module = "qoqo")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllToAllDeviceWrapper {
    /// Internal storage of [roqoqo::devices::AllToAllDevice]
    pub internal: AllToAllDevice,
}

#[devicewrapper]
impl AllToAllDeviceWrapper {
    /// Create new AllToAllDevice.
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits in the device.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     multi_qubit_gates (List[str]): A list of 'hqslang' names of basic multi-qubit-gate supported by the device.
    ///
    /// Returns:
    ///     An initiated AllToAllDevice with empty gate times and decoherence rates set to zero.
    ///
    #[new]
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gates: Vec<String>,
        multi_qubit_gates: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: AllToAllDevice::new(
                number_qubits,
                &single_qubit_gates,
                &two_qubit_gates,
                &multi_qubit_gates,
            ),
        })
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// .. math::
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_2 = \sigma^{z}
    ///
    /// Args:
    ///     qubit[int]: The qubit for which the rate matrix M is returned
    ///
    /// Returns:
    ///
    /// Decoherence rates: a 2d array of real numbers
    ///
    fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
        Python::with_gil(|py| -> Py<PyArray2<f64>> {
            match self.internal.qubit_decoherence_rates(&qubit) {
                Some(matrix) => matrix.to_pyarray(py).to_owned(),
                None => {
                    let matrix = Array2::<f64>::zeros((3, 3));
                    matrix.to_pyarray(py).to_owned()
                }
            }
        })
    }
}

/// A device struct with public fields for a qoqo device
/// with all-to-all connectivity between all involved qubits.
///
#[pyclass(name = "GenericDevice", module = "qoqo")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericDeviceWrapper {
    /// Internal storage of [roqoqo::devices::GenericDevice]
    pub internal: GenericDevice,
}

#[devicewrapper]
impl GenericDeviceWrapper {
    /// Create new GenericDevice.
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits in the device.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     multi_qubit_gates (List[str]): A list of 'hqslang' names of basic multi-qubit-gate supported by the device.
    ///
    /// Returns:
    ///     An initiated GenericDevice with empty gate times and decoherence rates set to zero.
    ///
    #[new]
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gates: Vec<String>,
        multi_qubit_gates: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: GenericDevice::new(
                number_qubits,
                &single_qubit_gates,
                &two_qubit_gates,
                &multi_qubit_gates,
            ),
        })
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// .. math::
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_2 = \sigma^{z}
    ///
    /// Args:
    ///     qubit[int]: The qubit for which the rate matrix M is returned
    ///
    /// Returns:
    ///
    /// Decoherence rates: a 2d array of real numbers
    ///
    fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
        Python::with_gil(|py| -> Py<PyArray2<f64>> {
            match self.internal.qubit_decoherence_rates(&qubit) {
                Some(matrix) => matrix.to_pyarray(py).to_owned(),
                None => {
                    let matrix = Array2::<f64>::zeros((3, 3));
                    matrix.to_pyarray(py).to_owned()
                }
            }
        })
    }
}

/// A generic device containing a linear chain of qubits with next neighbour connectivity.
///
#[pyclass(name = "GenericChain", module = "qoqo")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericChainWrapper {
    /// Internal storage of [roqoqo::devices::GenericChain]
    pub internal: GenericChain,
}

#[devicewrapper]
impl GenericChainWrapper {
    /// Create new GenericChain device.
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits in the device.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     multi_qubit_gates (List[str]): A list of 'hqslang' names of basic multi-qubit-gate supported by the device.
    ///
    /// Returns:
    ///     An initialized GenericChain device with empty gate times and decoherence rates set to zero.
    ///
    #[new]
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gates: Vec<String>,
        multi_qubit_gates: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: GenericChain::new(
                number_qubits,
                &single_qubit_gates,
                &two_qubit_gates,
                &multi_qubit_gates,
            ),
        })
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// .. math::
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_2 = \sigma^{z}
    ///
    /// Args:
    ///     qubit[int]: The qubit for which the rate matrix M is returned
    ///
    /// Returns:
    ///
    /// Decoherence rates: a 2d array of real numbers
    ///
    fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
        Python::with_gil(|py| -> Py<PyArray2<f64>> {
            match self.internal.qubit_decoherence_rates(&qubit) {
                Some(matrix) => matrix.to_pyarray(py).to_owned(),
                None => {
                    let matrix = Array2::<f64>::zeros((3, 3));
                    matrix.to_pyarray(py).to_owned()
                }
            }
        })
    }
}
