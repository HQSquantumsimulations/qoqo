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

//! Classical registers for roqoqo.
//!
//! Registers are used to store classical information during the execution of a
//! roqoqo circuit and to provide a unified output interface for the different backends.

use std::collections::HashMap;

use num_complex::Complex64;

// This could be optimized by using bit-vec or bitvec traits
// but needs to be tested first.
/// Represents classical register of bits for computation in qoqo.
pub type BitRegister = Vec<bool>;

/// Represents classical register of float numbers for computation in qoqo.
pub type FloatRegister = Vec<f64>;

/// Represents classical register of complex numbers for computation in qoqo.
pub type ComplexRegister = Vec<Complex64>;

/// Represents classical bit registers for output of computations in qoqo.
///
/// Vector over single bit registers that are each the output of a single run
/// of a quantum program.
pub type BitOutputRegister = Vec<BitRegister>;

/// Represents classical float registers for output of computations in qoqo.
///
/// Vector over single float registers that are each the output of a single run
/// of a quantum program.

pub type FloatOutputRegister = Vec<FloatRegister>;

/// Represents classical complex registers for output of computations in qoqo.
///
/// Vector over single complex registers that are each the output of a single run
/// of a quantum program.
pub type ComplexOutputRegister = Vec<ComplexRegister>;

/// Registers passed to and from functions running a full circuit.
pub type Registers = (
    HashMap<String, BitOutputRegister>,
    HashMap<String, FloatOutputRegister>,
    HashMap<String, ComplexOutputRegister>,
);
