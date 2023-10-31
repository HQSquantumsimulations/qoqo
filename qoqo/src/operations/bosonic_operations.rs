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
/// The single-mode squeezing gate with tunable squeezing.
///
/// The squeezing gate is a quantum operation that allows for precise manipulation of quantum states,
/// by reducing the uncertainty in one variable and therefore increasing the uncertainty of another.
/// https://arxiv.org/pdf/quant-ph/0106157.pdf
///
/// Args:
///     mode (int): The mode the squeezing gate is applied to.
///     squeezing (CalculatorFloat): The coefficient of the squeezing operation.
///     phase (CalculatorFloat): The squeezing phase angle of the squeezing operation.
pub struct Squeezing {
    mode: usize,
    squeezing: CalculatorFloat,
    phase: CalculatorFloat,
}

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
/// The single-mode phase-displacement gate with variable magnitude and phase.
///
/// Args:
///     mode (int): The mode the phase-shift gate is applied to.
///     displacement (CalculatorFloat): The magnitude by which to displace the mode.
///     phase (CalculatorFloat): The angle by which to displace the mode.
pub struct PhaseDisplacement {
    mode: usize,
    displacement: CalculatorFloat,
    phase: CalculatorFloat,
}

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
/// The single-mode phase-shift gate with variable phase, given by R(θ) = eexp(i * θ * 𝑁̂).
///
/// https://arxiv.org/pdf/2104.03241.pdf
///
/// Args:
///     mode (int): The mode the phase-shift gate is applied to.
///     phase (CalculatorFloat): The phase by which to shift the mode.
pub struct PhaseShift {
    mode: usize,
    phase: CalculatorFloat,
}

#[wrap(
    OperateModeGate,
    OperateTwoModeGate,
    Operate,
    Substitute,
    InvolveModes,
    SubstituteModes,
    OperateTwoMode,
    JsonSchema
)]
/// The 2-mode beam splitter which splits a beam with a transmission amplitude cos(θ) and a reflection amplitude exp(i * φ) * sin(θ).
///
/// Args:
///     mode_0 (int): The first mode the beam-splitter is applied to.
///     mode_1 (int): The second mode the beam-splitter is applied to.
///     theta (CalculatorFloat): The transmittivity angle of the beam-splitter.
///     phi (CalculatorFloat): The phase angle of the beam-splitter.
pub struct BeamSplitter {
    mode_0: usize,
    mode_1: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}

#[wrap(
    Operate,
    Substitute,
    InvolveModes,
    SubstituteModes,
    OperateSingleMode,
    JsonSchema
)]
/// The photon number-resolving detector measurement for bosons.
///
/// This can be used as a single-shot measurement of the photon number.
/// https://arxiv.org/pdf/0902.4824.pdf
///
/// Args:
///     mode (int): The mode the detector (measurement) is applied to.
///     readout (str): The register for the readout.
///     readout_index (int): The index in the readout the result is saved to.
pub struct PhotonDetection {
    mode: usize,
    readout: String,
    readout_index: usize,
}
