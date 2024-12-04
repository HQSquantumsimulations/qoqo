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

use std::collections::HashSet;

use crate::operations::{
    ImplementedIn1point6, ImplementedIn1point8, InvolveModes, InvolveQubits, InvolvedClassical,
    InvolvedModes, InvolvedQubits, Operate, OperateModeGate, OperateSingleMode,
    OperateSingleModeGate, OperateTwoMode, OperateTwoModeGate, Substitute, SubstituteModes,
    SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;

/// The single-mode squeezing gate with tunable squeezing.
///
/// The squeezing gate is a quantum operation that allows for precise manipulation of quantum states,
/// by reducing the uncertainty in one variable and therefore increasing the uncertainty of another.
/// https://arxiv.org/pdf/quant-ph/0106157.pdf
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateSingleMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Squeezing {
    /// The mode the squeezing gate is applied to.
    mode: usize,
    /// The coefficient of the squeezing operation.
    squeezing: CalculatorFloat,
    /// The squeezing phase angle of the squeezing operation.
    phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_Squeezing: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "Squeezing",
];

impl InvolveQubits for Squeezing {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

impl ImplementedIn1point6 for Squeezing {}

impl SupportedVersion for Squeezing {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The single-mode phase-displacement gate with variable magnitude and phase.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateSingleMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseDisplacement {
    /// The mode the phase-displacement gate is applied to.
    mode: usize,
    /// The magnitude by which to displace the mode.
    displacement: CalculatorFloat,
    /// The angle by which to displace the mode.
    phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseDisplacement: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "PhaseDisplacement",
];

impl InvolveQubits for PhaseDisplacement {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

impl ImplementedIn1point8 for PhaseDisplacement {}

impl SupportedVersion for PhaseDisplacement {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 8, 0)
    }
}

/// The single-mode phase-shift gate with variable phase, given by R(θ) = exp(i * θ * 𝑁̂).
///
/// https://arxiv.org/pdf/2104.03241.pdf
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateSingleMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseShift {
    /// The mode the phase-shift gate is applied to.
    mode: usize,
    /// The phase by which to shift the mode.
    phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseShift: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "PhaseShift",
];

impl InvolveQubits for PhaseShift {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

impl ImplementedIn1point6 for PhaseShift {}

impl SupportedVersion for PhaseShift {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The 2-mode beam splitter which splits a beam with a transmission amplitude cos(θ) and a reflection amplitude exp(i * φ) * sin(θ).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateTwoModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateTwoMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct BeamSplitter {
    /// The first mode the beam-splitter is applied to.
    mode_0: usize,
    /// The second mode the beam-splitter is applied to.
    mode_1: usize,
    /// The transmittivity angle of the beam-splitter.
    theta: CalculatorFloat,
    /// The phase angle of the beam-splitter.
    phi: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_BeamSplitter: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "TwoModeGateOperation",
    "BeamSplitter",
];

impl InvolveQubits for BeamSplitter {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

impl ImplementedIn1point6 for BeamSplitter {}

impl SupportedVersion for BeamSplitter {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The photon number-resolving detector measurement for bosons.
///
/// This can be used as a single-shot measurement of the photon number.
/// https://arxiv.org/pdf/0902.4824.pdf
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateSingleMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhotonDetection {
    /// The mode the detector (measurement) is applied to.
    mode: usize,
    /// The register for the readout.
    readout: String,
    /// The index in the readout the result is saved to.
    readout_index: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PhotonDetection: &[&str; 3] = &["Operation", "Measurement", "PhotonDetection"];

impl InvolveQubits for PhotonDetection {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }

    fn involved_classical(&self) -> InvolvedClassical {
        let mut a: HashSet<(String, usize)> = HashSet::new();
        a.insert((self.readout.clone(), self.readout_index));
        InvolvedClassical::Set(a)
    }
}

impl ImplementedIn1point6 for PhotonDetection {}

impl SupportedVersion for PhotonDetection {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}
