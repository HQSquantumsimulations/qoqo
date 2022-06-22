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
use roqoqo::devices::*;
use std::collections::HashMap;
// use test_case::test_case;

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

// /// Test new() function for AllToAllDevice
// #[test]
// fn alltoalldevice_new() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let device = AllToAllDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test number of qubits
//     assert_eq!(device.number_qubits(), number_qubits);
//     // Test that available single-qubit-gate is initialized with gate time set to zero
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.0));
//     // Test that for non-available gates the returned gate time is Non
//     assert_eq!(device.single_qubit_gate_time("RotateY", &0), None);

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );

//     let empty_rates = array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
//     assert_eq!(device.qubit_decoherence_rates(&0), Some(empty_rates));
// }

// /// Test set gate time functions for AllToAllDevice
// #[test]
// fn test_alltoalldevice_settimes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = AllToAllDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     device = device.set_all_single_qubit_gate_times(&"RotateX", 0.07);
//     device = device.set_all_single_qubit_gate_times(&"RotateZ", 0.1);

//     device = device.set_all_two_qubit_gate_times(&"CNOT", 0.05);

//     device = device.set_all_multi_qubit_gate_times(&"test", 0.0);

//     assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.07f64));
//     assert_eq!(
//         device.single_qubit_gate_time("RotateX", &number_qubits),
//         None
//     );
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.1f64));

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );
// }

// // Test set decoherence and two_qubit_edges for AllToAllDevice
// #[test]
// fn test_alltoalldevice_setattributes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = AllToAllDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     let rates_invalid = array![[0.1], [0.2], [0.3]];
//     let error = device
//         .clone()
//         .set_all_qubit_decoherence_rates(rates_invalid.clone());
//     assert!(error.is_err());

//     let rates = array![[0.1, 0.1, 0.1], [0.2, 0.2, 0.2], [0.3, 0.3, 0.3]];
//     device = device
//         .set_all_qubit_decoherence_rates(rates.clone())
//         .unwrap();
//     assert_eq!(device.qubit_decoherence_rates(&1), Some(rates));
//     assert_eq!(device.qubit_decoherence_rates(&number_qubits), None);

//     let test_edges = vec![(0, 1), (0, 2), (1, 2)];
//     let edges = device.two_qubit_edges();
//     assert_eq!(test_edges, edges);
// }

// /// Test new() function for GenericChain device
// #[test]
// fn genericchain_new() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let device = GenericChain::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test number of qubits
//     assert_eq!(device.number_qubits(), number_qubits);
//     // Test that available single-qubit-gate is initialized with gate time set to zero
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.0));
//     // Test that for non-available gates the returned gate time is Non
//     assert_eq!(device.single_qubit_gate_time("RotateY", &0), None);

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );

//     let empty_rates = array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
//     assert_eq!(device.qubit_decoherence_rates(&0), Some(empty_rates));
// }

// /// Test set gate time functions for GenericChain device
// #[test]
// fn test_genericchain_settimes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = GenericChain::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     device = device.set_all_single_qubit_gate_times(&"RotateX", 0.07);
//     device = device.set_all_single_qubit_gate_times(&"RotateZ", 0.1);

//     device = device.set_all_two_qubit_gate_times(&"CNOT", 0.05);

//     assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.07f64));
//     assert_eq!(
//         device.single_qubit_gate_time("RotateX", &number_qubits),
//         None
//     );
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.1f64));

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &2), None);
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );
// }

// // Test set decoherence and two_qubit_edges for GenericChain device
// #[test]
// fn test_genericchain_setattributes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = GenericChain::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     let rates_invalid = array![[0.1], [0.2], [0.3]];
//     let error = device
//         .clone()
//         .set_all_qubit_decoherence_rates(rates_invalid.clone());
//     assert!(error.is_err());

//     let rates = array![[0.1, 0.1, 0.1], [0.2, 0.2, 0.2], [0.3, 0.3, 0.3]];
//     device = device
//         .set_all_qubit_decoherence_rates(rates.clone())
//         .unwrap();
//     assert_eq!(device.qubit_decoherence_rates(&1), Some(rates));
//     assert_eq!(device.qubit_decoherence_rates(&number_qubits), None);

