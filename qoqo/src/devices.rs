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

//! Devices in qoqo can be abstract devices or actual hardware devices.
//!
//! * Abstract devices: Contain abstract information for the model of a quantum computer and its parameters.
//! * Actual hardware devices: These devices are provided by roqoqo backends and
//! contain the necessary information for accessing the quantum computing hardware.
//! The devices also encode a connectivity model.

use bincode::{deserialize, serialize};
use ndarray::Array2;
use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyType};
use qoqo_macros::devicewrapper;
use roqoqo::devices::{AllToAllDevice, Device, GenericChain, GenericDevice, GenericGrid};

/// Devices in qoqo have two use cases:
///
/// * Abstract devices: Contain abstract information for the model of a quantum computer and its parameters.
///     They can be used to determine which Operations are available on a specific device model.
///     A typical example are abstract linear chains of square lattices in which two-qubit operations are only
///     available between neighbouring qubits.  
///
///     The abstract devices can also encode a noise model. Q/// Qoqo devicesoqo noise models are in general based on a (pseudo) time
///     needed to execute a quantum operation and Lindblad rates for the qubits in the device.
///     Specifically in the noise model each qubit undergoes a continuous Lindblad-type decoherence time evolution:
///
///     math::
///     \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
///         L_0 = \sigma^{+} \\\\
///         L_1 = \sigma^{-} \\\\
///         L_3 = \sigma^{z}
///     $$
///     Note that as long as gate times and decoherence rates are scaled inversely any kind of units can be used,
///     but we recommend using nanoseconds and inverse nanosecconds as units for gate times and decoherence rates.
///
///
/// * Actual hardware devices: These devices are provided by roqoqo backends and contain the necessary information for
///     accessing the quantum computing hardware. The devices also encode a connectivity model.
///
/// .. autosummary::
///    :toctree: generated/
///
///    AllToAllDevice
///    GenericDevice
///     GenericChain
///     GenericGrid
///
#[pymodule]
pub fn devices(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<GenericGridWrapper>()?;
    module.add_class::<GenericChainWrapper>()?;
    module.add_class::<GenericDeviceWrapper>()?;
    module.add_class::<AllToAllDeviceWrapper>()?;
    Ok(())
}

/// A generic 2D Grid Device with only next-neighbours-connectivity.
///
/// Args:
///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
///     two_qubit_gate (str): The 'hqslang' name of the basic two-qubit-gate supported by the device.
///
#[pyclass(name = "GenericGrid", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
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
    ///     qubit[int]: The qubit for which the rate matrix M is returned.
    ///
    /// Returns:
    ///     Decoherence rates: a 2d array of real numbers.
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

    /// Function to set the decoherence rates for all qubits in the GenericGrid device.
    ///
    /// Args:
    ///     rates[2d array]: Decoherence rates provided as (3x3)-matrix for all qubits in the device.
    ///
    /// Returns:
    ///     self: GenericGrid with updated decoherence rates.
    ///
    /// Raises:
    ///     PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
    pub fn set_all_qubit_decoherence_rates(&self, rates: PyReadonlyArray2<f64>) -> PyResult<Self> {
        let rates_matrix = rates.as_array().to_owned();
        Ok(Self {
            internal: self
                .internal
                .clone()
                .set_all_qubit_decoherence_rates(rates_matrix)
                .map_err(|_| {
                    PyValueError::new_err("The input parameter `rates` needs to be a (3x3)-matrix.")
                })?,
        })
    }
}

/// A device assuming all-to-all connectivity between all involved qubits.
///
#[pyclass(name = "AllToAllDevice", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
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

    /// Function to set the decoherence rates for all qubits in the AllToAllDevice.
    ///
    /// Args:
    ///     rates[2d array]: Decoherence rates provided as (3x3)-matrix for all qubits in the device.
    ///
    /// Returns:
    ///     AllToAllDevice with updated decoherence rates.
    ///
    /// Raises:
    ///     PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
    ///
    pub fn set_all_qubit_decoherence_rates(&self, rates: PyReadonlyArray2<f64>) -> PyResult<Self> {
        let rates_matrix = rates.as_array().to_owned();
        Ok(Self {
            internal: self
                .internal
                .clone()
                .set_all_qubit_decoherence_rates(rates_matrix)
                .map_err(|_| {
                    PyValueError::new_err("The input parameter `rates` needs to be a (3x3)-matrix.")
                })?,
        })
    }
}

/// A device struct with public fields for a qoqo device
/// with all-to-all connectivity between all involved qubits.
///
#[pyclass(name = "GenericDevice", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
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

    /// Function to set the decoherence rates for all qubits in the GenericDevice device.
    ///
    /// Args:
    ///     rates[2d array]: Decoherence rates provided as (3x3)-matrix for all qubits in the device.
    ///
    /// Returns:
    ///     GenericDevice with updated decoherence rates.
    ///
    /// Raises:
    ///     PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
    ///
    pub fn set_all_qubit_decoherence_rates(&self, rates: PyReadonlyArray2<f64>) -> PyResult<Self> {
        let rates_matrix = rates.as_array().to_owned();
        Ok(Self {
            internal: self
                .internal
                .clone()
                .set_all_qubit_decoherence_rates(rates_matrix)
                .map_err(|_| {
                    PyValueError::new_err("The input parameter `rates` needs to be a (3x3)-matrix.")
                })?,
        })
    }
}

/// A device containing a linear chain of qubits with next neighbour connectivity.
///
#[pyclass(name = "GenericChain", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
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

    /// Function to set the decoherence rates for all qubits in the GenericChain device.
    ///
    /// Args:
    ///     rates[2d array]: Decoherence rates provided as (3x3)-matrix for all qubits in the device.
    ///
    /// Returns:
    ///     GenericChain with updated decoherence rates.
    ///
    /// Raises:
    ///     PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
    ///
    pub fn set_all_qubit_decoherence_rates(&self, rates: PyReadonlyArray2<f64>) -> PyResult<Self> {
        let rates_matrix = rates.as_array().to_owned();
        Ok(Self {
            internal: self
                .internal
                .clone()
                .set_all_qubit_decoherence_rates(rates_matrix)
                .map_err(|_| {
                    PyValueError::new_err("The input parameter `rates` needs to be a (3x3)-matrix.")
                })?,
        })
    }
}
