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
use roqoqo::devices::Device;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct TestDevice {
    number_qubits: usize,
    single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
    multi_qubit_gates: HashMap<String, HashMap<(usize, usize, usize), f64>>,
    dephasing_rates: HashMap<usize, f64>,
    depolarising_rates: HashMap<usize, f64>,
    damping_rates: HashMap<usize, f64>,
}

impl TestDevice {
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
        two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
        multi_qubit_gates: HashMap<String, HashMap<(usize, usize, usize), f64>>,
        dephasing_rates: HashMap<usize, f64>,
        depolarising_rates: HashMap<usize, f64>,
        damping_rates: HashMap<usize, f64>,
    ) -> Self {
        TestDevice {
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
            dephasing_rates,
            depolarising_rates,
            damping_rates,
        }
    }
}

impl Device for TestDevice {
    fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> Option<&f64> {
        match self.single_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&qubit),
            None => None,
        }
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: usize, target: usize) -> Option<&f64> {
        match self.two_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(control, target)),
            None => None,
        }
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<&f64> {
        match self.multi_qubit_gates.get(&hqslang.to_string()) {
            Some(x) => x.get(&(qubits[0], qubits[1], qubits[2])), // ask,
            None => None,
        }
    }

    fn qubit_decoherence_rates(&self, qubits: &[usize]) -> Option<Array2<f64>> {
        let default: Array2<f64> = array![[qubits[0] as f64]];
        Some(default)
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

    let mut multi_ms_map: HashMap<(usize, usize, usize), f64> = HashMap::new();
    multi_ms_map.insert((0, 1, 2), 0.8);
    let mut multi_qubit_gates: HashMap<String, HashMap<(usize, usize, usize), f64>> =
        HashMap::new();
    multi_qubit_gates.insert("MultiQubitMS".to_string(), multi_ms_map);

    let mut deph_map: HashMap<usize, f64> = HashMap::new();
    deph_map.insert(0, 0.003);
    deph_map.insert(1, 0.002);
    deph_map.insert(2, 0.001);
    let mut depol_map: HashMap<usize, f64> = HashMap::new();
    depol_map.insert(0, 0.009);
    depol_map.insert(1, 0.008);
    depol_map.insert(2, 0.007);
    let mut damp_map: HashMap<usize, f64> = HashMap::new();
    damp_map.insert(0, 0.006);
    damp_map.insert(1, 0.005);
    damp_map.insert(2, 0.004);

    let device = TestDevice::new(
        3,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
        deph_map,
        depol_map,
        damp_map,
    );

    assert_eq!(device.number_qubits(), 3usize);
    assert_eq!(device.qubit_decoherence_rates(&[0]), Some(array![[0.0]]));

    assert_eq!(device.single_qubit_gate_time("RotateX", 0), Some(&0.1f64));
    assert_eq!(device.single_qubit_gate_time("RotateX", 3), None);
    assert_eq!(device.single_qubit_gate_time("RotateZ", 0), None);

    assert_eq!(device.two_qubit_gate_time("CNOT", 0, 1), Some(&0.5f64));
    assert_eq!(device.two_qubit_gate_time("CNOT", 0, 3), None);
    assert_eq!(device.two_qubit_gate_time("CZ", 0, 1), None);

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 2]),
        Some(&0.8f64)
    );
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1, 3]),
        None
    );
    assert_eq!(device.multi_qubit_gate_time("Other", &[0, 1, 2]), None);
}
