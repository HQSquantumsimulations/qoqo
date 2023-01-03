// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::measurements::Measure;
use crate::Circuit;
use crate::RoqoqoError;
use std::collections::HashMap;

/// Classical register measurement.
///
/// Runs a sequence of circuits and returns the classical registers written during circuit execution.
#[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassicalRegister {
    /// Constant Circuit that is executed before each Circuit in circuits.
    pub constant_circuit: Option<Circuit>,
    /// Collection of quantum circuits for the separate basis rotations.
    pub circuits: Vec<Circuit>,
}

impl Measure for ClassicalRegister {
    /// Returns the constant Circuit that is executed before each Circuit in circuits.
    ///
    /// # Returns
    ///
    /// * `&Option<Circuit` - The constant Circuit (None if not defined).
    fn constant_circuit(&self) -> &Option<Circuit> {
        &self.constant_circuit
    }

    /// Returns iterator over circuits for measurement.
    ///
    /// # Returns
    ///
    /// * `Box<dyn Iterator<Item = &'a Circuit> + 'a>` - The quantum circuits.
    fn circuits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Circuit> + 'a> {
        Box::new(self.circuits.iter())
    }

    /// Returns clone of Measurement with symbolic parameters replaced.
    ///
    /// # Arguments
    ///
    /// * `substituted_parameters` - The HashMap containing the substitutions to use in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The Circuits with the parameters substituted.
    /// * `Err(RoqoqoError)` - The substitution failed.
    ///
    fn substitute_parameters(
        &self,
        substituted_parameters: HashMap<String, f64>,
    ) -> Result<Self, RoqoqoError> {
        let mut calculator = qoqo_calculator::Calculator::new();
        for (name, val) in substituted_parameters.iter() {
            calculator.set_variable(name, *val)
        }
        let new_constant_circuit = match &self.constant_circuit {
            None => None,
            Some(c) => Some(c.substitute_parameters(&calculator)?),
        };
        let mut new_circuits = Vec::new();
        for circ in self.circuits.iter() {
            let mut calculator = qoqo_calculator::Calculator::new();
            for (name, val) in substituted_parameters.iter() {
                calculator.set_variable(name, *val)
            }
            new_circuits.push(circ.substitute_parameters(&calculator)?)
        }
        Ok(Self {
            constant_circuit: new_constant_circuit,
            circuits: new_circuits,
        })
    }
}

impl crate::operations::SupportedVersion for ClassicalRegister {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        let mut current_minimum_version = (1, 0, 0);
        if let Some(circuit) = self.constant_circuit() {
            let comparison_version = circuit.minimum_supported_roqoqo_version();
            crate::update_roqoqo_version(&mut current_minimum_version, comparison_version);
        }
        for circuit in self.circuits.iter() {
            let comparison_version = circuit.minimum_supported_roqoqo_version();
            crate::update_roqoqo_version(&mut current_minimum_version, comparison_version);
        }
        current_minimum_version
    }
}
