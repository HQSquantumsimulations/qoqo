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
use super::GenericDevice;
use crate::RoqoqoError;
use ndarray::Array2;
/// A device assuming all-to-all connectivity between all involved qubits.
///
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct AllToAllDevice {
    number_qubits: usize,
    generic_device: GenericDevice,
}

impl AllToAllDevice {
    /// Creates a new AllToAllDevice.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    /// * `default_gate_time` - The default gate time of all gates
    ///
    /// # Returns
    ///
    /// * `Self` - An initiated AllToAllDevice with single and two-qubit gates and decoherence rates set to zero.
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: &[String],
        two_qubit_gates: &[String],
        default_gate_time: f64,
    ) -> Self {
        // Initialization of single qubit gates with empty times
        let generic = GenericDevice {
            number_qubits,
            single_qubit_gates: HashMap::with_capacity(single_qubit_gates.len()),
            two_qubit_gates: HashMap::with_capacity(two_qubit_gates.len()),
            multi_qubit_gates: HashMap::new(),
            decoherence_rates: HashMap::with_capacity(number_qubits),
        };
        let mut new = Self {
            number_qubits,
            generic_device: generic,
        };
        for gate_name in single_qubit_gates {
            new = new.set_all_single_qubit_gate_times(gate_name, default_gate_time);
        }
        for gate_name in two_qubit_gates {
            new = new.set_all_two_qubit_gate_times(gate_name, default_gate_time);
        }
        new = new
            .set_all_qubit_decoherence_rates(Array2::zeros((3, 3)))
            .expect("Internal bug in roqoqo devices.");
        new
    }

    /// Function that allows to set a uniform gate time per gate type for the single-qubit-gates.
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
        if let Some(gate_times) = self.generic_device.single_qubit_gates.get_mut(gate) {
            for (_, gatetime) in gate_times.iter_mut() {
                *gatetime = gate_time
            }
        } else {
            let mut gatetimes: HashMap<usize, f64> = HashMap::with_capacity(self.number_qubits);
            for qubit in 0..self.number_qubits {
                gatetimes.insert(qubit, gate_time);
            }
            self.generic_device
                .single_qubit_gates
                .insert(gate.to_string(), gatetimes);
        }
        self
    }

    /// Function that allows to set a unifromg gate time per gate type for the two-qubit-gates.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// `Self` - An AllToAllDevice with updated gate times.
    pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if let Some(gate_times) = self.generic_device.two_qubit_gates.get_mut(gate) {
            for (_, gatetime) in gate_times.iter_mut() {
                *gatetime = gate_time
            }
        } else {
            let mut gatetimes: HashMap<(usize, usize), f64> =
                HashMap::with_capacity(self.number_qubits.pow(2));
            for control in 0..self.number_qubits {
                for target in 0..self.number_qubits {
                    if control != target {
                        gatetimes.insert((control, target), gate_time);
                    }
                }
            }
            self.generic_device
                .two_qubit_gates
                .insert(gate.to_string(), gatetimes);
        }
        self
    }

    /// Setting the gate time of a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `qubit` - The qubit for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The gate time was correctly set and nothing is returned
    /// * `Err(RoqoqoError::GenericError)` - Qubit set is larger than the number qubits in the device
    pub fn set_single_qubit_gate_time(
        &mut self,
        gate: &str,
        qubit: usize,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        self.generic_device
            .set_single_qubit_gate_time(gate, qubit, gate_time)
    }

    /// Setting the gate time of a two qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `control` - The control qubit for which the gate time is set
    /// * `target` - The target qubit for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The gate time was correctly set and nothing is returned
    /// * `Err(RoqoqoError::GenericError)` - Control qubit set is larger than the number qubits in the device
    /// * `Err(RoqoqoError::GenericError)` - Target qubit set is larger than the number qubits in the device
    pub fn set_two_qubit_gate_time(
        &mut self,
        gate: &str,
        control: usize,
        target: usize,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        self.generic_device
            .set_two_qubit_gate_time(gate, control, target, gate_time)
    }

    /// Setting the gate time of a mulit qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the multi-qubit-gate.
    /// * `qubits` - The qubits for which the gate time is set
    /// * `gate_time` - gate time for the given gate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The gate time was correctly set and nothing is returned
    /// * `Err(RoqoqoError::GenericError)` - A qubit in the qubits vector is larger than the number qubits in the device
    pub fn set_multi_qubit_gate_time(
        &mut self,
        gate: &str,
        qubits: Vec<usize>,
        gate_time: f64,
    ) -> Result<(), RoqoqoError> {
        self.generic_device
            .set_multi_qubit_gate_time(gate, qubits, gate_time)
    }

    /// Function to set the decoherence rates for one qubit in the device.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate is set
    /// * `rates` - decoherence rates for one qubit in the device, provided as a (3x3)-matrix.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The decoherence rate was correctly set and nothing is returned
    /// * `Err(RoqoqoError::GenericError)` - Qubit set is larger than the number qubits in the device
    pub fn set_qubit_decoherence_rates(
        &mut self,
        qubit: usize,
        rates: Array2<f64>,
    ) -> Result<(), RoqoqoError> {
        self.generic_device
            .set_qubit_decoherence_rates(qubit, rates)
    }

    /// Function to set the decoherence rates for all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The device with updated decoherence rates.
    /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
    pub fn set_all_qubit_decoherence_rates(
        mut self,
        rates: Array2<f64>,
    ) -> Result<Self, RoqoqoError> {
        // Check if input matrix has the dimension (3x3)
        let shape = rates.shape();
        if shape == [3, 3] {
            for qubit in 0..self.number_qubits() {
                self.generic_device
                    .set_qubit_decoherence_rates(qubit, rates.clone())?;
            }
            Ok(self)
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
    /// * `qubit` - The qubit for which the damping is added
    /// * `damping` - The damping rates.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The device with updated damping.
    /// * `Err(RoqoqoError::GenericError)` - Qubit set is larger than the number qubits in the device
    pub fn add_damping(&mut self, qubit: usize, damping: f64) -> Result<(), RoqoqoError> {
        self.generic_device.add_damping(qubit, damping)
    }

    /// Adds qubit dephasing to noise rates.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the dephasing is added
    /// * `dephasing` - The dephasing rates.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The device with updated dephasing.
    /// * `Err(RoqoqoError::GenericError)` - Qubit set is larger than the number qubits in the device
    pub fn add_dephasing(&mut self, qubit: usize, dephasing: f64) -> Result<(), RoqoqoError> {
        self.generic_device.add_dephasing(qubit, dephasing)
    }

    /// Adds qubit depolarising to noise rates.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the depolarising noise is added
    /// * `depolarising` - The depolarising rates.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The device with updated depolarising.
    /// * `Err(RoqoqoError::GenericError)` - Qubit set is larger than the number qubits in the device
    pub fn add_depolarising(&mut self, qubit: usize, depolarising: f64) -> Result<(), RoqoqoError> {
        self.generic_device.add_depolarising(qubit, depolarising)
    }

    /// Adds damping to allnoise rates.
    ///
    /// # Arguments
    ///
    /// * `damping` - The damping rates.
    ///
    /// # Returns
    ///
    /// * `Self` - An AllToAllDevice with updated damping.
    pub fn add_damping_all(mut self, damping: f64) -> Self {
        for qubit in 0..self.number_qubits {
            self.generic_device
                .add_damping(qubit, damping)
                .expect("Checked insertion fails");
        }
        self
    }

    /// Adds dephasing to all noise rates.
    ///
    /// # Arguments
    ///
    /// * `dephasing` - The dephasing rates.
    ///
    /// # Returns
    ///
    /// * `Self` - An AllToAllDevice with updated dephasing.
    pub fn add_dephasing_all(mut self, dephasing: f64) -> Self {
        for qubit in 0..self.number_qubits {
            self.generic_device
                .add_dephasing(qubit, dephasing)
                .expect("Checked insertion fails");
        }
        self
    }

    /// Adds depolarising to all noise rates.
    ///
    /// # Arguments
    ///
    /// * `depolarising` - The depolarising rates.
    ///
    /// # Returns
    ///
    /// * `Self` - An AllToAllDevice with updated depolarising.
    pub fn add_depolarising_all(mut self, depolarising: f64) -> Self {
        for qubit in 0..self.number_qubits {
            self.generic_device
                .add_depolarising(qubit, depolarising)
                .expect("Checked insertion fails");
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
    fn number_qubits(&self) -> usize {
        self.generic_device.number_qubits
    }

    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        self.generic_device.single_qubit_gate_time(hqslang, qubit)
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        self.generic_device
            .two_qubit_gate_time(hqslang, control, target)
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        self.generic_device.multi_qubit_gate_time(hqslang, qubits)
    }

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
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.generic_device.qubit_decoherence_rates(qubit)
    }

    fn to_generic_device(&self) -> GenericDevice {
        self.generic_device.clone()
    }

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

impl crate::operations::SupportedVersion for AllToAllDevice {}
