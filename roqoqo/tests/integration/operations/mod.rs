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

#![allow(clippy::redundant_clone)]

mod define_operations;
pub use define_operations::*;

mod measurement_operations;
pub use measurement_operations::*;

mod pragma_operations;
pub use pragma_operations::*;

mod single_qubit_gate_operations;
pub use single_qubit_gate_operations::*;

mod two_qubit_gate_operations;
pub use two_qubit_gate_operations::*;

mod three_qubit_gate_operations;
pub use three_qubit_gate_operations::*;

mod multi_qubit_gate_operations;
pub use multi_qubit_gate_operations::*;

mod involved_classical;
pub use involved_classical::*;

mod supported_version;
pub use supported_version::*;

mod bosonic_operations;
pub use bosonic_operations::*;

use nalgebra as na;
use ndarray::Array2;
use num_complex::Complex64;

// Helper function to convert a two-dimensional ndarray to a NxM matrix (N, M depending on the vector)
// The output can be used to be converted into a nalgebra matrix with `na::Matrix4::from()`
// for a 4x4 matrix or `na::DMatrix::from()` for a more general matrix
pub fn convert_matrix(customarray: Array2<Complex64>) -> na::DMatrix<Complex64> {
    let dim = customarray.dim();
    na::DMatrix::<Complex64>::from_iterator(dim.0, dim.1, customarray.t().iter().cloned())
}

// Test InvolvedQubits clone
#[test]
fn test_involved_qubits_clone() {
    let iq = roqoqo::operations::InvolvedQubits::All;
    let iq2 = iq.clone();
    assert_eq!(iq, iq2);
    let iq3 = roqoqo::operations::InvolvedQubits::None;
    let helper = iq != iq3;
    assert!(helper);
}
