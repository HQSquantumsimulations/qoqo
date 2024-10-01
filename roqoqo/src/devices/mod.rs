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
//!     
//!     Note that as long as gate times and decoherence rates are scaled inversely any kind of units can be used,
//!     but we recommend using nanoseconds and inverse nanosecconds as units for gate times and decoherence rates.
//!
//!     Specifically in the noise model each qubit undergoes a continuous Lindblad-type decoherence time evolution.
//!
//! * Actual hardware devices: These devices are provided by roqoqo backends and contain the necessary information for
//!     accessing the quantum computing hardware. The devices also encode a connectivity model
//!

#[cfg(feature = "unstable_chain_with_environment")]
use std::collections::HashMap;
#[cfg(feature = "unstable_qoqo_devices")]
use std::collections::HashSet;

use crate::RoqoqoBackendError;
#[cfg(feature = "unstable_qoqo_devices")]
use crate::{prelude::InvolveQubits, Circuit};
use ndarray::Array2;
mod generic_device;
pub use generic_device::GenericDevice;
mod all_to_all;
pub use all_to_all::AllToAllDevice;
mod square_lattice;
pub use square_lattice::SquareLatticeDevice;
// use crate::RoqoqoError;
// use std::collections::HashMap;

/// Trait for roqoqo devices.
///
/// Defines standard functions available for roqoqo devices.
///
pub trait Device {
    /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
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
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64>;

    /// Returns the names of a single qubit operations available on the device.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn single_qubit_gate_names(&self) -> Vec<String> {
        self.to_generic_device().single_qubit_gate_names()
    }

    /// Returns the names of a two qubit operations available on the device.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn two_qubit_gate_names(&self) -> Vec<String> {
        self.to_generic_device().two_qubit_gate_names()
    }

    /// Returns the names of a mutli qubit operations available on the device.
    ///
    /// The list of names also includes the three qubit gate operations.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn multi_qubit_gate_names(&self) -> Vec<String> {
        self.to_generic_device().multi_qubit_gate_names()
    }

    /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64>;

    /// Returns the gate time of a three qubit operation if the three qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control_0` - The control_0 qubit the gate acts on.
    /// * `control_1` - The control_1 qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64>;

    /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
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
    ///
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64>;

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>>;

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize;

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)>;

    /// Changes the device topology based on a Pragma operation.
    ///
    /// Specific devices and backends can allow changes to the device topology.
    /// These changes are represented by Pragma operations that are only available for
    /// the corresponding backend.
    /// This function provides a generic interface for changing the devices with the help of
    /// these Pragma operations.
    /// In normal operation the backend specific Pragma operations are wrapped in a [crate::operations::PragmaChangeDevice]
    /// wrapper operation and encoded in binary form with the [bincode] crate.
    /// This function takes the encoded binary representation, tries to deserialize it internally
    ///  and applies the corresponding changes.
    ///
    /// For most devices the default behaviour is that the device cannot be changed
    /// and the function returns a corresponding RoqoqoBackendError
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the wrapped operation
    /// * `operation` - The Pragma operation encoded in binary form using the [bincode] crate
    ///
    /// # Returns
    ///
    /// Result of changing the device.
    /// In case the device is not allowed to be changed a generic RoqoqoBackendError is returned.
    /// If not implemented, a default `method has not been implemented` error is returned.
    ///
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        Err(RoqoqoBackendError::GenericError {
            msg: "The `change_device()` method has not been implemented.".to_string(),
        })
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// # Note
    ///
    /// [crate::devices::GenericDevice] uses nested HashMaps to represent the most general device connectivity.
    /// The memory usage will be inefficient for devices with large qubit numbers.
    fn to_generic_device(&self) -> GenericDevice;
}

#[cfg(feature = "unstable_qoqo_devices")]
/// Trait for new qoqo devices.
///
/// Defines standard functions available for roqoqo devices.
///
pub trait QoqoDevice {
    /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
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
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64>;

    /// Returns the names of a single qubit operations available on the device.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn single_qubit_gate_names(&self) -> Vec<String>;

    /// Returns the names of a two qubit operations available on the device.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn two_qubit_gate_names(&self) -> Vec<String>;

    /// Returns the names of a mutli qubit operations available on the device.
    ///
    /// The list of names also includes the three qubit gate operations.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The list of gate names.
    ///
    fn multi_qubit_gate_names(&self) -> Vec<String>;

    /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64>;

