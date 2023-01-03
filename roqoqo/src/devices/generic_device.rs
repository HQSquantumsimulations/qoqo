// Copyright © 2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use std::collections::HashMap;

use super::Device;
use crate::RoqoqoError;
use crate::RoqoqoVersion;
use ndarray::{array, Array2};
/// A generic device assuming all-to-all connectivity between all involved qubits.
///
/// # Note
///
/// GenericDevice uses nested HashMaps to represent the most general device connectivity.
/// The memory usage will be inefficient for devices with large qubit numbers.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(from = "GenericDeviceSerialize"))]
#[cfg_attr(feature = "serialize", serde(into = "GenericDeviceSerialize"))]
pub struct GenericDevice {
    /// The number of qubits
    pub number_qubits: usize,
    /// Gate times for all single qubit gates
    pub single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    /// Gate times for all two qubit gates
    pub two_qubit_gates: HashMap<String, TwoQubitGates>,
    /// Gate times for all multi qubit gates
    pub multi_qubit_gates: HashMap<String, HashMap<Vec<usize>, f64>>,
    /// Decoherence rates for all qubits
    pub decoherence_rates: HashMap<usize, Array2<f64>>,
}

type TwoQubitGates = HashMap<(usize, usize), f64>;
type TwoQubitGatesVec = Vec<((usize, usize), f64)>;

#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]

struct GenericDeviceSerialize {
    number_qubits: usize,
    /// Gate times for all single qubit gates
    single_qubit_gates: HashMap<String, Vec<(usize, f64)>>,
    /// Gate times for all two qubit gates
    two_qubit_gates: HashMap<String, TwoQubitGatesVec>,
    /// Gate times for all multi qubit gates
    multi_qubit_gates: HashMap<String, Vec<(Vec<usize>, f64)>>,
    /// Decoherence rates for all qubits
    decoherence_rates: Vec<(usize, Array2<f64>)>,
    _roqoqo_version: RoqoqoVersion,
}

impl From<GenericDeviceSerialize> for GenericDevice {
    fn from(value: GenericDeviceSerialize) -> Self {
        let mut two_qubit_gates: HashMap<String, TwoQubitGates> =
            HashMap::with_capacity(value.two_qubit_gates.len());

        let mut single_qubit_gates: HashMap<String, HashMap<usize, f64>> =
            HashMap::with_capacity(value.two_qubit_gates.len());
        let mut multi_qubit_gates: HashMap<String, HashMap<Vec<usize>, f64>> =
            HashMap::with_capacity(value.two_qubit_gates.len());

        let decoherence_rates: HashMap<usize, Array2<f64>> =
            value.decoherence_rates.into_iter().collect();

        for (name, map) in value.two_qubit_gates.into_iter() {
            let new_map: HashMap<(usize, usize), f64> = map.into_iter().collect();
            two_qubit_gates.insert(name, new_map);
        }
        for (name, map) in value.single_qubit_gates.into_iter() {
            let new_map: HashMap<usize, f64> = map.into_iter().collect();
            single_qubit_gates.insert(name, new_map);
        }
        for (name, map) in value.multi_qubit_gates.into_iter() {
            let new_map: HashMap<Vec<usize>, f64> = map.into_iter().collect();
            multi_qubit_gates.insert(name, new_map);
        }

        let new_device: GenericDevice = GenericDevice {
            number_qubits: value.number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
            decoherence_rates,
        };
        new_device
    }
}

