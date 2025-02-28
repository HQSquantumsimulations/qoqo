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

use num_complex::Complex64;
use pyo3::prelude::*;
use qoqo::measurements::{
    CheatedInputWrapper, CheatedPauliZProductInputWrapper, CheatedPauliZProductWrapper,
    CheatedWrapper, ClassicalRegisterWrapper, PauliZProductInputWrapper, PauliZProductWrapper,
};
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::{
    convert_into_quantum_program, CircuitWrapper, QoqoError, QuantumProgramWrapper, QOQO_VERSION,
};
use roqoqo::measurements::{
    Cheated, CheatedInput, CheatedPauliZProduct, CheatedPauliZProductInput, ClassicalRegister,
    PauliZProduct, PauliZProductInput,
};
use roqoqo::operations::Operation;
use roqoqo::operations::*;
use roqoqo::{Circuit, QuantumProgram, ROQOQO_VERSION};

#[pyclass(name = "TestBackend", module = "qoqo")]
#[derive(Debug, Clone, Copy)]
struct TestBackend;

#[pymethods]
impl TestBackend {
    fn run_measurement(&self, measurement: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Ok(measurement)
    }

    fn run_measurement_registers(&self, measurement: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Ok(measurement)
    }
}

fn create_measurement(py: Python) -> Bound<CheatedPauliZProductWrapper> {
    let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
    let binding = input_type.call0().unwrap();
    let input = binding
        .downcast::<CheatedPauliZProductInputWrapper>()
        .unwrap();
    let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

    let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br_type = py.get_type::<CheatedPauliZProductWrapper>();
    br_type
        .call1((Some(CircuitWrapper::new()), circs.clone(), input))
        .unwrap()
        .downcast::<CheatedPauliZProductWrapper>()
        .unwrap()
        .to_owned()
}

/// Test basic traits of QuantumProgramWrapper
#[test]
fn test_basic_traits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let helper_ne: bool =
            QuantumProgramWrapper::new(&input, vec!["error".into()]).unwrap() != program_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            QuantumProgramWrapper::new(&input, vec!["test".into()]).unwrap() == program_wrapper;
        assert!(helper_eq);

        let helper_eq: bool = program_wrapper == program_wrapper;
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", QuantumProgramWrapper::new(&input, vec!["test".into()]).unwrap()),
            "QuantumProgramWrapper { internal: CheatedPauliZProduct { measurement: CheatedPauliZProduct { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }, Circuit { definitions: [], operations: [RotateX(RotateX { qubit: 0, theta: Float(0.0) })], _roqoqo_version: RoqoqoVersion }], input: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } }, input_parameter_names: [\"test\"] } }"
        );
    })
}

/// Test new and run functions of QuantumProgram with all PauliZProduct measurement input
#[test]
fn test_new_run_br() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<PauliZProductInputWrapper>();
        let binding = input_type.call1((3, false)).unwrap();
        let input_instance = binding.downcast::<PauliZProductInputWrapper>().unwrap();
        let _ = input_instance
            .call_method1("add_pauliz_product", ("ro", vec![0]))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += RotateX::new(0, 0.0.into());
        circs.push(circ1.clone());
        let br_type = py.get_type::<PauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
            .unwrap();
        let input = binding.downcast::<PauliZProductWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let mut bri = PauliZProductInput::new(3, false);
        let _ = bri.add_pauliz_product("ro".to_string(), vec![0]);
        let br = PauliZProduct {
            constant_circuit: Some(Circuit::new()),
            circuits: vec![Circuit::new(), circ1.internal],
            input: bri,
        };

        assert_eq!(
            program_wrapper,
            QuantumProgramWrapper {
                internal: QuantumProgram::PauliZProduct {
                    measurement: br.clone(),
                    input_parameter_names: vec!["test".to_string()]
                }
            }
        );

        let measurement = PauliZProductWrapper::extract_bound(
            &program
                .call_method1("run", (TestBackend, Some(vec![0.0])))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(measurement.internal, br);
    })
}

/// Test new and run functions of QuantumProgram with all CheatedPauliZProduct measurement input
#[test]
fn test_new_run_cheated_br() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input_instance = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input_instance
            .call_method1("add_pauliz_product", ("ro",))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += RotateX::new(0, 0.0.into());
        circs.push(circ1.clone());
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
            .unwrap();
        let input = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let mut cbri = CheatedPauliZProductInput::new();
        cbri.add_pauliz_product("ro".to_string());
        let cbr = CheatedPauliZProduct {
            constant_circuit: Some(Circuit::new()),
            circuits: vec![Circuit::new(), circ1.internal],
            input: cbri,
        };

        assert_eq!(
            program_wrapper,
            QuantumProgramWrapper {
                internal: QuantumProgram::CheatedPauliZProduct {
                    measurement: cbr.clone(),
                    input_parameter_names: vec!["test".to_string()]
                }
            }
        );

        let measurement = CheatedPauliZProductWrapper::extract_bound(
            &program
                .call_method1("run", (TestBackend, Some(vec![0.0])))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(measurement.internal, cbr);
        let error = program.call_method1("run_registers", (TestBackend, Some(vec![0.0])));
        assert!(error.is_err());
    })
}

