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

/// The quantum Rabi interaction exp(-i * θ * X * (b^{dagger} + b))
#[wrap(
    Operate,
    OperateModeGate,
    Substitute,
    SubstituteModes,
    InvolveModes,
    OperateSingleMode,
    InvolveQubits,
    OperateSingleModeGate,
    JsonSchema
)]
pub struct QuantumRabi {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}

/// Longitudinal coupling gate exp(-i * θ * Z * (b^{dagger} + b))
#[wrap(
    Operate,
    OperateModeGate,
    Substitute,
    SubstituteModes,
    InvolveModes,
    OperateSingleMode,
    InvolveQubits,
    OperateSingleModeGate,
    JsonSchema
)]
pub struct LongitudinalCoupling {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}

/// The Jaynes-Cummings gate exp(-i * θ * (σ^- * b^{dagger} + σ^+ * b))
#[wrap(
    Operate,
    OperateModeGate,
    Substitute,
    SubstituteModes,
    InvolveModes,
    OperateSingleMode,
    InvolveQubits,
    OperateSingleModeGate,
    JsonSchema
)]
pub struct JaynesCummings {
    qubit: usize,
    mode: usize,
    theta: CalculatorFloat,
}
