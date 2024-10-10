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

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateGate, OperateThreeQubitGate, JsonSchema)]
/// Implements ControlledSWAP gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the controlling qubit of the operation.
///     target_0 (int): The index of the second most significant qubit in the unitary representation. Here, the first targeting qubit of the operation.
///     target_1 (int): The index of the least significant qubit in the unitary representation. Here, the second targeting qubit of the operation.
pub struct ControlledSWAP {
    control: usize,
    target_0: usize,
    target_1: usize,
}

#[pymethods]
impl ControlledSWAPWrapper {
    /// Returns control qubit of the three-qubit operation
    pub fn control(&self) -> usize {
        *self.internal.control_0()
    }
    /// Returns target_0 qubit of the three-qubit operation
    pub fn target_0(&self) -> usize {
        *self.internal.control_1()
    }
    /// Returns target_1 qubit of the three-qubit operation
    pub fn target_1(&self) -> usize {
        *self.internal.target()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// The phased-shifted double-controlled-Z gate.
///
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & e^{i \phi} & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & e^{i \phi} & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi + \pi)}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first qubit that controls the application of the phase-shift on the target qubit.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///
pub struct PhaseShiftedControlledControlledZ {
    control_0: usize,
    control_1: usize,
    target: usize,
    phi: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    Rotate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// The phased-shifted double-controlled-Z gate.
///
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & e^{i \phi} & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & e^{i \phi} & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi + \theta)}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first qubit that controls the application of the phase-shift on the target qubit.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///     theta (CalculatorFloat): The phase rotation $\theta$.
///
pub struct PhaseShiftedControlledControlledPhase {
    control_0: usize,
    control_1: usize,
    target: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}
