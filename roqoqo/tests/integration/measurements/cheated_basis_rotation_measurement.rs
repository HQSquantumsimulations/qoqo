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

//! Integration test for public API of cheated Basis rotation measurement

use std::collections::HashMap;

// use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo::{
    measurements::{CheatedPauliZProduct, CheatedPauliZProductInput},
    registers::FloatOutputRegister,
};
// use schemars::schema_for;

#[test]
fn test_returning_circuits() {
    let bri = CheatedPauliZProductInput::new();
    let mut circs: Vec<Circuit> = vec![Circuit::new()];
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = CheatedPauliZProduct {
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
    let bri = CheatedPauliZProductInput::new();
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = CheatedPauliZProduct {
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
    let br2 = CheatedPauliZProduct {
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
    let bri = CheatedPauliZProductInput::new();
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
    let br = CheatedPauliZProduct {
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
    let bri = CheatedPauliZProductInput::new();
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
    let br = CheatedPauliZProduct {
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

#[test]
fn test_evaluate_linear() {
    let mut bri = CheatedPauliZProductInput::new();
    let _ = bri.add_pauliz_product("ro_pauli_product_0".to_string());
    let _ = bri.add_pauliz_product("ro_pauli_product_1".to_string());
    let _ = bri.add_pauliz_product("ro_pauli_product_2".to_string());

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 3.0);
    bri.add_linear_exp_val("single_pp_val".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 4.0);
    linear_map.insert(1, 5.0);
    linear_map.insert(2, 6.0);
    bri.add_linear_exp_val("multi_pp_val".to_string(), linear_map)
        .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = CheatedPauliZProduct {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
    let _ = measured_registers.insert("ro_pauli_product_0".to_string(), vec![vec![1.0]]);
    let _ = measured_registers.insert("ro_pauli_product_1".to_string(), vec![vec![0.0]]);
    let _ = measured_registers.insert("ro_pauli_product_2".to_string(), vec![vec![-0.5]]);
    let result = br
        .evaluate(HashMap::new(), measured_registers, HashMap::new())
        .unwrap()
        .unwrap();
    assert_eq!(result.get("single_pp_val").unwrap(), &3.0);
    assert_eq!(result.get("multi_pp_val").unwrap(), &1.0);
}

#[test]
fn test_evaluate_symbolic() {
    let mut bri = CheatedPauliZProductInput::new();
    let _ = bri.add_pauliz_product("ro_pauli_product_0".to_string());
    let _ = bri.add_pauliz_product("ro_pauli_product_1".to_string());
    let _ = bri.add_pauliz_product("ro_pauli_product_2".to_string());
    let symbolic: CalculatorFloat =
        "sin(3.0 * pauli_product_0) + sin(-1.0 * pauli_product_1)".into();
    bri.add_symbolic_exp_val("single_pp_val".to_string(), symbolic)
        .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = CheatedPauliZProduct {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, FloatOutputRegister> = HashMap::new();

    let _ = measured_registers.insert("ro_pauli_product_0".to_string(), vec![vec![1.0]]);
    let _ = measured_registers.insert("ro_pauli_product_1".to_string(), vec![vec![-1.0]]);
    let _ = measured_registers.insert("ro_pauli_product_2".to_string(), vec![vec![-0.5]]);
    let result = br
        .evaluate(HashMap::new(), measured_registers, HashMap::new())
        .unwrap()
        .unwrap();
    assert!(
        (result.get("single_pp_val").unwrap() - (3.0_f64.sin() + 1.0_f64.sin())).abs()
            < f64::EPSILON
    );
}

// #[cfg(feature = "json_schema")]
// #[test]
// fn test_cheated_basis_rotation_json() {
//     // setting up cheated BR measurement
//     let bri = CheatedPauliZProductInput::new();
//     let mut circs: Vec<Circuit> = Vec::new();
//     let mut circ1 = Circuit::new();
//     let mut circ1_subs = Circuit::new();
//     circ1 += operations::RotateX::new(0, "theta".into());
//     circ1_subs += operations::RotateX::new(0, 0.0.into());
//     let mut circ2 = Circuit::new();
//     let mut circ2_subs = Circuit::new();
//     circ2 += operations::RotateZ::new(0, "theta2".into());
//     circ2_subs += operations::RotateZ::new(0, 1.0.into());
//     circs.push(circ1);
//     let br = CheatedPauliZProduct {
//         constant_circuit: Some(circ2),
//         circuits: circs.clone(),
//         input: bri,
//     };

//     // Serialize CheatedPauliZProduct
//     let test_json = serde_json::to_string(&br).unwrap();
//     let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

//     // Create JSONSchema
//     let test_schema = schema_for!(CheatedPauliZProduct);
//     let schema = serde_json::to_string(&test_schema).unwrap();
//     let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
//     let compiled_schema = JSONSchema::options()
//         .with_draft(Draft::Draft7)
//         .compile(&schema_value)
//         .unwrap();

//     let validation_result = compiled_schema.validate(&test_value);
//     assert!(validation_result.is_ok());
// }
