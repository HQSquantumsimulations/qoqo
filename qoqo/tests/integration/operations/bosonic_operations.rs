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

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    BeamSplitterWrapper, PhaseDisplacementWrapper, PhaseShiftWrapper, PhotonDetectionWrapper,
    SqueezingWrapper,
};
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

/// Test new() function for Squeezing
#[test_case(Operation::from(Squeezing::new(1, 0.1.into(), 0.1.into())), (1, 0.1, 0.1,), "__eq__"; "Squeezing_eq")]
#[test_case(Operation::from(Squeezing::new(1, 0.1.into(), 0.1.into())), (0, 0.1, 0.1,), "__ne__"; "Squeezing_ne")]
fn test_new_squeezing(input_operation: Operation, arguments: (u32, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SqueezingWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<SqueezingWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SqueezingWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 0.1, 0.0))
            .unwrap()
            .downcast::<PyCell<SqueezingWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SqueezingWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SqueezingWrapper { internal: Squeezing { mode: 2, squeezing: Float(0.1), phase: Float(0.0) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "squeezing")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "phase")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for PhaseDisplacement
#[test_case(Operation::from(PhaseDisplacement::new(1, 0.1.into(), 0.1.into())), (1, 0.1, 0.1,), "__eq__"; "PhaseDisplacement_eq")]
#[test_case(Operation::from(PhaseDisplacement::new(1, 0.1.into(), 0.1.into())), (0, 0.1, 0.1,), "__ne__"; "PhaseDisplacement_ne")]
fn test_new_phasedisplacement(
    input_operation: Operation,
    arguments: (u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PhaseDisplacementWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PhaseDisplacementWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PhaseDisplacementWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 0.1, 0.1))
            .unwrap()
            .downcast::<PyCell<PhaseDisplacementWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PhaseDisplacementWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseDisplacementWrapper { internal: PhaseDisplacement { mode: 2, displacement: Float(0.1), phase: Float(0.1) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "displacement")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "phase")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for PhaseShift
#[test_case(Operation::from(PhaseShift::new(1, 0.1.into())), (1, 0.1,), "__eq__"; "PhaseShift_eq")]
#[test_case(Operation::from(PhaseShift::new(1, 0.1.into())), (0, 0.1,), "__ne__"; "PhaseShift_ne")]
fn test_new_phaseshift(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PhaseShiftWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PhaseShiftWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PhaseShiftWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 0.1))
            .unwrap()
            .downcast::<PyCell<PhaseShiftWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PhaseShiftWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftWrapper { internal: PhaseShift { mode: 2, phase: Float(0.1) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "phase")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for BeamSplitter
#[test_case(Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())), (0, 1, 0.1, 0.5,), "__eq__"; "BeamSplitter_eq")]
#[test_case(Operation::from(BeamSplitter::new(1, 0, 0.1.into(), 0.5.into())), (0, 1, 0.1, 0.5,), "__ne__"; "BeamSplitter_ne")]
fn test_new_beamsplitter(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<BeamSplitterWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<BeamSplitterWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<BeamSplitterWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 1, 0.1, 0.1))
            .unwrap()
            .downcast::<PyCell<BeamSplitterWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<BeamSplitterWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "BeamSplitterWrapper { internal: BeamSplitter { mode_0: 2, mode_1: 1, theta: Float(0.1), phi: Float(0.1) } }"
        );

        let comparison_copy: bool = bool::extract(
            operation
                .call_method0(py, "theta")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_copy: bool = bool::extract(
            operation
                .call_method0(py, "phi")
                .unwrap()
                .as_ref(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(0.5)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for PhotonDetection
#[test_case(Operation::from(PhotonDetection::new(1, "ro".into(), 0)), (1, "ro".into(), 0,), "__eq__"; "PhotonDetection_eq")]
#[test_case(Operation::from(PhotonDetection::new(1, "ro".into(), 0)), (0, "ro".into(), 0,), "__ne__"; "PhotonDetection_ne")]
fn test_new_photondetection(
    input_operation: Operation,
    arguments: (u32, String, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PhotonDetectionWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PhotonDetectionWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PhotonDetectionWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, "ro", 0))
            .unwrap()
            .downcast::<PyCell<PhotonDetectionWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PhotonDetectionWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhotonDetectionWrapper { internal: PhotonDetection { mode: 2, readout: \"ro\", readout_index: 0 } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "readout")
                .unwrap()
                .as_ref(py)
                .call_method1("__eq__", ("ro",))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "readout_index")
                .unwrap()
                .as_ref(py)
                .call_method1("__eq__", (0_u32,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test is_parametrized() function for SingleModeGate Operations
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from("theta"), CalculatorFloat::from(0.0))); "Squeezing_theta")]
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(0.0), CalculatorFloat::from("phase"))); "Squeezing_phase")]
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from("theta"), CalculatorFloat::from("phase"))); "Squeezing_theta_phase")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from("theta"), 0.1.into())); "PhaseDisplacement_magnitude")]
#[test_case(Operation::from(PhaseDisplacement::new(0, 0.1.into(), CalculatorFloat::from("theta"))); "PhaseDisplacement_phase")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from("theta"))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from(0.1))); "BeamSplitter_theta")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from("phi"))); "BeamSplitter_phi")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "BeamSplitter_theta_phi")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
        .unwrap());
    })
}

