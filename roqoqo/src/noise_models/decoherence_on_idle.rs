// Copyright © 2023-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
#[cfg(feature = "serialize")]
use crate::RoqoqoError;
use struqture::{
    spins::PlusMinusLindbladNoiseOperator, spins::PlusMinusProduct, OperateOnDensityMatrix,
};

/// Noise model representing a continuous decoherence process on idle qubits.
///
/// The purpose of this noise model is to enable defining background noise models that exclude the qubits involved
/// in a gate, as these might be decoupled from the background noise.The noise for each qubit can be different but
/// only single qubit noise is included in the model.
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
/// To create a complex decoherence model first create the Lindblad noise and then turn it into a DecoherenceOnIdleModel.
/// For a simple decoherence model, use new to create an empty model and use the add_damping, add_excitation, add_depolarising
/// and add_dephasing methods.
/// For more fine control access the internal lindblad_noise directly and modify it.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(try_from = "DecoherenceOnIdleModelSerialize")
)]
#[cfg_attr(feature = "serialize", serde(into = "DecoherenceOnIdleModelSerialize"))]
pub struct DecoherenceOnIdleModel {
    /// Decoherence rates for all qubits
    pub lindblad_noise: PlusMinusLindbladNoiseOperator,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for DecoherenceOnIdleModel {
    fn schema_name() -> String {
        "DecoherenceOnIdleModel".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <DecoherenceOnIdleModelSerialize>::json_schema(gen)
    }
}

#[cfg(feature = "serialize")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "DecoherenceOnIdleModel"))]
#[cfg_attr(
    feature = "json_schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
struct DecoherenceOnIdleModelSerialize {
    /// Decoherence rates for all qubits.
    lindblad_noise: struqture_1::spins::PlusMinusLindbladNoiseOperator,
}

#[cfg(feature = "serialize")]
impl TryFrom<DecoherenceOnIdleModelSerialize> for DecoherenceOnIdleModel {
    type Error = RoqoqoError;
    fn try_from(value: DecoherenceOnIdleModelSerialize) -> Result<Self, Self::Error> {
        Ok(DecoherenceOnIdleModel {
            lindblad_noise: PlusMinusLindbladNoiseOperator::from_struqture_1(&value.lindblad_noise).expect("Failed to convert PlusMinusLindbladNoiseOperator from struqture 1.x for serialization."),
        })
    }
}

#[cfg(feature = "serialize")]
impl From<DecoherenceOnIdleModel> for DecoherenceOnIdleModelSerialize {
    fn from(value: DecoherenceOnIdleModel) -> Self {
        let lindblad_noise = value.lindblad_noise.to_struqture_1().expect(
            "Failed to convert PlusMinusLindbladNoiseOperator to struqture 1.x for serialization.",
        );
        Self { lindblad_noise }
    }
}

impl SupportedVersion for DecoherenceOnIdleModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

impl DecoherenceOnIdleModel {
    /// Create a new empty DecoherenceOnIdleModel.
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
    /// * `self` - The DecoherenceOnIdleModel with the damping added.
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
    /// * `self` - The DecoherenceOnIdleModel with the rate of excitation added.
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
    /// * `self` - The DecoherenceOnIdleModel with the dephasing added.
    pub fn add_dephasing_rate(mut self, qubits: &[usize], rate: f64) -> Self {
        for qubit in qubits {
            // This can never fail here
            self.lindblad_noise
                .add_operator_product(
                    (
                        PlusMinusProduct::new().z(*qubit),
                        PlusMinusProduct::new().z(*qubit),
                    ),
                    (0.5 * rate).into(),
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
    /// * `self` - The DecoherenceOnIdleModel with the rate of depolarising added.
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

impl From<PlusMinusLindbladNoiseOperator> for DecoherenceOnIdleModel {
    fn from(value: PlusMinusLindbladNoiseOperator) -> Self {
        Self {
            lindblad_noise: value,
        }
    }
}

impl From<DecoherenceOnIdleModel> for PlusMinusLindbladNoiseOperator {
    fn from(value: DecoherenceOnIdleModel) -> Self {
        value.lindblad_noise
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "json_schema")]
    use jsonschema::Validator;

    #[test]
    fn test_decoherence_on_idle_model_new() {
        let model = DecoherenceOnIdleModel::new();
        assert_eq!(model.lindblad_noise, PlusMinusLindbladNoiseOperator::new());
    }

    #[test]
    fn test_decoherence_on_idle_model_add_damping() {
        let model = DecoherenceOnIdleModel::new();
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
    fn test_decoherence_on_idle_model_add_depolarising() {
        let model = DecoherenceOnIdleModel::new();
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
    fn test_decoherence_on_idle_model_add_excitation_rate() {
        let mut model = DecoherenceOnIdleModel::new();
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
    fn test_decoherence_on_idle_model_add_dephasing_rate() {
        let mut model = DecoherenceOnIdleModel::new();
        model = model.add_dephasing_rate(&[0, 1], 0.9);
        let mut lindblad_operator = PlusMinusLindbladNoiseOperator::new();
        lindblad_operator
            .add_operator_product(
                (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
                0.45.into(),
            )
            .unwrap();
        lindblad_operator
            .add_operator_product(
                (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
                0.45.into(),
            )
            .unwrap();

        assert_eq!(model.lindblad_noise, lindblad_operator);
    }
    #[test]
    fn test_from_decoherence_on_idle_model_to_plus_minus_lindblad_noise_operator() {
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

        let model = DecoherenceOnIdleModel { lindblad_noise };
        let converted_model: PlusMinusLindbladNoiseOperator = model.clone().into();
        assert_eq!(model.lindblad_noise, converted_model);
    }

    #[test]
    fn test_from_plus_minus_lindblad_noise_operator_to_decoherence_on_idle_model() {
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
        let converted_model: DecoherenceOnIdleModel = lindblad_noise.clone().into();
        assert_eq!(DecoherenceOnIdleModel { lindblad_noise }, converted_model);
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_feature() {
        let mut model = DecoherenceOnIdleModel::new();
        model = model.add_dephasing_rate(&[0, 1], 0.9);
        let schema = schemars::schema_for!(DecoherenceOnIdleModel);
        let schema_checker =
            Validator::new(&serde_json::to_value(&schema).unwrap()).expect("schema is valid");
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
