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

//! Integration test for public API of multi qubit gate operations

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, JSONSchema};
use ndarray::array;
use num_complex::Complex64;
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::Circuit;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

/// Test circuit function of MultiQubitMolmerSorensen
#[test_case(vec![0,1]; "two_qubit")]
#[test_case(vec![0,1,2]; "three_qubit")]
fn test_circuit_multi_ms(qubits: Vec<usize>) {
    let gate = MultiQubitMS::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    let c = gate.circuit();
    if qubits.len() == 2 {
        let mut comparison_circuit = Circuit::new();
        comparison_circuit += Hadamard::new(0);
        comparison_circuit += Hadamard::new(1);
        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += RotateZ::new(1, CalculatorFloat::FRAC_PI_2);
        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += Hadamard::new(0);
        comparison_circuit += Hadamard::new(1);
        assert!(c == comparison_circuit);
    }
    if qubits.len() == 3 {
        let mut comparison_circuit = Circuit::new();
        comparison_circuit += Hadamard::new(0);
        comparison_circuit += Hadamard::new(1);
        comparison_circuit += Hadamard::new(2);
        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += CNOT::new(1, 2);
        comparison_circuit += RotateZ::new(2, CalculatorFloat::FRAC_PI_2);
        comparison_circuit += CNOT::new(1, 2);
        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += Hadamard::new(0);
        comparison_circuit += Hadamard::new(1);
        comparison_circuit += Hadamard::new(2);
        assert!(c == comparison_circuit);
    }
}

#[test_case(vec![0,1]; "two_qubit")]
fn test_matrix_output(qubits: Vec<usize>) {
    let gate = MultiQubitMS::new(qubits, CalculatorFloat::FRAC_PI_2);
    let f: f64 = 1.0 / ((2.0_f64).sqrt());
    let test_array = array![
        [
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0)
        ],
    ];
    let unit = gate.unitary_matrix().unwrap();
    let should_be_zero = unit - test_array;
    assert!(should_be_zero.iter().all(|x| x.norm() < f64::EPSILON));
}

#[test_case(vec![0,1,2]; "three_qubit")]
fn test_matrix_output_three(qubits: Vec<usize>) {
    let gate = MultiQubitMS::new(qubits, CalculatorFloat::FRAC_PI_2);
    let f: f64 = 1.0 / ((2.0_f64).sqrt());
    let test_array = array![
        [
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, 0.0)
        ],
    ];
    let unit = gate.unitary_matrix().unwrap();
    let should_be_zero = unit - test_array;
    assert!(should_be_zero.iter().all(|x| x.norm() < f64::EPSILON));
}

#[test]
fn test_clone_partial_eq() {
    let qubits = vec![0, 1, 2];

    let gate = MultiQubitMS::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    assert_eq!(gate.hqslang(), "MultiQubitMS");
    assert_eq!(
        gate.tags(),
        &[
            "Operation",
            "GateOperation",
            "MultiQubitGateOperation",
            "MultiQubitMS",
        ]
    );
    assert!(!gate.is_parametrized());
    let gate1 = MultiQubitMS::new(qubits, "theta".into());
    assert!(gate1.is_parametrized());
    let helper = gate != gate1;
    assert!(helper);
    #[allow(clippy::redundant_clone)]
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

#[test]
fn test_operate() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitMS::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    assert_eq!(gate.hqslang(), "MultiQubitMS");
    assert_eq!(
        gate.tags(),
        &[
            "Operation",
            "GateOperation",
            "MultiQubitGateOperation",
            "MultiQubitMS",
        ]
    );
    assert_eq!(gate.qubits(), &vec![0, 1, 2]);
    assert!(!gate.is_parametrized());
    let gate1 = MultiQubitMS::new(qubits, "theta".into());
    assert!(gate1.is_parametrized());
}

#[test]
fn test_substitute() {
    let qubits = vec![0, 1, 2];
    let gate1 = MultiQubitMS::new(qubits.clone(), "theta".into());
    let gate = MultiQubitMS::new(qubits, CalculatorFloat::FRAC_PI_2);
    let mut calc = Calculator::new();
    calc.set_variable("theta", std::f64::consts::FRAC_PI_2);
    let gate_substituted = roqoqo::operations::Substitute::substitute_parameters(&gate1, &calc);
    let subs = gate_substituted.unwrap();
    assert_eq!(gate, subs);
    let mut mapping: HashMap<usize, usize> = std::collections::HashMap::new();
    let _ = mapping.insert(0, 1);
    let _ = mapping.insert(1, 2);
    let _ = mapping.insert(2, 0);
    let remapped = gate1.remap_qubits(&mapping).unwrap();
    let qubits = remapped.qubits();
    assert_eq!(qubits, &vec![1, 2, 0]);
}

