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
use std::collections::HashMap;
use serde::ser::{Serializer, SerializeStruct};
// use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

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

/// A device assuming all-to-all connectivity between all involved qubits.
///
#[derive(Debug, Clone, PartialEq)]
pub struct AllToAllDevice {
    number_qubits: usize,
    single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
    multi_qubit_gates: HashMap<String, HashMap<Vec<usize>, f64>>,
    decoherence_rates: HashMap<usize, Array2<f64>>,
}

impl AllToAllDevice {
    /// Create new AllToAllDevice.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    // * `multi_qubit_gates` - A list of 'hqslang' names of basic multi-qubit-gates supported by the device.
    ///
    /// # Returns
    ///
    /// An initiated AllToAllDevice with empty gate times and decoherence rates set to zero.
    ///
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: &[String],
        two_qubit_gates: &[String],
        multi_qubit_gates: &[String],
    ) -> Self {
        // Initialization of single qubit gates with empty times
        let mut single_qubit_gate_map: HashMap<String, HashMap<usize, f64>> = HashMap::new();
        for gate in single_qubit_gates.iter() {
            let mut empty_times: HashMap<usize, f64> = HashMap::new();
            for qubit in 0..number_qubits {
                empty_times.insert(qubit, 0.0);
            }
            single_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        // Initialization of two qubit gates with empty times
        let mut two_qubit_gate_map: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
        for gate in two_qubit_gates.iter() {
            let mut empty_times: HashMap<(usize, usize), f64> = HashMap::new();
            for qubit0 in 0..number_qubits {
                for qubit1 in 0..number_qubits {
                    if qubit0 != qubit1 {
                        empty_times.insert((qubit0, qubit1), 0.0);
                    }
                }
            }
            two_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        // Initilization of multi qubit gates with empty times when applied to all qubits
        let mut multi_qubit_gate_map: HashMap<String, HashMap<Vec<usize>, f64>> = HashMap::new();
        for gate in multi_qubit_gates.iter() {
            let mut empty_times: HashMap<Vec<usize>, f64> = HashMap::new();
            let qubits: Vec<usize> = (0..number_qubits).collect();
            empty_times.insert(qubits, 0.0);
            multi_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
        for qubit0 in 0..number_qubits {
            decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
        }

        AllToAllDevice {
            number_qubits,
            single_qubit_gates: single_qubit_gate_map,
            two_qubit_gates: two_qubit_gate_map,
            multi_qubit_gates: multi_qubit_gate_map,
            decoherence_rates,
        }
    }

    /// Function that allows to set one gate time per gate type for the single-qubit-gates.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times.
    ///
    pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.single_qubit_gates.get(&gate.to_string()).is_some() {
            let mut times: HashMap<usize, f64> = HashMap::new();
            for qubit in 0..self.number_qubits {
                times.insert(qubit, gate_time);
            }
            self.single_qubit_gates.insert(gate.to_string(), times);
        }
        self
    }

    /// Function to set the decoherence rates for all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `rates` - decoherence rates for the qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated decoherence rates.
    ///
    pub fn set_all_qubit_decoherence_rates(mut self, rates: Array2<f64>) -> Self {
        for qubit in 0..self.number_qubits {
            self.decoherence_rates.insert(qubit, rates.clone());
        }
        self
    }

    /// Function that allows to set the gate time for the two-qubit-gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times.
    ///
    pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.two_qubit_gates.get(&gate.to_string()).is_some() {
            let mut times: HashMap<(usize, usize), f64> = HashMap::new();
            for qubit0 in 0..self.number_qubits {
                for qubit1 in 0..self.number_qubits {
                    if qubit0 != qubit1 {
                        times.insert((qubit0, qubit1), gate_time);
                    }
                }
            }
            self.two_qubit_gates.insert(gate.to_string(), times);
        }
        self
    }
}

/// Implements Device trait for AllToAllDevice.
///
/// The Device trait defines standard functions available for roqoqo devices.
///
impl Device for AllToAllDevice {
    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        self.number_qubits
    }

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
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self.single_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(qubit).copied(),
            None => None,
        }
    }

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
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self.two_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*control, *target)).copied(),
            None => None,
        }
    }

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
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self.multi_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*qubits)).copied(),
            None => None,
        }
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// $$
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_2 = \sigma^{z}
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
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.decoherence_rates
            .get(qubit)
            .map(|rates| rates.to_owned())
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
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                vector.push((row, column));
            }
        }
        vector
    }
}

