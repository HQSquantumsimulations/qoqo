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

use std::collections::HashSet;

use crate::operations::{Define, InvolveQubits, InvolvedQubits, Operate, RoqoqoError, Substitute};

use super::SupportedVersion;

/// DefinitionFloat is the Definition for a floating point type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }
    fn involved_classical(&self) -> super::InvolvedClassical {
        super::InvolvedClassical::All(self.name.clone())
    }
}

/// DefinitionComplex is the Definition for a Complex type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]

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

    fn involved_classical(&self) -> super::InvolvedClassical {
        super::InvolvedClassical::All(self.name.clone())
    }
}

/// DefinitionUsize is the Definition for an Integer type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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

    fn involved_classical(&self) -> super::InvolvedClassical {
        super::InvolvedClassical::All(self.name.clone())
    }
}

/// DefinitionBit is the Definition for a Bit type register.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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

    fn involved_classical(&self) -> super::InvolvedClassical {
        super::InvolvedClassical::All(self.name.clone())
    }
}

/// InputSymbolic is the Definition for a floating point type parameter which will replace a certain symbolic parameter.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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

    fn involved_classical(&self) -> super::InvolvedClassical {
        let mut a: HashSet<(String, usize)> = HashSet::new();
        a.insert((self.name.clone(), 0));
        super::InvolvedClassical::Set(a)
    }
}

/// InputBit sets a certain bit in an existing BitRegister of the circuit.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct InputBit {
    /// The name of the register that where the bit is set.
    name: String,
    /// The index in the register that is set.
    index: usize,
    /// The value the bit is set to
    value: bool,
}

impl SupportedVersion for InputBit {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 1, 0)
    }
}

impl super::ImplementedIn1point1 for InputBit {}

#[allow(non_upper_case_globals)]
const TAGS_InputBit: &[&str; 3] = &["Operation", "Definition", "InputBit"];

// Implementing the InvolveQubits trait for InputSymbolic.
impl InvolveQubits for InputBit {
    /// Lists all involved Qubits (here, none).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::None
    }

    fn involved_classical(&self) -> super::InvolvedClassical {
        let mut a: HashSet<(String, usize)> = HashSet::new();
        a.insert((self.name.clone(), self.index));
        super::InvolvedClassical::Set(a)
    }
}
