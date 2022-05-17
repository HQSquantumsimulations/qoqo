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
//!         L_0 = \sigma^{+} \\\\
//!         L_1 = \sigma^{-} \\\\
//!         L_3 = \sigma^{z}
//!     $$
//!     Note that as long as gate times and decoherence rates are scaled inversely any kind of units can be used,
//!     but we recommend using nanoseconds and inverse nanosecconds as units for gate times and decoherence rates.
//!
//!
//! * Actual hardware devices: These devices are provided by roqoqo backends and contain the necessary information for
//!     accessing the quantum computing hardware. The devices also encode a connectivity model
//!

use crate::RoqoqoBackendError;
use ndarray::Array2;
// use crate::RoqoqoError;
// use std::collections::HashMap;

/// Trait for roqoqo devices.
///
/// Defines standard functions available for roqoqo devices.
///
pub trait Device {
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
    ///
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
    ///
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
    ///
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
    ///
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        Err(RoqoqoBackendError::GenericError {
            msg: "The device ".to_string(),
        })
    }
}

// /// A device assuming all-to-all connectivity between all involved qubits.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct AllToAllDevice {
//     number_qubits: usize,
//     single_qubit_gates: HashMap<String, Vec<SingleQubitMap>>,
//     two_qubit_gates: HashMap<String, Vec<TwoQubitMap>>,
//     multi_qubit_gates: HashMap<String, f64>,
//     decoherence_rates: HashMap<usize, Array2<f64>>,
// }

// impl AllToAllDevice {
//     /// Create new AllToAllDevice.
//     ///
//     /// # Arguments
//     ///
//     /// * `number_qubits` - The number of qubits in the device.
//     /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
//     /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
//     // * `multi_qubit_gates` - A list of 'hqslang' names of basic multi-qubit-gates supported by the device.
//     ///
//     /// # Returns
//     ///
//     /// An initiated AllToAllDevice with empty gate times and decoherence rates set to zero.
//     ///
//     pub fn new(
//         number_qubits: usize,
//         single_qubit_gates: &[String],
//         two_qubit_gates: &[String],
//         multi_qubit_gates: &[String],
//     ) -> Self {
//         // Initialization of single qubit gates with empty times
//         let mut single_qubit_gate_map: HashMap<String, Vec<SingleQubitMap>> = HashMap::new();
//         for gate in single_qubit_gates.iter() {
//             let mut empty_times: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..number_qubits {
//                 let qubittime = SingleQubitMap { qubit, time: 0.0 };
//                 empty_times.push(qubittime);
//             }
//             single_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initialization of two qubit gates with empty times
//         let mut two_qubit_gate_map: HashMap<String, Vec<TwoQubitMap>> = HashMap::new();
//         for gate in two_qubit_gates.iter() {
//             let mut empty_times: Vec<TwoQubitMap> = Vec::new();
//             for qubit0 in 0..number_qubits {
//                 for qubit1 in 0..number_qubits {
//                     if qubit0 != qubit1 {
//                         let qubittime1 = TwoQubitMap {
//                             control: qubit0,
//                             target: qubit1,
//                             time: 0.0,
//                         };
//                         let qubittime2 = TwoQubitMap {
//                             control: qubit1,
//                             target: qubit0,
//                             time: 0.0,
//                         };
//                         empty_times.push(qubittime1);
//                         empty_times.push(qubittime2);
//                     }
//                 }
//             }
//             two_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initilization of multi qubit gates with empty times when applied to any qubits
//         let mut multi_qubit_gate_map: HashMap<String, f64> = HashMap::new();
//         for gate in multi_qubit_gates.iter() {
//             multi_qubit_gate_map.insert(gate.clone(), 0.0);
//         }

//         let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
//         for qubit0 in 0..number_qubits {
//             decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
//         }

//         AllToAllDevice {
//             number_qubits,
//             single_qubit_gates: single_qubit_gate_map,
//             two_qubit_gates: two_qubit_gate_map,
//             multi_qubit_gates: multi_qubit_gate_map,
//             decoherence_rates,
//         }
//     }

//     /// Function that allows to set one gate time per gate type for the single-qubit-gates.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the single-qubit-gate.
//     /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An AllToAllDevice with updated gate times.
//     ///
//     pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.single_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut gatetimes: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..self.number_qubits() {
//                 let qubittime = SingleQubitMap {
//                     qubit,
//                     time: gate_time,
//                 };
//                 gatetimes.push(qubittime);
//             }
//             self.single_qubit_gates.insert(gate.to_string(), gatetimes);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the two-qubit-gates in AllToAllDevice.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the two-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An AllToAllDevice with updated gate times.
//     ///
//     pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.two_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut times: Vec<TwoQubitMap> = Vec::new();
//             for qubit0 in 0..self.number_qubits {
//                 for qubit1 in 0..self.number_qubits {
//                     if qubit0 != qubit1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit0,
//                             target: qubit1,
//                             time: gate_time,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit1,
//                             target: qubit0,
//                             time: gate_time,
//                         };
//                         times.push(map1);
//                         times.push(map2);
//                     }
//                 }
//             }
//             self.two_qubit_gates.insert(gate.to_string(), times);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the multi-qubit-gates in AllToAllDevice,
//     /// when applied to any qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the multi-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An AllToAllDevice with updated gate times.
//     ///
//     pub fn set_all_multi_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.multi_qubit_gates.get(&gate.to_string()).is_some() {
//             self.multi_qubit_gates.insert(gate.to_string(), gate_time);
//         }
//         self
//     }

