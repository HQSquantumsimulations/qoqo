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

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PySet;
use pyo3::PyObjectProtocol;
use qoqo_macros::*;
use roqoqo::operations::*;
use std::collections::HashMap;

#[wrap(Operate, Define)]
/// DefinitionFloat is the Definition for a Float type register.
///
/// Args:
///     name (string): The name of the register that is defined.
///     length (int): The length of the register that is defined, usually the number of qubits to be measured.
///     is_output (bool): True/False if the variable is an output to the program.
pub struct DefinitionFloat {
    name: String,
    length: usize,
    is_output: bool,
}

#[wrap(Operate, Define)]
/// DefinitionComplex is the Definition for a Complex type register.
///
/// Args:
///     name (string): The name of the register that is defined.
///     length (int): The length of the register that is defined, usually the number of qubits to be measured.
///     is_output (bool): True/False if the variable is an output to the program.
pub struct DefinitionComplex {
    name: String,
    length: usize,
    is_output: bool,
}

#[wrap(Operate, Define)]
/// DefinitionUsize is the Definition for an Integer type register.
///
/// Args:
///     name (string): The name of the register that is defined.
///     length (int): The length of the register that is defined, usually the number of qubits to be measured.
///     is_output (bool): True/False if the variable is an output to the program.
pub struct DefinitionUsize {
    name: String,
    length: usize,
    is_output: bool,
}

#[wrap(Operate, Define)]
/// DefinitionBit is the Definition for a Bit type register.
///
/// Args:
///     name (string): The name of the register that is defined.
///     length (int): The length of the register that is defined, usually the number of qubits to be measured.
///     is_output (bool): True/False if the variable is an output to the program.
pub struct DefinitionBit {
    name: String,
    length: usize,
    is_output: bool,
}

#[wrap(Operate, Define)]
/// InputSymbolic is the Definition for a Float which will replace a certain symbolic parameter.
///
/// Args:
///     name (string): The name of the register that is defined.
///     input (float): The float by which to replace the quantities marked as "name".
pub struct InputSymbolic {
    name: String,
    input: f64,
}
