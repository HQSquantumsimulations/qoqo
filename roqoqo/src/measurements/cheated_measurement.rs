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

use crate::measurements::{CheatedInput, Measure, MeasureExpectationValues};
use crate::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use crate::Circuit;
use crate::RoqoqoError;
use ndarray::{Array1, ArrayView1, ArrayView2};
//use ndarray::{Array2};
use num_complex::Complex64;
//use sprs::{CsMat, TriMat};
use std::collections::HashMap;

/// Cheated measurement using state obtained from simulator backend.
///
/// Cheated measurements are only possible witch simulator backends that can return the state vector or the density matrix of the quantum computer.
/// The expectation values are defined by a matrix representation of the measured observables.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Cheated {
    /// Constant Circuit that is executed before each Circuit in circuits.
    pub constant_circuit: Option<Circuit>,
    /// Collection of quantum circuits for the separate basis rotations.
    pub circuits: Vec<Circuit>,
    /// Additional input information required for measurement.
    pub input: CheatedInput,
}

impl Measure for Cheated {
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

impl MeasureExpectationValues for Cheated {
    /// Executes the cheated measurement.
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
    /// * `Ok(None)` - The measurement did not fail but is incomplete. A new round of measurements is needed.
    /// * `Err([RoqoqoError::MissingRegister])` - The OutputRegister is missing.
    /// * `Err([RoqoqoError::MismatchedRegisterDimension])` - The dimension of register exceeds Hilbert space dimension of qubits.
    #[allow(unused_variables)]
    fn evaluate(
        &self,
        bit_registers: HashMap<String, BitOutputRegister>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoError> {
        let dimension = 2_usize.pow(self.input.number_qubits as u32);
        // Evaluating expectation values
        let mut results: HashMap<String, f64> = HashMap::new();
        for (name, (operator, readout)) in self.input.measured_operators.iter() {
            // let row_inds: Vec<usize> = operator.iter().map(|(row, _, _)| row).cloned().collect();
            // let col_inds: Vec<usize> = operator.iter().map(|(_, col, _)| col).cloned().collect();
            // let data: Vec<Complex64> = operator.iter().map(|(_, _, val)| val).cloned().collect();
            // let tmp_tri = TriMat::from_triplets((dimension, dimension), row_inds, col_inds, data);
            // let matrix: CsMat<Complex64> = tmp_tri.to_csc();
            let register_vec =
                complex_registers
                    .get(readout)
                    .ok_or_else(|| RoqoqoError::MissingRegister {
                        name: readout.clone(),
                    })?;
            let mut local_results: Array1<f64> = Array1::zeros(register_vec.len());
            for (index, register) in register_vec.iter().enumerate() {
                if register.len() == dimension {
                    let vector: ArrayView1<Complex64> = ArrayView1::<Complex64>::from(register);
                    // let complex_conj_vec: Vec<Complex64> =
                    //     register.iter().map(|x| x.conj()).collect();
                    // let complex_conj: ArrayView1<Complex64> =
                    //     ArrayView1::<Complex64>::from(&complex_conj_vec);
                    // let tmp_val: Complex64 = complex_conj.dot(&(&matrix * &vector));
                    let tmp_val: Complex64 =
                        sparse_matrix_vector_expectation_value(operator, &vector);
                    local_results[index] = tmp_val.re;
                } else if register.len() == dimension * dimension {
                    let vector: ArrayView2<Complex64> =
                        ArrayView2::<Complex64>::from_shape((dimension, dimension), register)
                            .expect("Unexpected error reshaping array");
                    // let complex_conj_vec: Vec<Complex64> =
                    //     register.iter().map(|x| x.conj()).collect();
                    // let complex_conj: Array2<Complex64> = Array2::<Complex64>::from_shape_vec(
                    //     (dimension, dimension),
                    //     complex_conj_vec,
                    // )
                    // .expect("Unexpected error reshaping array");
                    // let tmp_val: Complex64 =
                    //     ((complex_conj.t()).dot(&(&matrix * &vector))).diag().sum();
                    let tmp_val =
                        sparse_matrix_matrix_expectation_value(operator, &vector, dimension);
                    local_results[index] = tmp_val.re;
                } else {
                    return Err(RoqoqoError::MismatchedRegisterDimension {
                        dim: register.len(),
                        number_qubits: self.input.number_qubits,
                    });
                }
            }

            results.insert(
                name.clone(),
                local_results.mean().expect(
                    "Unexpectedly could not calculate mean of expectation values of register",
                ),
            );
        }
        Ok(Some(results))
    }
}

#[inline]
fn sparse_matrix_vector_expectation_value(
    matrix: &[(usize, usize, Complex64)],
    vector: &ArrayView1<Complex64>,
) -> Complex64 {
    let mut val: Complex64 = Complex64::new(0.0, 0.0);
    for (index_i, index_j, value) in matrix {
        val += vector[*index_i].conj() * value * vector[*index_j]
    }
    val
}

#[inline]
fn sparse_matrix_matrix_expectation_value(
    matrix: &[(usize, usize, Complex64)],
    dense_matrix: &ArrayView2<Complex64>,
    dimension: usize,
) -> Complex64 {
    let mut val: Complex64 = Complex64::new(0.0, 0.0);
    for (index_j, index_k, value) in matrix {
        for index_i in 0..dimension {
            val +=
                dense_matrix[(*index_j, index_i)].conj() * value * dense_matrix[(*index_k, index_i)]
        }
    }
    val
}

impl crate::operations::SupportedVersion for Cheated {
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
