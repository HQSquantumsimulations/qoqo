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

//! Integration test for public API of measurement inputs
#[cfg(feature = "json_schema")]
use jsonschema::{Draft, Validator};
use num_complex::Complex64;
use roqoqo::measurements::{
    CheatedInput, CheatedPauliZProductInput, PauliProductsToExpVal, PauliZProductInput,
};
use roqoqo::RoqoqoError;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
use std::collections::HashMap;
use std::default::Default;

#[test]
fn test_pp_to_exp_val() {
    let mut map: HashMap<usize, f64> = HashMap::new();
    map.insert(0, 3.0);
    let lin = PauliProductsToExpVal::Linear(map);
    #[allow(clippy::redundant_clone)]
    let lin2 = lin.clone();
    let helper = lin == lin2;
    assert!(helper);
    let string = format!("{lin:?}");
    assert!(string.contains("3.0"));
    let sym = PauliProductsToExpVal::Symbolic("theta".into());
    let helper = sym != lin;
    assert!(helper);
}
#[test]
fn test_clone_br() {
    let bri = PauliZProductInput::new(3, false);
    #[allow(clippy::redundant_clone)]
    let bri1 = bri.clone();
    assert_eq!(bri1, bri);
    let bri2 = PauliZProductInput::new(4, true);
    let helper = bri != bri2;
    assert!(helper)
}

#[test]
fn test_clone_cbr() {
    let mut bri = CheatedPauliZProductInput::new();
    let a = bri.add_pauliz_product("test".to_string());
    assert_eq!(a, 0);
    let bri1 = bri.clone();
    assert_eq!(bri1, bri);
    let bri2 = CheatedPauliZProductInput::new();
    let helper = bri != bri2;
    assert!(helper)
}

#[test]
fn test_clone_cheated() {
    let mut bri = CheatedInput::new(2);
    let test_matrix = vec![(0, 0, Complex64::new(1.0, 0.0))];
    bri.add_operator_exp_val("test".to_string(), test_matrix, "ro".to_string())
        .unwrap();
    let bri1 = bri.clone();
    assert_eq!(bri1, bri);
    let bri2 = CheatedInput::new(2);
    let helper = bri != bri2;
    assert!(helper)
}

#[test]
fn test_format_br() {
    let bri = PauliZProductInput::new(3, false);
    let string = format!("{bri:?}");
    assert!(string.contains('3'));
    assert!(string.contains("false"));
}

#[test]
fn test_format_cbr() {
    let mut bri = CheatedPauliZProductInput::new();
    let _ = bri.add_pauliz_product("test".to_string());
    let string = format!("{bri:?}");
    assert!(string.contains("test"));
    assert!(string.contains('0'));
}

#[test]
fn test_format_cheated() {
    let mut bri = CheatedInput::new(2);
    let test_matrix = vec![(0, 0, Complex64::new(1.0, 0.0))];
    bri.add_operator_exp_val("test".to_string(), test_matrix, "ro".to_string())
        .unwrap();
    let string = format!("{bri:?}");
    assert!(string.contains("test"));
    assert!(string.contains("1.0"));
}

#[test]
fn double_insertion_br() {
    let mut bri = PauliZProductInput::new(3, false);
    let x = bri.add_pauliz_product("ro".to_string(), vec![0]).unwrap();
    assert_eq!(x, 0);
    let x = bri.add_pauliz_product("ro".to_string(), vec![1]).unwrap();
    assert_eq!(x, 1);
    let x = bri.add_pauliz_product("ro".to_string(), vec![1]).unwrap();
    assert_eq!(x, 1);
    let x = bri.add_pauliz_product("rx".to_string(), vec![1]).unwrap();
    assert_eq!(x, 2);
}

#[test]
fn double_insertion_cbr() {
    let mut bri = CheatedPauliZProductInput::new();
    let a = bri.add_pauliz_product("test".to_string());
    assert_eq!(a, 0);
    let a = bri.add_pauliz_product("test2".to_string());
    assert_eq!(a, 1);
    let a = bri.add_pauliz_product("test".to_string());
    assert_eq!(a, 0);
    let a = bri.add_pauliz_product("tset".to_string());
    assert_eq!(a, 2);
}