//     let test_edges = vec![(0, 1), (1, 2)];
//     let edges = device.two_qubit_edges();
//     assert_eq!(test_edges, edges);
// }

// /// Test new() function for GenericGrid
// #[test]
// fn genericgrid_new() {
//     let number_rows = 3usize;
//     let number_columns = 4usize;
//     let number_qubits = number_rows * number_columns;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let device = GenericGrid::new(
//         number_rows,
//         number_columns,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test number of qubits
//     assert_eq!(device.number_qubits(), number_qubits);
//     // Test that available single-qubit-gate is initialized with gate time set to zero
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.0));
//     // Test that for non-available gates the returned gate time is Non
//     assert_eq!(device.single_qubit_gate_time("RotateY", &0), None);

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &5), None);
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );

//     let empty_rates = array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
//     assert_eq!(device.qubit_decoherence_rates(&0), Some(empty_rates));
// }

// /// Test set gate time functions for GenericGrid
// #[test]
// fn test_genericgrid_settimes() {
//     let number_rows = 3usize;
//     let number_columns = 4usize;
//     let number_qubits = number_rows * number_columns;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &["MultiQubitMS".to_string()];
//     let mut device = GenericGrid::new(
//         number_rows,
//         number_columns,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     device = device.set_all_single_qubit_gate_times(&"RotateX", 0.07);
//     device = device.set_all_single_qubit_gate_times(&"RotateZ", 0.1);

//     device = device.set_all_two_qubit_gate_times(&"CNOT", 0.05);
//     device = device.set_all_multi_qubit_gate_times(&"MultiQubitMS", 0.2);

//     assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.07f64));
//     assert_eq!(
//         device.single_qubit_gate_time("RotateX", &number_qubits),
//         None
//     );
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.1f64));

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &3), None);
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     // test for all qubits in 2nd column
//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[1, 5, 9]),
//         Some(0.2f64),
//     );
//     // test for all qubits in 3rd row
//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[8, 9, 10, 11]),
//         Some(0.2f64),
//     );
//     // test a combination not covered by the standard function
//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );
// }

// // Test set decoherence for GenericGrid
// #[test]
// fn test_genericgrid_setattributes() {
//     let number_rows = 3usize;
//     let number_columns = 4usize;
//     let number_qubits = number_rows * number_columns;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = GenericGrid::new(
//         number_rows,
//         number_columns,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     let rates_invalid = array![[0.1], [0.2], [0.3]];
//     let error = device
//         .clone()
//         .set_all_qubit_decoherence_rates(rates_invalid.clone());
//     assert!(error.is_err());

//     let rates = array![[0.1, 0.1, 0.1], [0.2, 0.2, 0.2], [0.3, 0.3, 0.3]];
//     device = device
//         .set_all_qubit_decoherence_rates(rates.clone())
//         .unwrap();
//     assert_eq!(device.qubit_decoherence_rates(&1), Some(rates));
//     assert_eq!(device.qubit_decoherence_rates(&number_qubits), None);
// }

// // Test two_qubit_edges() for GenericGrid
// #[test_case(3, 4, vec![(0, 1), (0, 4), (1, 2), (1, 5), (2, 3), (2, 6), (3, 7), (4, 5), (4, 8), (5, 6), (5, 9), (6, 7), (6, 10), (7, 11), (8, 9), (9, 10), (10, 11)]; "3_4")]
// #[test_case(2, 3, vec![(0, 1),(0, 3),(1, 2),(1, 4),(2, 5), (3, 4), (4, 5)]; "2_3")]
// fn test_genericgrid_edges(rows: usize, columns: usize, test_edges: Vec<(usize, usize)>) {
//     let number_rows = rows;
//     let number_columns = columns;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let device = GenericGrid::new(
//         number_rows,
//         number_columns,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     let edges = device.two_qubit_edges();
//     assert_eq!(test_edges, edges);
// }

