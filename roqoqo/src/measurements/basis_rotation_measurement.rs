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
use ndarray::{Array1, Array2};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Collected information for executing a measurement of PauliZ product.
#[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PauliZProduct {
    /// Constant Circuit that is executed before each Circuit in circuits.
    pub constant_circuit: Option<Circuit>,
    /// Collection of quantum circuits for the separate basis rotations.
    pub circuits: Vec<Circuit>,
    /// Additional input information required for measurement.
    pub input: PauliZProductInput,
}

impl Measure for PauliZProduct {
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
            input: self.input.clone(),
        })
    }
}

impl MeasureExpectationValues for PauliZProduct {
    // TODO add optional device later for use with flipped measurement
    #[allow(unused_variables)]
    /// Executes the PauliZ product measurement.
    ///
    /// # Arguments
    ///
    /// * `bit_registers` - The classical bit registers as a HashMap with the register name as key.
    /// * `float_registers` - The classical float registers as a HashMap with the register name as key.
    /// * `complex_registers` - The classical complex registers as a HashMap with the register name as key.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(HashMap<String, f64>))` - The measurement has been evaluated successfully. The HashMap contains the measured expectation values.
    /// * `Ok(None)` - The measurement did not fail but is incomplete. A new round of measurements is needed
    /// * `Err([RoqoqoError::PauliZProductMeasurementError])` - An error occured in PauliZ product measurement.
    ///
    fn evaluate(
        &self,
        bit_registers: HashMap<String, BitOutputRegister>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoError> {
        // todo replace with actual input
        let measurement_fidelities = vec![1.0; self.input.number_qubits];

        // Setting up measurement correction factors for flipped measurement
        let mut measurement_correction_factors: HashMap<String, Vec<f64>> = HashMap::new();
        let flipped_and_extension: Vec<(bool, &'static str)>;
        if self.input.use_flipped_measurement {
            // helper vector to iterate over when evaluating the pauli products
            flipped_and_extension = vec![(false, ""), (true, "_flipped")];
            for (name, pauli_product_mask) in self.input.pauli_product_qubit_masks.iter() {
                let mut measurement_correction_factor: Vec<f64> =
                    (0..self.input.number_pauli_products)
                        .into_iter()
                        .map(|_| 1.0)
                        .collect();
                for (pp_index, indices) in pauli_product_mask.iter() {
                    if !indices.is_empty() {
                        for i in indices {
                            measurement_correction_factor[*pp_index] *= measurement_fidelities[*i]
                        }
                    }
                }
                measurement_correction_factors.insert(name.clone(), measurement_correction_factor);
            }
        } else {
            flipped_and_extension = vec![(false, "")];
        }
        let mut pauli_product_dict: HashMap<String, Array1<f64>> = HashMap::new();
        for (register_name, mask) in self.input.pauli_product_qubit_masks.iter() {
            for (flip_measurement, extension) in flipped_and_extension.iter() {
                let register = bit_registers
                    .get(&format!("{}{}", register_name.as_str(), extension))
                    .ok_or(RoqoqoError::PauliZProductMeasurementError {
                        msg: format!(
                            "bit register {}{} not found",
                            register_name.as_str(),
                            extension
                        ),
                    })?;
                let mut single_shot_pauli_products: Array2<f64> =
                    Array2::zeros((register.len(), self.input.number_pauli_products));
                for (index, mask_val) in mask.iter() {
                    if mask_val.is_empty() {
                        single_shot_pauli_products.column_mut(*index).fill(1.0);
                    } else {
                        // Accessing column of single_shot_pauli_products that corresponds to pauli product designated by index
                        let mut column = single_shot_pauli_products.column_mut(*index);
                        // Iterate over all single shot readouts for all qubits and construct Pauli Product
                        for (row_index, values) in register.iter().enumerate() {
                            // Determine the value of the pauli product with the parity of the number of 0 and 1 measurements of the qubits
                            // false is even parity and true is odd parity
                            let mut parity = false;
                            for i in mask_val.iter() {
                                // For flipped readout a false (0) qubit measurement will flip the parity
                                // For a not-flipped measurement a true (1) qubit measurement will flip the parity
                                if values[*i] ^ flip_measurement {
                                    parity = !parity
                                }
                            } // Map even parity measurement result to 1 and odd parity result to -1
                            column[row_index] = match parity {
                                false => 1.0,
                                true => -1.0,
                            };
                        }
                    }
                }
                let mut pauli_products_tmp: Array1<f64> =
                    Array1::zeros(self.input.number_pauli_products);
                for i in 0..self.input.number_pauli_products {
                    pauli_products_tmp[i] = single_shot_pauli_products.column(i).mean().ok_or(
                        RoqoqoError::PauliZProductMeasurementError {
                            msg: format!(
                                "Column {} out of index for sinlge_shot_pauli_products",
                                i
                            ),
                        },
                    )?;
                }
                pauli_product_dict.insert(
                    format!("{}{}", register_name.as_str(), extension),
                    pauli_products_tmp,
                );
            }
        }

        let mut pauli_products: Array1<f64> = Array1::zeros(self.input.number_pauli_products);
        for (register_name, _) in self.input.pauli_product_qubit_masks.iter() {
            if !register_name.ends_with("flipped") {
                // Create temporary averaged vector of pauli_products
                // Averaging between normal and flipped readout when flipped measurement is used
                if self.input.use_flipped_measurement {
                    let tmp_pauli_products = (&pauli_product_dict
                        .get(register_name.as_str())
                        .ok_or(RoqoqoError::PauliZProductMeasurementError {
                            msg: format!("Register name {} not fount", register_name),
                        })?
                        .view()
                        + &pauli_product_dict
                            .get(format!("{}_flipped", register_name).as_str())
                            .ok_or(RoqoqoError::PauliZProductMeasurementError {
                                msg: format!("Register name {}_flipped not fount", register_name),
                            })?
                            .view())
                        / 2.0;
                    // reinserting in dict of pauli products
                    pauli_products += &tmp_pauli_products.view();
                } else {
                    pauli_products += &pauli_product_dict
                        .get(register_name.as_str())
                        .ok_or(RoqoqoError::PauliZProductMeasurementError {
                            msg: format!("Register name {} not fount", register_name),
                        })?
                        .view()
                }
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

impl crate::operations::SupportedVersion for PauliZProduct {
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
