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

use super::*;
use ndarray::Array1;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Collected information for executing a cheated measurement of a PauliZ product.
#[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CheatedPauliZProduct {
    /// Constant Circuit that is executed before each Circuit in circuits.
    pub constant_circuit: Option<Circuit>,
    /// Collection of quantum circuits for the separate basis rotations.
    pub circuits: Vec<Circuit>,
    /// Additional input information required for measurement.
    pub input: CheatedPauliZProductInput,
}

impl Measure for CheatedPauliZProduct {
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
    /// * `Err(RoqoqoError)` - The subsitution failed.
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
            input: self.input.clone(),
        })
    }
}

impl MeasureExpectationValues for CheatedPauliZProduct {
    /// Executes the cheated PauliZ product measurement
    ///
    /// # Arguments
    ///
    /// * `bit_registers` - The classical bit registers as a HashMap with the register name as key
    /// * `float_registers` - The classical float registers as a HashMap with the register name as key
    /// * `complex_registers` - The classical complex registers as a HashMap with the register name as key
    ///
    /// # Returns
    ///
    /// * `Ok(Some(HashMap<String, f64>))` - The measurement has been evaluated successfully. The HashMap contains the measured expectation values
    /// * `Ok(None)` - The measurement did not fail but is incomplete. A new round of measurements is needed
    /// * `Err(RoqoqoError)` - Calculator parsing error.
    ///
    #[allow(unused_variables)]
    fn evaluate(
        &self,
        bit_registers: HashMap<String, BitOutputRegister>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoError> {
        let mut pauli_products: Array1<f64> = Array1::zeros(self.input.pauli_product_keys.len());
        for (register_name, register) in float_registers.iter() {
            if let Some(index) = self.input.pauli_product_keys.get(register_name) {
                pauli_products[*index] = register[0][0];
            } else {
                return Err(RoqoqoError::MissingRegister {
                    name: register_name.clone(),
                });
            }
        }
        // Evaluating expectation values
        let mut results: HashMap<String, f64> = HashMap::new();

        for (name, evaluation) in self.input.measured_exp_vals.iter() {
            results.insert(
                name.clone(),
                match evaluation {
                    PauliProductsToExpVal::Linear(hm) => {
                        let mut value: f64 = 0.0;
                        for (index, coefficient) in hm {
                            value += pauli_products[*index] * coefficient;
                        }
                        value
                    }
                    PauliProductsToExpVal::Symbolic(x) => {
                        let mut calculator = qoqo_calculator::Calculator::new();
                        for (ind, p) in pauli_products.iter().enumerate() {
                            calculator.set_variable(format!("pauli_product_{}", ind).as_str(), *p);
                        }
                        calculator.parse_get(x.clone())?
                    }
                },
            );
        }

        Ok(Some(results))
    }
}

impl crate::operations::SupportedVersion for CheatedPauliZProduct {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        let mut current_minimum_version = (1, 0, 0);
        let comparison_version = self.input.minimum_supported_roqoqo_version();
        crate::update_roqoqo_version(&mut current_minimum_version, comparison_version);
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