/// A generic 2D Grid Device with only next-neighbours-connectivity.
///
/// To construct the geometry of the GenericGrid device the qubits are numbered
/// in a row-major order.
///
/// # Example:
/// Let `m=3` be the number of rows and `n=4` the number of columns.
/// The number of qubits are numbered as follows:
/// 0   1   2   3
/// 4   5   6   7
/// 8   9   10  11
/// Resulting in qubit #6 being in the 2nd row in the 3rd column.
///
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericGrid {
    number_rows: usize,
    number_columns: usize,
    single_qubit_gates: HashMap<SingleQubitKey, f64>,
    two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
    multi_qubit_gates: HashMap<String, HashMap<Vec<usize>, f64>>,
    decoherence_rates: HashMap<usize, Array2<f64>>,
}

impl GenericGrid {
    /// Create new GenericGrid.
    ///
    /// # Arguments
    ///
    /// * `number_rows` - The number of rows in the device.
    /// * `number_columns` - The number of columns in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gates` - A list of 'hqslang' namse of the basic two-qubit-gates supported by the device.
    /// * `multi_qubit_gates` - Optional. A list of 'hqslang' namse of the basic multi-qubit-gates supported by the device.
    ///
    /// # Returns
    ///
    /// An initiated GenericGrid with empty gate times and decoherence rates set to zero.
    ///
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        single_qubit_gates: &[String],
        two_qubit_gates: &[String],
        multi_qubit_gates: &[String],
    ) -> Self {
        let number_qubits = number_rows * number_columns;

        // initialization of single qubit gates with empty times
        let mut single_qubit_gate_map: HashMap<SingleQubitKey, f64> = HashMap::new();
        for gate in single_qubit_gates.iter() {
            for qubit in 0..number_qubits {
                let hashmapkey = SingleQubitKey {gate: gate.clone(), qubit: qubit};
                single_qubit_gate_map.insert(hashmapkey, 0.0);
            }
        }

        // initialization of two qubit gates with empty times
        let mut two_qubit_gate_map: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
        for gate in two_qubit_gates.iter() {
            let mut empty_times: HashMap<(usize, usize), f64> = HashMap::new();
            for row in 0..(number_rows) {
                for column in 0..(number_columns) {
                    let qubit = row * number_columns + column;
                    if column < number_columns - 1 {
                        empty_times.insert((qubit, qubit + 1), 0.0);
                        empty_times.insert((qubit + 1, qubit), 0.0);
                    }
                    if row < number_rows - 1 {
                        empty_times.insert((qubit, qubit + number_columns), 0.0);
                        empty_times.insert((qubit + number_columns, qubit), 0.0);
                    }
                }
            }
            two_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        // initialization of multi qubit gates with empty times applied to all qubits in the device
        let mut multi_qubit_gate_map: HashMap<String, HashMap<Vec<usize>, f64>> = HashMap::new();
        for gate in multi_qubit_gates.iter() {
            let mut empty_times: HashMap<Vec<usize>, f64> = HashMap::new();
            let qubits: Vec<usize> = (0..number_qubits).collect();
            empty_times.insert(qubits, 0.0);
            multi_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
        for qubit0 in 0..number_qubits {
            decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
        }

        GenericGrid {
            number_rows,
            number_columns,
            single_qubit_gates: single_qubit_gate_map,
            two_qubit_gates: two_qubit_gate_map,
            multi_qubit_gates: multi_qubit_gate_map,
            decoherence_rates,
        }
    }

    /// Function that allows to set one gate time per gate type for the single-qubit-gates.
    ///
    /// # Arguments
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// A GenericGrid with updated gate times.
    ///
    pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        let keytocheck = SingleQubitKey {gate: gate.clone().to_string(), qubit: 0};
        if self.single_qubit_gates.get(&keytocheck).is_some() {
            for qubit in 0..self.number_qubits() {
                let hashmapkey = SingleQubitKey {gate: gate.clone().to_string(), qubit: qubit};
                self.single_qubit_gates.insert(hashmapkey, gate_time);
            }
        }
        self
    }

    /// Function that allows to set the gate time for the two-qubit-gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// A GenericGrid with updated gate times.
    ///
    pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.two_qubit_gates.get(&gate.to_string()).is_some() {
            let mut times: HashMap<(usize, usize), f64> = HashMap::new();
            for row in 0..(self.number_rows) {
                for column in 0..(self.number_columns) {
                    let qubit = row * self.number_columns + column;
                    if column < self.number_columns - 1 {
                        times.insert((qubit, qubit + 1), gate_time);
                        times.insert((qubit + 1, qubit), gate_time);
                    }
                    if row < self.number_rows - 1 {
                        times.insert((qubit, qubit + self.number_columns), gate_time);
                        times.insert((qubit + self.number_columns, qubit), gate_time);
                    }
                }
            }
            self.two_qubit_gates.insert(gate.to_string(), times);
        }
        self
    }

    /// Function that allows to set the gate time for the multi-qubit-gate,
    /// when applied to all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the multi-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// A GenericGrid with updated gate times.
    ///
    pub fn set_all_multi_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.multi_qubit_gates.get(&gate.to_string()).is_some() {
            let mut times: HashMap<Vec<usize>, f64> = HashMap::new();
            let qubits: Vec<usize> = (0..self.number_qubits()).collect();
            times.insert(qubits, gate_time);
            self.multi_qubit_gates.insert(gate.to_string(), times);
        }
        self
    }

    /// Function to set the decoherence rates for all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `rates` - decoherence rates for the qubits in the device.
    ///
    /// # Returns
    ///
    /// A GenericGrid with updated decoherence rates.
    ///
    pub fn set_all_qubit_decoherence_rates(mut self, rates: Array2<f64>) -> Self {
        for qubit in 0..self.number_qubits() {
            self.decoherence_rates.insert(qubit, rates.clone());
        }
        self
    }

    /// Returns the number of rows in the device.
    ///
    /// # Returns
    ///
    /// The number of rows in the device.
    ///
    pub fn number_rows(&self) -> usize {
        self.number_rows
    }

    /// Returns the number of columns in the device.
    ///
    /// # Returns
    ///
    /// The number of columns in the device.
    ///
    pub fn number_columns(&self) -> usize {
        self.number_columns
    }
}

