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


//! Abstract operations for qubit-resonator hardware


use ndarray::{array, Array2};
use num_complex::Complex64;

use crate::operations::{
    ImplementedIn1point9,
    InvolveModes,
    InvolvedModes,
    InvolveQubits,
    InvolvedQubits,
    Operate,
    OperateGate,
    OperateModeGate,
    OperateSingleQubit,
    Substitute,
    SubstituteModes,
    SupportedVersion,
};
use crate::RoqoqoError;
use qoqo_calculator::CalculatorFloat;


/// The quantum Rabi interaction exp(-i * θ * σ^x * (b^{dagger} + b))
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
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
    "SingleQubitGate",
];


impl ImplementedIn1point10 for QuantumRabi {}

impl SupportedVersion for QuantumRabi {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 10, 0)
    }
}


/// The Jaynes-Cummings gate exp(-i * theta * (σ^- * b^{dagger} + σ^+ * b))
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
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
    "SingleQubitGate",
];


impl ImplementedIn1point10 for JaynesCummings {}

impl SupportedVersion for JaynesCummings {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 10, 0)
    }
}