//     /// Function to set the decoherence rates for all qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(Self)` -  The device with updated decoherence rates.
//     /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
//     ///
//     pub fn set_all_qubit_decoherence_rates(
//         mut self,
//         rates: Array2<f64>,
//     ) -> Result<Self, RoqoqoError> {
//         // Check if input matrix has the dimension (3x3)
//         let shape = &(*rates.shape());
//         if shape == [3, 3] {
//             for qubit in 0..self.number_qubits() {
//                 self.decoherence_rates.insert(qubit, rates.clone());
//             }
//             Ok(self)
//         } else {
//             Err(RoqoqoError::GenericError {
//                 msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
//             })
//         }
//     }
// }

// /// Implements Device trait for AllToAllDevice.
// ///
// /// The Device trait defines standard functions available for roqoqo devices.
// ///
// impl Device for AllToAllDevice {
//     /// Returns the number of qubits the device supports.
//     ///
//     /// # Returns
//     ///
//     /// The number of qubits in the device.
//     ///
//     fn number_qubits(&self) -> usize {
//         self.number_qubits
//     }

//     /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
//     ///
//     /// The base assumption
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a single qubit gate.
//     /// * `qubit` - The qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
//         match self.single_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x.iter().filter(|item| item.qubit == *qubit);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a two qubit gate.
//     /// * `control` - The control qubit the gate acts on
//     /// * `target` - The target qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
//         match self.two_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x
//                     .iter()
//                     .filter(|item| item.control == *control && item.target == *target);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
//     /// Note: in AllToAllDevice the gate time of multi qubit gates is treated uniformly for all qubits.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a multi qubit gate.
//     /// * `qubits` - The qubits the gate acts on.
//     ///
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
//         // variable unused in AllToAllDevice, is kept here for consistency purposes.
//         let _qubits = qubits;
//         self.multi_qubit_gates.get(&hqslang.to_string()).copied()
//     }

//     /// Returns the matrix of the decoherence rates of the Lindblad equation.
//     ///
//     /// $$
//     /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
//     ///     L_0 = \sigma^{+} \\\\
//     ///     L_1 = \sigma^{-} \\\\
//     ///     L_2 = \sigma^{z}
//     /// $$
//     ///
//     /// # Arguments
//     ///
//     /// * `qubit` - The qubit for which the rate matrix M is returned
//     ///
//     /// # Returns
//     ///
//     /// * `Some<Array2<f64>>` - The decoherence rates.
//     /// * `None` - The qubit is not part of the device.
//     ///
//     fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
//         self.decoherence_rates
//             .get(qubit)
//             .map(|rates| rates.to_owned())
//     }

//     /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     /// A pair of qubits is considered linked by a native two-qubit-gate if the device
//     /// can implement a two-qubit-gate between the two qubits without decomposing it
//     /// into a sequence of gates that involves a third qubit of the device.
//     /// The two-qubit-gate also has to form a universal set together with the available
//     /// single qubit gates.
//     ///
//     /// The returned vectors is a simple, graph-library independent, representation of
//     /// the undirected connectivity graph of the device.
//     /// It can be used to construct the connectivity graph in a graph library of the users
//     /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
//     ///
//     /// # Returns
//     ///
//     /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
//         let mut vector: Vec<(usize, usize)> = Vec::new();
//         for row in 0..self.number_qubits() {
//             for column in row + 1..self.number_qubits() {
//                 vector.push((row, column));
//             }
//         }
//         vector
//     }
// }

// /// A generic 2D Grid Device with only next-neighbours-connectivity.
// ///
// /// To construct the geometry of the GenericGrid device the qubits are numbered
// /// in a row-major order.
// ///
// /// # Example:
// /// Let `m=3` be the number of rows and `n=4` the number of columns.
// /// The number of qubits are numbered as follows:
// /// 0   1   2   3
// /// 4   5   6   7
// /// 8   9   10  11
// /// Resulting in qubit #6 being in the 2nd row in the 3rd column.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct GenericGrid {
//     number_rows: usize,
//     number_columns: usize,
//     single_qubit_gates: HashMap<String, Vec<SingleQubitMap>>,
//     two_qubit_gates: HashMap<String, Vec<TwoQubitMap>>,
//     multi_qubit_gates: HashMap<String, Vec<MultiQubitMap>>,
//     decoherence_rates: HashMap<usize, Array2<f64>>,
// }