// /// Test new() function for GenericDevice
// #[test]
// fn genericdevice_new() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let device = GenericDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test number of qubits
//     assert_eq!(device.number_qubits(), number_qubits);
//     // Test that available single-qubit-gate is initialized with gate time set to zero
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.0));
//     // Test that for non-available gates the returned gate time is Non
//     assert_eq!(device.single_qubit_gate_time("RotateY", &0), None);

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.0));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );

//     let empty_rates = array![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
//     assert_eq!(device.qubit_decoherence_rates(&0), Some(empty_rates));
// }

// /// Test public fields of the GenericDevice
// #[test]
// fn genericdevice_fields() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let mapvec: Vec<SingleQubitMap> = vec![
//         SingleQubitMap {
//             qubit: 0,
//             time: 0.0,
//         },
//         SingleQubitMap {
//             qubit: 1,
//             time: 0.0,
//         },
//         SingleQubitMap {
//             qubit: 2,
//             time: 0.0,
//         },
//     ];
//     let mut single_qubit_gate_map: HashMap<String, Vec<SingleQubitMap>> = HashMap::new();
//     for gate in single_qubit_gates {
//         single_qubit_gate_map.insert(gate.clone(), mapvec.clone());
//     }
//     let mut two_qubit_gate_map: HashMap<String, Vec<TwoQubitMap>> = HashMap::new();
//     two_qubit_gate_map.insert(
//         "CNOT".to_string(),
//         vec![
//             TwoQubitMap {
//                 control: 0,
//                 target: 1,
//                 time: 0.0,
//             },
//             TwoQubitMap {
//                 control: 1,
//                 target: 2,
//                 time: 0.0,
//             },
//         ],
//     );
//     let multi_qubit_gates: HashMap<String, f64> = HashMap::new();

//     let mut decoherence_rates: HashMap<usize, Array2<f64>> = HashMap::new();
//     for qubit0 in 0..number_qubits {
//         decoherence_rates.insert(qubit0, Array2::<f64>::zeros((3, 3)));
//     }

//     let device = GenericDevice {
//         number_qubits: number_qubits.clone(),
//         single_qubit_gates: single_qubit_gate_map.clone(),
//         two_qubit_gates: two_qubit_gate_map.clone(),
//         multi_qubit_gates: multi_qubit_gates,
//         decoherence_rates: decoherence_rates.clone(),
//     };

//     // Test public fields of the GenericDevice
//     assert_eq!(device.number_qubits, number_qubits);
//     assert_eq!(device.single_qubit_gates, single_qubit_gate_map);
//     assert_eq!(device.two_qubit_gates, two_qubit_gate_map);
//     assert_eq!(device.multi_qubit_gates, HashMap::new());
//     assert_eq!(device.decoherence_rates, decoherence_rates);
// }

// /// Test set gate time functions for GenericDevice
// #[test]
// fn test_genericldevice_settimes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = GenericDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     device = device.set_all_single_qubit_gate_times(&"RotateX", 0.07);
//     device = device.set_all_single_qubit_gate_times(&"RotateZ", 0.1);

//     device = device.set_all_two_qubit_gate_times(&"CNOT", 0.05);
//     device = device.set_all_multi_qubit_gate_times(&"test", 0.0);

//     assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(0.07f64));
//     assert_eq!(
//         device.single_qubit_gate_time("RotateX", &number_qubits),
//         None
//     );
//     assert_eq!(device.single_qubit_gate_time("RotateZ", &0), Some(0.1f64));

//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &1), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &1, &0), Some(0.05f64));
//     assert_eq!(device.two_qubit_gate_time("CNOT", &0, &number_qubits), None);
//     assert_eq!(device.two_qubit_gate_time("CZ", &0, &1), None);

//     assert_eq!(
//         device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
//         None,
//     );
// }

// // Test set decoherence and two_qubit_edges for GenericDevice
// #[test]
// fn test_genericdevice_setattributes() {
//     let number_qubits = 3usize;
//     let single_qubit_gates = &["RotateX".to_string(), "RotateZ".to_string()];
//     let two_qubit_gates = &["CNOT".to_string()];
//     let multi_qubit_gates = &[];
//     let mut device = GenericDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     let rates_invalid = array![[0.1], [0.2], [0.3]];
//     let error = device
//         .clone()
//         .set_all_qubit_decoherence_rates(rates_invalid.clone());
//     assert!(error.is_err());

