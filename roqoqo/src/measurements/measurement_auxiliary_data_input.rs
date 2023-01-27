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

use crate::CalculatorFloat;
use crate::RoqoqoError;
use num_complex::Complex64;
use std::collections::HashMap;

/// Provides PauliProductMasks for all Pauli Products measured from one readout register.
pub type SingleReadoutPauliProductMasks = HashMap<usize, PauliProductMask>;

/// Provides Mask for a single PauliProduct.
pub type PauliProductMask = Vec<usize>;

/// Defines how Pauli Products expectation values are post-processed into observable expectation value.
#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum PauliProductsToExpVal {
    /// Expectation value of observable is a linear combination of Pauli Product expectation values.
    ///
    /// Only scalar real expectation values are supported.  
    /// For complex observables or vector/matrix observables
    /// components have to be postprocessed separately.
    Linear(HashMap<usize, f64>),
    /// Expectation value of observable is derived from symbolic expression.
    ///
    /// Symbolic expression is given by [qoqo_calculator::CalculatorFloat]
    /// The i-th PauliProduct us hardcoded as the variable `pauli_product_i`
    /// in the string expression of CalculatorFloat.
    Symbolic(CalculatorFloat),
}

/// Provides Necessary Information to run a [crate::measurements::PauliZProduct] measurement.
///
/// PauliZProductInput is the input struct for a PauliZProduct measurement, dictating which expectation
/// values are measured by PauliZProduct. These expecation values are defined as
/// expectation values of pauli products.
#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PauliZProductInput {
    /// Collection of PauliProductMasks for each readout register in Measurement.
    pub pauli_product_qubit_masks: HashMap<String, SingleReadoutPauliProductMasks>,
    /// Number of qubits that are measured.
    pub number_qubits: usize,
    /// Number of Pauli Products that are measured.
    pub number_pauli_products: usize,
    /// Collection of names and construction methods of  expectation values.
    ///
    /// The construction methods are given by [PauliProductsToExpVal] enums.
    pub measured_exp_vals: HashMap<String, PauliProductsToExpVal>,
    /// Determines whether measurement errors are symmetrized.
    ///
    /// Measurement errors are symmetrized by repeating measurement with final flip of all qubits.
    pub use_flipped_measurement: bool,
}

impl PauliZProductInput {
    /// Creates new PauliZProductInput.
    ///
    /// The PauliZProductInput starts with just the number of qubtis and flipped measurements set.
    /// The pauli_product_qubit_masks and measured_exp_vals start empty
    /// and can be extended with [PauliZProductInput::add_pauliz_product],
    /// [PauliZProductInput::add_linear_exp_val] and [PauliZProductInput::add_symbolic_exp_val].
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the PauliZProduct measurement.
    /// * `use_flipped_measurement` - Whether or not to use flipped measurements.
    ///
    pub fn new(number_qubits: usize, use_flipped_measurement: bool) -> Self {
        Self {
            pauli_product_qubit_masks: HashMap::new(),
            number_qubits,
            number_pauli_products: 0,
            measured_exp_vals: HashMap::new(),
            use_flipped_measurement,
        }
    }

    /// Adds measured Pauli product to PauliZProductInput and returns index of Pauli product.
    ///
    /// When the pauli product is already in the measurement input the function only returns
    /// it index.
    ///
    /// # Arguments
    ///
    /// * `readout` - The name of the readout register the pauli_product is defined on.
    /// * `pauli_product_mask` - The list of the qubits involved in the Pauli product measurement.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The index of the added Pauli product in the list of all Pauli products.
    /// * `Err([RoqoqoError::PauliProductExceedsQubits])` - The pauli product involves a qubit exceeding the maximum number of qubits.
    pub fn add_pauliz_product(
        &mut self,
        readout: String,
        pauli_product_mask: PauliProductMask,
    ) -> Result<usize, RoqoqoError> {
        if let Some(i) = &pauli_product_mask
            .iter()
            .find(|i| i >= &&self.number_qubits)
        {
            return Err(RoqoqoError::PauliProductExceedsQubits {
                pp_qubit: **i,
                number_qubits: self.number_qubits,
            });
        }

        // Readout already in pauli_product_qubit_masks
        if let Some(m) = self.pauli_product_qubit_masks.get_mut(&readout) {
            // Check if PauliProduct has already been added
            if let Some((k, _)) = m.iter().find(|(_, v)| v == &&pauli_product_mask) {
                return Ok(*k);
            }
            m.insert(self.number_pauli_products, pauli_product_mask);
        } else {
            // Readout not yet in pauli_product_qubit_masks
            let mut new_map = HashMap::new();
            new_map.insert(self.number_pauli_products, pauli_product_mask);
            self.pauli_product_qubit_masks.insert(readout, new_map);
        }
        self.number_pauli_products += 1;

        Ok(self.number_pauli_products - 1)
    }

