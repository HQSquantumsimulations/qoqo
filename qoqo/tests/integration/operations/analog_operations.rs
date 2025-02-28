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

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    ApplyConstantSpinHamiltonianWrapper, ApplyTimeDependentSpinHamiltonianWrapper,
};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinHamiltonian};
use struqture_py::spins::SpinHamiltonianSystemWrapper;
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
    ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into())
}

fn create_apply_constant_spin_hamiltonian_spin_test() -> ApplyConstantSpinHamiltonian {
    let pp = PauliProduct::new().z(0).x(2).y(4);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into())
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

    ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

fn create_apply_timedependent_spin_hamiltonian_spin_test() -> ApplyTimeDependentSpinHamiltonian {
    let pp = PauliProduct::new().z(0).x(2).y(4);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from("omega"))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

fn new_system(py: Python, number_spins: Option<usize>) -> Bound<SpinHamiltonianSystemWrapper> {
    let system_type = py.get_type::<SpinHamiltonianSystemWrapper>();
    system_type
        .call1((number_spins,))
        .unwrap()
        .downcast::<SpinHamiltonianSystemWrapper>()
        .unwrap()
        .to_owned()
}

/// Test new() function for ApplyConstantSpinHamiltonian
#[test]
fn test_new_constantspinhamiltionian() {
    let input_operation = Operation::from(create_apply_constant_spin_hamiltonian(1.0));
    let method = "__eq__";
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let new_system = new_system(py, number_spins);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system
            .extract::<SpinHamiltonianSystemWrapper>()
            .unwrap();

        let operation_type = py.get_type::<ApplyConstantSpinHamiltonianWrapper>();
        let binding = operation_type.call1((system_wrapper.clone(), 1.0)).unwrap();
        let operation_py = binding
            .downcast::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();
        let binding = operation_type.call1((system_wrapper.clone(), 0.0)).unwrap();
        let new_op_diff = binding
            .downcast::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyConstantSpinHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ApplyConstantSpinHamiltonianWrapper { internal: ApplyConstantSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(0.0) } }"
        );
    })
}

/// Test new() function for ApplyTimeDepenendentSpinHamiltonian
#[test]
fn test_new_timedependentspinhamiltionian() {
    let input_operation = Operation::from(create_apply_timedependent_spin_hamiltonian(1.0));
    let method = "__eq__";
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let new_system = new_system(py, number_spins);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system
            .extract::<SpinHamiltonianSystemWrapper>()
            .unwrap();

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![1.0]);

        let operation_type = py.get_type::<ApplyTimeDependentSpinHamiltonianWrapper>();
        let binding = operation_type
            .call1((system_wrapper.clone(), vec![1.0], values.clone()))
            .unwrap();
        let operation_py = binding
            .downcast::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![0.1]);

        let def_wrapper = operation_py
            .extract::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();
        let binding = operation_type
            .call1((system_wrapper.clone(), vec![0.1], values.clone()))
            .unwrap();
        let new_op_diff = binding
            .downcast::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyTimeDependentSpinHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ApplyTimeDependentSpinHamiltonianWrapper { internal: ApplyTimeDependentSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: [0.1], values: {\"omega\": [0.1]} } }"
        );
    })
}

/// Test is_parametrized() function for Analog Operations
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "constant_spin_hamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "time_depenent")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test is_parametrized = false for Analog Operations
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "constant_spin_hamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian(1.0)); "time_depenent")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test hqslang() function for Analog Operations
#[test_case("ApplyConstantSpinHamiltonian", Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantSpinHamiltonian")]
#[test_case("ApplyTimeDependentSpinHamiltonian", Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentSpinHamiltonian")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian(1.0)),
    vec![
        "Operation",
        "SpinsAnalogOperation",
        "ApplyConstantSpinHamiltonian",
        ];
    "ApplyConstantSpinHamiltonian")]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    vec![
        "Operation",
        "SpinsAnalogOperation",
        "ApplyTimeDependentSpinHamiltonian",
        ];
    "ApplyTimeDependentSpinHamiltonian")]
/// Test tags() function for Analog Operations
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let tags_op: Vec<String> = operation
            .call_method0(py, "tags")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantSpinHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentSpinHamiltonian")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .bind(py)
                .call_method1("__eq__", (copy_deepcopy_param.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .bind(py)
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test format and repr functions
#[test_case(
    "ApplyConstantSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(1.0) }",
    Operation::from(create_apply_constant_spin_hamiltonian(1.0));
    "ApplyConstantSpinHamiltonian")]
#[test_case(
    "ApplyTimeDependentSpinHamiltonian { hamiltonian: SpinHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Str(\"omega\")} }, time: [1.0], values: {\"omega\": [1.0]} }",
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega"));
    "ApplyTimeDependentSpinHamiltonian")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        assert_eq!(repr_op, format_repr);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantSpinHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentSpinHamiltonian_theta")]
