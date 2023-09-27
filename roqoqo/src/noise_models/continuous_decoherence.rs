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
use struqture::{
    spins::PlusMinusLindbladNoiseOperator, spins::PlusMinusProduct, OperateOnDensityMatrix,
};

/// Noise model representing a continuous decoherence process on qubits.
///
/// This noise model assumes that all qubits are constantly experiencing
/// decoherence over time (e.g. due to coupling to the environment).
/// The noise for each qubit can be different but only single qubit noise is
/// included in the model.
///
/// Noise is given by the rates of the Lindblad equation.
/// The Lindblad equation is a so-called master equation for the time evolution of a density matrix.
/// For the example of a single qubit the non-coherent part of the Lindblad equation can take the following form:
///
/// d/dt * ρ = Σ Mij * Li * ρ * Lj† - 1/2 * ( Lj† * Li * ρ + ρ * Lj† * Li),
///
/// where the indices i and j run from 0 to 2
///
/// with L0 = σ+, L1 = σ- and L3 = σz.
///
/// Here the general incoherent part of the Lindblad equation is internally represented by a [struqture::spins::PlusMinusLindbladNoiseOperator].
///
/// To create a complex decoherence model first create the Lindblad noise and then turn it into a ContinuousDecoherenceModel.
/// For a simple decoherence model, use new to create an empty model and use the add_damping, add_excitation, add_depolarising
/// and add_dephasing methods.
/// For more fine control access the internal lindblad_noise directly and modify it.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ContinuousDecoherenceModel {
    /// Decoherence rates for all qubits
    pub lindblad_noise: PlusMinusLindbladNoiseOperator,
}

impl SupportedVersion for ContinuousDecoherenceModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

impl ContinuousDecoherenceModel {
    /// Create a new empty ContinuousDecoherenceModel.
    pub fn new() -> Self {
        Self {
            lindblad_noise: PlusMinusLindbladNoiseOperator::default(),
        }
    }

    /// Convenience function to add damping to several qubits
    ///
    /// # Arguments
    ///
    /// * `qubits` - The qubits to add damping to.
    /// * `rate` - The damping rate.
    ///
    /// # Returns
    ///
    /// `self` - The ContinuousDecoherenceModel with the damping added.
    pub fn add_damping_rate(mut self, qubits: &[usize], rate: f64) -> Self {
        for qubit in qubits {
            // This can never fail here
            self.lindblad_noise
                .add_operator_product(
                    (
                        // In the qc convention in a single qubit state vector v[0] corresponds to state |0>
                        // and v[1] corresponds to state |1>
                        // The plus-minus plus operator brings state |1> to state |0>
                        // Therefore the plus operator corresponds to damping
                        PlusMinusProduct::new().plus(*qubit),
                        PlusMinusProduct::new().plus(*qubit),
                    ),
                    rate.into(),
                )
                .expect("Internal struqture bug.");
        }
        self
    }
    /// Convenience function to add rate of excitation from 0 to 1 state to several qubits
    ///
    /// # Arguments
    ///
    /// * `qubits` - The qubits to add the rate of excitation to.
    /// * `rate` - The rate of excitation.
    ///
    /// # Returns
    ///
    /// * `self` - The ContinuousDecoherenceModel with the rate of excitation added.
    pub fn add_excitation_rate(mut self, qubits: &[usize], rate: f64) -> Self {
        for qubit in qubits {
            // This can never fail here
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().minus(*qubit),
                        PlusMinusProduct::new().minus(*qubit),
                    ),
                    rate.into(),
                )
                .expect("Internal struqture bug.");
        }
        self
    }
    /// Convenience function to add damping to several qubits
    ///
    /// # Arguments
    ///
    /// * `qubits` - The qubits to add dephasing to.
    /// * `rate` - The dephasing rate.
    ///
    /// # Returns
    ///
    /// * `self` - The ContinuousDecoherenceModel with the dephasing added.
    pub fn add_dephasing_rate(mut self, qubits: &[usize], rate: f64) -> Self {
        for qubit in qubits {
            // This can never fail here
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().z(*qubit),
                        PlusMinusProduct::new().z(*qubit),
                    ),
                    rate.into(),
                )
                .expect("Internal struqture bug.");
        }
        self
    }
    /// Convenience function to add depolarising to several qubits
    ///
    /// # Arguments
    ///
    /// * `qubits` - The qubits to add depolarising to.
    /// * `rate` - The rate of depolarising.
    ///
    /// # Returns
    ///
    /// * `self` - The ContinuousDecoherenceModel with the rate of depolarising added.
    pub fn add_depolarising_rate(mut self, qubits: &[usize], rate: f64) -> Self {
        for qubit in qubits {
            // This can never fail here
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().plus(*qubit),
                        PlusMinusProduct::new().plus(*qubit),
                    ),
                    (rate / 2.0).into(),
                )
                .expect("Internal struqture bug.");
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().minus(*qubit),
                        PlusMinusProduct::new().minus(*qubit),
                    ),
                    (rate / 2.0).into(),
                )
                .expect("Internal struqture bug.");
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().z(*qubit),
                        PlusMinusProduct::new().z(*qubit),
                    ),
                    (rate / 4.0).into(),
                )
                .expect("Internal struqture bug.");
        }
        self
    }
}

