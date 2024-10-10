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
use crate::{RoqoqoBackendError, RoqoqoError};
use std::collections::HashMap;
/// Noise model representing readout errors.
///
/// Readout errors are modeled by two probabilities in this simple model.
/// One probability to detect a 1 instead of a 0 when the quantum measurement gives 0 and
/// one probability to detect a 0 instead of a 1 when the quantum measurement gives 1.
///
/// # Example
///
/// ```rust
/// use roqoqo::noise_models::ImperfectReadoutModel;
///
/// let model = ImperfectReadoutModel::new_with_uniform_error(3, 0.5, 0.5).unwrap();
/// let model = model.set_error_probabilites(2, 0.3, 0.7).unwrap();
/// let uniform_prob = model.prob_detect_0_as_1(&0);
/// assert_eq!(uniform_prob, 0.5);
/// let lower_prob = model.prob_detect_0_as_1(&2);
/// assert_eq!(lower_prob, 0.3);
/// let higher_prob = model.prob_detect_1_as_0(&2);
/// assert_eq!(higher_prob, 0.7);
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ImperfectReadoutModel {
    /// Decoherence rates for all qubits
    prob_detect_0_as_1: HashMap<usize, f64>,
    prob_detect_1_as_0: HashMap<usize, f64>,
}

impl ImperfectReadoutModel {
    /// Create a new empty ContinuousDecoherenceModel.
    ///
    /// # Returns
    ///
    /// * `ImperfectReadoutModel` - The new model
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience function to create uniform error probabilities
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits the uniform error probabilites should be initialized for.
    /// * `prob_detect_0_as_1` - The error probability to detect a 1 instead of a 0 when measuring 0
    /// * `prob_detect_1_as_0` - The error probability to detect a 0 instead of a 1 when measuring 1
    ///
    /// # Returns
    ///
    /// * `Ok(ImperfectReadoutModel)` - The new error model
    /// * `Err(RoqoqoError)` - Probabilities are not valid (< 0 or > 1)
    pub fn new_with_uniform_error(
        number_qubits: usize,
        prob_detect_0_as_1: f64,
        prob_detect_1_as_0: f64,
    ) -> Result<Self, RoqoqoBackendError> {
        check_is_probability(&prob_detect_0_as_1)?;
        check_is_probability(&prob_detect_1_as_0)?;
        let prob_detect_0_as_1: HashMap<usize, f64> = (0..number_qubits)
            .map(|i| (i, prob_detect_0_as_1))
            .collect();
        let prob_detect_1_as_0: HashMap<usize, f64> = (0..number_qubits)
            .map(|i| (i, prob_detect_1_as_0))
            .collect();

        Ok(Self {
            prob_detect_0_as_1,
            prob_detect_1_as_0,
        })
    }

    /// Set and overwrite the measurement error probabilities
    ///
    /// # Arguments
    ///
    /// * `qubits` - The qubits for which error rates are set.
    /// * `prob_detect_0_as_1` - The error probability to detect a 1 instead of a 0 when measuring 0
    /// * `prob_detect_1_as_0` - The error probability to detect a 0 instead of a 1 when measuring 1
    ///
    /// # Returns
    ///
    /// * `Ok(ImperfectReadoutModel)` - The new error model
    /// * `Err(RoqoqoError)` - Probabilities are not valid (< 0 or > 1)
    pub fn set_error_probabilites(
        mut self,
        qubit: usize,
        prob_detect_0_as_1: f64,
        prob_detect_1_as_0: f64,
    ) -> Result<Self, RoqoqoBackendError> {
        check_is_probability(&prob_detect_0_as_1)?;
        check_is_probability(&prob_detect_1_as_0)?;
        self.prob_detect_0_as_1.insert(qubit, prob_detect_0_as_1);
        self.prob_detect_1_as_0.insert(qubit, prob_detect_1_as_0);

        Ok(self)
    }

    /// Return probability to detect 0 as 1 for a qubit
    ///
    /// # Arguments
    ///
    /// `qubit` - The qubit for which the probability is returned.
    ///
    /// # Returns
    ///
    /// `f64` - The probability to detect 0 as 1 for the qubit
    pub fn prob_detect_0_as_1(&self, qubit: &usize) -> f64 {
        *self.prob_detect_0_as_1.get(qubit).unwrap_or(&0.0)
    }

