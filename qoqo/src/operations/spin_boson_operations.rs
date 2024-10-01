// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};
use qoqo_macros::*;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::HashMap;

/// The quantum Rabi interaction exp(-i * θ * X * (b^† + b))
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
///     theta (CalculatorFloat): The strength of the interaction.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct QuantumRabi {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}

/// Longitudinal coupling gate exp(-i * θ * Z * (b^† + b))
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
///     theta (CalculatorFloat): The strength of the interaction.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct LongitudinalCoupling {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}

/// The Jaynes-Cummings gate exp(-i * θ * (σ^- * b^† + σ^+ * b))
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
///     theta (CalculatorFloat): The strength of the interaction.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct JaynesCummings {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}

/// Loads a single excitation from a bosonic mode into a qubit as follows
/// (c1 |0⟩_B + c2 |1⟩_B) ⨂ |0⟩_Q -> |0⟩_B ⨂ (c1 |0⟩_Q + c2 |1⟩_Q)
///
/// Note: if the initial qubit state is |1⟩_Q the operation is only defined if c2 = 0
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct SingleExcitationLoad {
    qubit: usize,
    mode: usize,
}

/// Stores a single excitation from the involved qubit into the involved bosonic mode as follows
/// |0⟩_B ⨂ (a |0⟩_Q + b |1⟩_Q) -> (a|0⟩_B + b |1⟩_B ) ⨂ |0⟩_Q
///
/// Note: not defined if the bosonic mode is in a state |n⟩ with n != 0
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct SingleExcitationStore {
    qubit: usize,
    mode: usize,
}

/// Controlled-Z operation between a qubit and a bosonic mode, where the two-dimensional subspace of
/// the bosonic mode spanned by the occupation number states |0⟩_B and |1⟩_B is considered
/// as the second qubit involved in the CZ operation.
///
/// Args:
///     qubit (int): The qubit the gate is applied to.
///     mode (int): The mode the gate is applied to.
#[wrap(
    Operate,
    Substitute,
    OperateSingleMode,
    SubstituteModes,
    InvolveModes,
    OperateSingleQubit,
    InvolveQubits,
    JsonSchema
)]
pub struct CZQubitResonator {
    qubit: usize,
    mode: usize,
}
