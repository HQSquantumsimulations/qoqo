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

//! Integration test for public API of cheated measurement

use std::collections::HashMap;

use jsonschema::{Draft, JSONSchema};
use num_complex::Complex64;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo::{
    measurements::{Cheated, CheatedInput},
    registers::ComplexOutputRegister,
};
use roqoqo::{operations, RoqoqoError};
use schemars::schema_for;
use test_case::test_case;

#[test]
fn test_returning_circuits() {
    let bri = CheatedInput::new(2);
    let mut circs: Vec<Circuit> = vec![Circuit::new()];
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = Cheated {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
        input: bri,
    };
    for (index, b) in br.circuits().enumerate() {
        assert_eq!(b, circs.get(index).unwrap());
    }
    assert_eq!(&Circuit::new(), br.constant_circuit().as_ref().unwrap());
}

#[test]
fn test_clone_eq_format() {
    let bri = CheatedInput::new(2);
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = Cheated {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
        input: bri.clone(),
    };

    let br_cloned = br.clone();
    let helper = br == br_cloned;
    assert!(helper);

    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(1, "theta".into());
    circs.push(circ1);
    let br2 = Cheated {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
        input: bri,
    };

    let helper = br != br2;
    assert!(helper);

    assert!(format!("{:?}", br2).contains("theta"));
}

