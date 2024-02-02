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

//! Integration test for public API of analog operations

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};

#[cfg(feature = "json_schema")]
#[test]
fn constant_spin_hamiltonian_json_schema() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let time = CalculatorFloat::from(1.0);
    let op = ApplyConstantSpinHamiltonian::new(hamiltonian, time.clone());

    // Serialize
    let test_json = serde_json::to_string(&op).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(ApplyConstantSpinHamiltonian);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn timedependent_spin_hamiltonian_json_schema() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    let op = ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone());

    // Serialize
    let test_json = serde_json::to_string(&op).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(ApplyTimeDependentSpinHamiltonian);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "serialize")]
#[test]
fn constant_spin_hamiltonian_serde() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let time = CalculatorFloat::from(1.0);
    let op = ApplyConstantSpinHamiltonian::new(hamiltonian, time.clone());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "ApplyConstantSpinHamiltonian",
                len: 2,
            },
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "SpinHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Str("0Z"),
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(1),
            Token::Str("minor_version"),
            Token::U32(0),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("time"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "ApplyConstantSpinHamiltonian",
                len: 2,
            },
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "SpinHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleSpinOperator",
                variant: "Z",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(1),
            Token::Str("minor_version"),
            Token::U32(0),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "serialize")]
#[test]
fn timedependent_hamiltonian_serde() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    let op = ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone());
    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "ApplyTimeDependentSpinHamiltonian",
                len: 3,
            },
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "SpinHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Str("0Z"),
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(1),
            Token::Str("minor_version"),
            Token::U32(0),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("time"),
            Token::Seq { len: Some(1) },
            Token::F64(1.0),
            Token::SeqEnd,
            Token::Str("values"),
            Token::Map { len: Some(1) },
            Token::Str("omega"),
            Token::Seq { len: Some(1) },
            Token::F64(1.0),
            Token::SeqEnd,
            Token::MapEnd,
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "ApplyTimeDependentSpinHamiltonian",
                len: 3,
            },
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "SpinHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleSpinOperator",
                variant: "Z",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(1),
            Token::Str("minor_version"),
            Token::U32(0),
            Token::StructEnd,
            Token::StructEnd,
            Token::Str("time"),
            Token::Seq { len: Some(1) },
            Token::F64(1.0),
            Token::SeqEnd,
            Token::Str("values"),
            Token::Map { len: Some(1) },
            Token::Str("omega"),
            Token::Seq { len: Some(1) },
            Token::F64(1.0),
            Token::SeqEnd,
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn operate_analog_const_spin() {
    let pp = PauliProduct::new().z(0);
    let mut unparam_hamiltonian = SpinHamiltonian::new();
    unparam_hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let mut param_hamlitonian = SpinHamiltonian::new();
    param_hamlitonian
        .add_operator_product(pp, "parametrized".into())
        .unwrap();
    let time = CalculatorFloat::from(1.0);

    let name = "ApplyConstantSpinHamiltonian";
    let unparam_analog = ApplyConstantSpinHamiltonian::new(unparam_hamiltonian, time.clone());
    let param_analog = ApplyConstantSpinHamiltonian::new(param_hamlitonian, time.clone());

    // (1) Test tags functionval
    let tags: &[&str; 4] = &["Operation", "ModeGateOperation", "OperateSpinsAnalog", name];

    assert_eq!(unparam_analog.tags(), tags);
    assert_eq!(param_analog.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(unparam_analog.hqslang(), String::from(name));
    assert_eq!(param_analog.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!unparam_analog.is_parametrized());
    assert!(param_analog.is_parametrized());
}

#[test]
fn analog_spins() {
    let pp1 = PauliProduct::new().z(0).x(2);
    let pp2 = PauliProduct::new().y(3).z(0);
    let mut hamlitonian = SpinHamiltonian::new();
    hamlitonian.add_operator_product(pp1, (1.0).into()).unwrap();
    hamlitonian.add_operator_product(pp2, (1.0).into()).unwrap();
    let time = CalculatorFloat::from(1.0);

    let analog = ApplyConstantSpinHamiltonian::new(hamlitonian, time);

    assert_eq!(analog.spin(), vec![0, 2, 3]);
}

#[test]
fn operate_analog_timedependent_spin() {
    let pp = PauliProduct::new().z(0);
    let mut unparam_hamiltonian = SpinHamiltonian::new();
    unparam_hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    let mut param_hamlitonian = SpinHamiltonian::new();
    param_hamlitonian
        .add_operator_product(pp, "parametrized".into())
        .unwrap();
    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    let name = "ApplyTimeDependentSpinHamiltonian";
    let unparam_analog =
        ApplyTimeDependentSpinHamiltonian::new(unparam_hamiltonian, vec![1.0], values.clone());
    let param_analog =
        ApplyTimeDependentSpinHamiltonian::new(param_hamlitonian, vec![1.0], values.clone());

    // (1) Test tags function
    let tags: &[&str; 4] = &["Operation", "ModeGateOperation", "OperateSpinsAnalog", name];

    assert_eq!(unparam_analog.tags(), tags);
    assert_eq!(param_analog.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(unparam_analog.hqslang(), String::from(name));
    assert_eq!(param_analog.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!unparam_analog.is_parametrized());
    assert!(param_analog.is_parametrized());
}
