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

//! Devices in qoqo can be abstract devices or actual hardware devices.
//!
//! * Abstract devices: Contain abstract information for the model of a quantum computer and its parameters.
//! * Actual hardware devices: These devices are provided by roqoqo backends and
//! contain the necessary information for accessing the quantum computing hardware.
//! The devices also encode a connectivity model.

use pyo3::prelude::*;

mod square_lattice;
pub use square_lattice::SquareLatticeDeviceWrapper;
mod generic_device;
pub use generic_device::GenericDeviceWrapper;
mod all_to_all;
pub use all_to_all::AllToAllDeviceWrapper;

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
///    SquareLatticeDevice
///    GenericDevice
///

#[pymodule]
pub fn devices(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<AllToAllDeviceWrapper>()?;
    module.add_class::<GenericDeviceWrapper>()?;
    module.add_class::<SquareLatticeDeviceWrapper>()?;
    Ok(())
}