// impl GenericGrid {
//     /// Create new GenericGrid.
//     ///
//     /// # Arguments
//     ///
//     /// * `number_rows` - The number of rows in the device.
//     /// * `number_columns` - The number of columns in the device.
//     /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
//     /// * `two_qubit_gates` - A list of 'hqslang' namse of the basic two-qubit-gates supported by the device.
//     /// * `multi_qubit_gates` - Optional. A list of 'hqslang' namse of the basic multi-qubit-gates supported by the device.
//     ///
//     /// # Returns
//     ///
//     /// An initialized GenericGrid with empty gate times and decoherence rates set to zero.
//     ///
//     pub fn new(
//         number_rows: usize,
//         number_columns: usize,
//         single_qubit_gates: &[String],
//         two_qubit_gates: &[String],
//         multi_qubit_gates: &[String],
//     ) -> Self {
//         let number_qubits = number_rows * number_columns;

//         // initialization of single qubit gates with empty times
//         let mut single_qubit_gate_map: HashMap<String, Vec<SingleQubitMap>> = HashMap::new();
//         for gate in single_qubit_gates.iter() {
//             let mut empty_times: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..number_qubits {
//                 let qubittime = SingleQubitMap { qubit, time: 0.0 };
//                 empty_times.push(qubittime);
//             }
//             single_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // initialization of two qubit gates with empty times
//         let mut two_qubit_gate_map: HashMap<String, Vec<TwoQubitMap>> = HashMap::new();
//         for gate in two_qubit_gates.iter() {
//             let mut empty_times: Vec<TwoQubitMap> = Vec::new();
//             for row in 0..(number_rows) {
//                 for column in 0..(number_columns) {
//                     let qubit = row * number_columns + column;
//                     if column < number_columns - 1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit,
//                             target: qubit + 1,
//                             time: 0.0,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit + 1,
//                             target: qubit,
//                             time: 0.0,
//                         };
//                         empty_times.push(map1);
//                         empty_times.push(map2);
//                     }
//                     if row < number_rows - 1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit,
//                             target: qubit + number_columns,
//                             time: 0.0,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit + number_columns,
//                             target: qubit,
//                             time: 0.0,
//                         };
//                         empty_times.push(map1);
//                         empty_times.push(map2);
//                     }
//                 }
//             }
//             two_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // initialization of multi qubit gates with empty times applied to all qubits in the device
//         let mut multi_qubit_gate_map: HashMap<String, Vec<MultiQubitMap>> = HashMap::new();
//         for gate in multi_qubit_gates.iter() {
//             let mut empty_times: Vec<MultiQubitMap> = Vec::new();
//             let mut qubits: Vec<Vec<usize>> = Vec::new();
//             // collect qubits per row
//             for m in (0..number_qubits).step_by(number_columns) {
//                 let vec: Vec<usize> = (m..m + number_columns).collect();
//                 qubits.push(vec);
//             }
//             // collect qubits per column
//             for n in 0..number_columns {
//                 let mut column: Vec<usize> = Vec::new();
//                 for row in 0..number_rows {
//                     column.push(n + row * number_columns);
//                 }
//                 qubits.push(column);
//             }
//             // fill empty times for all collected qubit constellations
//             for item in qubits {
//                 let map = MultiQubitMap {
//                     qubits: item,
//                     time: 0.0,
//                 };
//                 empty_times.push(map);
//             }
//             multi_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
//         for qubit0 in 0..number_qubits {
//             decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
//         }

//         GenericGrid {
//             number_rows,
//             number_columns,
//             single_qubit_gates: single_qubit_gate_map,
//             two_qubit_gates: two_qubit_gate_map,
//             multi_qubit_gates: multi_qubit_gate_map,
//             decoherence_rates,
//         }
//     }

