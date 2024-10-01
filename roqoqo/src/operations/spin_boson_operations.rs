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

//! Abstract operations for qubit-resonator hardware

use crate::operations::{
    ImplementedIn1point11, InvolveModes, InvolveQubits, InvolvedModes, InvolvedQubits, Operate,
    OperateModeGate, OperateSingleMode, OperateSingleModeGate, OperateSingleQubit, Substitute,
    SubstituteModes, SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;

/// The quantum Rabi interaction exp(-i * θ * X * (b^† + b))
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct QuantumRabi {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
    /// The parameter θ of the interaction
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_QuantumRabi: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "QuantumRabi",
];

impl ImplementedIn1point11 for QuantumRabi {}

impl SupportedVersion for QuantumRabi {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

/// Longitudinal coupling gate exp(-i * θ * Z * (b^† + b))
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct LongitudinalCoupling {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
    /// The parameter θ of the interaction
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_LongitudinalCoupling: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "LongitudinalCoupling",
];

impl ImplementedIn1point11 for LongitudinalCoupling {}

impl SupportedVersion for LongitudinalCoupling {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

/// The Jaynes-Cummings gate exp(-i * θ * (σ^- * b^† + σ^+ * b))
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct JaynesCummings {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
    /// The parameter θ of the interaction
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_JaynesCummings: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "JaynesCummings",
];

impl ImplementedIn1point11 for JaynesCummings {}

impl SupportedVersion for JaynesCummings {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

/// Stores a single excitation from the involved qubit into the involved bosonic mode as follows
/// |0⟩_B ⨂ (a |0⟩_Q + b |1⟩_Q) -> (a|0⟩_B + b |1⟩_B ) ⨂ |0⟩_Q
///
/// Note: not defined if the bosonic mode is in a state |n⟩ with n != 0
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SingleExcitationStore {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_SingleExcitationStore: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "SingleExcitationStore",
];

impl ImplementedIn1point11 for SingleExcitationStore {}

impl SupportedVersion for SingleExcitationStore {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

/// Loads a single excitation from a bosonic mode into a qubit as follows
/// (c1 |0⟩_B + c2 |1⟩_B) ⨂ |0⟩_Q -> |0⟩_B ⨂ (c1 |0⟩_Q + c2 |1⟩_Q)
///
/// Note: if the initial qubit state is |1⟩_Q the operation is only defined if c2 = 0
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SingleExcitationLoad {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_SingleExcitationLoad: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "SingleExcitationLoad",
];

impl ImplementedIn1point11 for SingleExcitationLoad {}

impl SupportedVersion for SingleExcitationLoad {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}

/// Controlled-Z operation between a qubit and a bosonic mode, where the two-dimensional subspace of
/// the bosonic mode spanned by the occupation number states |0⟩_B and |1⟩_B is considered
/// as the second qubit involved in the CZ operation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleMode,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SubstituteModes,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct CZQubitResonator {
    /// The qubit involved.
    qubit: usize,
    /// The bosonic mode involved.
    mode: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_CZQubitResonator: &[&str; 6] = &[
    "Operation",
    "GateOperation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SingleQubitGateOperation",
    "CZQubitResonator",
];

impl ImplementedIn1point11 for CZQubitResonator {}

impl SupportedVersion for CZQubitResonator {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 11, 0)
    }
}