    /// Return probability to detect 1 as 0 for a qubit
    ///
    /// # Arguments
    ///
    /// `qubit` - The qubit for which the probability is returned.
    ///
    /// # Returns
    ///
    /// `f64` - The probability to detect 1 as 0 for the qubit
    pub fn prob_detect_1_as_0(&self, qubit: &usize) -> f64 {
        *self.prob_detect_1_as_0.get(qubit).unwrap_or(&0.0)
    }
}

impl SupportedVersion for ImperfectReadoutModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// Checks that probability is in range 0.0 to 1.0
fn check_is_probability(prob: &f64) -> Result<(), RoqoqoError> {
    if prob < &0.0 {
        Err(RoqoqoError::GenericError {
            msg: format!("Probabilities need to be > 0, {} is not > 0", prob),
        })
    } else if prob > &1.0 {
        Err(RoqoqoError::GenericError {
            msg: format!("Probabilities need to be < 1, {} is not < 1", prob),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "json_schema")]
    use jsonschema::Validator;

    #[test]
    fn test_check_is_probability_valid() {
        assert!(check_is_probability(&0.5).is_ok());
        assert!(check_is_probability(&0.1).is_ok());
        assert!(check_is_probability(&0.9).is_ok());
    }

    #[test]
    fn test_check_is_probability_out_of_range() {
        assert!(check_is_probability(&-0.1).is_err());
        assert!(check_is_probability(&1.1).is_err());
    }
    #[test]
    fn test_imperfect_readout_model_new() {
        let model = ImperfectReadoutModel::new();
        assert_eq!(model.prob_detect_0_as_1, HashMap::<usize, f64>::new());
        assert_eq!(model.prob_detect_1_as_0, HashMap::<usize, f64>::new());
    }

    #[test]
    fn test_imperfect_readout_model_new_with_uniform_error() {
        let model = ImperfectReadoutModel::new_with_uniform_error(2, 0.2, 0.8).unwrap();
        assert_eq!(
            model.clone().prob_detect_0_as_1,
            [(0, 0.2), (1, 0.2)].iter().cloned().collect()
        );
        assert_eq!(
            model.prob_detect_1_as_0,
            [(0, 0.8), (1, 0.8)].iter().cloned().collect()
        );
    }

    #[test]
    fn test_imperfect_readout_model_set_error_probabilites() {
        let mut model = ImperfectReadoutModel::new();
        model = model.set_error_probabilites(0, 0.2, 0.8).unwrap();
        model = model.set_error_probabilites(1, 0.3, 0.3).unwrap();
        assert_eq!(
            model.clone().prob_detect_0_as_1,
            [(0, 0.2), (1, 0.3)].iter().cloned().collect()
        );
        assert_eq!(
            model.prob_detect_1_as_0,
            [(0, 0.8), (1, 0.3)].iter().cloned().collect()
        );
    }

    #[test]
    fn test_imperfect_readout_model_prob_detect_0_as_1() {
        let model = ImperfectReadoutModel::new_with_uniform_error(2, 0.2, 0.8).unwrap();
        assert_eq!(model.prob_detect_0_as_1(&0), 0.2);
        assert_eq!(model.prob_detect_0_as_1(&0), 0.2);
        assert_eq!(model.prob_detect_0_as_1(&4), 0.0);
    }

    #[test]
    fn test_imperfect_readout_model_prob_detect_1_as_0() {
        let model = ImperfectReadoutModel::new_with_uniform_error(2, 0.2, 0.8).unwrap();
        assert_eq!(model.prob_detect_1_as_0(&0), 0.8);
        assert_eq!(model.prob_detect_1_as_0(&1), 0.8);
        assert_eq!(model.prob_detect_1_as_0(&4), 0.0);
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_feature() {
        let model = ImperfectReadoutModel::new_with_uniform_error(2, 0.2, 0.8).unwrap();
        let schema = schemars::schema_for!(ImperfectReadoutModel);
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