impl From<GenericDevice> for GenericDeviceSerialize {
    fn from(value: GenericDevice) -> Self {
        let mut two_qubit_gates: HashMap<String, TwoQubitGatesVec> =
            HashMap::with_capacity(value.two_qubit_gates.len());

        let mut single_qubit_gates: HashMap<String, Vec<(usize, f64)>> =
            HashMap::with_capacity(value.two_qubit_gates.len());
        let mut multi_qubit_gates: HashMap<String, Vec<(Vec<usize>, f64)>> =
            HashMap::with_capacity(value.two_qubit_gates.len());

        let decoherence_rates: Vec<(usize, Array2<f64>)> =
            value.decoherence_rates.into_iter().collect();

        for (name, map) in value.two_qubit_gates.into_iter() {
            let new_map: TwoQubitGatesVec = map.into_iter().collect();
            two_qubit_gates.insert(name, new_map);
        }
        for (name, map) in value.single_qubit_gates.into_iter() {
            let new_map: Vec<(usize, f64)> = map.into_iter().collect();
            single_qubit_gates.insert(name, new_map);
        }
        for (name, map) in value.multi_qubit_gates.into_iter() {
            let new_map: Vec<(Vec<usize>, f64)> = map.into_iter().collect();
            multi_qubit_gates.insert(name, new_map);
        }

        let new_device: GenericDeviceSerialize = GenericDeviceSerialize {
            number_qubits: value.number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
            decoherence_rates,
            _roqoqo_version: RoqoqoVersion,
        };
        new_device
    }
}

impl GenericDevice {
    /// Creates a new GenericDevice.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the device.
    ///
    pub fn new(number_qubits: usize) -> Self {
        // Initialization of single qubit gates with empty times
        Self {
            number_qubits,
            single_qubit_gates: HashMap::new(),
            two_qubit_gates: HashMap::new(),
            multi_qubit_gates: HashMap::new(),
            decoherence_rates: HashMap::new(),
        }
    }