/// Test is_parametrized = false for SingleModeGate Operations
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from(1.3), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), CalculatorFloat::from(1.3))); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(!bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
        .unwrap());
    })
}

/// Test mode() function for SingleMode Operations
#[test_case(0, Operation::from(Squeezing::new(0, CalculatorFloat::from(0), 0.0.into())); "Squeezing")]
#[test_case(0, Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0), 0.1.into())); "PhaseDisplacement")]
#[test_case(0, Operation::from(PhaseShift::new(0, CalculatorFloat::from(0))); "PhaseShift")]
#[test_case(0, Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_mode(mode: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mode_op: usize =
            usize::extract(operation.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_op, mode);
    })
}

/// Test mode_0() and mode_1 function for TwoMode Operations
#[test_case(0, 1, Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))); "BeamSplitter")]
fn test_pyo3_mode0_mode_1(mode_0: usize, mode_1: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mode_op_0: usize =
            usize::extract(operation.call_method0(py, "mode_0").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_op_0, mode_0);
        let mode_op_1: usize =
            usize::extract(operation.call_method0(py, "mode_1").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_op_1, mode_1);
    })
}

/// Test hqslang() function for SingleModeGate Operations
#[test_case("Squeezing", Operation::from(Squeezing::new(0, CalculatorFloat::from(0), 0.0.into())); "Squeezing")]
#[test_case("PhaseDisplacement", Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0), 0.1.into())); "PhaseDisplacement")]
#[test_case("PhaseShift", Operation::from(PhaseShift::new(0, CalculatorFloat::from(0))); "PhaseShift")]
#[test_case("BeamSplitter", Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0), CalculatorFloat::from(0))); "BeamSplitter")]
#[test_case("PhotonDetection", Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String =
            String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

/// Test tags() function for SingleModeGate Operations
#[test_case(
    Operation::from(Squeezing::new(0, CalculatorFloat::from(0), 0.0.into())),
    vec![
        "Operation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "Squeezing",
        ];
    "Squeezing")]
#[test_case(
    Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0), 0.1.into())),
    vec![
        "Operation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "PhaseDisplacement",
        ];
    "PhaseDisplacement")]
#[test_case(
    Operation::from(PhaseShift::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "PhaseShift",
        ];
    "PhaseShift")]
#[test_case(
    Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0), CalculatorFloat::from(0))),
    vec![
        "Operation",
        "ModeGateOperation",
        "TwoModeGateOperation",
        "BeamSplitter",
        ];
    "BeamSplitter")]
#[test_case(
    Operation::from(PhotonDetection::new(0, "ro".into(), 0)),
    vec![
        "Operation",
        "Measurement",
        "PhotonDetection",
        ];
    "PhotonDetection")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let tags_op: Vec<String> =
            Vec::<String>::extract(operation.call_method0(py, "tags").unwrap().as_ref(py)).unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

/// Test involved_modes() function for SingleModeGate Operations
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(1.3), 0.0.into())), HashSet::<usize>::from([0]); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), 0.1.into())), HashSet::<usize>::from([0]); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))), HashSet::<usize>::from([0]); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(1.3))), HashSet::<usize>::from([0, 1]); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)), HashSet::<usize>::from([0]); "PhotonDetection")]
fn test_pyo3_involved_modes(input_operation: Operation, modes: HashSet<usize>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let involved_modes: HashSet<usize> = HashSet::<usize>::extract(
            operation
                .call_method0(py, "involved_modes")
                .unwrap()
                .as_ref(py),
        )
        .unwrap();
        assert_eq!(involved_modes, modes);
    })
}

