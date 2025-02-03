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
use std::collections::HashMap;

/// Description of single qubit overrotation noise model, [roqoqo::noise_models::SingleQubitOverrotationOnGate].
///
/// Consists of the raw data needed to construct a rotation gate that adds
/// overrotation: gate name and statistics (mean and standard deviation) of a Gaussian distribution
/// from which the overrotation angle is sampled.
///
/// Example:
///
/// ```
/// use roqoqo::noise_models::SingleQubitOverrotationDescription;
/// let gate = "RotateX";
/// let theta_mean = 0.0;
/// let theta_std = 1.0;
/// let mut noise_desc = SingleQubitOverrotationDescription::new(gate, theta_mean, theta_std);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SingleQubitOverrotationDescription {
    /// Name of the single qubit rotation gate
    gate: String,
    /// Mean value for the overrotation: overrottation are stochastically distributed around this base overrotation value
    theta_mean: f64,
    /// Standard deviation of Gaussian distribution around mean value of theta
    theta_std: f64,
}

impl SupportedVersion for SingleQubitOverrotationDescription {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

impl SingleQubitOverrotationDescription {
    /// Creates a new SingleQubitOverrotationDescription.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `theta_mean` - The mean of Gaussian distribution from which overrotation angle is sampled.
    /// * `theta_std` - The standard deviation of Gaussian distribution from which overrotation angle is sampled.
    ///
    /// # Returns
    ///
    /// `Self` - New description for overrotation noise model.
    pub fn new(gate: &str, theta_mean: f64, theta_std: f64) -> Self {
        SingleQubitOverrotationDescription {
            gate: gate.to_string(),
            theta_mean,
            theta_std,
        }
    }

    /// Returns gate name of SingleQubitOverrotationDescription.
    ///
    /// # Returns
    ///
    /// `gate` - Returns gate name.
    pub fn gate(&self) -> &String {
        &self.gate
    }

    /// Returns mean of Gaussian distribution of overrotation angles name of SingleQubitOverrotationDescription.
    ///
    /// # Returns
    ///
    /// `theta_mean` - mean of distribution.
    pub fn theta_mean(&self) -> f64 {
        self.theta_mean
    }

    /// Returns standard deviation of Gaussian distribution of overrotation angles name of SingleQubitOverrotationDescription.
    ///
    /// # Returns
    ///
    /// `theta_std` - standard deviation of distribution.
    pub fn theta_std(&self) -> f64 {
        self.theta_std
    }
}

/// Single qubit overrotation noise model on gate.
///
/// Adds a rotatation gate with a randomly distributed rotation angle after specified gates in a quantum circuit.
/// Example:
///
/// ```
/// use roqoqo::noise_models::SingleQubitOverrotationDescription;
/// use roqoqo::noise_models::SingleQubitOverrotationOnGate;
/// use roqoqo::noise_models::NoiseModel;
/// let gate = "RotateX";
/// let theta_mean = 0.0;
/// let theta_std = 1.0;
/// let mut noise_desc = SingleQubitOverrotationDescription::new(gate, theta_mean, theta_std);
///
/// let mut noise = SingleQubitOverrotationOnGate::new();
/// let circuit_gate_with_noise = "RotateZ";
/// let qubit = 0;
/// noise = noise.set_single_qubit_overrotation(circuit_gate_with_noise, qubit, noise_desc);
/// let noise_model = NoiseModel::SingleQubitOverrotationOnGate(noise);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(from = "SingleQubitOverrotationOnGateSerialize")
)]
#[cfg_attr(
    feature = "serialize",
    serde(into = "SingleQubitOverrotationOnGateSerialize")
)]
pub struct SingleQubitOverrotationOnGate {
    /// Overrotation noise information for single qubit gates in a quantum circuit.
    single_qubit_overrotation: HashMap<(String, usize), SingleQubitOverrotationDescription>,
    /// Overrotation noise information for two qubit gates in a quantum circuit.
    two_qubit_overrotation: HashMap<
        (String, (usize, usize)),
        (
            SingleQubitOverrotationDescription,
            SingleQubitOverrotationDescription,
        ),
    >,
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

#[cfg(feature = "serialize")]
type SingleQubitGateIndex = (String, usize);
#[cfg(feature = "serialize")]
type SingleQubitOverrotation = Vec<(SingleQubitGateIndex, SingleQubitOverrotationDescription)>;
#[cfg(feature = "serialize")]
type TwoQubitGateIndex = (String, (usize, usize));
#[cfg(feature = "serialize")]
type TwoQubitOverrotation = Vec<(
    TwoQubitGateIndex,
    (
        SingleQubitOverrotationDescription,
        SingleQubitOverrotationDescription,
    ),
)>;
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json_schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[cfg(feature = "serialize")]
struct SingleQubitOverrotationOnGateSerialize {
    /// Overrotation for single qubit gates.
    single_qubit_overrotation: SingleQubitOverrotation,
    /// Overrotation for two qubit gates.
    two_qubit_overrotation: TwoQubitOverrotation,
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationOnGate> for SingleQubitOverrotationOnGateSerialize {
    fn from(value: SingleQubitOverrotationOnGate) -> Self {
        let single_qubit_overrotation: SingleQubitOverrotation =
            value.single_qubit_overrotation.into_iter().collect();
        let two_qubit_overrotation: TwoQubitOverrotation =
            value.two_qubit_overrotation.into_iter().collect();
        SingleQubitOverrotationOnGateSerialize {
            single_qubit_overrotation,
            two_qubit_overrotation,
        }
    }
}

#[cfg(feature = "serialize")]
impl From<SingleQubitOverrotationOnGateSerialize> for SingleQubitOverrotationOnGate {
    fn from(value: SingleQubitOverrotationOnGateSerialize) -> Self {
        let single_qubit_overrotation: HashMap<
            (String, usize),
            SingleQubitOverrotationDescription,
        > = value.single_qubit_overrotation.into_iter().collect();
        let two_qubit_overrotation: HashMap<
            (String, (usize, usize)),
            (
                SingleQubitOverrotationDescription,
                SingleQubitOverrotationDescription,
            ),
        > = value.two_qubit_overrotation.into_iter().collect();

        SingleQubitOverrotationOnGate {
            single_qubit_overrotation,
            two_qubit_overrotation,
        }
    }
}

impl SupportedVersion for SingleQubitOverrotationOnGate {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

impl SingleQubitOverrotationOnGate {
    /// Creates a new SingleQubitOverrotationOnGate with default values.
    pub fn new() -> Self {
        Self {
            single_qubit_overrotation: HashMap::new(),
            two_qubit_overrotation: HashMap::new(),
        }
    }