    /// Setting the gate time of a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `qubit` - The qubit for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    pub fn set_single_qubit_gate_time(
        &mut self,
        gate: &str,
        qubit: usize,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        if qubit >= self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} larger than number qubits {}",
                    qubit, self.number_qubits
                ),
            });
        }
        match self.single_qubit_gates.get_mut(gate) {
            Some(gate_times) => {
                let gatetime = gate_times.entry(qubit).or_insert(gate_time);
                *gatetime = gate_time;
            }
            None => {
                let mut new_map = HashMap::new();
                new_map.insert(qubit, gate_time);
                self.single_qubit_gates.insert(gate.to_string(), new_map);
            }
        }
        Ok(())
    }

    /// Setting the gate time of a two qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `control` - The control qubit for which the gate time is set
    /// * `target` - The target qubit for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    pub fn set_two_qubit_gate_time(
        &mut self,
        gate: &str,
        control: usize,
        target: usize,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        if control >= self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} larger than number qubits {}",
                    control, self.number_qubits
                ),
            });
        }
        if target >= self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} larger than number qubits {}",
                    target, self.number_qubits
                ),
            });
        }

        match self.two_qubit_gates.get_mut(gate) {
            Some(gate_times) => {
                let gatetime = gate_times.entry((control, target)).or_insert(gate_time);
                *gatetime = gate_time;
            }
            None => {
                let mut new_map = HashMap::new();
                new_map.insert((control, target), gate_time);
                self.two_qubit_gates.insert(gate.to_string(), new_map);
            }
        }
        Ok(())
    }

    /// Setting the gate time of a mulit qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the multi-qubit-gate.
    /// * `qubits` - The qubits for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    pub fn set_multi_qubit_gate_time(
        &mut self,
        gate: &str,
        qubits: Vec<usize>,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        for qubit in qubits.iter() {
            if qubit >= &self.number_qubits {
                return Err(RoqoqoError::GenericError {
                    msg: format!(
                        "Qubit {} larger than number qubits {}",
                        qubit, self.number_qubits
                    ),
                });
            }
        }
        match self.multi_qubit_gates.get_mut(gate) {
            Some(gate_times) => {
                let gatetime = gate_times.entry(qubits).or_insert(gate_time);
                *gatetime = gate_time;
            }
            None => {
                let mut new_map = HashMap::new();
                new_map.insert(qubits, gate_time);
                self.multi_qubit_gates.insert(gate.to_string(), new_map);
            }
        }
        Ok(())
    }

    /// Function to set the decoherence rates for all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the decoherence rates are set
    /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
    pub fn set_qubit_decoherence_rates(
        &mut self,
        qubit: usize,
        rates: Array2<f64>,
    ) -> Result<(), RoqoqoError> {
        // Check if input matrix has the dimension (3x3)
        let shape = rates.shape();
        if shape == [3, 3] {
            if qubit > self.number_qubits {
                return Err(RoqoqoError::GenericError {
                    msg: format!(
                        "Qubit {} out of range for device of size {}",
                        qubit, self.number_qubits
                    ),
                });
            }
            let aa = self
                .decoherence_rates
                .entry(qubit)
                .or_insert_with(|| Array2::zeros((3, 3)));
            *aa = rates;
            Ok(())
        } else {
            Err(RoqoqoError::GenericError {
                msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
            })
        }
    }

    /// Adds qubit damping to noise rates.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the dampins is added
    /// * `daming` - The damping rates.
    pub fn add_damping(&mut self, qubit: usize, damping: f64) -> Result<(), RoqoqoError> {
        if qubit > self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} out of range for device of size {}",
                    qubit, self.number_qubits
                ),
            });
        }
        let aa = self
            .decoherence_rates
            .entry(qubit)
            .or_insert_with(|| Array2::zeros((3, 3)));
        *aa = aa.clone() + array![[damping, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        Ok(())
    }

    /// Adds qubit dephasing to noise rates.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the dephasing is added
    /// * `dephasing` - The dephasing rates.
    pub fn add_dephasing(&mut self, qubit: usize, dephasing: f64) -> Result<(), RoqoqoError> {
        if qubit > self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} out of range for device of size {}",
                    qubit, self.number_qubits
                ),
            });
        }
        let aa = self
            .decoherence_rates
            .entry(qubit)
            .or_insert_with(|| Array2::zeros((3, 3)));
        *aa = aa.clone() + array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, dephasing]];
        Ok(())
    }

    /// Adds qubit depolarising to noise rates.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the depolarising noise is added
    /// * `depolarising` - The deporalising rate.
    pub fn add_depolarising(&mut self, qubit: usize, depolarising: f64) -> Result<(), RoqoqoError> {
        if qubit > self.number_qubits {
            return Err(RoqoqoError::GenericError {
                msg: format!(
                    "Qubit {} out of range for device of size {}",
                    qubit, self.number_qubits
                ),
            });
        }
        let aa = self
            .decoherence_rates
            .entry(qubit)
            .or_insert_with(|| Array2::zeros((3, 3)));
        *aa = aa.clone()
            + array![
                [depolarising / 2.0, 0.0, 0.0],
                [0.0, depolarising / 2.0, 0.0],
                [0.0, 0.0, depolarising / 4.0]
            ];
        Ok(())
    }
}

/// Implements Device trait for AllToAllDevice.
///
/// The Device trait defines standard functions available for roqoqo devices.
///
impl Device for GenericDevice {
    fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self.single_qubit_gates.get(hqslang) {
            Some(x) => x.get(qubit).copied(),
            None => None,
        }
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self.two_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*control, *target)).copied(),
            None => None,
        }
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        // variable unused in AllToAllDevice, is kept here for consistency purposes.

        match self.multi_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => {
                let qubits: Vec<usize> = qubits.to_vec();
                x.get(&qubits).copied()
            }
            None => None,
        }
    }

    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.decoherence_rates.get(qubit).cloned()
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gates
                    .iter()
                    .filter(|(key, _)| key.as_str() != "SWAP")
                    .any(|(_, val)| {
                        val.contains_key(&(row, column)) || val.contains_key(&(column, row))
                    })
                {
                    vector.push((row, column))
                }
            }
        }
        vector
    }

    fn to_generic_device(&self) -> GenericDevice {
        self.clone()
    }
}

impl crate::operations::SupportedVersion for GenericDevice {}
