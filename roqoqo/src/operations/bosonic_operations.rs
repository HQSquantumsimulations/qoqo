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

use std::collections::HashSet;

use crate::operations::{
    InvolveModes, InvolveQubits, InvolvedClassical, InvolvedModes, InvolvedQubits, Operate,
    OperateModeGate, OperateSingleMode, OperateSingleModeGate, OperateTwoMode, OperateTwoModeGate,
    Substitute, SubstituteModes, SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;

/// The single-mode squeezing gate with tunable squeezing.
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
pub struct Squeezing {
    /// The mode the squeezing gate is applied to.
    mode: usize,
    /// The coefficient of the squeezing operation.
    squeezing: CalculatorFloat,
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

impl SupportedVersion for Squeezing {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The single-mode phase-shift gate with variable phase.
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

impl SupportedVersion for PhaseShift {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The 2-mode beam splitter which splits a beam with a transmission amplitude cos(θ) and a reflection amplitude exp(i * φ) * sin(θ)
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

impl SupportedVersion for BeamSplitter {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}

/// The photon number-resolving detector measurement for bosons.
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
pub struct PNRDetection {
    /// The mode the detector (measurement) is applied to.
    mode: usize,
    /// The register for the readout.
    readout: String,
    /// The index in the readout the result is saved to.
    readout_index: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PNRDetection: &[&str; 3] = &["Operation", "Measurement", "PNRDetection"];

impl InvolveQubits for PNRDetection {
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

impl SupportedVersion for PNRDetection {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 6, 0)
    }
}