#[test]
fn test_substitute_parameters() {
    let bri = CheatedInput::new(2);
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    let mut circ1_subs = Circuit::new();
    circ1 += operations::RotateX::new(0, "theta".into());
    circ1_subs += operations::RotateX::new(0, 0.0.into());
    let mut circ2 = Circuit::new();
    let mut circ2_subs = Circuit::new();
    circ2 += operations::RotateZ::new(0, "theta2".into());
    circ2_subs += operations::RotateZ::new(0, 1.0.into());
    circs.push(circ1);
    let br = Cheated {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert("theta".to_string(), 0.0);
    map.insert("theta2".to_string(), 1.0);
    let br_substitutes = br.substitute_parameters(map).unwrap();
    for b in br_substitutes.circuits() {
        assert_eq!(b, &circ1_subs);
    }
    assert_eq!(
        &circ2_subs,
        br_substitutes.constant_circuit().as_ref().unwrap()
    );
}

#[test]
fn test_substitute_parameters_fail() {
    let bri = CheatedInput::new(2);
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    let mut circ1_subs = Circuit::new();
    circ1 += operations::RotateX::new(0, "theta".into());
    circ1_subs += operations::RotateX::new(0, 0.0.into());
    let mut circ2 = Circuit::new();
    let mut circ2_subs = Circuit::new();
    circ2 += operations::RotateZ::new(0, "theta2".into());
    circ2_subs += operations::RotateZ::new(0, 1.0.into());
    circs.push(circ1);
    let br = Cheated {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert("teta".to_string(), 0.0);
    map.insert("teta2".to_string(), 1.0);
    let br_substitutes = br.substitute_parameters(map);
    assert!(br_substitutes.is_err());
}

#[test_case(vec![vec![Complex64::new(1.0,0.0), Complex64::new(0.0,0.0)]], 1.0, 0.0; "simple_state_diagonal")]
#[test_case(vec![vec![Complex64::new(0.0,0.0), Complex64::new(0.0,1.0)]], -1.0, 0.0; "simple_state_2_diagonal")]
#[test_case(vec![vec![Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0), Complex64::new(0.0,std::f64::consts::FRAC_1_SQRT_2)]], 0.0, 1.0; "simple_state_off_diagonal")]
#[test_case(vec![vec![Complex64::new(0.5, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.5, 0.0)]], 0.0, 0.0; "density_matrix_diagonal")]
#[test_case(vec![vec![Complex64::new(0.5, 0.0), Complex64::new(0.0, -0.5), Complex64::new(0.0, 0.5), Complex64::new(0.5, 0.0)]], 0.0, 1.0; "density_matrix_off_diagonal")]
#[test_case(vec![vec![Complex64::new(1.0,0.0), Complex64::new(0.0,0.0)], vec![Complex64::new(0.0,0.0), Complex64::new(1.0,0.0)], vec![Complex64::new(1.0,0.0), Complex64::new(0.0,0.0)]], 1.0/3.0, 0.0; "simple_state_diagonal_averaginv")]
fn test_evaluate(register: Vec<Vec<Complex64>>, value_diagonal: f64, value_off_diagonal: f64) {
    let mut bri = CheatedInput::new(1);
    let test_matrix = vec![
        (0, 0, Complex64::new(1.0, 0.0)),
        (0, 1, Complex64::new(0.0, 0.0)),
        (1, 0, Complex64::new(0.0, 0.0)),
        (1, 1, Complex64::new(-1.0, 0.0)),
    ];
    bri.add_operator_exp_val("test_diagonal".to_string(), test_matrix, "ro".to_string())
        .unwrap();
    let test_matrix = vec![
        (0, 0, Complex64::new(0.0, 0.0)),
        (0, 1, Complex64::new(0.0, -1.0)),
        (1, 0, Complex64::new(0.0, 1.0)),
        (1, 1, Complex64::new(0.0, 0.0)),
    ];
    bri.add_operator_exp_val(
        "test_off_diagonal".to_string(),
        test_matrix,
        "ro".to_string(),
    )
    .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = Cheated {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
    let _ = measured_registers.insert("ro".to_string(), register);
    let result = br
        .evaluate(HashMap::new(), HashMap::new(), measured_registers)
        .unwrap()
        .unwrap();
    assert!((result.get("test_diagonal").unwrap() - value_diagonal).abs() < 1e-10);
    assert!((result.get("test_off_diagonal").unwrap() - value_off_diagonal).abs() < 1e-10);
}

#[test]
fn test_evaluate_error() {
    let register = vec![vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
    ]];
    let mut bri = CheatedInput::new(1);
    let test_matrix = vec![
        (0, 0, Complex64::new(1.0, 0.0)),
        (0, 1, Complex64::new(0.0, 0.0)),
        (1, 0, Complex64::new(0.0, 0.0)),
        (1, 1, Complex64::new(-1.0, 0.0)),
    ];
    bri.add_operator_exp_val("test_diagonal".to_string(), test_matrix, "ro".to_string())
        .unwrap();
    let test_matrix = vec![
        (0, 0, Complex64::new(0.0, 0.0)),
        (0, 1, Complex64::new(0.0, -1.0)),
        (1, 0, Complex64::new(0.0, 1.0)),
        (1, 1, Complex64::new(0.0, 0.0)),
    ];
    bri.add_operator_exp_val(
        "test_off_diagonal".to_string(),
        test_matrix,
        "ro".to_string(),
    )
    .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = Cheated {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
    let _ = measured_registers.insert("ro".to_string(), register.clone());
    let result = br.evaluate(HashMap::new(), HashMap::new(), measured_registers);
    assert_eq!(
        result,
        Err(RoqoqoError::MismatchedRegisterDimension {
            dim: 3,
            number_qubits: 1
        })
    );

    let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
    let _ = measured_registers.insert("rx".to_string(), register);
    let result = br.evaluate(HashMap::new(), HashMap::new(), measured_registers);
    assert_eq!(
        result,
        Err(RoqoqoError::MissingRegister {
            name: "ro".to_string()
        })
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_cheated_json() {
    // setting up cheated measurement
    let bri = CheatedInput::new(2);
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    let mut circ1_subs = Circuit::new();
    circ1 += operations::RotateX::new(0, "theta".into());
    circ1_subs += operations::RotateX::new(0, 0.0.into());
    let mut circ2 = Circuit::new();
    let mut circ2_subs = Circuit::new();
    circ2 += operations::RotateZ::new(0, "theta2".into());
    circ2_subs += operations::RotateZ::new(0, 1.0.into());
    circs.push(circ1);
    let br = Cheated {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };

    // Serialize Measurement
    let test_json = serde_json::to_string(&br).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(Cheated);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