#[test]
fn test_substitute_error() {
    let qubits = vec![0, 1, 2];
    let gate1 = MultiQubitMS::new(qubits, "theta".into());
    let calc = Calculator::new();
    let gate_substituted = gate1.substitute_parameters(&calc);
    assert!(gate_substituted.is_err());
    let mut mapping: HashMap<usize, usize> = std::collections::HashMap::new();
    let _ = mapping.insert(1, 2);
    let _ = mapping.insert(2, 0);
    let remapped = gate1.remap_qubits(&mapping);
    if !cfg!(feature = "unstable_remapping_validity_check") {
        assert!(remapped.is_err());
    } else {
        assert!(remapped.is_ok());
    }
}

#[test]
fn test_format() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitMS::new(qubits, "theta".into());
    let string = format!("{:?}", gate);
    assert!(string.contains("theta"));
    assert!(string.contains("MultiQubitMS"));
}

#[test]
fn test_involved_qubits() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitMS::new(qubits, "theta".into());
    let involved_qubits = gate.involved_qubits();
    let mut comp_set: HashSet<usize> = HashSet::new();
    let _ = comp_set.insert(0);
    let _ = comp_set.insert(1);
    let _ = comp_set.insert(2);
    assert_eq!(involved_qubits, InvolvedQubits::Set(comp_set));
}

/// Test powerfc function for MultiQubitMS with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(2.0); "power_2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0 / 2.0); "power_1/2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0); "power_1")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(0.0); "power_0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(-2.0); "power_-2.0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_rotatex_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitMS::new(qubits.clone(), theta);

    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = MultiQubitMS::new(qubits, test_theta);
    assert_eq!(power_gate, test_gate);
    assert_eq!(power_gate.theta(), test_gate.theta());
}

#[test_case(vec![0,1]; "two_qubit")]
#[test_case(vec![0,1,2]; "three_qubit")]
fn test_circuit_multi_zz(qubits: Vec<usize>) {
    let gate = MultiQubitZZ::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    let c = gate.circuit();
    if qubits.len() == 2 {
        let mut comparison_circuit = Circuit::new();

        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += RotateZ::new(1, CalculatorFloat::FRAC_PI_2);
        comparison_circuit += CNOT::new(0, 1);

        assert!(c == comparison_circuit);
    }
    if qubits.len() == 3 {
        let mut comparison_circuit = Circuit::new();

        comparison_circuit += CNOT::new(0, 1);
        comparison_circuit += CNOT::new(1, 2);
        comparison_circuit += RotateZ::new(2, CalculatorFloat::FRAC_PI_2);
        comparison_circuit += CNOT::new(1, 2);
        comparison_circuit += CNOT::new(0, 1);

        assert!(c == comparison_circuit);
    }
}

#[test_case(vec![0,1]; "two_qubit")]
fn test_matrix_output_multi_qubit_zz(qubits: Vec<usize>) {
    let gate = MultiQubitZZ::new(qubits, CalculatorFloat::FRAC_PI_2);
    let f: f64 = 1.0 / ((2.0_f64).sqrt());
    let test_array = array![
        [
            Complex64::new(f, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, (-1.0) * f)
        ],
    ];
    let unit = gate.unitary_matrix().unwrap();
    let should_be_zero = unit - test_array;
    assert!(should_be_zero.iter().all(|x| x.norm() < f64::EPSILON));
}

#[test_case(vec![0,1,2]; "three_qubit")]
fn test_matrix_output_three_multi_qubit_zz(qubits: Vec<usize>) {
    let gate = MultiQubitZZ::new(qubits, CalculatorFloat::FRAC_PI_2);
    let f: f64 = 1.0 / ((2.0_f64).sqrt());
    let test_array = array![
        [
            Complex64::new(f, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, (-1.0) * f),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, (-1.0) * f),
            Complex64::new(0.0, 0.0)
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(f, f)
        ],
    ];
    let unit = gate.unitary_matrix().unwrap();
    let should_be_zero = unit - test_array;
    assert!(should_be_zero.iter().all(|x| x.norm() < f64::EPSILON));
}

#[test]
fn test_clone_partial_eq_multi_qubit_ms() {
    let qubits = vec![0, 1, 2];

    let gate = MultiQubitZZ::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    assert_eq!(gate.hqslang(), "MultiQubitZZ");
    assert_eq!(
        gate.tags(),
        &[
            "Operation",
            "GateOperation",
            "MultiQubitGateOperation",
            "MultiQubitZZ",
        ]
    );
    assert!(!gate.is_parametrized());
    let gate1 = MultiQubitZZ::new(qubits, "theta".into());
    assert!(gate1.is_parametrized());
    let helper = gate != gate1;
    assert!(helper);
    #[allow(clippy::redundant_clone)]
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

#[test]
fn test_operate_multi_qubit_zz() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitZZ::new(qubits.clone(), CalculatorFloat::FRAC_PI_2);
    assert_eq!(gate.hqslang(), "MultiQubitZZ");
    assert_eq!(
        gate.tags(),
        &[
            "Operation",
            "GateOperation",
            "MultiQubitGateOperation",
            "MultiQubitZZ",
        ]
    );
    assert_eq!(gate.qubits(), &vec![0, 1, 2]);
    assert!(!gate.is_parametrized());
    let gate1 = MultiQubitZZ::new(qubits, "theta".into());
    assert!(gate1.is_parametrized());
}

