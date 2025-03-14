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

use pyo3::exceptions::PyRuntimeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};
use qoqo_macros::*;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use struqture::spins::PauliHamiltonian;
use struqture_py::spins::PauliHamiltonianWrapper;

use std::collections::HashMap;

#[wrap(Operate, OperateSpinsAnalog, JsonSchema)]
///  Constant Hamiltonian operation on analog quantum device (PASQAL)
///
/// Args:
///     hamiltonian (PauliHamiltonian): The hamiltonian that is to be simulated.
///     time (CalculatorFloat): The duration for which the state evolution takes place.
pub struct ApplyConstantPauliHamiltonian {
    hamiltonian: PauliHamiltonian,
    time: CalculatorFloat,
}

#[wrap(Operate, OperateSpinsAnalog, JsonSchema)]
///  Constant Hamiltonian operation on analog quantum device (PASCAL)
///
/// Args:
///     hamiltonian (PauliHamiltonian): The hamiltonian that is to be simulated.
///     time (List[float]): Range of time stored as a vector. The total duration of the simulations is given by the last value in the range.
///     values (Mapping[str, List[float]]): /// Values of time-dependent parameters, appearing in `hamiltonian`, at instances given by the vector `time`.
pub struct ApplyTimeDependentPauliHamiltonian {
    hamiltonian: PauliHamiltonian,
    time: Vec<f64>,
    values: HashMap<String, Vec<f64>>,
}
