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
//

use super::GenericDeviceWrapper;
use bincode::{deserialize, serialize};
use ndarray::Array2;
use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_macros::devicewrapper;
use roqoqo::devices::{Device, SquareLatticeDevice};

/// A generic square lattice device with only next-neighbours-connectivity.
///
/// Args:
///     number_rows (int): The fixed number of rows in device..
///     number_columns (int): Fixed number of columns in device.
///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
///     two_qubit_gate (str): The 'hqslang' name of the basic two-qubit-gate supported by the device.
///     default_gate_time (float): The default startig gate time.
#[pyclass(name = "SquareLatticeDevice", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
#[pyo3(
    text_signature = "(number_rows, number_columns, single_qubit_gates, two_qubit_gates, default_gate_time)"
)]
pub struct SquareLatticeDeviceWrapper {
    /// Internal storage of [roqoqo::devices::SquareLatticeDevice]
    pub internal: SquareLatticeDevice,
}

#[devicewrapper]
impl SquareLatticeDeviceWrapper {
    /// Create new SquareLatticeDevice device
    ///
    /// Args:
    ///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
    ///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     default_gate_time (float): The default startig gate time.
    ///
    /// Returns:
    ///     SquareLatticeDevice
    #[new]
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        single_qubit_gates: Vec<String>,
        two_qubit_gates: Vec<String>,
        default_gate_time: f64,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: SquareLatticeDevice::new(
                number_rows,
                number_columns,
                &single_qubit_gates,
                &two_qubit_gates,
                default_gate_time,
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

    /// Set gate time of all two-qubit gates of specific type
    ///
    /// Args:
    ///     gate[str]: The hqslang name of the two-qubit-gate.
    ///     gate_time[f64]: Gate time for the given gate, valid for all qubits in the device.
    ///
    /// Returns:
    ///     A qoqo Device with updated gate times.
    ///
    #[pyo3(text_signature = "(gate, gate_time)")]
    pub fn set_all_two_qubit_gate_times(&self, gate: &str, gate_time: f64) -> Self {
        Self {
            internal: self
                .internal
                .clone()
                .set_all_two_qubit_gate_times(gate, gate_time),
        }
    }

    /// Set gate time of all single-qubit gates of specific type
    ///
    /// Args:
    ///     gate[str]: The hqslang name of the single-qubit-gate.
    ///     gate_time[f64]: New gate time.
    ///
    /// Returns:
    ///     A qoqo Device with updated gate times.
    ///
    #[pyo3(text_signature = "(gate, gate_time)")]
    pub fn set_all_single_qubit_gate_times(&self, gate: &str, gate_time: f64) -> Self {
        Self {
            internal: self
                .internal
                .clone()
                .set_all_single_qubit_gate_times(gate, gate_time),
        }
    }

    /// Set the decoherence rates for all qubits in the SquareLatticeDevice device.
    ///
    /// Args:
    ///     rates[2d array]: Decoherence rates provided as (3x3)-matrix for all qubits in the device.
    ///
    /// Returns:
    ///     SquareLatticeDevice
    ///
    /// Raises:
    ///     PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
    #[pyo3(text_signature = "(rates)")]
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

    /// Adds qubit damping to noise rates.
    ///
    /// Args:
    ///     daming (float): The damping rates.
    ///
    /// Returns:
    ///     SquareLatticeDevice
    #[pyo3(text_signature = "(damping)")]
    pub fn add_damping_all(&mut self, damping: f64) -> Self {
        Self {
            internal: self.internal.clone().add_damping_all(damping),
        }
    }

    /// Adds qubit dephasing to noise rates.
    ///
    /// Args:
    ///     dephasing (float): The dephasing rates.
    ///
    /// Returns:
    ///     SquareLatticeDevice
    #[pyo3(text_signature = "(dephasing)")]
    pub fn add_dephasing_all(&mut self, dephasing: f64) -> Self {
        Self {
            internal: self.internal.clone().add_dephasing_all(dephasing),
        }
    }

    /// Adds qubit depolarising to noise rates.
    ///
    /// Args:
    ///     depolarising (float): The depolarising rates.
    ///
    /// Returns:
    ///     SquareLatticeDevice
    #[pyo3(text_signature = "(depolarising)")]
    pub fn add_depolarising_all(&mut self, depolarising: f64) -> Self {
        Self {
            internal: self.internal.clone().add_depolarising_all(depolarising),
        }
    }
}

impl SquareLatticeDeviceWrapper {
    /// Fallible conversion of generic python object..
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<SquareLatticeDevice> {
        Python::with_gil(|py| -> PyResult<SquareLatticeDevice> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<SquareLatticeDeviceWrapper>() {
                Ok(try_downcast.internal)
            } else {
                let get_bytes = input.call_method0("to_bincode")?;
                let bytes = get_bytes.extract::<Vec<u8>>()?;
                deserialize(&bytes[..]).map_err(|err| {
                    PyValueError::new_err(format!(
                        "Cannot treat input as SquareLatticeDevice: {}",
                        err
                    ))
                })
            }
        })
    }
}
