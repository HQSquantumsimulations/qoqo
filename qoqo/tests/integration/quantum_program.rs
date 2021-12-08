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

use pyo3::prelude::*;
use qoqo::measurements::{
    BasisRotationInputWrapper, BasisRotationWrapper, CheatedBasisRotationInputWrapper,
    CheatedBasisRotationWrapper, CheatedInputWrapper, CheatedWrapper, ClassicalRegisterWrapper,
};
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::{
    convert_into_quantum_program, CircuitWrapper, QoqoError, QuantumProgramWrapper, QOQO_VERSION,
};
use roqoqo::measurements::{
    BasisRotation, BasisRotationInput, Cheated, CheatedBasisRotation, CheatedBasisRotationInput,
    CheatedInput, ClassicalRegister,
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
        return Ok(measurement);
    }
}

fn create_measurement(py: Python) -> &PyCell<CheatedBasisRotationWrapper> {
    let input_type = py.get_type::<CheatedBasisRotationInputWrapper>();
    let input = input_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CheatedBasisRotationInputWrapper>>()
        .unwrap();
    let _ = input.call_method1("add_pauli_product", ("ro",)).unwrap();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br_type = py.get_type::<CheatedBasisRotationWrapper>();
    br_type
        .call1((Some(CircuitWrapper::new()), circs.clone(), input))
        .unwrap()
        .cast_as::<PyCell<CheatedBasisRotationWrapper>>()
        .unwrap()
}

/// Test basic traits of QuantumProgramWrapper
#[test]
fn test_basic_traits() {
    Python::with_gil(|py| -> () {
        let input = create_measurement(py);
        let program_type = py.get_type::<QuantumProgramWrapper>();
        let program = program_type
            .call1((input, vec!["test".to_string()]))
            .unwrap()
            .cast_as::<PyCell<QuantumProgramWrapper>>()
            .unwrap();
        let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

        let helper_ne: bool = QuantumProgramWrapper::new(input, vec!["error".into()]).unwrap()
            != program_wrapper.clone();
        assert!(helper_ne);
        let helper_eq: bool = QuantumProgramWrapper::new(input, vec!["test".into()]).unwrap()
            == program_wrapper.clone();
        assert!(helper_eq);

        let helper_eq: bool = program_wrapper.clone() == program_wrapper;
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", QuantumProgramWrapper::new(input, vec!["test".into()]).unwrap()),
            "QuantumProgramWrapper { internal: CheatedBasisRotation { measurement: CheatedBasisRotation { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }, Circuit { definitions: [], operations: [RotateX(RotateX { qubit: 0, theta: Float(0.0) })], _roqoqo_version: RoqoqoVersion }], input: CheatedBasisRotationInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } }, input_parameter_names: [\"test\"] } }"
        );
    })
}

/// Test new and run functions of QuantumProgram with all BasisRotation measurement input
#[test]
fn test_new_run_br() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let input_type = py.get_type::<BasisRotationInputWrapper>();
    let input_instance = input_type
        .call1((3, false))
        .unwrap()
        .cast_as::<PyCell<BasisRotationInputWrapper>>()
        .unwrap();
    let _ = input_instance
        .call_method1("add_pauli_product", ("ro", vec![0]))
        .unwrap();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += RotateX::new(0, 0.0.into());
    circs.push(circ1.clone());
    let br_type = py.get_type::<BasisRotationWrapper>();
    let input = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
        .unwrap()
        .cast_as::<PyCell<BasisRotationWrapper>>()
        .unwrap();

    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

    let mut bri = BasisRotationInput::new(3, false);
    let _ = bri.add_pauli_product("ro".to_string(), vec![0]);
    let br = BasisRotation {
        constant_circuit: Some(Circuit::new()),
        circuits: vec![Circuit::new(), circ1.internal],
        input: bri,
    };

    assert_eq!(
        program_wrapper,
        QuantumProgramWrapper {
            internal: QuantumProgram::BasisRotation {
                measurement: br.clone(),
                input_parameter_names: vec!["test".to_string()]
            }
        }
    );

    let measurement = BasisRotationWrapper::extract(
        program
            .call_method1("run", (TestBackend, Some(vec![0.0])))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(measurement.internal, br);
}

/// Test new and run functions of QuantumProgram with all CheatedBasisRotation measurement input
#[test]
fn test_new_run_cheated_br() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let input_type = py.get_type::<CheatedBasisRotationInputWrapper>();
    let input_instance = input_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CheatedBasisRotationInputWrapper>>()
        .unwrap();
    let _ = input_instance
        .call_method1("add_pauli_product", ("ro",))
        .unwrap();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += RotateX::new(0, 0.0.into());
    circs.push(circ1.clone());
    let br_type = py.get_type::<CheatedBasisRotationWrapper>();
    let input = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
        .unwrap()
        .cast_as::<PyCell<CheatedBasisRotationWrapper>>()
        .unwrap();

    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let program_wrapper = program.extract::<QuantumProgramWrapper>().unwrap();

    let mut cbri = CheatedBasisRotationInput::new();
    cbri.add_pauli_product("ro".to_string());
    let cbr = CheatedBasisRotation {
        constant_circuit: Some(Circuit::new()),
        circuits: vec![Circuit::new(), circ1.internal],
        input: cbri,
    };

    assert_eq!(
        program_wrapper,
        QuantumProgramWrapper {
            internal: QuantumProgram::CheatedBasisRotation {
                measurement: cbr.clone(),
                input_parameter_names: vec!["test".to_string()]
            }
        }
    );

    let measurement = CheatedBasisRotationWrapper::extract(
        program
            .call_method1("run", (TestBackend, Some(vec![0.0])))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(measurement.internal, cbr);
    let error = program.call_method1("run_registers", (TestBackend, Some(vec![0.0])));
    assert!(error.is_err());
}

