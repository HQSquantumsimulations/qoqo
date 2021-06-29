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

use roqoqo::operations::{PauliX, PauliZ, RotateX, SingleQubitGateOperation};
use roqoqo_test::prepare_monte_carlo_gate_test;
#[test]
fn test_with_single_options() {
    let gate = RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    let preparation_gates: Vec<SingleQubitGateOperation> = vec![PauliX::new(0).into()];
    let basis_rotations_gates: Vec<SingleQubitGateOperation> = vec![PauliZ::new(0).into()];
    let (measurement, expectation_values) = prepare_monte_carlo_gate_test(
        gate.into(),
        preparation_gates,
        basis_rotations_gates,
        None,
        1,
        10,
    );
    assert_eq!(measurement.circuits.len(), 1);
    assert!(expectation_values.get("exp_val_0").is_some());
}