/// Test remap_qubits() function for SingleModeGate Operations
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(1.3), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), 0.1.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(1.3))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial qubit
        let involved_qubits: HashSet<usize> = HashSet::<usize>::extract(
            operation
                .call_method0(py, "involved_qubits")
                .unwrap()
                .as_ref(py),
        )
        .unwrap();
        assert_eq!(involved_qubits, HashSet::<usize>::new());
        // remap qubits
        let result = operation
            .call_method1(py, "remap_qubits", (HashMap::<usize, usize>::new(),))
            .unwrap();
        // test re-mapped qubit
        let involved_qubits: HashSet<usize> = HashSet::<usize>::extract(
            result
                .call_method0(py, "involved_qubits")
                .unwrap()
                .as_ref(py),
        )
        .unwrap();
        assert_eq!(involved_qubits, HashSet::<usize>::new());
    })
}

/// Test remap_modes() function for SingleModeGate Operations
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(1.3), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), 0.1.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))); "PhaseShift")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_remapmodes_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let mode: usize =
            usize::extract(operation.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode.clone(), 0);
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(0, 1);
        mode_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_modes", (mode_mapping,))
            .unwrap();
        // test re-mapped mode
        let mode_new: usize =
            usize::extract(result.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_new.clone(), 1);
        // test that initial and rempapped modes are different
        assert_ne!(mode, mode_new);
    })
}

/// Test remap_modes() function for TwoModeGate Operations
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(1.3))); "BeamSplitter")]
fn test_pyo3_remapmodes_two(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let mode_0: usize =
            usize::extract(operation.call_method0(py, "mode_0").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_0.clone(), 0);
        let mode_1: usize =
            usize::extract(operation.call_method0(py, "mode_1").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_1.clone(), 1);
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(0, 1);
        mode_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_modes", (mode_mapping,))
            .unwrap();
        // test re-mapped mode
        let mode_new_0: usize =
            usize::extract(result.call_method0(py, "mode_0").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_new_0.clone(), 1);
        let mode_new_1: usize =
            usize::extract(result.call_method0(py, "mode_1").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_new_1.clone(), 0);
        // test that initial and rempapped modes are different
        assert_ne!(mode_0, mode_new_0);
        assert_ne!(mode_1, mode_new_1);
    })
}

// test remap_modes() function returning an error.
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(1.3), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), 0.1.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(1.3))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_remapmodes_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_modes", (mode_mapping,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from(1.3), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(1.3), 0.1.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(1.3))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(1.3))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract(
            copy_op
                .as_ref(py)
                .call_method1("__eq__", (copy_deepcopy_param.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract(
            deepcopy_op
                .as_ref(py)
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test format and repr functions
#[test_case(
    "Squeezing { mode: 0, squeezing: Float(0.0), phase: Float(0.0) }",
    Operation::from(Squeezing::new(0, CalculatorFloat::from(0), 0.0.into()));
    "Squeezing")]
#[test_case(
    "PhaseDisplacement { mode: 0, displacement: Float(0.0), phase: Float(0.1) }",
    Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0), 0.1.into()));
    "PhaseDisplacement")]
#[test_case(
    "PhaseShift { mode: 0, phase: Float(0.0) }",
    Operation::from(PhaseShift::new(0, CalculatorFloat::from(0)));
    "PhaseShift")]
#[test_case(
    "BeamSplitter { mode_0: 0, mode_1: 1, theta: Float(0.0), phi: Float(0.0) }",
    Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0), CalculatorFloat::from(0)));
    "BeamSplitter")]
#[test_case(
    "PhotonDetection { mode: 0, readout: \"ro\", readout_index: 0 }",
    Operation::from(PhotonDetection::new(0, "ro".into(), 0));
    "PhotonDetection")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
        assert_eq!(repr_op, format_repr);
    })
}

