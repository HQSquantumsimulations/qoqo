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

use bincode::{deserialize, serialize};
use ndarray::Array2;
use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyType};
use qoqo_macros::devicewrapper;
use roqoqo::devices::{Device, GenericDevice};
/// A generic square lattice device with only next-neighbours-connectivity.
///
/// Args:
///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
///     two_qubit_gate (str): The 'hqslang' name of the basic two-qubit-gate supported by the device.
///
#[pyclass(name = "SquareLatticeDevice", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
pub struct GenericDeviceWrapper {
    /// Internal storage of [roqoqo::devices::SquareLatticeDevice]
    pub internal: GenericDevice,
}

#[devicewrapper]
impl GenericDeviceWrapper {
    /// Create new geneeric device
    ///
    /// Args:
    ///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
    ///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
    ///     single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    ///     two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    ///     multi_qubit_gates (List[str]): A list of 'hqslang' names of basic multi-qubit-gate supported by the device.
    ///
    /// Returns:
    ///     An initialized SquareLatticeDevice device with empty gate times and decoherence rates set to zero.
    ///
    #[new]
    pub fn new(number_qubits: usize) -> PyResult<Self> {
        Ok(Self {
            internal: GenericDevice::new(number_qubits),
        })
    }
}

impl GenericDeviceWrapper {
    /// Fallible conversion of generic python object..
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<GenericDevice> {
        Python::with_gil(|py| -> PyResult<GenericDevice> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<GenericDeviceWrapper>() {
                Ok(try_downcast.internal)
            } else {
                // This allows all devices to be imported as generic device
                let generic_device_candidate = input.call_method0("generic_device")?;
                let get_bytes = generic_device_candidate.call_method0("to_bincode")?;
                let bytes = get_bytes.extract::<Vec<u8>>()?;
                deserialize(&bytes[..]).map_err(|err| {
                    PyValueError::new_err(format!("Cannot treat input as GenericDevice: {}", err))
                })
            }
        })
    }
}
