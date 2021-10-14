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

//! Traits defining the standard functions for roqoqo devices.
//!
//! Devices in roqoqo have two use cases:
//!
//! * Abstract devices: Contain abstract information for the model of a quantum computer and its parameters.
//!     They can be used to determine which Operations are available on a specific device model.
//!     A typical example are abstract linear chains of square lattices in which two-qubit operations are only
//!     available between neighbouring qubits.  
//!
//!     The abstract devices can also encode a noise model. Roqoqo noise models are in general based on a (pseudo) time
//!     needed to execute a quantum operation and Lindblad rates for the qubits in the device.
//!     Specifically in the noise model each qubit undergoes a continuous Lindblad-type decoherence time evolution:
//!
//!     $$
//!     \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
//!         L_0 = \sigma^{-} \\\\
//!         L_1 = \sigma^{+} \\\\
//!         L_3 = \sigma^{z}
//!     $$
//!     Note that as long as gate times and decoherence rates are scaled inversely any kind of units can be used,
//!     but we recommend using nanoseconds and inverse nanosecconds as units for gate times and decoherence rates.
//!
//!
//! * Actual hardware devices: These devices are provided by roqoqo backends and contain the necessary information for
//!     accessing the quantum computing hardware. The devices also encode a connectivity model
//!

use ndarray::Array2;

/// Trait for roqoqo devices.
///
/// Defines standard functions available for roqoqo devices.
pub trait Device: Sized {
    /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
    ///
    /// The base assumption
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a single qubit gate.
    /// * `qubit` - The qubit the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64>;

    /// Returns the gate time of a two qubit operation if the two qubit operation is available on device-.
    ///
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64>;

    /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
    ///
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi qubit gate.
    /// * `qubits` - The qubits the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64>;

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// $$
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_3 = \sigma^{z}
    /// $$
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix M is returned
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
<<<<<<< HEAD
    fn qubit_decoherence_rates(&self, qubit: usize) -> Option<Array2<f64>>;
=======
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>>;
>>>>>>> f5be12b58b2a622780f7b40391e5d642b3fe153e

    /// Returns the number of qubits the device supports.
    fn number_qubits(&self) -> usize;
}
