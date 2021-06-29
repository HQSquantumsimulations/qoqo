// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//

//! Definition Operation that defines the classical registers and variables in the Circuit.
//!
//! In general, every circuit will have at least one Definition operation. The main
//! reasons one would need a Definition operation in a circuit is for measurements.
//! If you need to measure something in a circuit using a MeasureQubit operation
//! (see qoqo.operations.measurememnt_operations.py), you need to define the classical
//! register used to store the measurement result.
//!
//! You need to add the Define operation to the circuit before what you are defining is used.
//! Therefore, qoqo uses the convention that Definition operations are added first to the circuit,
//! before you add any other operations.
//!
//! There are 5 types of Definitions:
//! (1) DefinitionFloat, where the register defined contains a float or floats.
//! (2) DefinitionComplex, where the register defined contains a complex or complexes.
//! (3) DefinitionUsize, where the register defined contains an integer or integers.
//! (4) DefinitionBit, where the register defined contains a bit or bits.
//! (5) InputSymbolic, where the user can define a floating point type value to replace a certain symbolic parameter.
//!

use crate::operations::{Define, InvolveQubits, InvolvedQubits, Operate, RoqoqoError, Substitute};

/// DefinitionFloat is the Definition for a floating point type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DefinitionFloat {
    /// The name of the register that is defined.
    name: String,
    /// The length of the register that is defined, usually the number of qubits to be measured.
    length: usize,
    /// True/False if the variable is an output to the program.
    is_output: bool,
}

#[allow(non_upper_case_globals)]
const TAGS_DefinitionFloat: &[&str; 3] = &["Operation", "Definition", "DefinitionFloat"];

// Implementing the InvolveQubits trait for DefinitionFloat.
impl InvolveQubits for DefinitionFloat {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// DefinitionComplex is the Definition for a Complex type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DefinitionComplex {
    /// The name of the register that is defined.
    name: String,
    /// The length of the register that is defined, usually the number of qubits to be measured.
    length: usize,
    /// True/False if the variable is an output to the program.
    is_output: bool,
}

#[allow(non_upper_case_globals)]
const TAGS_DefinitionComplex: &[&str; 3] = &["Operation", "Definition", "DefinitionComplex"];

// Implementing the InvolveQubits trait for DefinitionComplex.
impl InvolveQubits for DefinitionComplex {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// DefinitionUsize is the Definition for an Integer type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DefinitionUsize {
    /// The name of the register that is defined.
    name: String,
    /// The length of the register that is defined, usually the number of qubits to be measured.
    length: usize,
    /// True/False if the variable is an output to the program.
    is_output: bool,
}

#[allow(non_upper_case_globals)]
const TAGS_DefinitionUsize: &[&str; 3] = &["Operation", "Definition", "DefinitionUsize"];

// Implementing the InvolveQubits trait for DefinitionUsize.
impl InvolveQubits for DefinitionUsize {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// DefinitionBit is the Definition for a Bit type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DefinitionBit {
    /// The name of the register that is defined.
    name: String,
    /// The length of the register that is defined, usually the number of qubits to be measured.
    length: usize,
    /// True/False if the variable is an output to the program.
    is_output: bool,
}

#[allow(non_upper_case_globals)]
const TAGS_DefinitionBit: &[&str; 3] = &["Operation", "Definition", "DefinitionBit"];

// Implementing the InvolveQubits trait for DefinitionBit.
impl InvolveQubits for DefinitionBit {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}

/// InputSymbolic is the Definition for a floating point type parameter which will replace a certain symbolic parameter.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct InputSymbolic {
    /// The name of the register that is defined.
    name: String,
    /// The floating point type value by which to replace the quantities marked as "name".
    input: f64,
}

#[allow(non_upper_case_globals)]
const TAGS_InputSymbolic: &[&str; 3] = &["Operation", "Definition", "InputSymbolic"];

// Implementing the InvolveQubits trait for InputSymbolic.
impl InvolveQubits for InputSymbolic {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
}
