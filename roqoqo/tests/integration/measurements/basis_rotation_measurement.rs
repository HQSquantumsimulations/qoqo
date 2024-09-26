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

//! Integration test for public API of Basis rotation measurement

#[cfg(feature = "jsonschema")]
use jsonschema::{Draft, Validator};
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo::{
    measurements::{PauliZProduct, PauliZProductInput},
    registers::BitOutputRegister,
};
#[cfg(feature = "jsonschema")]
use schemars::schema_for;
use std::collections::HashMap;
use test_case::test_case;

#[test]
fn test_returning_circuits() {
    let mut bri = PauliZProductInput::new(3, false);
    let _ = bri.add_pauliz_product("ro".to_string(), vec![]);
    let _ = bri.add_pauliz_product("ro".to_string(), vec![0]);
    let _ = bri.add_pauliz_product("ro".to_string(), vec![0, 1]);
    let mut circs: Vec<Circuit> = vec![Circuit::new()];
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = PauliZProduct {
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
    let bri = PauliZProductInput::new(3, false);
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = PauliZProduct {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
        input: bri.clone(),
    };

    #[allow(clippy::redundant_clone)]
    let br_cloned = br.clone();
    let helper = br == br_cloned;
    assert!(helper);

    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(1, "theta".into());
    circs.push(circ1);
    let br2 = PauliZProduct {
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
    let bri = PauliZProductInput::new(3, false);
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
    let br = PauliZProduct {
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
    let bri = PauliZProductInput::new(3, false);
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
    let br = PauliZProduct {
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

#[test_case(vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
], 3.0, 4.0, 5.0, 13.0; "All measurements zero")]
#[test_case(vec![
    vec![true, true, true],
    vec![true, true, true],
    vec![true, true, true],
], 3.0, -4.0, 5.0, -1.0; "All measurements one")]
#[test_case(vec![
    vec![true, true, true],
    vec![true, true, true],
    vec![false, false, false],
    vec![false, false, false],
], 3.0, 0.0, 5.0, 6.0; "Average 0")]
#[test_case(vec![
    vec![true, true, false],
    vec![true, true, false],
    vec![false, false, true],
    vec![false, false, true],
], 3.0, 0.0, -5.0, 6.0; "Cross correlation 0")]
fn test_evaluate_linear(
    register: Vec<Vec<bool>>,
    constant: f64,
    single_qubit_exp_val: f64,
    two_qubit_exp_val: f64,
    two_pp_exp_val: f64,
) {
    let mut bri = PauliZProductInput::new(3, false);
    let _a = bri.add_pauliz_product("ro".to_string(), vec![]);
    let _b = bri.add_pauliz_product("ro".to_string(), vec![0]);
    let _c = bri.add_pauliz_product("ro".to_string(), vec![1, 2]);
    let _d = bri.add_pauliz_product("rx".to_string(), vec![1, 2]);
    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 3.0);
    bri.add_linear_exp_val("constant".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(1, 4.0);
    bri.add_linear_exp_val("single_qubit_exp_val".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(2, 5.0);
    bri.add_linear_exp_val("two_qubit_exp_val".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 6.0);
    linear_map.insert(1, 7.0);
    bri.add_linear_exp_val("two_pp_exp_val".to_string(), linear_map)
        .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = PauliZProduct {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, BitOutputRegister> = HashMap::new();
    let new_output_register: BitOutputRegister = register; // vec![
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           // ];
    let _ = measured_registers.insert("ro".to_string(), new_output_register);
    let _ = measured_registers.insert(
        "rx".to_string(),
        vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    );
    let result = br
        .evaluate(measured_registers, HashMap::new(), HashMap::new())
        .unwrap()
        .unwrap();
    assert_eq!(result.get("constant").unwrap(), &constant);
    assert_eq!(
        result.get("single_qubit_exp_val").unwrap(),
        &single_qubit_exp_val
    );
    assert_eq!(result.get("two_qubit_exp_val").unwrap(), &two_qubit_exp_val);
    assert_eq!(result.get("two_pp_exp_val").unwrap(), &two_pp_exp_val);
}

#[test_case(vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
],vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
], 3.0, 0.0, 5.0, 6.0; "flipped the same zero")]
#[test_case(vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
],vec![
    vec![true, true, true],
    vec![true, true, true],
    vec![true, true, true],
], 3.0, 4.0, 5.0, 13.0; "flipped the opposite zero")]
#[test_case(vec![
    vec![false, false, false],
    vec![false, false, false],
],vec![
    vec![false, false, false],
    vec![true, true, true],
], 3.0, 4.0*0.5, 5.0, 6.0+7.0*0.5; "flipped half zero")]
fn test_evaluate_linear_flipped(
    register: Vec<Vec<bool>>,
    flipped_register: Vec<Vec<bool>>,
    constant: f64,
    single_qubit_exp_val: f64,
    two_qubit_exp_val: f64,
    two_pp_exp_val: f64,
) {
    let mut bri = PauliZProductInput::new(3, true);
    let _a = bri.add_pauliz_product("ro".to_string(), vec![]);
    let _b = bri.add_pauliz_product("ro".to_string(), vec![0]);
    let _c = bri.add_pauliz_product("ro".to_string(), vec![1, 2]);
    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 3.0);
    bri.add_linear_exp_val("constant".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(1, 4.0);
    bri.add_linear_exp_val("single_qubit_exp_val".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(2, 5.0);
    bri.add_linear_exp_val("two_qubit_exp_val".to_string(), linear_map)
        .unwrap();

    let mut linear_map: HashMap<usize, f64> = HashMap::new();
    linear_map.insert(0, 6.0);
    linear_map.insert(1, 7.0);
    bri.add_linear_exp_val("two_pp_exp_val".to_string(), linear_map)
        .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = PauliZProduct {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, BitOutputRegister> = HashMap::new();
    let new_output_register: BitOutputRegister = register; // vec![
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           // ];
    let _ = measured_registers.insert("ro".to_string(), new_output_register);
    let _ = measured_registers.insert("ro_flipped".to_string(), flipped_register);

    let result = br
        .evaluate(measured_registers, HashMap::new(), HashMap::new())
        .unwrap()
        .unwrap();
    assert_eq!(result.get("constant").unwrap(), &constant);
    assert_eq!(
        result.get("single_qubit_exp_val").unwrap(),
        &single_qubit_exp_val
    );
    assert_eq!(result.get("two_qubit_exp_val").unwrap(), &two_qubit_exp_val);
    assert_eq!(result.get("two_pp_exp_val").unwrap(), &two_pp_exp_val);
}

#[test_case(vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
], 3.0_f64.sin()-1.0_f64.sin() ; "All measurements zero")]
#[test_case(vec![
    vec![true, true, true],
    vec![true, true, true],
    vec![true, true, true],
], 3.0_f64.sin() + 1.0_f64.sin() ; "All measurements one")]
fn test_evaluate_symbolic(register: Vec<Vec<bool>>, constant: f64) {
    let mut bri = PauliZProductInput::new(3, false);
    let _a = bri.add_pauliz_product("ro".to_string(), vec![]);
    let _b = bri.add_pauliz_product("ro".to_string(), vec![0]);
    let _c = bri.add_pauliz_product("ro".to_string(), vec![1, 2]);
    let _d = bri.add_pauliz_product("rx".to_string(), vec![1, 2]);
    let symbolic: CalculatorFloat =
        "sin(3.0 * pauli_product_0) + sin(-1.0 * pauli_product_1)".into();
    bri.add_symbolic_exp_val("constant".to_string(), symbolic)
        .unwrap();

    let circs: Vec<Circuit> = vec![Circuit::new()];
    let br = PauliZProduct {
        constant_circuit: None,
        circuits: circs,
        input: bri,
    };

    let mut measured_registers: HashMap<String, BitOutputRegister> = HashMap::new();
    let new_output_register: BitOutputRegister = register; // vec![
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           //     vec![false, false, false],
                                                           // ];
    let _ = measured_registers.insert("ro".to_string(), new_output_register);
    let _ = measured_registers.insert(
        "rx".to_string(),
        vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    );
    let result = br
        .evaluate(measured_registers, HashMap::new(), HashMap::new())
        .unwrap()
        .unwrap();
    assert!((result.get("constant").unwrap() - constant).abs() < f64::EPSILON);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    // setting up BR measurement
    let bri = PauliZProductInput::new(3, false);
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
    let br = PauliZProduct {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };

    // Serialize PauliZProduct
    let test_json = serde_json::to_string(&br).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(PauliZProduct);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
