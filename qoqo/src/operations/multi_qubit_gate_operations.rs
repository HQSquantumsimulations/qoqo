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

use crate::CircuitWrapper;
use num_complex::Complex64;
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::convert_into_calculator_float;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use qoqo_macros::*;
use roqoqo::operations::*;
use std::collections::HashMap;

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, Rotate, OperateMultiQubit, OperateGate, OperateMultiQubitGate)]
/// The Molmer-Sorensen gate between multiple qubits.
///
/// The gate applies the rotation under the product of Pauli X operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * theta/2 * X_i0 * X_i1 * ... * X_in).
pub struct MultiQubitMS {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, Rotate, OperateMultiQubit, OperateGate, OperateMultiQubitGate)]
/// The multi qubit Pauli-Z-Product gate.
///
/// The gate applies the rotation under the product of Pauli Z operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * theta/2 * Z_i0 * Z_i1 * ... * Z_in).
pub struct MultiQubitZZ {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}
