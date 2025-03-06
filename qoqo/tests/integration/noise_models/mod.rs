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

mod continuous_decoherence;
mod decoherence_on_gate;
mod decoherence_on_idle;
mod imperfect_readout;
mod overrotation;

use pyo3::prelude::*;
use qoqo::{STRUQTURE_OPERATOR, STRUQTURE_VERSION};
fn initialise_struqture_version(_py: Python) {
    let binding = _py
        .import("importlib.metadata")
        .expect("Could not import importlib.metadata module for function")
        .getattr("version")
        .expect("Could not get version function of importlib.metadata")
        .call1(("struqture_py",))
        .expect("Could not get version attribute of struqture_py")
        .unbind();
    let version: String = binding
        .extract(_py)
        .expect("Could not extract version string");
    STRUQTURE_VERSION.get_or_init(|| version);
    let operator: Py<PyAny> = _py
        .import("struqture_py.spins")
        .unwrap_or_else(|_| {
            panic!("Could not import struqture_py.spins module for get_noise_operator")
        })
        .getattr("PlusMinusLindbladNoiseOperator")
        .expect("Could not get PlusMinusLindbladOperator class")
        .unbind();
    STRUQTURE_OPERATOR.get_or_init(|| operator);
}
