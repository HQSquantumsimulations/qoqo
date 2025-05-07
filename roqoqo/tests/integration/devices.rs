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

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, Validator};
use ndarray::array;
use roqoqo::{
    devices::{AllToAllDevice, Device, GenericDevice, SquareLatticeDevice},
    RoqoqoError,
};
#[cfg(feature = "json_schema")]
use schemars::schema_for;
// use test_case::test_case;

#[test]
fn test_all_to_all() {
    let mut device = AllToAllDevice::new(3, &["RotateX".to_string()], &["CNOT".to_string()], 0.1);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
    );
    device
        .set_qubit_decoherence_rates(2, array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    assert_eq!(
        device.qubit_decoherence_rates(&2),
        Some(array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
    );
    device = device.add_damping_all(0.1);
    device = device.add_dephasing_all(10.0);
    device = device.add_depolarising_all(1.0);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![
            [0.1 + 1.0 / 2.0, 0.0, 0.0],
            [0.0, 1.0 / 2.0, 0.0],
            [0.0, 0.0, 1.0 / 4.0 + 5.0]
        ])
    );

    device.add_damping(0, 0.1).unwrap();
    device.add_dephasing(0, 10.0).unwrap();
    device.add_depolarising(0, 1.0).unwrap();
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![
            [0.2 + 2.0 / 2.0, 0.0, 0.0],
            [0.0, 2.0 / 2.0, 0.0],
            [0.0, 0.0, 2.0 / 4.0 + 10.0]
        ])
    );
    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.1f64));
    assert_eq!(device.single_qubit_gate_time("RotateX", &3), None);
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), None);

    device = device.set_all_single_qubit_gate_times("RotateX", 0.2);
    device = device.set_all_two_qubit_gate_times("CNOT", 0.2);

    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.2f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.2f64));

    device
        .set_multi_qubit_gate_time("MultiQubitMS", vec![0, 1, 2], 0.8)
        .unwrap();

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        Some(0.8f64)
    );

    device.set_two_qubit_gate_time("CNOT", 0, 1, 0.5).unwrap();
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.5f64));

    device
        .set_three_qubit_gate_time("ControlledControlledPauliZ", 0, 1, 2, 0.5_f64)
        .unwrap();
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &1, &2),
        Some(0.5_f64)
    );

    device
        .set_single_qubit_gate_time("RotateX", 2, 0.07)
        .unwrap();
    assert_eq!(device.single_qubit_gate_time("RotateX", &2), Some(0.07f64));

    let test_edges = [(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

    let change_device_error = device.change_device("PragmaTest", Vec::<u8>::new().as_ref());
    assert!(change_device_error.is_err());
    assert!(change_device_error
        .unwrap_err()
        .to_string()
        .contains("The `change_device()` method has not been implemented."));

    let gen_dev = device.to_generic_device();
    assert_eq!(gen_dev.two_qubit_edges().len(), 3);
}

/// Basic functional test
#[test]
fn generic_device_works() {
    let mut device = GenericDevice::new(3);
    device
        .set_single_qubit_gate_time("RotateX", 0, 0.1)
        .unwrap();
    device
        .set_single_qubit_gate_time("RotateX", 1, 0.05)
        .unwrap();
    device
        .set_single_qubit_gate_time("RotateX", 2, 0.07)
        .unwrap();
    assert!(device
        .set_single_qubit_gate_time("RotateX", 20, 0.07)
        .is_err());

    device.set_two_qubit_gate_time("CNOT", 0, 1, 0.5).unwrap();
    device.set_two_qubit_gate_time("CNOT", 0, 2, 0.5).unwrap();
    device.set_two_qubit_gate_time("CNOT", 1, 2, 0.5).unwrap();
    assert!(device.set_two_qubit_gate_time("CNOT", 30, 2, 0.5).is_err());
    assert!(device.set_two_qubit_gate_time("CNOT", 2, 20, 0.5).is_err());

    device
        .set_three_qubit_gate_time("ControlledControlledPauliZ", 0, 1, 2, 0.5)
        .unwrap();
    device
        .set_three_qubit_gate_time("ControlledControlledPauliZ", 2, 1, 0, 0.5)
        .unwrap();
    device
        .set_three_qubit_gate_time("ControlledControlledPhaseShift", 0, 1, 2, 0.5)
        .unwrap();
    assert!(device
        .set_three_qubit_gate_time("ControlledControlledPauliZ", 25, 1, 11, 0.5)
        .is_err());
    assert!(device
        .set_three_qubit_gate_time("ControlledControlledPhaseShift", 21, 14, 12, 0.5)
        .is_err());

    device
        .set_multi_qubit_gate_time("MultiQubitMS", vec![0, 1, 2], 0.8)
        .unwrap();
    device
        .set_multi_qubit_gate_time("MultiQubitMS", vec![0, 2], 0.8)
        .unwrap();
    assert!(device
        .set_multi_qubit_gate_time("MultiQubitMS", vec![0, 1, 100], 0.8)
        .is_err());
    device
        .set_qubit_decoherence_rates(0, array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    device
        .set_qubit_decoherence_rates(
            1,
            array![[0.0, 0.0, 0.0], [0.0, 0.002, 0.0], [0.0, 0.0, 0.0]],
        )
        .unwrap();
    device
        .set_qubit_decoherence_rates(
            2,
            array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.001, 0.0]],
        )
        .unwrap();

    assert!(device
        .set_qubit_decoherence_rates(
            2,
            array![
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.001, 0.0],
                [0.0, 0.001, 0.0]
            ]
        )
        .is_err());
    assert!(device
        .set_qubit_decoherence_rates(
            20,
            array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.001, 0.0]]
        )
        .is_err());

    device.add_damping(0, 0.1).unwrap();
    assert!(device.add_damping(10, 0.1).is_err());
    device.add_depolarising(0, 1.0).unwrap();
    assert!(device.add_depolarising(10, 0.1).is_err());
    device.add_dephasing(0, 10.0).unwrap();
    assert!(device.add_dephasing(10, 0.1).is_err());

    assert_eq!(device.number_qubits(), 3usize);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![
            [0.1 + 1.0 / 2.0, 0.0, 0.0],
            [0.0, 1.0 / 2.0, 0.0],
            [0.0, 0.0, 1.0 / 4.0 + 5.0]
        ])
    );

    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.1f64));
    assert_eq!(device.single_qubit_gate_time("RotateX", &3), None);
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), None);

    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.5f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &3), None);
    assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &1, &2),
        Some(0.5)
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &2, &1, &0),
        Some(0.5)
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &1, &2),
        Some(0.5)
    );

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        Some(0.8f64)
    );
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 2]),
        Some(0.8f64)
    );
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[1, 2, 3]),
        None
    );
    assert_eq!(device.multi_qubit_gate_time("Other", &[0, 1, 2]), None);

    let test_edges = [(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

    let change_device_error = device.change_device("PragmaTest", Vec::<u8>::new().as_ref());
    assert!(change_device_error.is_err());
    assert!(change_device_error
        .unwrap_err()
        .to_string()
        .contains("The `change_device()` method has not been implemented."));

    let gen_dev = device.clone().to_generic_device();
    assert_eq!(gen_dev, device);
}

/// Basic functional test
#[test]
fn change_device_test() {
    let mut device = GenericDevice::new(3);

    let empty_serialisation: Vec<u8> = Vec::new();
    let result = device.change_device("", &empty_serialisation);
    assert!(result.is_err());
}
#[test]
fn all_to_all_generic() {
    let mut generic_device = GenericDevice::new(2);
    let mut all_to_all =
        AllToAllDevice::new(2, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);

    generic_device
        .set_single_qubit_gate_time("RotateZ", 0, 1.0)
        .unwrap();
    generic_device
        .set_single_qubit_gate_time("RotateZ", 1, 1.0)
        .unwrap();
    generic_device
        .set_two_qubit_gate_time("CNOT", 0, 1, 1.0)
        .unwrap();
    generic_device
        .set_two_qubit_gate_time("CNOT", 1, 0, 1.0)
        .unwrap();
    // setting the decoherence rates directly
    generic_device
        .set_qubit_decoherence_rates(0, array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    // setting the decoherence rates directly
    generic_device
        .set_qubit_decoherence_rates(1, array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    assert_eq!(generic_device, all_to_all.to_generic_device());

    assert_eq!(
        all_to_all.add_damping(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 2".into()
        })
    );
    assert_eq!(
        all_to_all.add_depolarising(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 2".into()
        })
    );
    assert_eq!(
        all_to_all.add_dephasing(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 2".into()
        })
    );
    assert_eq!(
        all_to_all.single_qubit_gate_names(),
        vec!["RotateZ".to_string()]
    );
    assert_eq!(all_to_all.two_qubit_gate_names(), vec!["CNOT".to_string()]);
    assert_eq!(all_to_all.multi_qubit_gate_names(), Vec::<String>::new());

    let change_device_error = generic_device.change_device("PragmaTest", Vec::<u8>::new().as_ref());
    assert!(change_device_error.is_err());
    assert!(change_device_error
        .unwrap_err()
        .to_string()
        .contains("The `change_device()` method has not been implemented."));
}

