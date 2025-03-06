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

use struqture::spins::PlusMinusLindbladNoiseOperator;

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
/// use roqoqo::noise_models::DecoherenceOnGateModel;
/// use struqture::spins::{PlusMinusLindbladNoiseOperator, PlusMinusProduct};
/// use struqture::prelude::*;
///
/// let mut noise_model = DecoherenceOnGateModel::new();
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
#[cfg_attr(feature = "serialize", serde(from = "DecoherenceOnGateModelSerialize"))]
#[cfg_attr(feature = "serialize", serde(into = "DecoherenceOnGateModelSerialize"))]
pub struct DecoherenceOnGateModel {
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

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for DecoherenceOnGateModel {
    fn schema_name() -> String {
        "DecoherenceOnGateModel".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <DecoherenceOnGateModelSerialize>::json_schema(gen)
    }
}

#[cfg(feature = "serialize")]
type SingleQGateIndex = (String, usize);
#[cfg(feature = "serialize")]
type SingleQubitErrors = Vec<(
    SingleQGateIndex,
    struqture_1::spins::PlusMinusLindbladNoiseOperator,
)>;
#[cfg(feature = "serialize")]
type TwoQubitGateIndex = (String, (usize, usize));
#[cfg(feature = "serialize")]
type TwoQubitErrors = Vec<(
    TwoQubitGateIndex,
    struqture_1::spins::PlusMinusLindbladNoiseOperator,
)>;
#[cfg(feature = "serialize")]
type ThreeQubitGateIndex = (String, (usize, usize, usize));
#[cfg(feature = "serialize")]
type ThreeQubitErrors = Vec<(
    ThreeQubitGateIndex,
    struqture_1::spins::PlusMinusLindbladNoiseOperator,
)>;
#[cfg(feature = "serialize")]
type MultiQubitGateIndex = (String, Vec<usize>);
#[cfg(feature = "serialize")]
type MultiQubitErrors = Vec<(
    MultiQubitGateIndex,
    struqture_1::spins::PlusMinusLindbladNoiseOperator,
)>;
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json_schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[cfg(feature = "serialize")]
struct DecoherenceOnGateModelSerialize {
    /// Extra noise for single qubit gates.
    single_qubit_gate_errors: SingleQubitErrors,
    /// Extra noise for two qubit gates.
    two_qubit_gate_errors: TwoQubitErrors,
    /// Extra noise for three qubit gates.
    three_qubit_gate_errors: ThreeQubitErrors,
    /// Extra noise for multi qubit gates.
    multi_qubit_gate_errors: MultiQubitErrors,
}

#[cfg(feature = "serialize")]
impl From<DecoherenceOnGateModel> for DecoherenceOnGateModelSerialize {
    fn from(value: DecoherenceOnGateModel) -> Self {
        let single_qubit_gate_errors: SingleQubitErrors =
            value.single_qubit_gate_errors.into_iter().map(|(key, value)|(key, value.to_struqture_1().expect("Failed to convert PlusMinusLindbladNoiseOperator to struqture 1.x for serialization."))).collect();
        let two_qubit_gate_errors: TwoQubitErrors =
            value.two_qubit_gate_errors.into_iter().map(|(key, value)|(key, value.to_struqture_1().expect("Failed to convert PlusMinusLindbladNoiseOperator to struqture 1.x for serialization."))).collect();
        let three_qubit_gate_errors: ThreeQubitErrors =
            value.three_qubit_gate_errors.into_iter().map(|(key, value)|(key, value.to_struqture_1().expect("Failed to convert PlusMinusLindbladNoiseOperator to struqture 1.x for serialization."))).collect();
        let multi_qubit_gate_errors: MultiQubitErrors =
            value.multi_qubit_gate_errors.into_iter().map(|(key, value)|(key, value.to_struqture_1().expect("Failed to convert PlusMinusLindbladNoiseOperator to struqture 1.x for serialization."))).collect();
        DecoherenceOnGateModelSerialize {
            single_qubit_gate_errors,
            two_qubit_gate_errors,
            three_qubit_gate_errors,
            multi_qubit_gate_errors,
        }
    }
}

#[cfg(feature = "serialize")]
impl From<DecoherenceOnGateModelSerialize> for DecoherenceOnGateModel {
    fn from(value: DecoherenceOnGateModelSerialize) -> Self {
        let single_qubit_gate_errors: HashMap<
            (String, usize),
            struqture::spins::PlusMinusLindbladNoiseOperator,
        > = value.single_qubit_gate_errors.into_iter().map(|(key, value)|(key, PlusMinusLindbladNoiseOperator::from_struqture_1(&value).expect("Failed to convert PlusMinusLindbladNoiseOperator from struqture 1.x for serialization."))).collect();
        let two_qubit_gate_errors: HashMap<
            (String, (usize, usize)),
            struqture::spins::PlusMinusLindbladNoiseOperator,
        > = value.two_qubit_gate_errors.into_iter().map(|(key, value)|(key, PlusMinusLindbladNoiseOperator::from_struqture_1(&value).expect("Failed to convert PlusMinusLindbladNoiseOperator from struqture 1.x for serialization."))).collect();
        let three_qubit_gate_errors: HashMap<
            (String, (usize, usize, usize)),
            struqture::spins::PlusMinusLindbladNoiseOperator,
        > = value.three_qubit_gate_errors.into_iter().map(|(key, value)|(key, PlusMinusLindbladNoiseOperator::from_struqture_1(&value).expect("Failed to convert PlusMinusLindbladNoiseOperator from struqture 1.x for serialization."))).collect();
        let multi_qubit_gate_errors: HashMap<
            (String, Vec<usize>),
            struqture::spins::PlusMinusLindbladNoiseOperator,
        > = value.multi_qubit_gate_errors.into_iter().map(|(key, value)|(key, PlusMinusLindbladNoiseOperator::from_struqture_1(&value).expect("Failed to convert PlusMinusLindbladNoiseOperator from struqture 1.x for serialization."))).collect();
        DecoherenceOnGateModel {
            single_qubit_gate_errors,
            two_qubit_gate_errors,
            three_qubit_gate_errors,
            multi_qubit_gate_errors,
        }
    }
}

impl SupportedVersion for DecoherenceOnGateModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

impl DecoherenceOnGateModel {
    /// Creates a new DecoherenceOnGateModel with default values.
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
    #[cfg(feature = "json_schema")]
    use jsonschema::Validator;
    use struqture::spins::PlusMinusLindbladNoiseOperator;

