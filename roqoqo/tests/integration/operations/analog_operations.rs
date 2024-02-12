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
use qoqo_calculator::{Calculator, CalculatorError::VariableNotSet, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::RoqoqoError;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};
use test_case::test_case;

fn create_apply_constant_spin_hamiltonian<T>(p: T) -> ApplyConstantSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();
    return ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into());
}

fn create_apply_constant_spin_hamiltonian_param_time() -> ApplyConstantSpinHamiltonian {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    return ApplyConstantSpinHamiltonian::new(hamiltonian, "time".into());
}

fn create_param_apply_constant_spin_hamiltonian<T>(p: T) -> ApplyConstantSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp, CalculatorFloat::from(p))
        .unwrap();
    let pp = PauliProduct::new().x(1);
    hamiltonian.add_operator_product(pp, 1.0.into()).unwrap();

    return ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into());
}

fn create_apply_timedependent_spin_hamiltonian<T>(p: T) -> ApplyTimeDependentSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    return ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone());
}

/// Test inputs
#[test]
fn inputs() {
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();

    let op = create_apply_constant_spin_hamiltonian(1.0);
    assert_eq!(op.hamiltonian(), &hamiltonian);
    assert_eq!(op.time(), &CalculatorFloat::from(1.0));

    let op = create_apply_timedependent_spin_hamiltonian(1.0);
    assert_eq!(op.hamiltonian(), &hamiltonian);
    assert_eq!(op.time(), &(vec![1.0]));
    assert_eq!(op.values(), &values);
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian(1.0)),
    Operation::from(create_apply_constant_spin_hamiltonian(1.0)),
    Operation::from(create_apply_constant_spin_hamiltonian(2.0))
)]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    Operation::from(create_apply_timedependent_spin_hamiltonian("alpha"))
)]
fn partial_eq(op: Operation, op_0: Operation, op_1: Operation) {
    assert!(op_0 == op);
    assert!(op == op_0);
    assert!(op_1 != op);
    assert!(op != op_1);
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)))]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")))]
fn clone(op: Operation) {
    assert_eq!(op.clone(), op);
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)), "ApplyConstantSpinHamiltonian(ApplyConstantSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(1.0) })")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")), "ApplyTimeDependentSpinHamiltonian(ApplyTimeDependentSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Str(\"omega\")} }, time: [1.0], values: {\"omega\": [1.0]} })")]
fn debug(op: Operation, string: &str) {
    assert_eq!(format!("{:?}", op), string);
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian("omega")),
    Operation::from(create_apply_constant_spin_hamiltonian(1.5))
)]
#[test_case(
    Operation::from(create_param_apply_constant_spin_hamiltonian("omega")),
    Operation::from(create_param_apply_constant_spin_hamiltonian(1.5))
)]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    Operation::from(create_apply_timedependent_spin_hamiltonian(1.5))
)]
fn substitute(op: Operation, op_test: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("omega", 1.5);

    // (1) Substitute parameters function
    let result = op.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, op_test);
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian("omega")),
    "omega"
)]
#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian_param_time()),
    "time"
)]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    "omega"
)]
fn test_substitute_parameters_error(op: Operation, val: &str) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("error", 0.0);
    let result = op.substitute_parameters(&substitution_dict);
    assert!(result.is_err());
    let e = result.unwrap_err();
    assert_eq!(
        e,
        RoqoqoError::CalculatorError(VariableNotSet {
            name: val.to_string()
        })
    )
}

#[cfg(feature = "json_schema")]
#[test]
fn constant_spin_hamiltonian_json_schema() {
    let op = create_apply_constant_spin_hamiltonian(1.0);
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
    let op = create_apply_timedependent_spin_hamiltonian(1.0);

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
    let op = create_apply_constant_spin_hamiltonian(1.0);

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
    let op = create_apply_timedependent_spin_hamiltonian(1.0);

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
    let name = "ApplyConstantSpinHamiltonian";
    let unparam_analog = create_apply_constant_spin_hamiltonian(1.0);
    let param_analog = create_apply_constant_spin_hamiltonian("parametrized");

    // (1) Test tags functionval
    let tags: &[&str; 3] = &["Operation", "SpinsAnalogOperation", name];

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

    assert_eq!(analog.spin().unwrap(), vec![0, 2, 3]);
}

#[test]
fn operate_analog_timedependent_spin() {
    let name = "ApplyTimeDependentSpinHamiltonian";
    let unparam_analog = create_apply_timedependent_spin_hamiltonian(1.0);
    let param_analog = create_apply_timedependent_spin_hamiltonian("omega");
    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "SpinsAnalogOperation", name];

    assert_eq!(unparam_analog.tags(), tags);
    assert_eq!(param_analog.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(unparam_analog.hqslang(), String::from(name));
    assert_eq!(param_analog.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!unparam_analog.is_parametrized());
    assert!(param_analog.is_parametrized());
}
