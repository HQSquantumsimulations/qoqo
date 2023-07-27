// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use pyo3::types::PyByteArray;
use qoqo_macros::devicewrapper;
use roqoqo::devices::{Device, GenericDevice};
#[cfg(feature = "json_schema")]
use roqoqo::{operations::SupportedVersion, ROQOQO_VERSION};

/// A generic device assuming all-to-all connectivity between all involved qubits.
///
/// Args:
///     number_qubits (int): The number of qubits in the device
///
/// Note:
///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
///     The memory usage will be inefficient for devices with large qubit numbers.
#[pyclass(name = "GenericDevice", module = "devices")]
#[derive(Clone, Debug, PartialEq)]
pub struct GenericDeviceWrapper {
    /// Internal storage of [roqoqo::devices::SquareLatticeDevice]
    pub internal: GenericDevice,
}

#[devicewrapper]
impl GenericDeviceWrapper {
    /// Create new generic device
    #[new]
    #[pyo3(text_signature = "(number_qubits)")]
    pub fn new(number_qubits: usize) -> PyResult<Self> {
        Ok(Self {
            internal: GenericDevice::new(number_qubits),
        })
    }

    #[cfg(feature = "json_schema")]
    /// Return the JsonSchema for the json serialisation of the class.
    ///
    /// Returns:
    ///     str: The json schema serialized to json
    #[staticmethod]
    pub fn json_schema() -> String {
        let schema = schemars::schema_for!(GenericDevice);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    #[cfg(feature = "json_schema")]
    /// Returns the current version of the qoqo library .
    ///
    /// Returns:
    ///     str: The current version of the library.
    #[staticmethod]
    pub fn current_version() -> String {
        ROQOQO_VERSION.to_string()
    }

    #[cfg(feature = "json_schema")]
    /// Return the minimum version of qoqo that supports this object.
    ///
    /// Returns:
    ///     str: The minimum version of the qoqo library to deserialize this object.
    pub fn min_supported_version(&self) -> String {
        let min_version: (u32, u32, u32) =
            GenericDevice::minimum_supported_roqoqo_version(&self.internal);
        format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
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