    /// Sets overrotation for a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    /// * `noise_description` - overrotation description for gate.
    ///
    /// # Returns
    ///
    /// `Self` - The overrotation model with the new overrotation on gate set.
    pub fn set_single_qubit_overrotation(
        mut self,
        gate: &str,
        qubit: usize,
        noise_description: SingleQubitOverrotationDescription,
    ) -> Self {
        self.single_qubit_overrotation
            .insert((gate.to_string(), qubit), noise_description);
        self
    }
    /// Returns the overrotation description for a single qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// `Option<SingleQubitOverrotation>` - The overrotation applied when gate is applied.
    pub fn get_single_qubit_overrotation(
        &self,
        gate: &str,
        qubit: usize,
    ) -> Option<&SingleQubitOverrotationDescription> {
        self.single_qubit_overrotation
            .get(&(gate.to_string(), qubit))
    }

    /// Sets overrotation for a two qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - Controlling qubit.
    /// * `target` - Target qubit.
    /// * `noise_description` - overrotation description for gate.
    ///
    /// # Returns
    ///
    /// `Self` - The overrotation model with the new overrotation on gate set.
    pub fn set_two_qubit_overrotation(
        mut self,
        gate: &str,
        control: usize,
        target: usize,
        noise_description: (
            SingleQubitOverrotationDescription,
            SingleQubitOverrotationDescription,
        ),
    ) -> Self {
        self.two_qubit_overrotation
            .insert((gate.to_string(), (control, target)), noise_description);
        self
    }

    /// Returns the overrotation description for a two qubit gate, if it exists.
    ///
    /// # Arguments
    ///
    /// * `gate` - The name of the gate.
    /// * `control` - Controlling qubit.
    /// * `target` - Target qubit.
    ///
    /// # Returns
    ///
    /// `Option<(SingleQubitOverrotation,SingleQubitOverrotation)>` - The overrotation applied when gate is applied.
    pub fn get_two_qubit_overrotation(
        &self,
        gate: &str,
        control: usize,
        target: usize,
    ) -> Option<&(
        SingleQubitOverrotationDescription,
        SingleQubitOverrotationDescription,
    )> {
        self.two_qubit_overrotation
            .get(&(gate.to_string(), (control, target)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "json_schema")]
    use jsonschema::Validator;

    #[test]
    fn test_singe_qubit_overrotation_on_gate_single() {
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        noise_model = noise_model.set_single_qubit_overrotation("RotateX", 0, noise_descp.clone());
        assert_eq!(
            noise_model.get_single_qubit_overrotation("RotateX", 0),
            Some(&noise_descp)
        );
    }

    #[test]
    fn test_overrotation_on_gate_two() {
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        noise_model = noise_model.set_two_qubit_overrotation(
            "CNOT",
            0,
            1,
            (noise_descp.clone(), noise_descp.clone()),
        );
        assert_eq!(
            noise_model.get_two_qubit_overrotation("CNOT", 0, 1),
            Some(&(noise_descp.clone(), noise_descp.clone()))
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_json_serialization() {
        let noise_descp = SingleQubitOverrotationDescription::new("RotateZ", 1.0, 1.0);
        let mut noise_model = SingleQubitOverrotationOnGate::new();
        noise_model = noise_model.set_single_qubit_overrotation("RotateX", 0, noise_descp.clone());
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
        model = model.set_single_qubit_overrotation("RotateX", 0, noise_descp.clone());
        let schema = schemars::schema_for!(SingleQubitOverrotationOnGate);
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