#[test]
fn error_br() {
    let mut bri = PauliZProductInput::new(3, false);
    let _ = bri.add_symbolic_exp_val("test".to_string(), "3.0".into());
    let a = bri.add_symbolic_exp_val("test".to_string(), "3.0".into());
    assert_eq!(
        a,
        Err(RoqoqoError::ExpValUsedTwice {
            name: "test".to_string()
        })
    );
    let a = bri.add_linear_exp_val("test".to_string(), HashMap::new());
    assert_eq!(
        a,
        Err(RoqoqoError::ExpValUsedTwice {
            name: "test".to_string()
        })
    );
    let a = bri.add_pauliz_product("tset".to_string(), vec![3]);
    assert_eq!(
        a,
        Err(RoqoqoError::PauliProductExceedsQubits {
            pp_qubit: 3,
            number_qubits: 3,
        })
    );
}

#[test]
fn error_cbr() {
    let mut bri = CheatedPauliZProductInput::new();
    let _ = bri.add_symbolic_exp_val("test".to_string(), "3.0".into());
    let a = bri.add_symbolic_exp_val("test".to_string(), "3.0".into());
    assert_eq!(
        a,
        Err(RoqoqoError::ExpValUsedTwice {
            name: "test".to_string()
        })
    );
    let a = bri.add_linear_exp_val("test".to_string(), HashMap::new());
    assert_eq!(
        a,
        Err(RoqoqoError::ExpValUsedTwice {
            name: "test".to_string()
        })
    );
}

#[test]
fn error_cheated() {
    let mut bri = CheatedInput::new(2);
    let test_matrix = vec![(4, 4, Complex64::new(1.0, 0.0))];
    let a = bri.add_operator_exp_val("test".to_string(), test_matrix, "ro".to_string());
    assert_eq!(
        a,
        Err(RoqoqoError::MismatchedOperatorDimension {
            number_qubits: 2,
            index: (4, 4)
        })
    );
    let test_matrix = vec![(0, 0, Complex64::new(1.0, 0.0))];
    bri.add_operator_exp_val("test".to_string(), test_matrix.clone(), "ro".to_string())
        .unwrap();
    let a = bri.add_operator_exp_val("test".to_string(), test_matrix, "ro".to_string());
    assert_eq!(
        a,
        Err(RoqoqoError::ExpValUsedTwice {
            name: "test".to_string()
        })
    );
}

#[test]
fn default_cbr() {
    let bri: CheatedPauliZProductInput = Default::default();
    assert_eq!(bri, CheatedPauliZProductInput::new());
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema_pp_to_exp() {
    let mut map: HashMap<usize, f64> = HashMap::new();
    map.insert(0, 3.0);
    let lin = PauliProductsToExpVal::Linear(map);
    let sym = PauliProductsToExpVal::Symbolic("theta".into());

    // Serialize
    let test_json_lin = serde_json::to_string(&lin).unwrap();
    let test_json_sym = serde_json::to_string(&sym).unwrap();
    let test_value_lin: serde_json::Value = serde_json::from_str(&test_json_lin).unwrap();
    let test_value_sym: serde_json::Value = serde_json::from_str(&test_json_sym).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(PauliProductsToExpVal);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result_lin = compiled_schema.validate(&test_value_lin);
    let validation_result_sym = compiled_schema.validate(&test_value_sym);
    assert!(validation_result_lin.is_ok());
    assert!(validation_result_sym.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema_br() {
    let op = PauliZProductInput::new(3, false);

    // Serialize
    let test_json = serde_json::to_string(&op).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(PauliZProductInput);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema_cbr() {
    let op = CheatedPauliZProductInput::new();

    // Serialize
    let test_json = serde_json::to_string(&op).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(CheatedPauliZProductInput);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema_cheated() {
    let op = CheatedInput::new(2);

    // Serialize
    let test_json = serde_json::to_string(&op).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(CheatedInput);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