//     /// Function that allows to set one gate time per gate type for the single-qubit-gates.
//     ///
//     /// # Arguments
//     /// * `gate` - hqslang name of the single-qubit-gate.
//     /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// A GenericGrid with updated gate times.
//     ///
//     pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.single_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut gatetimes: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..self.number_qubits() {
//                 let qubittime = SingleQubitMap {
//                     qubit,
//                     time: gate_time,
//                 };
//                 gatetimes.push(qubittime);
//             }
//             self.single_qubit_gates.insert(gate.to_string(), gatetimes);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the two-qubit-gates
//     /// considered as connected in the GenericGrid.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the two-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// A GenericGrid with updated gate times.
//     ///
//     pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.two_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut times: Vec<TwoQubitMap> = Vec::new();
//             for row in 0..(self.number_rows) {
//                 for column in 0..(self.number_columns) {
//                     let qubit = row * self.number_columns + column;
//                     if column < self.number_columns - 1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit,
//                             target: qubit + 1,
//                             time: gate_time,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit + 1,
//                             target: qubit,
//                             time: gate_time,
//                         };
//                         times.push(map1);
//                         times.push(map2);
//                     }
//                     if row < self.number_rows - 1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit,
//                             target: qubit + self.number_columns,
//                             time: gate_time,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit + self.number_columns,
//                             target: qubit,
//                             time: gate_time,
//                         };
//                         times.push(map1);
//                         times.push(map2);
//                     }
//                 }
//             }
//             self.two_qubit_gates.insert(gate.to_string(), times);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the multi-qubit-gates in the GenericGrid.
//     /// In the GenericGrid device the provided `gate_time` is set for the given `gate`
//     /// for all qubits considered to be in one row and all qubits in one column.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the multi-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// A GenericGrid with updated gate times.
//     ///
//     pub fn set_all_multi_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         let number_qubits = self.number_rows * self.number_columns;
//         if self.multi_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut times: Vec<MultiQubitMap> = Vec::new();
//             let mut qubits: Vec<Vec<usize>> = Vec::new();
//             // collect qubits per row
//             for m in (0..number_qubits).step_by(self.number_columns) {
//                 let vec: Vec<usize> = (m..m + self.number_columns).collect();
//                 qubits.push(vec);
//             }
//             // collect qubits per column
//             for n in 0..self.number_columns {
//                 let mut column: Vec<usize> = Vec::new();
//                 for row in 0..self.number_rows {
//                     column.push(n + row * self.number_columns);
//                 }
//                 qubits.push(column);
//             }
//             // fill empty times for all collected qubit constellations
//             for item in qubits {
//                 let map = MultiQubitMap {
//                     qubits: item,
//                     time: gate_time,
//                 };
//                 times.push(map);
//             }
//             self.multi_qubit_gates.insert(gate.to_string(), times);
//         }
//         self
//     }

//     /// Function to set the decoherence rates for all qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(Self)` -  The device with updated decoherence rates.
//     /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
//     ///
//     pub fn set_all_qubit_decoherence_rates(
//         mut self,
//         rates: Array2<f64>,
//     ) -> Result<Self, RoqoqoError> {
//         // Check if input matrix has the dimension (3x3)
//         let shape = &(*rates.shape());
//         if shape == [3, 3] {
//             for qubit in 0..self.number_qubits() {
//                 self.decoherence_rates.insert(qubit, rates.clone());
//             }
//             Ok(self)
//         } else {
//             Err(RoqoqoError::GenericError {
//                 msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
//             })
//         }
//     }

//     /// Returns the number of rows in the device.
//     ///
//     /// # Returns
//     ///
//     /// The number of rows in the device.
//     ///
//     pub fn number_rows(&self) -> usize {
//         self.number_rows
//     }

//     /// Returns the number of columns in the device.
//     ///
//     /// # Returns
//     ///
//     /// The number of columns in the device.
//     ///
//     pub fn number_columns(&self) -> usize {
//         self.number_columns
//     }
// }

// /// Implements Device trait for GenericGrid.
// ///
// /// The Device trait defines standard functions available for roqoqo devices.
// ///
// impl Device for GenericGrid {
//     /// Returns the number of qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// The number of qubits in the device.
//     ///
//     fn number_qubits(&self) -> usize {
//         self.number_rows * self.number_columns
//     }

//     /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a single qubit gate.
//     /// * `qubit` - The qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
//         match self.single_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x.iter().filter(|item| item.qubit == *qubit);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a two qubit operation if the two qubit operation is available on device-.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a two qubit gate.
//     /// * `control` - The control qubit the gate acts on
//     /// * `target` - The target qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
//         match self.two_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x
//                     .iter()
//                     .filter(|item| item.control == *control && item.target == *target);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a multi qubit gate.
//     /// * `qubits` - The qubits the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
//         match self.multi_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x.iter().filter(|item| item.qubits == *qubits);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the matrix of the decoherence rates of the Lindblad equation.
//     ///
//     /// $$
//     /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
//     ///     L_0 = \sigma^{+} \\\\
//     ///     L_1 = \sigma^{-} \\\\
//     ///     L_3 = \sigma^{z}
//     /// $$
//     ///
//     /// # Arguments
//     ///
//     /// * `qubit` - The qubit for which the rate matrix M is returned
//     ///
//     /// # Returns
//     ///
//     /// * `Some<Array2<f64>>` - The decoherence rates.
//     /// * `None` - The qubit is not part of the device.
//     ///
//     fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
//         self.decoherence_rates
//             .get(qubit)
//             .map(|rates| rates.to_owned())
//     }

