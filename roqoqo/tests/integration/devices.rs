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

use ndarray::array;
use roqoqo::devices::*;
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
            [0.0, 0.0, 1.0 / 4.0 + 10.0]
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
            [0.0, 0.0, 2.0 / 4.0 + 20.0]
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
        .set_single_qubit_gate_time("RotateX", 2, 0.07)
        .unwrap();
    assert_eq!(device.single_qubit_gate_time("RotateX", &2), Some(0.07f64));

    let test_edges = vec![(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

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
            [0.0, 0.0, 1.0 / 4.0 + 10.0]
        ])
    );

    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.1f64));
    assert_eq!(device.single_qubit_gate_time("RotateX", &3), None);
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), None);

    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.5f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &3), None);
    assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

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

    let test_edges = vec![(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

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
    let all_to_all = AllToAllDevice::new(2, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);

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
            [0.0, 0.0, 1.0 / 4.0 + 10.0]
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
            [0.0, 0.0, 2.0 / 4.0 + 20.0]
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

    let test_edges = vec![(0, 1), (2, 3), (0, 2), (1, 3)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }

    let gen_dev = device.to_generic_device();
    assert_eq!(gen_dev.two_qubit_edges().len(), 4);
}
