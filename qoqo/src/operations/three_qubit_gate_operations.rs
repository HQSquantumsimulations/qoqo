// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use num_complex::Complex64;
use numpy::{PyArray2, ToPyArray};

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;

use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};

use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use std::collections::HashMap;

use crate::CircuitWrapper;

use qoqo_macros::*;

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
#[derive(Eq)]
/// Implements the double-controlled PauliZ gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & -1
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
pub struct ControlledControlledPauliZ {
    control_0: usize,
    control_1: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    Rotate,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// Implements the double-controlled PhaseShift gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i \theta}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
///     theta (float): The rotation angle θ.
pub struct ControlledControlledPhaseShift {
    control_0: usize,
    control_1: usize,
    target: usize,
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// Implements Toffoli gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the PauliX gate is applied to.
pub struct Toffoli {
    control_0: usize,
    control_1: usize,
    target: usize,
}