    /// Adds linear definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a linear combination
    /// of expectation values of Pauli products.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the expectation value.
    /// * `linear` - The linear combination of expectation values as a map between Pauli product index and coefficient.
    ///
    /// # Returns
    ///
    /// * `Ok()` - The value was successfully added.
    /// * `Err([RoqoqoError::ExpValUsedTwice])` - The name of expectation value is already taken.
    pub fn add_linear_exp_val(
        &mut self,
        name: String,
        linear: HashMap<usize, f64>,
    ) -> Result<(), RoqoqoError> {
        if self
            .measured_exp_vals
            .insert(name.clone(), PauliProductsToExpVal::Linear(linear))
            .is_some()
        {
            return Err(RoqoqoError::ExpValUsedTwice { name });
        }
        Ok(())
    }

    /// Adds symbolic definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a symbolic combination
    /// of expectation values of Pauli products.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the expectation value.
    /// * `symbolic` - The symbolic expression for the expectation values
    ///                given by [qoqo_calculator::CalculatorFloat].
    ///
    /// The i-th PauliProduct is hardcoded as variable `pauli_product_i`
    /// in the string expression of CalculatorFloat.
    ///
    /// # Returns
    ///
    /// * `Ok()` - The value was successfully added.
    /// * `Err([RoqoqoError::ExpValUsedTwice])` - The name of expectation value is already taken.
    pub fn add_symbolic_exp_val(
        &mut self,
        name: String,
        symbolic: CalculatorFloat,
    ) -> Result<(), RoqoqoError> {
        if self
            .measured_exp_vals
            .insert(name.clone(), PauliProductsToExpVal::Symbolic(symbolic))
            .is_some()
        {
            return Err(RoqoqoError::ExpValUsedTwice { name });
        }
        Ok(())
    }
}

/// Provides necessary information to run a [crate::measurements::CheatedPauliZProduct] measurement.
///
/// Is used by the full measurement struct [crate::measurements::CheatedPauliZProduct].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct CheatedPauliZProductInput {
    /// Collection of names and construction methods of  expectation values.
    ///
    /// The construction methods are given by [PauliProductsToExpVal] enums.
    pub measured_exp_vals: HashMap<String, PauliProductsToExpVal>,
    /// Mapping the pauli product indices to the readout keys.
    pub pauli_product_keys: HashMap<String, usize>,
}

impl Default for CheatedPauliZProductInput {
    /// Creates a default (here, new) instance of CheatedPauliZProductInput.
    fn default() -> Self {
        Self::new()
    }
}

impl CheatedPauliZProductInput {
    /// Creates new CheatedPauliZProductInput.
    ///
    /// # Returns
    ///
    /// * `Self` - The new instance of CheatedPauliZProductInput with measured_exp_vals = an empty
    ///            HashMap and pauli_product_keys = an empty HashMap.
    pub fn new() -> Self {
        Self {
            measured_exp_vals: HashMap::new(),
            pauli_product_keys: HashMap::new(),
        }
    }

    /// Adds measured Pauli product to CheatedPauliZProductInput and returns index of Pauli product.
    ///
    /// When the pauli product is already in the measurement input the function only returns
    /// it index.
    ///
    /// # Arguments
    ///
    /// * `readout` - The name of the readout register containing the the pauli_product expecation value.
    ///
    /// # Returns
    ///
    /// * `usize` - The index of the added Pauli product in the list of all Pauli products.
    pub fn add_pauliz_product(&mut self, readout: String) -> usize {
        if let Some((_, v)) = self.pauli_product_keys.iter().find(|(k, _)| k == &&readout) {
            return *v;
        }
        self.pauli_product_keys
            .insert(readout, self.pauli_product_keys.len());
        self.pauli_product_keys.len() - 1
    }