/// Test substitute_parameters() function for gates having multiple parameters
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from("theta"), CalculatorFloat::from(1.0))); "Squeezing_theta")]
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from(1.0), CalculatorFloat::from("phi"))); "Squeezing_phi")]
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "Squeezing_theta_phi")]
#[test_case(Operation::from(PhaseDisplacement::new(1, CalculatorFloat::from("theta"), CalculatorFloat::from(1.0))); "PhaseDisplacement_magnitude")]
#[test_case(Operation::from(PhaseDisplacement::new(1, CalculatorFloat::from(1.0), CalculatorFloat::from("phi"))); "PhaseDisplacement_phase")]
#[test_case(Operation::from(PhaseDisplacement::new(1, CalculatorFloat::from("theta"), CalculatorFloat::from("phi"))); "PhaseDisplacement_magnitude_phase")]
#[test_case(
    Operation::from(
        BeamSplitter::new(
            0,
            1,
            CalculatorFloat::from("theta"),
            CalculatorFloat::from(0.1),
        )
    ); "BeamSplitter_theta")]
#[test_case(
    Operation::from(
        BeamSplitter::new(
            0,
            1,
            CalculatorFloat::from(0.1),
            CalculatorFloat::from("phi"),
        )
    ); "BeamSplitter_phi")]
#[test_case(
    Operation::from(
        BeamSplitter::new(
            0,
            1,
            CalculatorFloat::from("theta"),
            CalculatorFloat::from("phi"),
        )
    ); "BeamSplitter_theta_phi")]
fn test_pyo3_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
        substitution_dict_py.insert("theta", 1.0);
        substitution_dict_py.insert("phi", 0.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        substitution_dict.set_variable("phi", 0.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(PhaseShift::new(1, CalculatorFloat::from("theta"))); "PhaseShift")]
fn test_pyo3_substitute_params_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
        substitution_dict_py.insert("theta", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters() causing an error `None`
#[test_case(Operation::from(Squeezing::new(1, CalculatorFloat::from("test"), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(1, CalculatorFloat::from("test"), 0.0.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(1, CalculatorFloat::from("test"))); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.1))); "BeamSplitter")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<&str, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test substitute parameters function for SingleModeGate Operations where it has no effect
#[test_case(Operation::from(Squeezing::new(1, 0.1.into(), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(1, 0.1.into(), 0.0.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(1, 0.1.into())); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
        substitution_dict_py.insert("theta", 0.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
                .call_method1("__eq__", (operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test the __richcmp__ function
#[test_case(
    Operation::from(Squeezing::new(0, CalculatorFloat::from(0), 0.0.into())),
    Operation::from(Squeezing::new(1, CalculatorFloat::from(0), 0.0.into())); "Squeezing")]
#[test_case(
    Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0), 0.0.into())),
    Operation::from(PhaseDisplacement::new(1, CalculatorFloat::from(0), 0.0.into())); "PhaseDisplacement")]
#[test_case(
    Operation::from(PhaseShift::new(0, CalculatorFloat::from(0))),
    Operation::from(PhaseShift::new(1, CalculatorFloat::from(0))); "PhaseShift")]
#[test_case(
    Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0), CalculatorFloat::from(0))),
    Operation::from(BeamSplitter::new(1, 2, CalculatorFloat::from(0), CalculatorFloat::from(0))); "BeamSplitter")]
#[test_case(
    Operation::from(PhotonDetection::new(0, "ro".into(), 0)),
    Operation::from(PhotonDetection::new(1, "ro".into(), 0)); "PhotonDetection")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2).unwrap();

        let comparison = bool::extract(
            operation_one
                .as_ref(py)
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract(
            operation_one
                .as_ref(py)
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1(py, "__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1(py, "__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

/// Test schema-related functions for all bosonic operations
#[cfg(feature = "json_schema")]
#[test_case(Operation::from(Squeezing::new(1, 0.1.into(), 0.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseDisplacement::new(1, 0.1.into(), 0.0.into())); "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(1, 0.1.into())); "PhaseShift")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))); "BeamSplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PNRDetection")]
fn test_pyo3_json_schema(operation: Operation) {
    let rust_schema = match operation {
        Operation::Squeezing(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(Squeezing)).unwrap()
        }
        Operation::PhaseShift(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhaseShift)).unwrap()
        }
        Operation::BeamSplitter(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(BeamSplitter)).unwrap()
        }
        Operation::PhotonDetection(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhotonDetection)).unwrap()
        }
        Operation::PhaseDisplacement(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhaseDisplacement)).unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = match operation {
            Operation::PhaseDisplacement(_) => "1.8.0".to_string(),
            _ => "1.6.0".to_string(),
        };
        let pyobject = convert_operation_to_pyobject(operation).unwrap();
        let operation = pyobject.as_ref(py);

        let schema: String =
            String::extract(operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract(operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract(operation.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, minimum_version);
    });
}
