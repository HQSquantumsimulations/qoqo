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

use std::collections::HashMap;
use roqoqo::registers::{BitOutputRegister, FloatOutputRegister, ComplexOutputRegister};
use roqoqo::operations;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo::{
    measurements::{BasisRotation, BasisRotationInput, CheatedBasisRotation, CheatedBasisRotationInput, Cheated, CheatedInput, ClassicalRegister},
};
use roqoqo::QuantumProgram;

#[derive(Debug, Clone, Copy)]
struct TestBackend;

impl EvaluatingBackend for TestBackend{
    fn run_circuit_iterator<'a>(&self, circuit: impl Iterator<Item = &'a operations::Operation>) -> roqoqo::backends::RegisterResult {
        
        let mut circ_subs = Circuit::new();
        circ_subs += operations::RotateZ::new(0, 1.0.into());
        circ_subs += operations::RotateX::new(0, 0.0.into());

        for (op_test, op_created) in circuit.zip(circ_subs.iter()){
            assert_eq!(op_test, op_created)
        }

        let result_bit: HashMap<String, BitOutputRegister> = HashMap::new();
        let result_float: HashMap<String, FloatOutputRegister> = HashMap::new();
        let result_complex: HashMap<String, ComplexOutputRegister> = HashMap::new();
        Ok((result_bit, result_float, result_complex))
    }
} 

#[test]
fn test_basis_rotation() {
    // setting ub BR measurement
    let bri = BasisRotationInput::new(3, false);
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
    let br = BasisRotation {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };

    let input_parameter_names = vec!["theta".to_string(), "theta2".to_string()];
    let program = QuantumProgram::BasisRotation{measurement:br, input_parameter_names};

    let backend = TestBackend;

    let result_fail = program.run_registers(&[0.0, 1.0], backend);
    assert!(result_fail.is_err());
    let result = program.run(&[0.0, 1.0], backend);
    assert!(result.is_ok());
    let result_fail = program.run(&[0.0], backend);
    assert!(result_fail.is_err());
    let result_fail = program.run(&[0.0, 1.0, 3.0], backend);
    assert!(result_fail.is_err());
}

#[test]
fn test_cheated_basis_rotation() {
    // setting ub BR measurement
    let bri = CheatedBasisRotationInput::new();
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
    let br = CheatedBasisRotation {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
        input: bri,
    };

    let input_parameter_names = vec!["theta".to_string(), "theta2".to_string()];
    let program = QuantumProgram::CheatedBasisRotation{measurement:br, input_parameter_names};

    let backend = TestBackend;

    let result_fail = program.run_registers(&[0.0, 1.0], backend);
    assert!(result_fail.is_err());
    let result = program.run(&[0.0, 1.0], backend);
    assert!(result.is_ok());
    let result_fail = program.run(&[0.0], backend);
    assert!(result_fail.is_err());
    let result_fail = program.run(&[0.0, 1.0, 3.0], backend);
    assert!(result_fail.is_err());
}

#[test]
fn test_cheated() {
    // setting ub BR measurement
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

    let input_parameter_names = vec!["theta".to_string(), "theta2".to_string()];
    let program = QuantumProgram::Cheated{measurement:br, input_parameter_names};

    let backend = TestBackend;

    let result_fail = program.run_registers(&[0.0, 1.0], backend);
    assert!(result_fail.is_err());
    let result = program.run(&[0.0, 1.0], backend);
    assert!(result.is_ok());
    let result_fail = program.run(&[0.0], backend);
    assert!(result_fail.is_err());
    let result_fail = program.run(&[0.0, 1.0, 3.0], backend);
    assert!(result_fail.is_err());
}

#[test]
fn test_registers() {
    // setting ub BR measurement
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
    let br = ClassicalRegister {
        constant_circuit: Some(circ2),
        circuits: circs.clone(),
    };

    let input_parameter_names = vec!["theta".to_string(), "theta2".to_string()];
    let program = QuantumProgram::ClassicalRegister{measurement:br, input_parameter_names};

    let backend = TestBackend;

    let result_fail = program.run(&[0.0, 1.0], backend);
    assert!(result_fail.is_err());
    let result = program.run_registers(&[0.0, 1.0], backend);
    assert!(result.is_ok());
    let result_fail = program.run_registers(&[0.0], backend);
    assert!(result_fail.is_err());
    let result_fail = program.run_registers(&[0.0, 1.0, 3.0], backend);
    assert!(result_fail.is_err());
}