//     /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     /// A pair of qubits is considered linked by a native two-qubit-gate if the device
//     /// can implement a two-qubit-gate between the two qubits without decomposing it
//     /// into a sequence of gates that involves a third qubit of the device.
//     /// The two-qubit-gate also has to form a universal set together with the available
//     /// single qubit gates.
//     ///
//     /// The returned vectors is a simple, graph-library independent, representation of
//     /// the undirected connectivity graph of the device.
//     /// It can be used to construct the connectivity graph in a graph library of the users
//     /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
//     ///
//     /// # Returns
//     ///
//     /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
//         let mut vector: Vec<(usize, usize)> = Vec::new();
//         for row in 0..(self.number_rows) {
//             for column in 0..(self.number_columns) {
//                 let qubit = row * self.number_columns + column;
//                 if column < self.number_columns - 1 {
//                     vector.push((qubit, qubit + 1));
//                 }
//                 if row < self.number_rows - 1 {
//                     vector.push((qubit, qubit + self.number_columns));
//                 }
//             }
//         }
//         vector
//     }
// }

// /// A generic device containing a linear chain of qubits with next neighbour connectivity.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct GenericChain {
//     number_qubits: usize,
//     single_qubit_gates: HashMap<String, Vec<SingleQubitMap>>,
//     two_qubit_gates: HashMap<String, Vec<TwoQubitMap>>,
//     multi_qubit_gates: HashMap<String, f64>,
//     decoherence_rates: HashMap<usize, Array2<f64>>,
// }

// impl GenericChain {
//     /// Create new GenericChain.
//     ///
//     /// # Arguments
//     ///
//     /// * `number_qubits` - The number of qubits in the device.
//     /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
//     /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
//     // * `multi_qubit_gates` - A list of 'hqslang' names of basic multi-qubit-gates supported by the device.
//     ///
//     /// # Returns
//     ///
//     /// An initiated GenericChain with empty gate times and decoherence rates set to zero.
//     ///
//     pub fn new(
//         number_qubits: usize,
//         single_qubit_gates: &[String],
//         two_qubit_gates: &[String],
//         multi_qubit_gates: &[String],
//     ) -> Self {
//         // Initialization of single qubit gates with empty times
//         let mut single_qubit_gate_map: HashMap<String, Vec<SingleQubitMap>> = HashMap::new();
//         for gate in single_qubit_gates.iter() {
//             let mut empty_times: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..number_qubits {
//                 let qubittime = SingleQubitMap { qubit, time: 0.0 };
//                 empty_times.push(qubittime);
//             }
//             single_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initialization of two qubit gates with empty times
//         let mut two_qubit_gate_map: HashMap<String, Vec<TwoQubitMap>> = HashMap::new();
//         for gate in two_qubit_gates.iter() {
//             let mut empty_times: Vec<TwoQubitMap> = Vec::new();
//             for qubit in 0..number_qubits - 1 {
//                 let qubittime1 = TwoQubitMap {
//                     control: qubit,
//                     target: qubit + 1,
//                     time: 0.0,
//                 };
//                 let qubittime2 = TwoQubitMap {
//                     control: qubit + 1,
//                     target: qubit,
//                     time: 0.0,
//                 };
//                 empty_times.push(qubittime1);
//                 empty_times.push(qubittime2);
//             }
//             two_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initilization of multi qubit gates with empty times when applied to any qubits
//         let mut multi_qubit_gate_map: HashMap<String, f64> = HashMap::new();
//         for gate in multi_qubit_gates.iter() {
//             multi_qubit_gate_map.insert(gate.clone(), 0.0);
//         }

//         let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
//         for qubit0 in 0..number_qubits {
//             decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
//         }

//         GenericChain {
//             number_qubits,
//             single_qubit_gates: single_qubit_gate_map,
//             two_qubit_gates: two_qubit_gate_map,
//             multi_qubit_gates: multi_qubit_gate_map,
//             decoherence_rates,
//         }
//     }

//     /// Function that allows to set one gate time per gate type for the single-qubit-gates.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the single-qubit-gate.
//     /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericChain with updated gate times.
//     ///
//     pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.single_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut gatetimes: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..self.number_qubits() {
//                 let qubittime = SingleQubitMap {
//                     qubit,
//                     time: gate_time,
//                 };
//                 gatetimes.push(qubittime);
//             }
//             self.single_qubit_gates.insert(gate.to_string(), gatetimes);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the two-qubit-gates in GenericChain.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the two-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericChain with updated gate times.
//     ///
//     pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.two_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut times: Vec<TwoQubitMap> = Vec::new();
//             for qubit in 0..self.number_qubits - 1 {
//                 let map1 = TwoQubitMap {
//                     control: qubit,
//                     target: qubit + 1,
//                     time: gate_time,
//                 };
//                 let map2 = TwoQubitMap {
//                     control: qubit + 1,
//                     target: qubit,
//                     time: gate_time,
//                 };
//                 times.push(map1);
//                 times.push(map2);
//             }
//             self.two_qubit_gates.insert(gate.to_string(), times);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the multi-qubit-gates in GenericChain,
//     /// when applied to any qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the multi-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericChain with updated gate times.
//     ///
//     pub fn set_all_multi_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.multi_qubit_gates.get(&gate.to_string()).is_some() {
//             self.multi_qubit_gates.insert(gate.to_string(), gate_time);
//         }
//         self
//     }

