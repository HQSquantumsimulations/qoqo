// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//!   contain the necessary information for accessing the quantum computing hardware.
//!   The devices also encode a connectivity model.

#[cfg(feature = "unstable_chain_with_environment")]
use std::collections::HashMap;

use pyo3::prelude::*;

mod square_lattice;
#[cfg(feature = "unstable_chain_with_environment")]
use roqoqo::{devices::ChainWithEnvironmentDevice, RoqoqoError};
pub use square_lattice::SquareLatticeDeviceWrapper;
mod generic_device;
pub use generic_device::GenericDeviceWrapper;
mod all_to_all;
pub use all_to_all::AllToAllDeviceWrapper;

#[cfg(feature = "unstable_chain_with_environment")]
/// A wrapper around a python object that implements the ChainWithEnvironment trait.
///
/// Can be used to avoid deserializain the python object.
#[derive(Debug)]
pub struct ChainWithEnvironmentCapsule {
    internal: Py<PyAny>,
}

#[cfg(feature = "unstable_chain_with_environment")]
impl ChainWithEnvironmentCapsule {
    /// Creates a new ChainWithEnvironmentCapsule for a Python object.
    ///
    /// # Arguments
    ///
    /// * `python_device` - The python object that should implement the
    pub fn new(python_device: &Bound<PyAny>) -> Result<Self, RoqoqoError> {
        let __implements_environment_with_chains =
            python_device.call_method0("__implements_environment_chains");
        let implements_protocol =
            __implements_environment_with_chains.map(|implement| implement.extract::<bool>());

        match implements_protocol {
            Ok(Ok(true)) => Python::with_gil(|py| -> Result<Self, RoqoqoError> {
                Ok(Self {
                    internal: python_device.into_py(py),
                })
            }),
            _ => Err(RoqoqoError::GenericError {
                msg: "Python device does not implement `environment_chains` method.".to_string(),
            }),
        }
    }
}

#[cfg(feature = "unstable_chain_with_environment")]
impl ChainWithEnvironmentDevice for ChainWithEnvironmentCapsule {
    fn environment_chains(&self) -> Vec<roqoqo::devices::ChainAndEnvironment> {
        Python::with_gil(|py| -> Vec<roqoqo::devices::ChainAndEnvironment> {
            let chains_with_environment = self
                .internal
                .call_method0(py, "__environment_chains")
                .expect("Internal error `environment_chains` on python device failed.");

            chains_with_environment
                  .extract::<Vec<(Vec<usize>, HashMap<usize, Vec<usize>>)>>(py)
                  .expect("Internal error `environment_chains` on python device does not return valid description.")
        })
    }
}

/// Devices in qoqo have two use cases:
///
/// * Abstract devices: Contain abstract information for the model of a quantum computer and its parameters.
///     They can be used to determine which Operations are available on a specific device model.
///     A typical example are abstract linear chains of square lattices in which two-qubit operations are only
///     available between neighbouring qubits.  
///
///     The abstract devices can also encode a noise model. Qoqo noise models are in general based on a (pseudo) time
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
/// The devices were introduced after qoqo 1.0.0, but their design may be refactored later for backwards compatibility
///
/// .. autosummary::
///     :toctree: generated/
///     
///     AllToAllDevice
///     GenericDevice
///     SquareLatticeDevice

#[pymodule]
pub fn devices(_py: Python, module: &Bound<PyModule>) -> PyResult<()> {
    module.add_class::<AllToAllDeviceWrapper>()?;
    module.add_class::<GenericDeviceWrapper>()?;
    module.add_class::<SquareLatticeDeviceWrapper>()?;
    Ok(())
}
