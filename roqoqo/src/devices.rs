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
    /// In case the device is not allowed to be change a generic RoqoqoBackendError is returned.
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
    two_qubit_gate: HashMap<String, HashMap<(usize, usize), f64>>,
    decoherence_rates: HashMap<usize, Array2<f64>>,
}

impl AllToAllDevice {
    /// Create new AllToAllDevice.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gate` - The 'hqslang' name of the basic two-qubit-gate supported by the device.
    ///
    /// # Returns
    ///
    /// An initiated AllToAllDevice with empty gate times and decoherence rates set to zero.
    ///
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: &[String],
        two_qubit_gate: String,
    ) -> Self {
        let mut single_qubit_gate_map: HashMap<String, HashMap<usize, f64>> = HashMap::new();
        for gate in single_qubit_gates.iter() {
            let mut empty_times: HashMap<usize, f64> = HashMap::new();
            for qubit in 0..number_qubits {
                empty_times.insert(qubit, 0.0);
            }
            single_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        let mut two_qubit_gate_map: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
        let mut empty_times: HashMap<(usize, usize), f64> = HashMap::new();
        for qubit0 in 0..number_qubits {
            for qubit1 in 0..number_qubits {
                if qubit0 != qubit1 {
                    empty_times.insert((qubit0, qubit1), 0.0);
                }
            }
        }
        two_qubit_gate_map.insert(two_qubit_gate, empty_times);

        let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
        for qubit0 in 0..number_qubits {
            decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
        }

        AllToAllDevice {
            number_qubits,
            single_qubit_gates: single_qubit_gate_map,
            two_qubit_gate: two_qubit_gate_map,
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

    /// Function that allows to set the gate time  for the two-qubit-gate.
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
        if self.two_qubit_gate.get(&gate.to_string()).is_some() {
            let mut times: HashMap<(usize, usize), f64> = HashMap::new();
            for qubit0 in 0..self.number_qubits {
                for qubit1 in 0..self.number_qubits {
                    if qubit0 != qubit1 {
                        times.insert((qubit0, qubit1), gate_time);
                    }
                }
            }
            self.two_qubit_gate.insert(gate.to_string(), times);
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
        match self.two_qubit_gate.get(&hqslang.to_string()) {
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
    fn multi_qubit_gate_time(&self, _hqslang: &str, _qubits: &[usize]) -> Option<f64> {
        None
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
#[derive(Debug, Clone, PartialEq)]
pub struct GenericGrid {
    number_rows: usize,
    number_columns: usize,
    single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    two_qubit_gate: HashMap<String, HashMap<(usize, usize), f64>>,
    decoherence_rates: HashMap<usize, Array2<f64>>,
    number_qubits: usize,
}

impl GenericGrid {
    /// Create new GenericGrid.
    ///
    /// # Arguments
    ///
    /// * `number_rows` - The number of rows in the device.
    /// * `number_columns` - The number of columns in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gate` - The 'hqslang' name of the basic two-qubit-gate supported by the device.
    ///
    /// # Returns
    ///
    /// An initiated GenericGrid with empty gate times and decoherence rates set to zero.
    ///
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        single_qubit_gates: &[String],
        two_qubit_gate: String,
    ) -> Self {
        let number_qubits = number_rows * number_columns;
        let mut single_qubit_gate_map: HashMap<String, HashMap<usize, f64>> = HashMap::new();
        for gate in single_qubit_gates.iter() {
            let mut empty_times: HashMap<usize, f64> = HashMap::new();
            for qubit in 0..number_qubits {
                empty_times.insert(qubit, 0.0);
            }
            single_qubit_gate_map.insert(gate.clone(), empty_times);
        }

        let mut two_qubit_gate_map: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
        let mut empty_times: HashMap<(usize, usize), f64> = HashMap::new();
        for qubit0 in 0..number_qubits {
            for qubit1 in 0..number_qubits {
                if qubit0 != qubit1 {
                    empty_times.insert((qubit0, qubit1), 0.0);
                }
            }
        }
        two_qubit_gate_map.insert(two_qubit_gate, empty_times);

        let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
        for qubit0 in 0..number_qubits {
            decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
        }

        GenericGrid {
            number_rows,
            number_columns,
            single_qubit_gates: single_qubit_gate_map,
            two_qubit_gate: two_qubit_gate_map,
            decoherence_rates,
            number_qubits,
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
    /// An GenericGrid with updated gate times.
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
    /// An GenericGrid with updated decoherence rates.
    ///
    pub fn set_all_qubit_decoherence_rates(mut self, rates: Array2<f64>) -> Self {
        for qubit in 0..self.number_qubits {
            self.decoherence_rates.insert(qubit, rates.clone());
        }
        self
    }

    /// Function that allows to set the gate time  for the two-qubit-gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An GenericGrid with updated gate times.
    ///
    pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.two_qubit_gate.get(&gate.to_string()).is_some() {
            let mut times: HashMap<(usize, usize), f64> = HashMap::new();
            for qubit0 in 0..self.number_qubits {
                for qubit1 in 0..self.number_qubits {
                    if qubit0 != qubit1 {
                        times.insert((qubit0, qubit1), gate_time);
                    }
                }
            }
            self.two_qubit_gate.insert(gate.to_string(), times);
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
        match self.two_qubit_gate.get(&hqslang.to_string()) {
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
    fn multi_qubit_gate_time(&self, _hqslang: &str, _qubits: &[usize]) -> Option<f64> {
        None
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
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                vector.push((row, column));
            }
        }
        vector
    }
}
