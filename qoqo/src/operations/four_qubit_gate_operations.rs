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

use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};

use crate::CircuitWrapper;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;

use std::collections::HashMap;

#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use roqoqo::operations::*;

use qoqo_macros::*;

#[wrap(
    Operate,
    OperateFourQubit,
    OperateGate,
    OperateFourQubitGate,
    JsonSchema
)]
/// The triple-controlled PauliX gate.
///
pub struct TripleControlledPauliX {
    control_0: usize,
    control_1: usize,
    control_2: usize,
    target: usize,
}

#[wrap(
    Operate,
    OperateFourQubit,
    OperateGate,
    OperateFourQubitGate,
    JsonSchema
)]
/// The triple-controlled PauliZ gate.
///
pub struct TripleControlledPauliZ {
    control_0: usize,
    control_1: usize,
    control_2: usize,
    target: usize,
}

#[wrap(
    Operate,
    OperateFourQubit,
    OperateGate,
    OperateFourQubitGate,
    JsonSchema
)]
/// The triple-controlled PhaseShift gate.
///
pub struct TripleControlledPhaseShift {
    control_0: usize,
    control_1: usize,
    control_2: usize,
    target: usize,
    theta: CalculatorFloat,
}