fn test_pyo3_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();

        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();

        assert!(comparison);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantSpinHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentSpinHamiltonian_theta")]
fn test_pyo3_substitute_params_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantSpinHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentSpinHamiltonian_theta")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    Python::with_gil(|py| {
        pyo3::prepare_freethreaded_python();
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let e = binding.value_bound(py);
        assert_eq!(format!("{:?}", e), "RuntimeError('Parameter Substitution failed: CalculatorError(VariableNotSet { name: \"theta\" })')");
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(0.1)), vec![0]; "ApplyConstantSpinHamiltonian_0")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")), vec![0]; "ApplyTimeDependentSpinHamiltonian_0")]
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian_spin_test()), vec![0,2,4]; "ApplyConstantSpinHamiltonian_024")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian_spin_test()), vec![0,2,4]; "ApplyTimeDependentSpinHamiltonian_024")]
fn test_spin(input_operation: Operation, test_result: Vec<usize>) {
    Python::with_gil(|py| {
        pyo3::prepare_freethreaded_python();
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let result: Vec<usize> = operation
            .call_method1(py, "spin", ())
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(result, test_result);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantSpinHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian(1.0)); "ApplyTimeDependentSpinHamiltonian_theta")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 0.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test]
fn test_pyo3_remapqubits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let pp1 = PauliProduct::new().z(0).x(1);
        let pp2 = PauliProduct::new().z(2).x(3);
        let mut hamiltonian = SpinHamiltonian::new();
        hamiltonian
            .add_operator_product(pp1, CalculatorFloat::from(1.0))
            .unwrap();
        hamiltonian
            .add_operator_product(pp2, CalculatorFloat::from(2.0))
            .unwrap();

        let pp1 = PauliProduct::new().z(2).x(1);
        let pp2 = PauliProduct::new().z(0).x(3);
        let mut test_hamiltonian = SpinHamiltonian::new();
        test_hamiltonian
            .add_operator_product(pp1, CalculatorFloat::from(1.0))
            .unwrap();
        test_hamiltonian
            .add_operator_product(pp2, CalculatorFloat::from(2.0))
            .unwrap();

        let input_op = Operation::from(ApplyConstantSpinHamiltonian::new(
            hamiltonian.clone(),
            1.0.into(),
        ));
        let test_op = Operation::from(ApplyConstantSpinHamiltonian::new(
            test_hamiltonian.clone(),
            1.0.into(),
        ));
        let operation = convert_operation_to_pyobject(input_op).unwrap();
        let operation_2 = convert_operation_to_pyobject(test_op).unwrap();
        // remap qubits
        let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
        qubit_mapping_test.insert(0, 2);
        qubit_mapping_test.insert(2, 0);
        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping_test.clone(),))
            .unwrap();

        let comparison = bool::extract_bound(
            &result
                .bind(py)
                .call_method1("__eq__", (operation_2.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![1.0]);
        let input_op = Operation::from(ApplyTimeDependentSpinHamiltonian::new(
            hamiltonian,
            vec![1.0],
            values.clone(),
        ));
        let test_op = Operation::from(ApplyTimeDependentSpinHamiltonian::new(
            test_hamiltonian,
            vec![1.0],
            values.clone(),
        ));
        let operation = convert_operation_to_pyobject(input_op).unwrap();
        let operation_2 = convert_operation_to_pyobject(test_op).unwrap();

        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping_test,))
            .unwrap();

        let comparison = bool::extract_bound(
            &result
                .bind(py)
                .call_method1("__eq__", (operation_2.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian(1.0)),
    Operation::from(create_apply_constant_spin_hamiltonian(2.0)); "ApplyConstantSpinHamiltonian")]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega1")),
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega2")); "ApplyTimeDependentSpinHamiltonian")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2).unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .bind(py)
                .call_method1("__eq__", (operation_two.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .bind(py)
                .call_method1("__ne__", (operation_two.clone_ref(py),))
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

#[cfg(feature = "json_schema")]
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantSpinHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentSpinHamiltonian")]
fn test_pyo3_json_schema(operation: Operation) {
    let rust_schema = match operation {
        Operation::ApplyConstantSpinHamiltonian(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ApplyConstantSpinHamiltonian))
                .unwrap()
        }
        Operation::ApplyTimeDependentSpinHamiltonian(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ApplyTimeDependentSpinHamiltonian))
                .unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = "1.11.0".to_string();
        let pyobject = convert_operation_to_pyobject(operation).unwrap();
        let operation = pyobject.bind(py);

        let schema: String =
            String::extract_bound(&operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let minimum_supported_version_string =
            String::extract_bound(&operation.call_method0("min_supported_version").unwrap())
                .unwrap();
        let current_version_string =
            String::extract_bound(&operation.call_method0("current_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, minimum_version);
    });
}
