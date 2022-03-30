// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Qoqo measurement inputs

use num_complex::Complex64;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use roqoqo::measurements::{
    CheatedInput, CheatedPauliZProductInput, PauliProductMask, PauliZProductInput,
};
use std::collections::HashMap;

#[pyclass(name = "PauliZProductInput", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Provides Necessary Information to run a [roqoqo::measurements::PauliZProduct] measurement.
pub struct PauliZProductInputWrapper {
    /// Internal storage of [roqoqo::PauliZProductInput].
    pub internal: PauliZProductInput,
}

#[pymethods]
impl PauliZProductInputWrapper {
    /// Create new PauliZProductInput.
    ///
    /// The PauliZProductInput starts with just the number of qubtis and flipped measurements set.
    /// The pauli_poduct_qubit_masks and measured_exp_vals start empty
    /// and can be extended with [PauliZProductInput::add_pauliz_product]
    /// [PauliZProductInput::add_linear_exp_val] and [PauliZProductInput::add_symbolic_exp_val]
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits in the PauliZProduct measurement.
    ///     use_flipped_measurement (bool): Whether or not to use flipped measurements.
    ///
    /// Returns:
    ///     self: The new instance of PauliZProductInput with pauli_product_qubit_masks = an empty dictionary, the
    ///           specified number of qubits in input, number_pauli_products = 0, measured_exp_vals = an empty
    ///           dictionary, and whether to use flipped measurements as specified in input.
    #[new]
    pub fn new(number_qubits: usize, use_flipped_measurement: bool) -> Self {
        Self {
            internal: PauliZProductInput::new(number_qubits, use_flipped_measurement),
        }
    }

    /// Add measured Pauli product to PauliZProductInput and returns index of Pauli product.
    ///
    /// When the pauli product is already in the measurement input the function only returns
    /// it index.
    ///
    /// Args:
    ///     readout (str): The name of the readout register the pauli_product is defined on.
    ///     pauli_product_mask (list[int]): List of the qubits involved in the Pauli produc measurement.
    ///
    /// Returns:
    ///     int: The index of the added Pauli product in the list of all Pauli products.
    ///
    /// Raises:
    ///     RuntimeError: Failed to add pauli product.
    pub fn add_pauliz_product(
        &mut self,
        readout: String,
        pauli_product_mask: PauliProductMask,
    ) -> PyResult<usize> {
        self.internal
            .add_pauliz_product(readout, pauli_product_mask)
            .map_err(|_| PyRuntimeError::new_err("Failed to add pauli product"))
    }

    /// Add linear definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a linear combination
    /// of expectation values of Pauli products.
    ///
    /// Args:
    ///     name (str): The name of the expectation value.
    ///     linear (dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.
    ///
    /// Raises:
    ///     RuntimeError: Failed to add linear expectation value.
    pub fn add_linear_exp_val(
        &mut self,
        name: String,
        linear: HashMap<usize, f64>,
    ) -> PyResult<()> {
        self.internal.add_linear_exp_val(name, linear).map_err(|x| {
            PyRuntimeError::new_err(format!("Failed to add linear expectation value {:?}", x))
        })
    }

    /// Add symbolic definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a symbolic combination
    /// of expectation values of Pauli products.
    ///
    /// Args:
    ///     name (str): The name of the expectation value.
    ///     symbolic (str): The symbolic expression for the expectation values
    ///                     given by [qoqo_calculator::CalculatorFloat].
    ///
    /// Raises:
    ///     RuntimeError: Failed to add symbolic expectation value.
    ///
    /// The i-th PauliProducts are hardcoded as variables pauli_product_i
    /// in the string expression of CalculatorFloat.
    pub fn add_symbolic_exp_val(&mut self, name: String, symbolic: String) -> PyResult<()> {
        self.internal
            .add_symbolic_exp_val(name, symbolic.into())
            .map_err(|x| {
                PyRuntimeError::new_err(format!("Failed to add symbolic expectation value {:?}", x))
            })
    }
}

#[pyclass(name = "CheatedPauliZProductInput", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Collected information for executing a cheated basis rotation measurement.
pub struct CheatedPauliZProductInputWrapper {
    /// Internal storage of [roqoqo::CheatedPauliZProductInput].
    pub internal: CheatedPauliZProductInput,
}

impl Default for CheatedPauliZProductInputWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl CheatedPauliZProductInputWrapper {
    /// Create new CheatedPauliZProductInput.
    ///
    /// The CheatedPauliZProductInput starts with just the number of qubtis and flipped measurements set.
    /// The pauli_poduct_qubit_masks and measured_exp_vals start empty
    /// and can be extended with [CheatedPauliZProductInput::add_linear_exp_val] and
    /// [CheatedPauliZProductInput::add_symbolic_exp_val].
    ///
    /// Returns:
    ///     self: The new instance of PauliZProductInput with pauli_product_qubit_masks = an empty dictionary, the
    ///           specified number of qubits in input, number_pauli_products = 0, measured_exp_vals = an empty
    ///           dictionary, and whether to use flipped measurements as specified in input.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: CheatedPauliZProductInput::new(),
        }
    }

    /// Add measured Pauli product to CheatedPauliZProductInput and returns index of Pauli product.
    ///
    /// When the pauli product is already in the measurement input the function only returns
    /// its index.
    ///
    /// Args:
    ///     readout (str): The name of the readout register containing the pauli_product expectation value.
    ///
    /// Returns:
    ///     int: The index of the added Pauli product in the list of all Pauli products.
    pub fn add_pauliz_product(&mut self, readout: String) -> usize {
        self.internal.add_pauliz_product(readout)
    }

    /// Add linear definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a linear combination
    /// of expectation values of Pauli products.
    ///
    /// Args:
    ///     name (str): The name of the expectation value.
    ///     linear (dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.
    ///
    /// Raises:
    ///     RuntimeError: Failed to add linear expectation value.
    pub fn add_linear_exp_val(
        &mut self,
        name: String,
        linear: HashMap<usize, f64>,
    ) -> PyResult<()> {
        self.internal.add_linear_exp_val(name, linear).map_err(|x| {
            PyRuntimeError::new_err(format!("Failed to add linear expectation value {:?}", x))
        })
    }

    /// Add symbolic definition of expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by a symbolic combination
    /// of expectation values of Pauli products.
    ///
    /// Args:
    ///     name (str): The name of the expectation value.
    ///     symbolic (str): The symbolic expression for the expectation values
    ///                     given by [qoqo_calculator::CalculatorFloat].
    ///
    /// Raises:
    ///     RuntimeError: Failed to add symbolic expectation value.
    ///
    /// The i-th PauliProducts are hardcoded as variables pauli_product_i
    /// in the string expression of CalculatorFloat.
    ///
    pub fn add_symbolic_exp_val(&mut self, name: String, symbolic: String) -> PyResult<()> {
        self.internal
            .add_symbolic_exp_val(name, symbolic.into())
            .map_err(|x| {
                PyRuntimeError::new_err(format!("Failed to add symbolic expectation value {:?}", x))
            })
    }
}

#[pyclass(name = "CheatedInput", module = "qoqo.measurements")]
#[derive(Clone, Debug)]
/// Provides Necessary Information to run a cheated measurement.
pub struct CheatedInputWrapper {
    /// Internal storage of [roqoqo::CheatedInput].
    pub internal: CheatedInput,
}

#[pymethods]
impl CheatedInputWrapper {
    /// Creates a new CheatedInput.
    ///
    /// The CheatedInput stores the number of qubits that are measured
    /// and a dictionary mapping expectation value names to operators on the Hilbert space
    /// of the qubits. The operators are represented by sparse lists of non-zero entry triples
    /// of an operator matrix.
    ///
    /// Args:
    ///     number_qubits (int): The number of qubits in the PauliZProduct measurement.
    ///
    /// Returns:
    ///     CheatedInput: The new instance of CheatedInput with the specified number of qubits in input,
    ///                   and an empty dictionay of expectation values.
    #[new]
    pub fn new(number_qubits: usize) -> Self {
        Self {
            internal: CheatedInput::new(number_qubits),
        }
    }

    /// Add operator based expectation value to measurement input.
    ///
    /// Adds an expectation value that is defined by an operator on the Hilbert space.
    ///
    /// Args:
    ///     name (str): The name of the expectation value.
    ///     operator (list[(int, int, complex)]): The measured operator on the Hilbert space,
    ///                                           given as a list of sparse matrix entries of the form (row, col, value).
    ///     readout (str): The mame of the readout register that contains the density matrix or satevector.
    ///
    /// Raises:
    ///     RuntimeError: Failed to add operator based expectation value.
    pub fn add_operator_exp_val(
        &mut self,
        name: String,
        operator: Vec<(usize, usize, Complex64)>,
        readout: String,
    ) -> PyResult<()> {
        self.internal
            .add_operator_exp_val(name, operator, readout)
            .map_err(|x| {
                PyRuntimeError::new_err(format!(
                    "Failed to add operator based expectation value {:?}",
                    x
                ))
            })
    }
}