/// Test new and run functions of QuantumProgram with all Cheated measurement input
#[test]
fn test_new_run_cheated() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((2,)).unwrap();
        let input_instance = binding.downcast::<CheatedInputWrapper>().unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += RotateX::new(0, 0.0.into());
        circs.push(circ1.clone());
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
            .unwrap();
        let input = binding.downcast::<CheatedWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let ci = CheatedInput::new(2);
        let cheated = Cheated {
            constant_circuit: Some(Circuit::new()),
            circuits: vec![Circuit::new(), circ1.internal],
            input: ci,
        };

        assert_eq!(
            program_wrapper,
            QuantumProgramWrapper {
                internal: QuantumProgram::Cheated {
                    measurement: cheated.clone(),
                    input_parameter_names: vec!["test".to_string()]
                }
            }
        );

        let measurement = CheatedWrapper::extract_bound(
            &program
                .call_method1("run", (TestBackend, Some(vec![0.0])))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(measurement.internal, cheated);
    })
}

/// Test new and run_register functions of QuantumProgram with all ClassicalRegister measurement input
#[test]
fn test_new_run_classical_register() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += RotateX::new(0, 0.0.into());
        circs.push(circ1.clone());
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let input = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let cr = ClassicalRegister {
            constant_circuit: Some(Circuit::new()),
            circuits: vec![Circuit::new(), circ1.internal],
        };

        assert_eq!(
            program_wrapper,
            QuantumProgramWrapper {
                internal: QuantumProgram::ClassicalRegister {
                    measurement: cr.clone(),
                    input_parameter_names: vec!["test".to_string()]
                }
            }
        );

        let measurement = ClassicalRegisterWrapper::extract_bound(
            &program
                .call_method1("run_registers", (TestBackend, Some(vec![0.0])))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(measurement.internal, cr);
        let error = program.call_method1("run", (TestBackend, Some(vec![0.0])));
        assert!(error.is_err());
    })
}

/// Test new function of QuantumProgram first error
#[test]
fn test_new_error_1() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circs: Vec<CircuitWrapper> = Vec::new();
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let program = program_type.call1((circs, vec!["test".to_string()]));
        assert!(program.is_err());
    })
}

/// Test copy and deepcopy functions of QuantumProgram
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let copy_circ = program.call_method0("__copy__").unwrap();
        let deepcopy_circ = program.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = program;

        let comparison_copy = bool::extract_bound(
            &copy_circ
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_circ
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test qoqo_versions function of QuantumProgram
#[test]
fn test_qoqo_versions() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let mut rsplit = ROQOQO_VERSION.split('.').take(2);
        let mut qsplit = QOQO_VERSION.split('.').take(2);
        let rver = format!(
            "{}.{}",
            rsplit.next().expect("ROQOQO_VERSION badly formatted"),
            rsplit.next().expect("ROQOQO_VERSION badly formatted")
        );
        let qver = format!(
            "{}.{}",
            qsplit.next().expect("QOQO_VERSION badly formatted"),
            qsplit.next().expect("QOQO_VERSION badly formatted")
        );

        let comparison_copy: Vec<String> =
            Vec::extract_bound(&program.call_method0("_qoqo_versions").unwrap()).unwrap();
        assert_eq!(comparison_copy, vec![rver.as_str(), qver.as_str()]);
    })
}

/// Test to_ and from_bincode functions of QuantumProgram
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        // testing 'to_bincode' and 'from_bincode' functions
        let serialised = program.call_method0("to_bincode").unwrap();
        let binding = program_type
            .call1((&input, vec!["new".to_string()]))
            .unwrap();
        let new = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let deserialised = new.call_method1("from_bincode", (&serialised,)).unwrap();
        let comparison = bool::extract_bound(
            &deserialised
                .call_method1("__eq__", (program.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        // testing that 'from_bincode' can be called directly on a QuantumProgram (python staticmethod)
        let deserialised_py = program_type
            .call_method1("from_bincode", (&serialised,))
            .unwrap();

        let comparison =
            bool::extract_bound(&deserialised_py.call_method1("__eq__", (program,)).unwrap())
                .unwrap();
        assert!(comparison);
    })
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let program_clone = program;
        let serialised = program.call_method0("to_bincode").unwrap();
        let binding = program_clone
            .call_method1("from_bincode", (&serialised,))
            .unwrap();
        let deserialised = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let binding = program_type
            .call1((&input, vec!["new".to_string()]))
            .unwrap();
        let new = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let deserialised_error = new.call_method1("from_bincode", (deserialised,));
        assert!(deserialised_error.is_err());
    })
}

