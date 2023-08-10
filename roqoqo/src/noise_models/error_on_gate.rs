// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use super::SupportedVersion;
use std::collections::HashMap;
/// Error model for noise that is only present on gate executions.
///
/// Adds additional noise when specific gates (identified by hqslang name and qubits acted on) are executed.
/// The noise is given in the form of a [struqture::spins::PlusMinusLindbladNoiseOperator] the same way it
/// is for the ContinuousDecoherence model.
/// Example:
///
/// ```
/// use roqoqo::noise_models::ErrorOnGateModel;
/// use struqture::spins::{PlusMinusLindbladNoiseOperator, PlusMinusProduct};
/// use struqture::prelude::*;
///
/// let mut noise_model = ErrorOnGateModel::new();
/// let mut lindblad_noise = PlusMinusLindbladNoiseOperator::new();
/// lindblad_noise.add_operator_product(
///    (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
///    0.9.into(),).unwrap();
/// lindblad_noise.add_operator_product(
///    (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
///    0.9.into(),).unwrap();
///
/// noise_model = noise_model.set_two_qubit_gate_error(
/// "CNOT", 0,1,
/// lindblad_noise
/// );
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorOnGateModel {
    /// Extra noise for single qubit gates.
    single_qubit_gate_errors:
        HashMap<(String, usize), struqture::spins::PlusMinusLindbladNoiseOperator>,
    /// Extra noise for two qubit gates.
    two_qubit_gate_errors:
        HashMap<(String, (usize, usize)), struqture::spins::PlusMinusLindbladNoiseOperator>,
    /// Extra noise for three qubit gates.
    three_qubit_gate_errors:
        HashMap<(String, (usize, usize, usize)), struqture::spins::PlusMinusLindbladNoiseOperator>,
    /// Extra noise for multi qubit gates.
    multi_qubit_gate_errors:
        HashMap<(String, Vec<usize>), struqture::spins::PlusMinusLindbladNoiseOperator>,
}

impl SupportedVersion for ErrorOnGateModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

impl ErrorOnGateModel {
    /// Creates a new ErrorOnGateModel with default values.
    pub fn new() -> Self {
        Self {
            single_qubit_gate_errors: HashMap::new(),
            two_qubit_gate_errors: HashMap::new(),
            three_qubit_gate_errors: HashMap::new(),
            multi_qubit_gate_errors: HashMap::new(),
        }
    }

    /// Sets extra noise for a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_single_qubit_gate_error(
        mut self,
        gate: &str,
        qubit: usize,
        noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator,
    ) -> Self {
        self.single_qubit_gate_errors
            .insert((gate.to_string(), qubit), noise_operator);
        self
    }

    /// Returns the extra noise for a single qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_single_qubit_gate_error(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<&struqture::spins::PlusMinusLindbladNoiseOperator> {
        self.single_qubit_gate_errors
            .get(&(gate.to_string(), qubit))
    }

    /// Sets extra noise for a two qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - Controlling qubit.
    /// * `target` - Target qubit.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn set_two_qubit_gate_error(
        mut self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator,
    ) -> Self {
        self.two_qubit_gate_errors
            .insert((gate.to_string(), (control, target)), noise_operator);
        self
    }

    /// Returns the extra noise for a two qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - Controlling qubit.
    /// * `target` - Target qubit.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_two_qubit_gate_error(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<&struqture::spins::PlusMinusLindbladNoiseOperator> {
        self.two_qubit_gate_errors
            .get(&(gate.to_string(), (control, target)))
    }

    /// Sets extra noise for a three qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control0` - First controlling qubit.
    /// * `control1` - Second controlling qubit.
    /// * `target` - Target qubit.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn set_three_qubit_gate_error(
        mut self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
        noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator,
    ) -> Self {
        self.three_qubit_gate_errors.insert(
            (gate.to_string(), (control0, control1, target)),
            noise_operator,
        );
        self
    }

    /// Returns the extra noise for a two qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control0` - First controlling qubit.
    /// * `control1` - Second controlling qubit.
    /// * `target` - Target qubit.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_three_qubit_gate_error(
        &self,
        gate: &str,
        control0: usize,
        control1: usize,
        target: usize,
    ) -> Option<&struqture::spins::PlusMinusLindbladNoiseOperator> {
        self.three_qubit_gate_errors
            .get(&(gate.to_string(), (control0, control1, target)))
    }

    /// Sets extra noise for a multi qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubits` - A vector of qubit indices.
    /// * `noise_operator` - The noise affecting system when gate is applied.
    ///
    /// # Returns
    ///
    /// `Self` - The error model with the new noise on gate set.
    pub fn set_multi_qubit_gate_error(
        mut self,
        gate: &str,
        qubits: Vec<usize>,
        noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator,
    ) -> Self {
        self.multi_qubit_gate_errors
            .insert((gate.to_string(), qubits), noise_operator);
        self
    }

    /// Returns the extra noise for a multi qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubits` - A vector of qubit indices.
    ///
    /// # Returns
    ///
    /// `Option<struqture::spins::PlusMinusLindbladNoiseOperator>` - The error model applied when gate is applied.
    pub fn get_multi_qubit_gate_error(
        &self,
        gate: &str,
        qubits: Vec<usize>,
    ) -> Option<&struqture::spins::PlusMinusLindbladNoiseOperator> {
        self.multi_qubit_gate_errors
            .get(&(gate.to_string(), qubits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use struqture::spins::PlusMinusLindbladNoiseOperator;

    #[test]
    fn test_error_on_gate_model_single() {
        let mut noise_model = ErrorOnGateModel::new();
        noise_model = noise_model.set_single_qubit_gate_error(
            "RotateX",
            0,
            PlusMinusLindbladNoiseOperator::new(),
        );
        assert_eq!(
            noise_model.get_single_qubit_gate_error("RotateX", 0),
            Some(&PlusMinusLindbladNoiseOperator::new())
        );
    }

    #[test]
    fn test_error_on_gate_model_two() {
        let mut noise_model = ErrorOnGateModel::new();
        noise_model = noise_model.set_two_qubit_gate_error(
            "CNOT",
            0,
            1,
            PlusMinusLindbladNoiseOperator::new(),
        );
        assert_eq!(
            noise_model.get_two_qubit_gate_error("CNOT", 0, 1),
            Some(&PlusMinusLindbladNoiseOperator::new())
        );
    }

    #[test]
    fn test_error_on_gate_model_three() {
        let mut noise_model = ErrorOnGateModel::new();
        noise_model = noise_model.set_three_qubit_gate_error(
            "ControlledControlledPauliZ",
            0,
            1,
            2,
            PlusMinusLindbladNoiseOperator::new(),
        );
        assert_eq!(
            noise_model.get_three_qubit_gate_error("ControlledControlledPauliZ", 0, 1, 2),
            Some(&PlusMinusLindbladNoiseOperator::new())
        );
    }

    #[test]
    fn test_error_on_gate_model_mulit() {
        let mut noise_model = ErrorOnGateModel::new();
        noise_model = noise_model.set_multi_qubit_gate_error(
            "MultiQubitMS",
            vec![0, 1, 2, 3],
            PlusMinusLindbladNoiseOperator::new(),
        );
        assert_eq!(
            noise_model.get_multi_qubit_gate_error("MultiQubitMS", vec![0, 1, 2, 3]),
            Some(&PlusMinusLindbladNoiseOperator::new())
        );
    }
}