//     /// Function to set the decoherence rates for all qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(Self)` -  The device with updated decoherence rates.
//     /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
//     ///
//     pub fn set_all_qubit_decoherence_rates(
//         mut self,
//         rates: Array2<f64>,
//     ) -> Result<Self, RoqoqoError> {
//         // Check if input matrix has the dimension (3x3)
//         let shape = &(*rates.shape());
//         if shape == [3, 3] {
//             for qubit in 0..self.number_qubits() {
//                 self.decoherence_rates.insert(qubit, rates.clone());
//             }
//             Ok(self)
//         } else {
//             Err(RoqoqoError::GenericError {
//                 msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
//             })
//         }
//     }
// }

// /// Implements Device trait for GenericChain.
// ///
// /// The Device trait defines standard functions available for roqoqo devices.
// ///
// impl Device for GenericChain {
//     /// Returns the number of qubits the device supports.
//     ///
//     /// # Returns
//     ///
//     /// The number of qubits in the device.
//     ///
//     fn number_qubits(&self) -> usize {
//         self.number_qubits
//     }

//     /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
//     ///
//     /// The base assumption
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a single qubit gate.
//     /// * `qubit` - The qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
//         match self.single_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x.iter().filter(|item| item.qubit == *qubit);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a two qubit gate.
//     /// * `control` - The control qubit the gate acts on
//     /// * `target` - The target qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
//         match self.two_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x
//                     .iter()
//                     .filter(|item| item.control == *control && item.target == *target);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
//     /// Note: in GenericChain the gate time of multi qubit gates is treated uniformly for all qubits.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a multi qubit gate.
//     /// * `qubits` - The qubits the gate acts on.
//     ///
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
//         // variable unused in GenericChain, is kept here for consistency purposes.
//         let _qubits = qubits;
//         self.multi_qubit_gates.get(&hqslang.to_string()).copied()
//     }

//     /// Returns the matrix of the decoherence rates of the Lindblad equation.
//     ///
//     /// $$
//     /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
//     ///     L_0 = \sigma^{+} \\\\
//     ///     L_1 = \sigma^{-} \\\\
//     ///     L_2 = \sigma^{z}
//     /// $$
//     ///
//     /// # Arguments
//     ///
//     /// * `qubit` - The qubit for which the rate matrix M is returned
//     ///
//     /// # Returns
//     ///
//     /// * `Some<Array2<f64>>` - The decoherence rates.
//     /// * `None` - The qubit is not part of the device.
//     ///
//     fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
//         self.decoherence_rates
//             .get(qubit)
//             .map(|rates| rates.to_owned())
//     }

//     /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     /// A pair of qubits is considered linked by a native two-qubit-gate if the device
//     /// can implement a two-qubit-gate between the two qubits without decomposing it
//     /// into a sequence of gates that involves a third qubit of the device.
//     /// The two-qubit-gate also has to form a universal set together with the available
//     /// single qubit gates.
//     ///
//     /// The returned vectors is a simple, graph-library independent, representation of
//     /// the undirected connectivity graph of the device.
//     /// It can be used to construct the connectivity graph in a graph library of the users
//     /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
//     ///
//     /// # Returns
//     ///
//     /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
//         let mut vector: Vec<(usize, usize)> = Vec::new();
//         for qubit in 0..self.number_qubits() - 1 {
//             vector.push((qubit, qubit + 1));
//         }
//         vector
//     }
// }

// /// A device struct with public fields for a roqoqo device
// /// with all-to-all connectivity between all involved qubits.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct GenericDevice {
//     /// The number of qubits in the device.
//     pub number_qubits: usize,
//     /// A map including 'hqslang' names of single-qubit-gates supported by the device,
//     /// with the according gate times specific for the particular qubit ([SingleQubitMap]).
//     pub single_qubit_gates: HashMap<String, Vec<SingleQubitMap>>,
//     /// A map including 'hqslang' names of two-qubit-gates supported by the device,
//     /// with the according gate times specific for the particular qubit ([TwoQubitMap]).
//     pub two_qubit_gates: HashMap<String, Vec<TwoQubitMap>>,
//     /// A map including 'hqslang' names of multi-qubit-gates supported by the device,
//     /// with the according gate times uniform for all qubits in the device.
//     pub multi_qubit_gates: HashMap<String, f64>,
//     /// A (3x3)-matrix of the decoherence rates.
//     pub decoherence_rates: HashMap<usize, Array2<f64>>,
// }