/// Test to_ and from_json functions of QuantumProgram
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        // testing 'from_json' and 'to_json' functions
        let serialised = program.call_method0("to_json").unwrap();
        let binding = program_type
            .call1((&input, vec!["new".to_string()]))
            .unwrap();
        let new = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let deserialised = new.call_method1("from_json", (&serialised,)).unwrap();
        let comparison =
            bool::extract_bound(&deserialised.call_method1("__eq__", (program,)).unwrap()).unwrap();
        assert!(comparison);

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        // testing that 'from_json' can be called directly on a QuantumProgram (python staticmethod)
        let deserialised_py = program_type
            .call_method1("from_json", (&serialised,))
            .unwrap();

        let comparison =
            bool::extract_bound(&deserialised_py.call_method1("__eq__", (program,)).unwrap())
                .unwrap();
        assert!(comparison);
    })
}

/// Test json_schema function of QuantumProgram
#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let schema: String =
            String::extract_bound(&program.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(QuantumProgram)).unwrap();
        assert_eq!(schema, rust_schema);
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["one".to_string()]))
            .unwrap();
        let program_one = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let binding = program_type
            .call1((&input, vec!["two".to_string()]))
            .unwrap();
        let program_two = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let operation1 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

        let comparison = bool::extract_bound(
            &program_one
                .call_method1("__eq__", (program_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);
        let comparison = bool::extract_bound(
            &program_one
                .call_method1("__eq__", (operation1.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&program_one.call_method1("__ne__", (program_two,)).unwrap())
                .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &program_one
                .call_method1("__ne__", (operation1.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = program_one.call_method1("__ge__", (&operation1,));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_convert_into_program() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let added_op = Operation::from(PauliX::new(0));
        let operation = convert_operation_to_pyobject(added_op).unwrap();
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((&input, vec!["one".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();
        let comparison =
            program.call_method1("convert_into_quantum_program", (operation.clone_ref(py),));
        assert!(comparison.is_err());
        assert_eq!(
            convert_into_quantum_program(operation.bind(py)),
            Err(QoqoError::CannotExtractObject)
        );
        // assert_eq!(convert_into_quantum_program(circ), Err(QoqoError::VersionMismatch));
    })
}

/// Test measurement() for CheatedPauliZProduct
#[test]
fn test_return_measurement_cheatedpaulizproduct() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Create measurement
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let measurement_input = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((measurement_input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let binding = program.call_method0("measurement").unwrap();
        let measurement_returned = &binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        assert_eq!(
            format!("{:?}", measurement_returned),
            format!("{:?}", measurement_input),
        );
    })
}

/// Test measurement() for PauliZProduct
#[test]
fn test_return_measurement_paulizproduct() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Create measurement
        let input_type = py.get_type::<PauliZProductInputWrapper>();
        let binding = input_type.call1((3, false)).unwrap();
        let input = binding.downcast::<PauliZProductInputWrapper>().unwrap();
        let tmp_vec: Vec<usize> = Vec::new();
        let _ = input
            .call_method1("add_pauliz_product", ("ro", tmp_vec))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<PauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let measurement_input = binding.downcast::<PauliZProductWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((measurement_input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let binding = program.call_method0("measurement").unwrap();
        let measurement_returned = binding.downcast::<PauliZProductWrapper>().unwrap();

        assert_eq!(
            format!("{:?}", measurement_returned),
            format!("{:?}", measurement_input),
        );
    })
}

/// Test measurement() for Cheated
#[test]
fn test_return_measurement_cheated() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Create measurement
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let measurement_input = binding.downcast::<CheatedWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((measurement_input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let binding = program.call_method0("measurement").unwrap();
        let measurement_returned = binding.downcast::<CheatedWrapper>().unwrap();

        assert_eq!(
            format!("{:?}", measurement_returned),
            format!("{:?}", measurement_input),
        );
    })
}

/// Test measurement() for ClassicalRegister
#[test]
fn test_return_measurement_classicalreg() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Create measurement
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let measurement_input = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((measurement_input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let binding = program.call_method0("measurement").unwrap();
        let measurement_returned = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        assert_eq!(
            format!("{:?}", measurement_returned),
            format!("{:?}", measurement_input),
        );
    })
}

/// Test input_parameter_names()
#[test]
fn test_input_parameter_names() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Create measurement
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "test".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let measurement_input = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let program_type = py.get_type::<QuantumProgramWrapper>();
        let binding = program_type
            .call1((measurement_input, vec!["test".to_string()]))
            .unwrap();
        let program = binding.downcast::<QuantumProgramWrapper>().unwrap();

        let params_returned = program.call_method0("input_parameter_names").unwrap();

        assert_eq!(params_returned.to_string(), "['test']".to_string());
    })
}