/// Test new and run functions of QuantumProgram with all Cheated measurement input
#[test]
fn test_new_run_cheated() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let input_type = py.get_type::<CheatedInputWrapper>();
    let input_instance = input_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<CheatedInputWrapper>>()
        .unwrap();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += RotateX::new(0, 0.0.into());
    circs.push(circ1.clone());
    let br_type = py.get_type::<CheatedWrapper>();
    let input = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone(), input_instance))
        .unwrap()
        .cast_as::<PyCell<CheatedWrapper>>()
        .unwrap();

    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
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

    let measurement = CheatedWrapper::extract(
        program
            .call_method1("run", (TestBackend, Some(vec![0.0])))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(measurement.internal, cheated);
}

/// Test new and run_register functions of QuantumProgram with all ClassicalRegister measurement input
#[test]
fn test_new_run_classical_register() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += RotateX::new(0, 0.0.into());
    circs.push(circ1.clone());
    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let input = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();

    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
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

    let measurement = ClassicalRegisterWrapper::extract(
        program
            .call_method1("run_registers", (TestBackend, Some(vec![0.0])))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(measurement.internal, cr);
    let error = program.call_method1("run", (TestBackend, Some(vec![0.0])));
    assert!(error.is_err());
}

/// Test new function of QuantumProgram first error
#[test]
fn test_new_error_1() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let circs: Vec<CircuitWrapper> = Vec::new();
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type.call1((circs, vec!["test".to_string()]));
    assert!(program.is_err());
}

/// Test copy and deepcopy functions of QuantumProgram
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let copy_circ = program.call_method0("__copy__").unwrap();
    let deepcopy_circ = program.call_method1("__deepcopy__", ("",)).unwrap();
    let copy_deepcopy_param = program.clone();

    let comparison_copy = bool::extract(
        copy_circ
            .call_method1("__eq__", (copy_deepcopy_param.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_copy);
    let comparison_deepcopy = bool::extract(
        deepcopy_circ
            .call_method1("__eq__", (copy_deepcopy_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_deepcopy);
}

/// Test qoqo_versions function of QuantumProgram
#[test]
fn test_qoqo_versions() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

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

    let comparison_copy: Vec<&str> =
        Vec::extract(program.call_method0("_qoqo_versions").unwrap()).unwrap();
    assert_eq!(comparison_copy, vec![rver.as_str(), qver.as_str()]);
}

/// Test to_ and from_bincode functions of QuantumProgram
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let serialised = program.call_method0("to_bincode").unwrap();
    let new = program_type
        .call1((input, vec!["new".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let deserialised = new.call_method1("from_bincode", (serialised,)).unwrap();

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

    let comparison =
        bool::extract(deserialised.call_method1("__eq__", (program,)).unwrap()).unwrap();
    assert!(comparison)
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let program_clone = program.clone();
    let serialised = program.call_method0("to_bincode").unwrap();
    let deserialised = program_clone
        .call_method1("from_bincode", (serialised,))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let new = program_type
        .call1((input, vec!["new".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let deserialised_error = new.call_method1("from_bincode", (deserialised,));
    assert!(deserialised_error.is_err());
}

/// Test to_ and from_json functions of QuantumProgram
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["test".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let serialised = program.call_method0("to_json").unwrap();
    let new = program_type
        .call1((input, vec!["new".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

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

    let comparison =
        bool::extract(deserialised.call_method1("__eq__", (program,)).unwrap()).unwrap();
    assert!(comparison)
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program_one = program_type
        .call1((input, vec!["one".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let program_two = program_type
        .call1((input, vec!["two".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();

    let operation1 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

    let comparison =
        bool::extract(program_one.call_method1("__eq__", (program_two,)).unwrap()).unwrap();
    assert!(!comparison);
    let comparison = bool::extract(
        program_one
            .call_method1("__eq__", (operation1.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(!comparison);

    let comparison =
        bool::extract(program_one.call_method1("__ne__", (program_two,)).unwrap()).unwrap();
    assert!(comparison);
    let comparison = bool::extract(
        program_one
            .call_method1("__ne__", (operation1.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let comparison = program_one.call_method1("__ge__", (operation1,));
    assert!(comparison.is_err());
}

#[test]
fn test_convert_into_program() {
    let added_op = Operation::from(PauliX::new(0));
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(added_op).unwrap();
    // let circuit_type = py.get_type::<CircuitWrapper>();
    // let circ = circuit_type
    //     .call0()
    //     .unwrap()
    //     .cast_as::<PyCell<CircuitWrapper>>()
    //     .unwrap();

    let input = create_measurement(py);
    let program_type = py.get_type::<QuantumProgramWrapper>();
    let program = program_type
        .call1((input, vec!["one".to_string()]))
        .unwrap()
        .cast_as::<PyCell<QuantumProgramWrapper>>()
        .unwrap();
    let comparison = program.call_method1("convert_into_quantum_program", (operation.clone(),));
    assert!(comparison.is_err());
    assert_eq!(
        convert_into_quantum_program(operation.as_ref(py)),
        Err(QoqoError::CannotExtractObject)
    );
    // assert_eq!(convert_into_quantum_program(circ), Err(QoqoError::VersionMismatch));
}
