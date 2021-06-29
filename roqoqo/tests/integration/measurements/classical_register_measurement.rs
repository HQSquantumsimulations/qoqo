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

//! Integration test for public API of classical register measurement

use std::collections::HashMap;

use roqoqo::measurements::ClassicalRegister;
use roqoqo::operations;
use roqoqo::prelude::*;
use roqoqo::Circuit;
#[test]
fn test_returning_circuits() {
    let mut circs: Vec<Circuit> = Vec::new();
    circs.push(Circuit::new());
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = ClassicalRegister {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
    };
    for (index, b) in br.circuits().enumerate() {
        assert_eq!(b, circs.get(index).unwrap());
    }
    assert_eq!(&Circuit::new(), br.constant_circuit().as_ref().unwrap());
}

#[test]
fn test_clone_eq_format() {
    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br = ClassicalRegister {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
    };

    let br_cloned = br.clone();
    let helper = br == br_cloned;
    assert!(helper);

    let mut circs: Vec<Circuit> = Vec::new();
    let mut circ1 = Circuit::new();
    circ1 += operations::RotateX::new(1, "theta".into());
    circs.push(circ1);
    let br2 = ClassicalRegister {
        constant_circuit: Some(Circuit::new()),
        circuits: circs.clone(),
    };

    let helper = br != br2;
    assert!(helper);

    assert!(format!("{:?}", br2).contains("theta"));
}

#[test]
fn test_substitute_parameters() {
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
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert("teta".to_string(), 0.0);
    map.insert("teta2".to_string(), 1.0);
    let br_substitutes = br.substitute_parameters(map);
    assert!(br_substitutes.is_err());
}