//     let rates = array![[0.1, 0.1, 0.1], [0.2, 0.2, 0.2], [0.3, 0.3, 0.3]];
//     device = device
//         .set_all_qubit_decoherence_rates(rates.clone())
//         .unwrap();
//     assert_eq!(device.qubit_decoherence_rates(&1), Some(rates));
//     assert_eq!(device.qubit_decoherence_rates(&number_qubits), None);

//     let test_edges = vec![(0, 1), (0, 2), (1, 2)];
//     let edges = device.two_qubit_edges();
//     assert_eq!(test_edges, edges);
// }

// // Very basic unit test for the fields in the created custom qubit map structs
// #[test]
// fn test_mapstructs() {
//     let singlequbitstruct = SingleQubitMap {
//         qubit: 0,
//         time: 4.3,
//     };
//     let twoqubitstruct = TwoQubitMap {
//         control: 0,
//         target: 2,
//         time: 0.0,
//     };
//     let qubits: Vec<usize> = vec![0, 1, 2, 3, 4];
//     let multiqubitstruct = MultiQubitMap {
//         qubits: qubits.clone(),
//         time: 5.0,
//     };
//     assert_eq!(singlequbitstruct.qubit, 0);
//     assert_eq!(singlequbitstruct.time, 4.3);
//     assert_eq!(twoqubitstruct.control, 0);
//     assert_eq!(twoqubitstruct.target, 2);
//     assert_eq!(twoqubitstruct.time, 0.0);
//     assert_eq!(multiqubitstruct.qubits, qubits);
//     assert_eq!(multiqubitstruct.time, 5.0);
// }

// // Test Clone, PartialEq and Debug for GenericDevice
// #[test]
// fn test_genericdevice_derive() {
//     let number_qubits = 1usize;
//     let single_qubit_gates = &[];
//     let two_qubit_gates = &[];
//     let multi_qubit_gates = &[];
//     let device = GenericDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test debug
//     let debug =  "GenericDevice { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} }";
//     assert_eq!(format!("{:?}", device.clone()), debug);

//     // Test Clone and PartialEq
//     assert!(device.clone() == device);
// }

// // Test Clone, PartialEq and Debug for GenericChain
// #[test]
// fn test_genericchain_derive() {
//     let number_qubits = 1usize;
//     let single_qubit_gates = &[];
//     let two_qubit_gates = &[];
//     let multi_qubit_gates = &[];
//     let device = GenericChain::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test debug
//     let debug =  "GenericChain { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} }";
//     assert_eq!(format!("{:?}", device.clone()), debug);

//     // Test Clone and PartialEq
//     assert!(device.clone() == device);
// }

// // Test Clone, PartialEq and Debug for AllToAllDevice
// #[test]
// fn test_alltoalldevice_derive() {
//     let number_qubits = 1usize;
//     let single_qubit_gates = &[];
//     let two_qubit_gates = &[];
//     let multi_qubit_gates = &[];
//     let device = AllToAllDevice::new(
//         number_qubits,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test debug
//     let debug =  "AllToAllDevice { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} }";
//     assert_eq!(format!("{:?}", device.clone()), debug);

//     // Test Clone and PartialEq
//     assert!(device.clone() == device);
// }

// // Test Clone, PartialEq and Debug for GenericGrid
// #[test]
// fn test_genericgrid_derive() {
//     let number_rows = 1usize;
//     let number_columns = 1usize;
//     let single_qubit_gates = &[];
//     let two_qubit_gates = &[];
//     let multi_qubit_gates = &[];
//     let device = GenericGrid::new(
//         number_rows,
//         number_columns,
//         single_qubit_gates,
//         two_qubit_gates,
//         multi_qubit_gates,
//     );

//     // Test debug
//     let debug =  "GenericGrid { number_rows: 1, number_columns: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} }";
//     assert_eq!(format!("{:?}", device.clone()), debug);

//     // Test Clone and PartialEq
//     assert!(device.clone() == device);
// }
