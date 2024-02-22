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
use struqture;
///
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(from = "SingleQubitOverrotationDescriptionSerialize"))]
#[cfg_attr(feature = "serialize", serde(into = "SingleQubitOverrotationDescriptionSerialize"))]
pub struct SingleQubitOverrotationDescription{
    /// Single qubit gate that describes the overrotation added
    gate: String,
    /// Mean value for the overroation: overroations are stochatically distributed around this base overroation value
    theta_mean: f64,
    /// Standard deviation of Gaussian distribution around mean value of theta
    theta_std: f64,
    }

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for SingleQubitOverrotationDescription {
    fn schema_name() -> String {
        "SingleQubitOverrotationDescription".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SingleQubitOverrotationDescriptionSerialize>::json_schema(gen)
    }
}


#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json_schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
struct SingleQubitOverrotationDescriptionSerialize {
    gate: String,
    theta_mean: f64,
    theta_std: f64,
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationDescription> for SingleQubitOverrotationDescriptionSerialize {
    fn from(value: SingleQubitOverrotationDescription) -> Self {
        let gate: String = value.gate;
        let theta_mean: f64 = value.theta_mean;
        let theta_std: f64 = value.theta_mean;

            SingleQubitOverrotationDescriptionSerialize {
            gate,
            theta_mean,
            theta_std
        }
    }
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationDescriptionSerialize> for SingleQubitOverrotationDescription {
    fn from(value: SingleQubitOverrotationDescriptionSerialize) -> Self {
        let gate: String = value.gate;
        let theta_mean: f64 = value.theta_mean;
        let theta_std: f64 = value.theta_mean;
        
        SingleQubitOverrotationDescription {
            gate,
            theta_mean,
            theta_std
        }
    }
}

impl SupportedVersion for SingleQubitOverrotationDescription {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 10, 0)
    }
}

impl SingleQubitOverrotationDescription{
    ///
    pub fn new(gate: &str, theta_mean: f64, theta_std: f64) -> Self {
        SingleQubitOverrotationDescription {
            gate: gate.to_string(),
            theta_mean: theta_mean,
            theta_std: theta_std,
        }
    }
}
/// 
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(from = "SingleQubitOverrotationOnGateSerialize"))]
#[cfg_attr(feature = "serialize", serde(into = "SingleQubitOverrotationOnGateSerialize"))]
pub struct SingleQubitOverrotationOnGate{
// Description of overroations. For each single gate in a circuit that corresponds to the String value acting on a qubit corresponding to the usize value, a single qubit overrotation gate is applied. The overrotation gate is described by the SingleQubitOverrotationDescription. The angle of the overrotation is randomly drawn from a normal distribution also given by the SingleQubitOverroation Description
single_qubit_overrotations: HashMap<(String, usize), SingleQubitOverrotationDescription>,
two_qubit_overrotations: HashMap<(String, (usize, usize)), struqture::spins::PlusMinusLindbladNoiseOperator>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for SingleQubitOverrotationOnGate {
    fn schema_name() -> String {
        "SingleQubitOverrotationOnGate".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SingleQubitOverrotationOnGateSerialize>::json_schema(gen)
    }
}

type SingleQGateIndex = (String, usize);
type SingleQubitOverroation = Vec<(
    SingleQGateIndex,
    SingleQubitOverrotationDescription,
)>;
type TwoQubitGateIndex = (String, (usize, usize));
type TwoQubitOverrotation = Vec<(
    TwoQubitGateIndex,
    struqture::spins::PlusMinusLindbladNoiseOperator,
)>;
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json_schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
struct SingleQubitOverrotationOnGateSerialize {
    /// Extra noise for single qubit gates.
    single_qubit_overrotations: SingleQubitOverroation,
    /// Extra noise for two qubit gates.
    two_qubit_overrotations: TwoQubitOverrotation,
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationOnGate> for SingleQubitOverrotationOnGateSerialize {
    fn from(value: SingleQubitOverrotationOnGate) -> Self {
        let single_qubit_overrotations: SingleQubitOverroation =
            value.single_qubit_overrotations.into_iter().collect();
        let two_qubit_overrotations: TwoQubitOverrotation =
            value.two_qubit_overrotations.into_iter().collect();
        SingleQubitOverrotationOnGateSerialize {
            single_qubit_overrotations,
            two_qubit_overrotations,
        }
    }
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationOnGateSerialize> for SingleQubitOverrotationOnGate {
    fn from(value: SingleQubitOverrotationOnGateSerialize) -> Self {
        let single_qubit_overrotations: HashMap<
            (String, usize),
            SingleQubitOverrotationDescription,
        > = value.single_qubit_overrotations.into_iter().collect();
        let two_qubit_overrotations: HashMap<
            (String, (usize, usize)),
            struqture::spins::PlusMinusLindbladNoiseOperator,
        > = value.two_qubit_overrotations.into_iter().collect();
        
        SingleQubitOverrotationOnGate {
            single_qubit_overrotations,
            two_qubit_overrotations,
        }
    }
}

impl SupportedVersion for SingleQubitOverrotationOnGate {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 10, 0)
    }
}

impl SingleQubitOverrotationOnGate {
    /// Creates a new DecoherenceOnGateModel with default values.
    pub fn new() -> Self {
        Self {
            single_qubit_overrotations: HashMap::new(),
            two_qubit_overrotations: HashMap::new(),
        }
    }