#[test]
fn test_substitute_multi_qubit_zz() {
    let qubits = vec![0, 1, 2];
    let gate1 = MultiQubitZZ::new(qubits.clone(), "theta".into());
    let gate = MultiQubitZZ::new(qubits, CalculatorFloat::FRAC_PI_2);
    let mut calc = Calculator::new();
    calc.set_variable("theta", std::f64::consts::FRAC_PI_2);
    let gate_substituted = roqoqo::operations::Substitute::substitute_parameters(&gate1, &calc);
    let subs = gate_substituted.unwrap();
    assert_eq!(gate, subs);
    let mut mapping: HashMap<usize, usize> = std::collections::HashMap::new();
    let _ = mapping.insert(0, 1);
    let _ = mapping.insert(1, 2);
    let _ = mapping.insert(2, 0);
    let remapped = gate1.remap_qubits(&mapping).unwrap();
    let qubits = remapped.qubits();
    assert_eq!(qubits, &vec![1, 2, 0]);
}

#[test]
fn test_substitute_error_multi_qubit_zz() {
    let qubits = vec![0, 1, 2];
    let gate1 = MultiQubitZZ::new(qubits, "theta".into());
    let calc = Calculator::new();
    let gate_substituted = gate1.substitute_parameters(&calc);
    assert!(gate_substituted.is_err());
    let mut mapping: HashMap<usize, usize> = std::collections::HashMap::new();
    let _ = mapping.insert(1, 2);
    let _ = mapping.insert(2, 0);
    let remapped = gate1.remap_qubits(&mapping);
    if !cfg!(feature = "unstable_remapping_validity_check") {
        assert!(remapped.is_err());
    } else {
        assert!(remapped.is_ok());
    }
}

#[test]
fn test_format_error_multi_qubit_zz() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitZZ::new(qubits, "theta".into());
    let string = format!("{:?}", gate);
    assert!(string.contains("theta"));
    assert!(string.contains("MultiQubitZZ"));
}

#[test]
fn test_involved_qubits_multi_qubit_zz() {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitZZ::new(qubits, "theta".into());
    let involved_qubits = gate.involved_qubits();
    let mut comp_set: HashSet<usize> = HashSet::new();
    let _ = comp_set.insert(0);
    let _ = comp_set.insert(1);
    let _ = comp_set.insert(2);
    assert_eq!(involved_qubits, InvolvedQubits::Set(comp_set));
}

/// Test powerfc function for MultiQubitMS with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(2.0); "power_2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0 / 2.0); "power_1/2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0); "power_1")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(0.0); "power_0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(-2.0); "power_-2.0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_rotatex_powercf_multi_qubit_zz(theta: CalculatorFloat, power: CalculatorFloat) {
    let qubits = vec![0, 1, 2];
    let gate = MultiQubitZZ::new(qubits.clone(), theta);

    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = MultiQubitZZ::new(qubits, test_theta);
    assert_eq!(power_gate, test_gate);
    assert_eq!(power_gate.theta(), test_gate.theta());
}

/// Test JsonSchema trait
#[cfg(feature = "json_schema")]
#[test_case(MultiQubitGateOperation::from(MultiQubitZZ::new(vec![0, 1, 2, 3], 0.23.into())); "MultiQubitZZ")]
#[test_case(MultiQubitGateOperation::from(MultiQubitMS::new(vec![0, 1, 2], 0.45.into())); "MultiQubitMS")]
pub fn test_json_schema_multi_qubit_gate_operations(gate: MultiQubitGateOperation) {
    // Serialize
    let test_json = match gate.clone() {
        MultiQubitGateOperation::MultiQubitMS(op) => serde_json::to_string(&op).unwrap(),
        MultiQubitGateOperation::MultiQubitZZ(op) => serde_json::to_string(&op).unwrap(),
        _ => unreachable!(),
    };
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = match gate {
        MultiQubitGateOperation::MultiQubitMS(_) => schema_for!(MultiQubitMS),
        MultiQubitGateOperation::MultiQubitZZ(_) => schema_for!(MultiQubitZZ),
        _ => unreachable!(),
    };
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