// impl GenericDevice {
//     /// Create new GenericDevice.
//     ///
//     /// # Arguments
//     ///
//     /// * `number_qubits` - The number of qubits in the device.
//     /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
//     /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
//     /// * `multi_qubit_gates` - A list of 'hqslang' names of basic multi-qubit-gates supported by the device.
//     ///
//     /// # Returns
//     ///
//     /// An initiated GenericDevice with empty gate times and decoherence rates set to zero.
//     ///
//     pub fn new(
//         number_qubits: usize,
//         single_qubit_gates: &[String],
//         two_qubit_gates: &[String],
//         multi_qubit_gates: &[String],
//     ) -> Self {
//         // Initialization of single qubit gates with empty times
//         let mut single_qubit_gate_map: HashMap<String, Vec<SingleQubitMap>> = HashMap::new();
//         for gate in single_qubit_gates.iter() {
//             let mut empty_times: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..number_qubits {
//                 let qubittime = SingleQubitMap { qubit, time: 0.0 };
//                 empty_times.push(qubittime);
//             }
//             single_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initialization of two qubit gates with empty times
//         let mut two_qubit_gate_map: HashMap<String, Vec<TwoQubitMap>> = HashMap::new();
//         for gate in two_qubit_gates.iter() {
//             let mut empty_times: Vec<TwoQubitMap> = Vec::new();
//             for qubit0 in 0..number_qubits {
//                 for qubit1 in 0..number_qubits {
//                     if qubit0 != qubit1 {
//                         let qubittime1 = TwoQubitMap {
//                             control: qubit0,
//                             target: qubit1,
//                             time: 0.0,
//                         };
//                         let qubittime2 = TwoQubitMap {
//                             control: qubit1,
//                             target: qubit0,
//                             time: 0.0,
//                         };
//                         empty_times.push(qubittime1);
//                         empty_times.push(qubittime2);
//                     }
//                 }
//             }
//             two_qubit_gate_map.insert(gate.clone(), empty_times);
//         }

//         // Initilization of multi qubit gates with empty times when applied to any qubits
//         let mut multi_qubit_gate_map: HashMap<String, f64> = HashMap::new();
//         for gate in multi_qubit_gates.iter() {
//             multi_qubit_gate_map.insert(gate.clone(), 0.0);
//         }

//         let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
//         for qubit0 in 0..number_qubits {
//             decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
//         }

//         GenericDevice {
//             number_qubits,
//             single_qubit_gates: single_qubit_gate_map,
//             two_qubit_gates: two_qubit_gate_map,
//             multi_qubit_gates: multi_qubit_gate_map,
//             decoherence_rates,
//         }
//     }

//     /// Function that allows to set one gate time per gate type for the single-qubit-gates.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the single-qubit-gate.
//     /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericDevice with updated gate times.
//     ///
//     pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.single_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut gatetimes: Vec<SingleQubitMap> = Vec::new();
//             for qubit in 0..self.number_qubits() {
//                 let qubittime = SingleQubitMap {
//                     qubit,
//                     time: gate_time,
//                 };
//                 gatetimes.push(qubittime);
//             }
//             self.single_qubit_gates.insert(gate.to_string(), gatetimes);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the two-qubit-gates in GenericDevice.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the two-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericDevice with updated gate times.
//     ///
//     pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.two_qubit_gates.get(&gate.to_string()).is_some() {
//             let mut times: Vec<TwoQubitMap> = Vec::new();
//             for qubit0 in 0..self.number_qubits {
//                 for qubit1 in 0..self.number_qubits {
//                     if qubit0 != qubit1 {
//                         let map1 = TwoQubitMap {
//                             control: qubit0,
//                             target: qubit1,
//                             time: gate_time,
//                         };
//                         let map2 = TwoQubitMap {
//                             control: qubit1,
//                             target: qubit0,
//                             time: gate_time,
//                         };
//                         times.push(map1);
//                         times.push(map2);
//                     }
//                 }
//             }
//             self.two_qubit_gates.insert(gate.to_string(), times);
//         }
//         self
//     }

//     /// Function that allows to set the gate time for the multi-qubit-gates in GenericDevice,
//     /// when applied to any qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `gate` - hqslang name of the multi-qubit-gate.
//     /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
//     ///
//     /// # Returns
//     ///
//     /// An GenericDevice with updated gate times.
//     ///
//     pub fn set_all_multi_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
//         if self.multi_qubit_gates.get(&gate.to_string()).is_some() {
//             self.multi_qubit_gates.insert(gate.to_string(), gate_time);
//         }
//         self
//     }