/// Implements Device trait for GenericGrid.
///
/// The Device trait defines standard functions available for roqoqo devices.
///
impl Device for GenericGrid {
    /// Returns the number of qubits in the device.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        self.number_rows * self.number_columns
    }

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
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        let key = SingleQubitKey {gate: hqslang.to_string(), qubit: qubit.clone()};
        match self.single_qubit_gates.get(&key) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

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
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self.two_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*control, *target)).copied(),
            None => None,
        }
    }

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
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self.multi_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*qubits)).copied(),
            None => None,
        }
    }

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
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.decoherence_rates
            .get(qubit)
            .map(|rates| rates.to_owned())
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
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();
        for row in 0..(self.number_rows) {
            for column in 0..(self.number_columns) {
                let qubit = row * self.number_columns + column;
                if column < self.number_columns - 1 {
                    vector.push((qubit, qubit + 1));
                }
                if row < self.number_rows - 1 {
                    vector.push((qubit, qubit + self.number_columns));
                }
            }
        }
        vector
    }
}

// A customized struct to use as a key in the HashMap for single_qubit_gates
// to access the gate times
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Deserialize)]
struct SingleQubitKey {
    gate: String,
    qubit: usize,
}

impl serde::Serialize for SingleQubitKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SingleQubitKey", 2)?;
        state.serialize_field("gate", &self.gate)?;
        state.serialize_field("qubit", &self.qubit)?;
        state.end()
    }
}

// This implementation builds. To be tested, if standard implementation is enough. 
// CODE parked here for the moment.
//
// impl<'de> Deserialize<'de> for SingleQubitKey {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         enum Field { Gate, Qubit }

//         // This part could also be generated independently by:
//         //
//         //    #[derive(Deserialize)]
//         //    #[serde(field_identifier, rename_all = "lowercase")]
//         //    enum Field { Gate, Qubit }
//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;

//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;

//                     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                         formatter.write_str("`gate` or `qubit`")
//                     }

//                     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//                     where
//                         E: de::Error,
//                     {
//                         match value {
//                             "gate" => Ok(Field::Gate),
//                             "qubit" => Ok(Field::Qubit),
//                             _ => Err(de::Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }

//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }


//         struct SingleQubitKeyVisitor;

//         impl<'de> Visitor<'de> for SingleQubitKeyVisitor {
//             type Value = SingleQubitKey;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct SingleQubitKey")
//             }

//             fn visit_seq<V>(self, mut seq: V) -> Result<SingleQubitKey, V::Error>
//             where
//                 V: SeqAccess<'de>,
//             {
//                 let gate = seq.next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(0, &self))?;
//                 let qubit = seq.next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(1, &self))?;
//                 Ok(SingleQubitKey{ gate: gate, qubit: qubit })
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<SingleQubitKey, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut gate = None;
//                 let mut qubit = None;
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         Field::Gate => {
//                             if gate.is_some() {
//                                 return Err(de::Error::duplicate_field("gate"));
//                             }
//                             gate = Some(map.next_value()?);
//                         }
//                         Field::Qubit => {
//                             if qubit.is_some() {
//                                 return Err(de::Error::duplicate_field("qubit"));
//                             }
//                             qubit = Some(map.next_value()?);
//                         }
//                     }
//                 }
//                 let gate = gate.ok_or_else(|| de::Error::missing_field("gate"))?;
//                 let qubit = qubit.ok_or_else(|| de::Error::missing_field("qubit"))?;
//                 Ok(SingleQubitKey{ gate: gate, qubit: qubit})
//             }
//         }

//         const FIELDS: &'static [&'static str] = &["gate", "qubit"];
//         deserializer.deserialize_struct("SingleQubitKey", FIELDS, SingleQubitKeyVisitor)
//     }
// }