    /// Returns the gate time of a three qubit operation if the three qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control_0` - The control_0 qubit the gate acts on.
    /// * `control_1` - The control_1 qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    #[allow(unused_variables)]
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        None
    }

    /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
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
    ///
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64>;

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>>;

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize;

    /// Return a list of longest linear chains through the device.
    ///
    /// Returns at least one chain of qubits with linear connectivity in the device,
    /// that has the maximum possible number of qubits with linear connectivity in the device.
    /// Can return more that one of the possible chains but is not guaranteed to return
    /// all possible chains. (For example for all-to-all connectivity only one chain will be returned).
    ///
    /// # Returns
    ///
    /// Vec<Vec<usize>> - A list of the longest chains given by vectors of qubits in the chain.
    ///
    fn longest_chains(&self) -> Vec<Vec<usize>>;

    /// Return a list of longest closed linear chains through the device.
    ///
    /// Returns at least one chain of qubits with linear connectivity in the device ,
    /// that has the maximum possible number of qubits with linear connectivity in the device.
    /// The chain must be closed, the first qubit needs to be connected to the last qubit.
    /// Can return more that one of the possible chains but is not guaranteed to return
    /// all possible chains. (For example for all-to-all connectivity only one chain will be returned).
    ///
    /// # Returns
    ///
    /// Vec<Vec<usize>> - A list of the longest chains given by vectors of qubits in the chain.
    ///
    fn longest_closed_chains(&self) -> Vec<Vec<usize>>;

    /// Adds the noise contributions due to qubit gates being applied
    ///
    /// This functions adds additional noise that occurs in the device,
    /// when one or more unitary gates are applied in parallel.
    /// This does NOT include the background noise that is defined by the
    /// gate time and the constant decoherence rates on all qubits.
    /// The noise is inserted in the form of roqoqo noise Pragmas.
    ///
    /// # Arguments
    ///
    /// `circuit` - The circuit of parallel native operations for which the noise
    ///             is inserted
    ///
    /// # Returns
    ///
    /// `Ok(Circuit)` - The circuit of parallel operations containing additional noise Pragmas
    /// `Err` - Error applying the noise. Usually occurs when the gates in the Circuit cannot
    ///         be executed or cannot be executed in parallel.
    fn add_active_gate_noise(&self, circuit: &Circuit) -> Result<Circuit, RoqoqoBackendError> {
        use crate::operations::GateOperation;
        use crate::operations::InvolvedQubits;
        let mut invovlved_qubits = HashSet::<usize>::new();
        for op in circuit.iter() {
            if let Ok(operation) = GateOperation::try_from(op) {
                if let InvolvedQubits::Set(involved_set) = operation.involved_qubits() {
                    if invovlved_qubits.is_disjoint(&involved_set) {
                        invovlved_qubits.extend(involved_set)
                    } else {
                        return Err(RoqoqoBackendError::GenericError { msg: "Error add_active_gate_noise: Several unitary gates operate on same qubit in a parallel set of operations".to_string()});
                    }
                }
            }
        }
        Ok(circuit.clone())
    }

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)>;

    /// Changes the device topology based on a Pragma operation.
    ///
    /// Specific devices and backends can allow changes to the device topology.
    /// These changes are represented by Pragma operations that are only available for
    /// the corresponding backend.
    /// This function provides a generic interface for changing the devices with the help of
    /// these Pragma operations.
    /// In normal operation the backend specific Pragma operations are wrapped in a [crate::operations::PragmaChangeDevice]
    /// wrapper operation and encoded in binary form with the [bincode] crate.
    /// This function takes the encoded binary representation, tries to deserialize it internally
    ///  and applies the corresponding changes.
    ///
    /// For most devices the default behaviour is that the device cannot be changed
    /// and the function returns a corresponding RoqoqoBackendError
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the wrapped operation
    /// * `operation` - The Pragma operation encoded in binary form using the [bincode] crate
    ///
    /// # Returns
    ///
    /// Result of changing the device.
    /// In case the device is not allowed to be changed a generic RoqoqoBackendError is returned.
    ///
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        Err(RoqoqoBackendError::GenericError {
            msg: "The `change_device()` method has not been implemented.".to_string(),
        })
    }
}

#[cfg(feature = "unstable_chain_with_environment")]
/// The description of a chain and environment.
/// The first list contains all the qubits in the chain,
/// the second entry the HashMap contains mapps each qubit in the chain
/// to the qubits of the environment it is connected to.
pub type ChainAndEnvironment = (Vec<usize>, HashMap<usize, Vec<usize>>);

#[cfg(feature = "unstable_chain_with_environment")]
/// Trait implemented by devices that can return a list of chains
///
pub trait ChainWithEnvironmentDevice {
    /// Return a list of linear chains with an environment through the device.
    ///
    /// Returns at least one chain of qubits with linear connectivity and an environment in the device.
    /// An environment is defined as at least one qubit that is connected to at least one qubit of the chain
    /// but not part of the chain.
    /// For each ratio of environment qubits to chain qubits, the list contains at least one of the longest chains
    /// in the devive with that ratio. (Unless that chain and environment is simply a subset
    /// of a chain with equal or longer length and equal or higher ratio).
    ///
    /// For example, take a device with the connectivity graph:
    /// ```ignore
    /// 0 - 3 - 6
    /// |   |   |
    /// 1 - 4 - 7
    /// |   |
    /// 2 - 5
    /// ```
    /// It would have one chain of length 1 and environment with ratio 4 to 1:
    ///
    /// ```ignore
    /// ([4], {4: [1,4,7,5]})
    /// ```
    ///
    /// One with length 2 and ratio 5 to 2:
    /// ```ignore
    /// ([3,4], {3:[0,6], 4: [1,7,5]})
    /// ```
    ///
    /// The chain and environment with length 2 and ratio 2 to 1 is a subset of the one above
    /// and does not need to be listed separately.
    ///
    /// The longest chain with ratio 1 to 1 is:
    /// ```ignore
    /// ([0,1,4,3], {1:[2], 4: [5,7], 3: [6]})
    /// ```
    /// One of the longest chains with ratio 2 to 6 is
    /// ```ignore
    /// ([0,1,2,5,4,3], {4: [7], 3: [6]})
    /// ```
    /// And one of the possible chains with just one environment qubit is:
    /// ```ignore
    /// ([0,1,2,5,4,3,6], {6: [7], 4: [7]})
    /// ```
    ///
    /// # Returns
    ///
    /// Vec<(Vec<usize>, HashMap<usize, Vec<usize>>)> - A list of the chains and environments.
    ///
    fn environment_chains(&self) -> Vec<ChainAndEnvironment>;
}