#[test]
fn square_lattice_generic() {
    let mut generic_device = GenericDevice::new(2);
    let square_lattice =
        SquareLatticeDevice::new(1, 2, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);

    generic_device
        .set_single_qubit_gate_time("RotateZ", 0, 1.0)
        .unwrap();
    generic_device
        .set_single_qubit_gate_time("RotateZ", 1, 1.0)
        .unwrap();
    generic_device
        .set_two_qubit_gate_time("CNOT", 0, 1, 1.0)
        .unwrap();
    generic_device
        .set_two_qubit_gate_time("CNOT", 1, 0, 1.0)
        .unwrap();
    // setting the decoherence rates directly
    generic_device
        .set_qubit_decoherence_rates(0, array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    // setting the decoherence rates directly
    generic_device
        .set_qubit_decoherence_rates(1, array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    assert_eq!(generic_device, square_lattice.to_generic_device());

    let change_device_error = generic_device.change_device("PragmaTest", Vec::<u8>::new().as_ref());
    assert!(change_device_error.is_err());
    assert!(change_device_error
        .unwrap_err()
        .to_string()
        .contains("The `change_device()` method has not been implemented."));
}

#[test]
fn test_square_lattice() {
    let mut device =
        SquareLatticeDevice::new(2, 2, &["RotateX".to_string()], &["CNOT".to_string()], 0.1);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
    );
    device
        .set_qubit_decoherence_rates(2, array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        .unwrap();
    assert_eq!(
        device.qubit_decoherence_rates(&2),
        Some(array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
    );
    device = device.add_damping_all(0.1);
    device = device.add_dephasing_all(10.0);
    device = device.add_depolarising_all(1.0);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![
            [0.1 + 1.0 / 2.0, 0.0, 0.0],
            [0.0, 1.0 / 2.0, 0.0],
            [0.0, 0.0, 1.0 / 4.0 + 5.0]
        ])
    );

    device.add_damping(0, 0.1).unwrap();
    device.add_dephasing(0, 10.0).unwrap();
    device.add_depolarising(0, 1.0).unwrap();
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(array![
            [0.2 + 2.0 / 2.0, 0.0, 0.0],
            [0.0, 2.0 / 2.0, 0.0],
            [0.0, 0.0, 2.0 / 4.0 + 10.0]
        ])
    );
    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.1f64));
    assert_eq!(device.single_qubit_gate_time("RotateX", &4), None);
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), None);

    device = device.set_all_single_qubit_gate_times("RotateX", 0.2);
    device = device.set_all_two_qubit_gate_times("CNOT", 0.2);

    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.2f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.2f64));

    device
        .set_three_qubit_gate_time("ControlledControlledPauliZ", 0, 1, 2, 0.8)
        .unwrap();
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &1, &2),
        Some(0.8),
    );

    device
        .set_multi_qubit_gate_time("MultiQubitMS", vec![0, 1, 2], 0.8)
        .unwrap();

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        Some(0.8f64)
    );

    device.set_two_qubit_gate_time("CNOT", 0, 1, 0.5).unwrap();
    assert!(device
        .clone()
        .set_two_qubit_gate_time("CNOT", 0, 3, 0.5)
        .is_err());
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.5f64));

    device
        .set_single_qubit_gate_time("RotateX", 2, 0.07)
        .unwrap();
    assert_eq!(device.single_qubit_gate_time("RotateX", &2), Some(0.07f64));

    let test_edges = [(0, 1), (2, 3), (0, 2), (1, 3)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

    let gen_dev = device.to_generic_device();
    assert_eq!(gen_dev.two_qubit_edges().len(), 4);

    assert_eq!(
        device.add_damping(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 4".into()
        })
    );
    assert_eq!(
        device.add_depolarising(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 4".into()
        })
    );
    assert_eq!(
        device.add_dephasing(10, 0.1),
        Err(RoqoqoError::GenericError {
            msg: "Qubit 10 out of range for device of size 4".into()
        })
    );
    assert_eq!(
        device.single_qubit_gate_names(),
        vec!["RotateX".to_string()]
    );
    assert_eq!(device.two_qubit_gate_names(), vec!["CNOT".to_string()]);
    assert!(device
        .multi_qubit_gate_names()
        .contains(&"ControlledControlledPauliZ".to_string(),));
    assert!(device
        .multi_qubit_gate_names()
        .contains(&"MultiQubitMS".to_string()));

    let change_device_error = device.change_device("PragmaTest", Vec::<u8>::new().as_ref());
    assert!(change_device_error.is_err());
    assert!(change_device_error
        .unwrap_err()
        .to_string()
        .contains("The `change_device()` method has not been implemented."));
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    let generic_device = GenericDevice::new(2);
    let squared_device =
        SquareLatticeDevice::new(2, 2, &["RotateX".to_string()], &["CNOT".to_string()], 0.1);
    let all_to_all_device =
        AllToAllDevice::new(3, &["RotateX".to_string()], &["CNOT".to_string()], 0.1);

    // Serialize
    let test_json_generic = serde_json::to_string(&generic_device).unwrap();
    let test_json_squared = serde_json::to_string(&squared_device).unwrap();
    let test_json_all_to_all = serde_json::to_string(&all_to_all_device).unwrap();

    let test_value_generic: serde_json::Value = serde_json::from_str(&test_json_generic).unwrap();
    let test_value_squared: serde_json::Value = serde_json::from_str(&test_json_squared).unwrap();
    let test_value_all_to_all: serde_json::Value =
        serde_json::from_str(&test_json_all_to_all).unwrap();

    // Create JSONSchema
    let test_schema_generic = schema_for!(GenericDevice);
    let test_schema_squared = schema_for!(SquareLatticeDevice);
    let test_schema_all_to_all = schema_for!(AllToAllDevice);
    let schema_generic = serde_json::to_string(&test_schema_generic).unwrap();
    let schema_squared = serde_json::to_string(&test_schema_squared).unwrap();
    let schema_all_to_all = serde_json::to_string(&test_schema_all_to_all).unwrap();
    let schema_value_generic: serde_json::Value = serde_json::from_str(&schema_generic).unwrap();
    let schema_value_squared: serde_json::Value = serde_json::from_str(&schema_squared).unwrap();
    let schema_value_all_to_all: serde_json::Value =
        serde_json::from_str(&schema_all_to_all).unwrap();
    let compiled_schema_generic = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value_generic)
        .unwrap();
    let compiled_schema_squared = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value_squared)
        .unwrap();
    let compiled_schema_all_to_all = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value_all_to_all)
        .unwrap();

    let validation_result_generic = compiled_schema_generic.validate(&test_value_generic);
    let validation_result_squared = compiled_schema_squared.validate(&test_value_squared);
    let validation_result_all_to_all = compiled_schema_all_to_all.validate(&test_value_all_to_all);
    assert!(validation_result_generic.is_ok());
    assert!(validation_result_squared.is_ok());
    assert!(validation_result_all_to_all.is_ok());
}