//     /// Function to set the decoherence rates for all qubits in the device.
//     ///
//     /// # Arguments
//     ///
//     /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(Self)` -  The device with updated decoherence rates.
//     /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
//     ///
//     pub fn set_all_qubit_decoherence_rates(
//         mut self,
//         rates: Array2<f64>,
//     ) -> Result<Self, RoqoqoError> {
//         // Check if input matrix has the dimension (3x3)
//         let shape = &(*rates.shape());
//         if shape == [3, 3] {
//             for qubit in 0..self.number_qubits() {
//                 self.decoherence_rates.insert(qubit, rates.clone());
//             }
//             Ok(self)
//         } else {
//             Err(RoqoqoError::GenericError {
//                 msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
//             })
//         }
//     }
// }

// /// Implements Device trait for GenericDevice.
// ///
// /// The Device trait defines standard functions available for roqoqo devices.
// ///
// impl Device for GenericDevice {
//     /// Returns the number of qubits the device supports.
//     ///
//     /// # Returns
//     ///
//     /// The number of qubits in the device.
//     ///
//     fn number_qubits(&self) -> usize {
//         self.number_qubits
//     }

//     /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
//     ///
//     /// The base assumption
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a single qubit gate.
//     /// * `qubit` - The qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
//         match self.single_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x.iter().filter(|item| item.qubit == *qubit);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a two qubit gate.
//     /// * `control` - The control qubit the gate acts on
//     /// * `target` - The target qubit the gate acts on
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
//         match self.two_qubit_gates.get(&hqslang.to_string()) {
//             Some(x) => {
//                 let mut item = x
//                     .iter()
//                     .filter(|item| item.control == *control && item.target == *target);
//                 item.next().map(|y| y.time)
//             }
//             None => None,
//         }
//     }

//     /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
//     /// Note: in GenericDevice the gate time of multi qubit gates is treated uniformly for all qubits.
//     ///
//     ///
//     /// # Arguments
//     ///
//     /// * `hqslang` - The hqslang name of a multi qubit gate.
//     /// * `qubits` - The qubits the gate acts on.
//     ///
//     ///
//     /// # Returns
//     ///
//     /// * `Some<f64>` - The gate time.
//     /// * `None` - The gate is not available on the device.
//     ///
//     fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
//         // variable unused in GenericDevice, is kept here for consistency purposes.
//         let _qubits = qubits;
//         self.multi_qubit_gates.get(&hqslang.to_string()).copied()
//     }

//     /// Returns the matrix of the decoherence rates of the Lindblad equation.
//     ///
//     /// $$
//     /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
//     ///     L_0 = \sigma^{+} \\\\
//     ///     L_1 = \sigma^{-} \\\\
//     ///     L_2 = \sigma^{z}
//     /// $$
//     ///
//     /// # Arguments
//     ///
//     /// * `qubit` - The qubit for which the rate matrix M is returned
//     ///
//     /// # Returns
//     ///
//     /// * `Some<Array2<f64>>` - The decoherence rates.
//     /// * `None` - The qubit is not part of the device.
//     ///
//     fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
//         self.decoherence_rates
//             .get(qubit)
//             .map(|rates| rates.to_owned())
//     }

//     /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     /// A pair of qubits is considered linked by a native two-qubit-gate if the device
//     /// can implement a two-qubit-gate between the two qubits without decomposing it
//     /// into a sequence of gates that involves a third qubit of the device.
//     /// The two-qubit-gate also has to form a universal set together with the available
//     /// single qubit gates.
//     ///
//     /// The returned vectors is a simple, graph-library independent, representation of
//     /// the undirected connectivity graph of the device.
//     /// It can be used to construct the connectivity graph in a graph library of the users
//     /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
//     ///
//     /// # Returns
//     ///
//     /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
//     ///
//     fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
//         let mut vector: Vec<(usize, usize)> = Vec::new();
//         for row in 0..self.number_qubits() {
//             for column in row + 1..self.number_qubits() {
//                 vector.push((row, column));
//             }
//         }
//         vector
//     }
// }

// /// A customized struct to use as a value in the HashMap for single_qubit_gates
// /// to access the gate times.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct SingleQubitMap {
//     ///The index of the qubit for which the gate time is set.
//     pub qubit: usize,
//     /// Gate time for the given qubit for the single qubit gate.
//     pub time: f64,
// }

// /// A customized struct to use as a value in the HashMap for two_qubit_gates
// /// to access the gate times.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct TwoQubitMap {
//     /// The index of the control qubit for which the gate time is set.
//     pub control: usize,
//     /// The index of the target qubit for which the gate time is set.
//     pub target: usize,
//     /// Gate time for the given qubits for the two qubit gate.
//     pub time: f64,
// }

// #[doc(hidden)]
// /// A customized struct to use as a value in the HashMap for multi_qubit_gates
// /// to access the gate times.
// ///
// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// pub struct MultiQubitMap {
//     /// A list of qubit indices for which the gate time is set.
//     pub qubits: Vec<usize>,
//     /// Gate time for the given qubits for the multi qubit gate.
//     pub time: f64,
// }