    /// Adds linear definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a linear combination
    /// of expectation values of Pauli products.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the expectation value.
    /// * `linear` - The linear combination of expectation values as a map between Pauli product index and coefficient.
    ///
    /// # Returns
    ///
    /// * `Ok()` - The value was successfully added.
    /// * `Err([RoqoqoError::ExpValUsedTwice])` - The name of expectation value already taken.
    pub fn add_linear_exp_val(
        &mut self,
        name: String,
        linear: HashMap<usize, f64>,
    ) -> Result<(), RoqoqoError> {
        if self
            .measured_exp_vals
            .insert(name.clone(), PauliProductsToExpVal::Linear(linear))
            .is_some()
        {
            return Err(RoqoqoError::ExpValUsedTwice { name });
        }
        Ok(())
    }

    /// Adds linear definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a linear combination
    /// of expectation values of Pauli products.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the expectation value.
    /// * `symbolic` - The symbolic expression for the expectation values
    ///                given by [qoqo_calculator::CalculatorFloat].
    ///
    /// The i-th PauliProducts are hardcoded as variables pauli_product_i
    /// in the string expression of CalculatorFloat.
    ///
    /// # Returns
    ///
    /// * `Ok()` - The value was successfully added.
    /// * `Err([RoqoqoError::ExpValUsedTwice])` - The name of expectation value already taken.
    pub fn add_symbolic_exp_val(
        &mut self,
        name: String,
        symbolic: CalculatorFloat,
    ) -> Result<(), RoqoqoError> {
        if self
            .measured_exp_vals
            .insert(name.clone(), PauliProductsToExpVal::Symbolic(symbolic))
            .is_some()
        {
            return Err(RoqoqoError::ExpValUsedTwice { name });
        }
        Ok(())
    }
}

/// Provides necessary information to run a [crate::measurements::Cheated] measurement.
///
/// Is used by the full measurement struct [crate::measurements::Cheated].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct CheatedInput {
    /// Map of expectation values and corresponding operator Matrices on the Hilbert Space.
    pub measured_operators: HashMap<String, (OperatorSparseVec, String)>,
    /// Number of qubits that defines the dimension of the Hilbertspace.
    pub number_qubits: usize,
}

/// Represents Operator acting on Hilbert space as a sparse list of two indices and a value.
///
/// The vector contains the triplets of non-zero elements of the sparse matrix
/// representing the operator on the Hilbert space.
/// The first element is the row index, the second the column index and the last the
/// complex value of the non-zero entry.
pub type OperatorSparseVec = Vec<(usize, usize, Complex64)>;

impl CheatedInput {
    /// Creates new CheatedInput.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the Cheated measurement.
    ///
    /// # Returns
    ///
    /// * `Self` - The new instance of CheatedInput with measured_operators = an empty HashMap and the
    ///            specified number of qubits in input.
    pub fn new(number_qubits: usize) -> Self {
        Self {
            measured_operators: HashMap::new(),
            number_qubits,
        }
    }

    /// Adds expectation value of an operator to measurement input.
    ///
    /// Adds an expectation value of a quantum operator defined by a complex matrix.
    ///
    /// # Arguments
    ///
    /// * `name` - The name. of the expectation value
    /// * `operator` - The measured operator on the Hilbert space given as a list of sparse matrix entries of the form (row, col, value).
    /// * `readout` - The name of the readout register that contains the density matrix or satevector.
    ///
    /// # Returns
    ///
    /// * `Ok()` - The operator was successfully added.
    /// * `Err([RoqoqoError::MismatchedOperatorDimension])` - The index of operator exceeds Hilbert space dimension of qubits.
    /// * `Err([RoqoqoError::ExpValUsedTwice])` - The name of expectation value already taken.
    pub fn add_operator_exp_val(
        &mut self,
        name: String,
        operator: OperatorSparseVec,
        readout: String,
    ) -> Result<(), RoqoqoError> {
        let dimension = 2_usize.pow(self.number_qubits as u32);
        if let Some((x, y, _)) = operator
            .iter()
            .find(|(x, y, _)| x >= &dimension || y >= &dimension)
        {
            return Err(RoqoqoError::MismatchedOperatorDimension {
                index: (*x, *y),
                number_qubits: self.number_qubits,
            });
        }
        if self
            .measured_operators
            .insert(name.clone(), (operator, readout))
            .is_some()
        {
            return Err(RoqoqoError::ExpValUsedTwice { name });
        }
        Ok(())
    }
}

impl crate::operations::SupportedVersion for CheatedInput {}

impl crate::operations::SupportedVersion for CheatedPauliZProductInput {}

impl crate::operations::SupportedVersion for PauliZProductInput {}
