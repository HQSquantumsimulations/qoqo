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

use qoqo_calculator::CalculatorFloat;
use crate::operations::{
    InvolveQubits, InvolvedQubits, InvolveModes, InvolvedModes, Operate, OperateModeGate, OperateSingleMode,
    OperateSingleModeGate, SubstituteModes, SupportedVersion,
};
use crate::RoqoqoError;

/// The most general unitary operation acting on one qubit.
///
/// # Warning
///
/// Due to the support of parameterized values it cannot be guaranteed that the unitary matrix of the gate
/// is always normalized to one.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    OperateModeGate,
    OperateSingleModeGate,
    roqoqo_derive::InvolveModes,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::SubstituteModes,
    roqoqo_derive::OperateSingleMode,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SqueezingGate {
    /// The mode the squeezing gate is applied to.
    mode: usize,
    /// The coefficient of the squeezing operation.
    squeezing: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_SqueezingGate: &[&str; 4] = &[
    "Operation",
    "ModeGateOperation",
    "SingleModeGateOperation",
    "SqueezingGate",
];

/// Trait for bosonic operations acting on exactly one qubit.
impl OperateSingleMode for SqueezingGate {
    /// Returns the mode the bosonic Operation acts on.
    ///
    /// # Returns
    ///
    /// * `mode` - The mode the operation acts on.
    fn mode(&self) -> usize {
        self.mode
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl InvolveQubits for SqueezingGate {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}