impl From<PlusMinusLindbladNoiseOperator> for ContinuousDecoherenceModel {
    fn from(value: PlusMinusLindbladNoiseOperator) -> Self {
        Self {
            lindblad_noise: value,
        }
    }
}

impl From<ContinuousDecoherenceModel> for PlusMinusLindbladNoiseOperator {
    fn from(value: ContinuousDecoherenceModel) -> Self {
        value.lindblad_noise
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_continuous_decoherence_model_new() {
        let model = ContinuousDecoherenceModel::new();
        assert_eq!(model.lindblad_noise, PlusMinusLindbladNoiseOperator::new());
    }

    #[test]
    fn test_continuous_decoherence_model_add_damping() {
        let model = ContinuousDecoherenceModel::new();
        let model = model.add_damping_rate(&[0], 0.9);
        let mut lindblad_operator = PlusMinusLindbladNoiseOperator::new();
        lindblad_operator
            .add_operator_product(
                (
                    PlusMinusProduct::new().plus(0),
                    PlusMinusProduct::new().plus(0),
                ),
                0.9.into(),
            )
            .unwrap();
        assert_eq!(model.lindblad_noise, lindblad_operator);
    }

    #[test]
    fn test_continuous_decoherence_model_add_depolarising() {
        let model = ContinuousDecoherenceModel::new();
        let model = model.add_depolarising_rate(&[0], 1.8);
        let mut lindblad_operator = PlusMinusLindbladNoiseOperator::new();
        lindblad_operator
            .add_operator_product(
                (
                    PlusMinusProduct::new().plus(0),
                    PlusMinusProduct::new().plus(0),
                ),
                0.9.into(),
            )
            .unwrap();
        lindblad_operator
            .add_operator_product(
                (
                    PlusMinusProduct::new().minus(0),
                    PlusMinusProduct::new().minus(0),
                ),
                0.9.into(),
            )
            .unwrap();
        lindblad_operator
            .add_operator_product(
                (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
                0.45.into(),
            )
            .unwrap();
        assert_eq!(model.lindblad_noise, lindblad_operator);
    }

    #[test]
    fn test_continuous_decoherence_model_add_excitation_rate() {
        let mut model = ContinuousDecoherenceModel::new();
        model = model.add_excitation_rate(&[0, 1], 0.9);
        let mut lindblad_operator = PlusMinusLindbladNoiseOperator::new();
        lindblad_operator
            .add_operator_product(
                (
                    PlusMinusProduct::new().minus(0),
                    PlusMinusProduct::new().minus(0),
                ),
                0.9.into(),
            )
            .unwrap();
        lindblad_operator
            .add_operator_product(
                (
                    PlusMinusProduct::new().minus(1),
                    PlusMinusProduct::new().minus(1),
                ),
                0.9.into(),
            )
            .unwrap();
        assert_eq!(model.lindblad_noise, lindblad_operator);
    }

    #[test]
    fn test_continuous_decoherence_model_add_dephasing_rate() {
        let mut model = ContinuousDecoherenceModel::new();
        model = model.add_dephasing_rate(&[0, 1], 0.9);
        let mut lindblad_operator = PlusMinusLindbladNoiseOperator::new();
        lindblad_operator
            .add_operator_product(
                (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
                0.9.into(),
            )
            .unwrap();
        lindblad_operator
            .add_operator_product(
                (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
                0.9.into(),
            )
            .unwrap();

        assert_eq!(model.lindblad_noise, lindblad_operator);
    }
    #[test]
    fn test_from_continuous_decoherence_model_to_plus_minus_lindblad_noise_operator() {
        let mut lindblad_noise = PlusMinusLindbladNoiseOperator::new();
        lindblad_noise
            .add_operator_product(
                (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
                0.9.into(),
            )
            .unwrap();
        lindblad_noise
            .add_operator_product(
                (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
                0.9.into(),
            )
            .unwrap();

        let model = ContinuousDecoherenceModel { lindblad_noise };
        let converted_model: PlusMinusLindbladNoiseOperator = model.clone().into();
        assert_eq!(model.lindblad_noise, converted_model);
    }

    #[test]
    fn test_from_plus_minus_lindblad_noise_operator_to_continuous_decoherence_model() {
        let mut lindblad_noise = PlusMinusLindbladNoiseOperator::new();
        lindblad_noise
            .add_operator_product(
                (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
                0.9.into(),
            )
            .unwrap();
        lindblad_noise
            .add_operator_product(
                (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
                0.9.into(),
            )
            .unwrap();
        let converted_model: ContinuousDecoherenceModel = lindblad_noise.clone().into();
        assert_eq!(
            ContinuousDecoherenceModel { lindblad_noise },
            converted_model
        );
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_feature() {
        let mut model = ContinuousDecoherenceModel::new();
        model = model.add_dephasing_rate(&[0, 1], 0.9);
        let schema = schemars::schema_for!(ContinuousDecoherenceModel);
        let schema_checker =
            jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
                .expect("schema is valid");
        let value = serde_json::to_value(&model).unwrap();
        let val = match value {
            serde_json::Value::Object(ob) => ob,
            _ => panic!(),
        };
        let value: serde_json::Value = serde_json::to_value(val).unwrap();
        let validation = schema_checker.validate(&value);
        assert!(validation.is_ok());
    }
}
