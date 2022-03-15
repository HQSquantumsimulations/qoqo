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

use ndarray::{array, Array2};
use roqoqo::devices::{AllToAllDevice, Device};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct TestDevice {
    number_qubits: usize,
    single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
    multi_qubit_gates: HashMap<String, f64>,
    rates: HashMap<usize, Array2<f64>>,
}

impl TestDevice {
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
        two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
        multi_qubit_gates: HashMap<String, f64>,
        rates: HashMap<usize, Array2<f64>>,
    ) -> Self {
        TestDevice {
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
            rates,
        }
    }
}

impl Device for TestDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self.single_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(qubit).copied(),
            None => None,
        }
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self.two_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*control, *target)).copied(),
            None => None,
        }
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, _qubits: &[usize]) -> Option<f64> {
        self.multi_qubit_gates.get(&hqslang.to_string()).copied()
    }

    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.rates.get(qubit).map(|x| x.to_owned())
    }

    fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits {
            for column in row + 1..self.number_qubits {
                edges.push((row, column));
            }
        }
        edges
    }
}

/// Basic functional test
#[test]
fn it_works() {
    let mut rotate_x_map: HashMap<usize, f64> = HashMap::new();
    rotate_x_map.insert(0, 0.1);
    rotate_x_map.insert(1, 0.05);
    rotate_x_map.insert(2, 0.07);
    let mut single_qubit_gates: HashMap<String, HashMap<usize, f64>> = HashMap::new();
    single_qubit_gates.insert("RotateX".to_string(), rotate_x_map);

    let mut cnot_map: HashMap<(usize, usize), f64> = HashMap::new();
    cnot_map.insert((0, 1), 0.5);
    cnot_map.insert((0, 2), 0.4);
    cnot_map.insert((1, 2), 0.3);
    let mut two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
    two_qubit_gates.insert("CNOT".to_string(), cnot_map);

    let mut multi_qubit_gates: HashMap<String, f64> = HashMap::new();
    multi_qubit_gates.insert("MultiQubitMS".to_string(), 0.8);

    let mut rates: HashMap<usize, Array2<f64>> = HashMap::new();
    rates.insert(
        0,
        array![[0.003, 0.0, 0.0], [0.0, 0.0, 00.0], [0.0, 0.0, 0.0]],
    );
    rates.insert(
        1,
        array![[0.0, 0.0, 0.0], [0.0, 0.002, 0.0], [0.0, 0.0, 0.0]],
    );
    rates.insert(
        2,
        array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.001, 0.0]],
    );

    let device = TestDevice::new(
        3,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
        rates,
    );

    let array: Array2<f64> = array![[0.003, 0.0, 0.0], [0.0, 0.0, 00.0], [0.0, 0.0, 0.0]];
    assert_eq!(device.number_qubits(), 3usize);
    assert_eq!(device.qubit_decoherence_rates(&0), Some(array));

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
    assert_eq!(device.multi_qubit_gate_time("Other", &[0, 1, 2]), None);

    let test_edges = vec![(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }
}

/// Basic functional test
#[test]
fn change_device_test() {
    let mut rotate_x_map: HashMap<usize, f64> = HashMap::new();
    rotate_x_map.insert(0, 0.1);
    rotate_x_map.insert(1, 0.05);
    rotate_x_map.insert(2, 0.07);
    let mut single_qubit_gates: HashMap<String, HashMap<usize, f64>> = HashMap::new();
    single_qubit_gates.insert("RotateX".to_string(), rotate_x_map);

    let mut cnot_map: HashMap<(usize, usize), f64> = HashMap::new();
    cnot_map.insert((0, 1), 0.5);
    cnot_map.insert((0, 2), 0.4);
    cnot_map.insert((1, 2), 0.3);
    let mut two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>> = HashMap::new();
    two_qubit_gates.insert("CNOT".to_string(), cnot_map);

    let mut multi_qubit_gates: HashMap<String, f64> = HashMap::new();
    multi_qubit_gates.insert("MultiQubitMS".to_string(), 0.8);

    let mut rates: HashMap<usize, Array2<f64>> = HashMap::new();
    rates.insert(
        0,
        array![[0.003, 0.0, 0.0], [0.0, 0.0, 00.0], [0.0, 0.0, 0.0]],
    );
    rates.insert(
        1,
        array![[0.0, 0.0, 0.0], [0.0, 0.002, 0.0], [0.0, 0.0, 0.0]],
    );
    rates.insert(
        2,
        array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.001, 0.0]],
    );

    let mut device = TestDevice::new(
        3,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
        rates,
    );

    let empty_serialisation: Vec<u8> = Vec::new();
    let result = device.change_device("", &empty_serialisation);
    assert!(result.is_err());
}

/// Test new() function for AllToAllDevice
#[test]
fn alltoalldevice_new() {
    let number_qubits = 3usize;
    let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gate = "CNOT".to_string();
    let device = AllToAllDevice::new(number_qubits, single_qubit_gates, two_qubit_gate);

    // Test number of qubits
    assert_eq!(device.number_qubits(), number_qubits);
    // Test that available single-qubit-gate is initialized with gate time set to zero
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.0));
    // Test that for non-available gates the returned gate time is Non
    assert_eq!(device.single_qubit_gate_time("RotateY", &0), None);

    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.0));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &3), None);
    assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        None,
    );

    let empty_rates = array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    assert_eq!(device.qubit_decoherence_rates(&0), Some(empty_rates));
}

/// Test set gate time functions for AllToAllDevice
#[test]
fn test_alltoalldevice_settimes() {
    let number_qubits = 3usize;
    let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gate = "CNOT".to_string();
    let mut device = AllToAllDevice::new(number_qubits, single_qubit_gates, two_qubit_gate);

    device = device.set_all_single_qubit_gate_times(&"RotateX", 0.07);
    device = device.set_all_single_qubit_gate_times(&"RotateZ", 0.1);

    device = device.set_all_two_qubit_gate_times(&"CNOT", 0.05);

    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.07f64));
    assert_eq!(
        device.single_qubit_gate_time("RotateX", &number_qubits),
        None
    );
    assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.1f64));

    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.05f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &3), None);
    assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        None,
    );
}

// Test set decoherence and two_qubit_edges for AllToAllDevice
#[test]
fn test_alltoalldevice_setattributes() {
    let number_qubits = 3usize;
    let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gate = "CNOT".to_string();
    let mut device = AllToAllDevice::new(number_qubits, single_qubit_gates, two_qubit_gate);

    let rates = array![[0.2], [0.3]];
    device = device.set_all_qubit_decoherence_rates(rates.clone());
    assert_eq!(device.qubit_decoherence_rates(&1), Some(rates));
    assert_eq!(device.qubit_decoherence_rates(&number_qubits), None);

    let test_edges = vec![(0, 1), (0, 2), (1, 2)];
    let edges = device.two_qubit_edges();
    assert_eq!(test_edges.len(), edges.len());
    for edge in edges {
        assert!(test_edges.contains(&edge));
    }
}