    /// Sets for a single qubit gate.
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
    pub fn set_single_qubit_overrotations(
        mut self,
        gate: &str,
        qubit: usize,
        noise_description: SingleQubitOverrotationDescription,
    ) -> Self {
        self.single_qubit_overrotations
            .insert((gate.to_string(), qubit), noise_description);
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
    pub fn get_single_qubit_overrotations(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<&SingleQubitOverrotationDescription> {
        self.single_qubit_overrotations
            .get(&(gate.to_string(), qubit))
    }

    /// Sets extra noise 
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
    pub fn set_two_qubit_overrotations(
        mut self,
        gate: &str,
        control: usize,
        target: usize,
        noise_operator: struqture::spins::PlusMinusLindbladNoiseOperator,
    ) -> Self {
        self.two_qubit_overrotations
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
    pub fn get_two_qubit_overrotations(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<&struqture::spins::PlusMinusLindbladNoiseOperator> {
        self.two_qubit_overrotations
            .get(&(gate.to_string(), (control, target)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use struqture::spins::PlusMinusLindbladNoiseOperator;

    #[test]
    fn test_singe_qubit_overrotations_on_gate_single() {
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        noise_model = noise_model.set_single_qubit_overrotations(
            "RotateX",
            0,
            noise_descp.clone(),
        );
        assert_eq!(
            noise_model.get_single_qubit_overrotations("RotateX", 0),
            Some(&noise_descp)
        );
    }

    #[test]
    fn test_overrotations_on_gate_two() {
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        noise_model = noise_model.set_two_qubit_overrotations(
            "CNOT",
            0,
            1,
            PlusMinusLindbladNoiseOperator::new(),
        );
        assert_eq!(
            noise_model.get_two_qubit_overrotations("CNOT", 0, 1),
            Some(&PlusMinusLindbladNoiseOperator::new())
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_json_serialization() {
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        noise_model = noise_model.set_single_qubit_overrotations(
            "RotateX",
            0,
            noise_descp.clone(),
        );
        let json_str = serde_json::to_string(&noise_model).unwrap();
        let deserialized_noise_model: SingleQubitOverrotationOnGate =
            serde_json::from_str(&json_str).unwrap();
        assert_eq!(noise_model, deserialized_noise_model);
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_feature() {
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        let mut model = SingleQubitOverrotationOnGate::new();
        model =
            model.set_single_qubit_overrotations("RotateX", 0, noise_descp.clone());
        let schema = schemars::schema_for!(SingleQubitOverrotationOnGate);
        let schema_checker =
            jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
                .expect("schema is valid");
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