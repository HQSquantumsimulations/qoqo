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

//! Integration test for public API of spin-boson operations

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{HashMap, HashSet};
use test_case::test_case;

/// Test inputs
fn inputs() {
    let op = QuantumRabi::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));

    let op = LongitudinalCoupling::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));

    let op = JaynesCummings::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));
}

#[test_case(Operation::from(QuantumRabi::new(4, 0, 1.5.into())))]
#[test_case(Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())))]
#[test_case(Operation::from(JaynesCummings::new(4, 0, 1.5.into())))]
fn clone(op: Operation) {
    assert_eq!(op.clone(), op);
}

#[test_case(
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    "QuantumRabi(QuantumRabi { qubit: 4, mode: 0, theta: 1.5})"  
)]
#[test_case(
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
   "LongitudinalCoupling(LongitudinalCpupling { qubit: 4, mode: 0, theta: 1.5})"  
)]
#[test_case(
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
  "JaynesCummings(JaynesCummings { qubit: 4, mode: 0, theta: 1.5})"  
)]
fn debug(op: Operation, string: &str) {
    assert_eq!(format!("{:?}", op), string);
}

#[test_case(
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    Operation::from(QuantumRabi::new(2, 1, 1.0.into()))
)]
#[test_case(
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
    Operation::from(LongitudinalCoupling::new(2, 1, 1.0.into()))
)]
#[test_case(
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
    Operation::from(JaynesCummings::new(2, 1, 1.0.into()))
)]
fn partial_eq(op: Operation, op_0: Operation, op_1: Operation) {
    assert!(op_0 == op);
    assert!(op == op_0);
    assert!(op_1 != op);
    assert!(op != op_1);
}
