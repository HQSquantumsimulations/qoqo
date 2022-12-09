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

//! Qoqo measurements
#[allow(unused_imports)] // reported unused import is wrong, compilation fails without import
use pyo3::prelude::*;
mod measurement_auxiliary_data_input;
pub use measurement_auxiliary_data_input::{
    CheatedInputWrapper, CheatedPauliZProductInputWrapper, PauliZProductInputWrapper,
};
mod basis_rotation_measurement;
pub use basis_rotation_measurement::PauliZProductWrapper;
mod cheated_basis_rotation_measurement;
pub use cheated_basis_rotation_measurement::CheatedPauliZProductWrapper;
mod cheated_measurement;
pub use cheated_measurement::CheatedWrapper;
mod classical_register_measurement;
pub use classical_register_measurement::ClassicalRegisterWrapper;

/// Measurements

#[pymodule]
pub fn measurements(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PauliZProductInputWrapper>()?;
    m.add_class::<CheatedPauliZProductInputWrapper>()?;
    m.add_class::<CheatedInputWrapper>()?;
    m.add_class::<PauliZProductWrapper>()?;
    m.add_class::<CheatedPauliZProductWrapper>()?;
    m.add_class::<CheatedWrapper>()?;
    m.add_class::<ClassicalRegisterWrapper>()?;

    Ok(())
}