    #[test]
    fn test_decoherence_on_gate_model_single() {
        let mut noise_model = DecoherenceOnGateModel::new();
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
    fn test_decoherence_on_gate_model_two() {
        let mut noise_model = DecoherenceOnGateModel::new();
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
    fn test_decoherence_on_gate_model_three() {
        let mut noise_model = DecoherenceOnGateModel::new();
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
    fn test_decoherence_on_gate_model_mulit() {
        let mut noise_model = DecoherenceOnGateModel::new();
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

    #[cfg(feature = "serialize")]
    #[test]
    fn test_json_serialization() {
        let mut noise_model = DecoherenceOnGateModel::new();
        noise_model = noise_model.set_single_qubit_gate_error(
            "RotateX",
            0,
            PlusMinusLindbladNoiseOperator::new(),
        );
        let json_str = serde_json::to_string(&noise_model).unwrap();
        let deserialized_noise_model: DecoherenceOnGateModel =
            serde_json::from_str(&json_str).unwrap();
        assert_eq!(noise_model, deserialized_noise_model);
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_feature() {
        let mut model = DecoherenceOnGateModel::new();
        model =
            model.set_single_qubit_gate_error("RotateX", 0, PlusMinusLindbladNoiseOperator::new());
        let schema = schemars::schema_for!(DecoherenceOnGateModel);
        let schema_checker =
            Validator::new(&serde_json::to_value(&schema).unwrap()).expect("schema is valid");
        let value = serde_json::to_value(model).unwrap();
        let val = match value {
            serde_json::Value::Object(ob) => ob,
            _ => panic!(),
        };
        let value: serde_json::Value = serde_json::to_value(val).unwrap();
        let validation = schema_checker.validate(&value);
        assert!(validation.is_ok());
    }
}
