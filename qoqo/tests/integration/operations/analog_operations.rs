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
    ApplyConstantPauliHamiltonianWrapper, ApplyTimeDependentPauliHamiltonianWrapper,
};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use std::collections::HashMap;
use struqture::prelude::*;
use struqture::spins::{PauliHamiltonian, PauliProduct};
use struqture_py::spins::PauliHamiltonianWrapper;
use test_case::test_case;

fn create_apply_constant_spin_hamiltonian<T>(p: T) -> ApplyConstantPauliHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = PauliHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();
    ApplyConstantPauliHamiltonian::new(hamiltonian, 1.0.into())
}

fn create_apply_constant_spin_hamiltonian_spin_test() -> ApplyConstantPauliHamiltonian {
    let pp = PauliProduct::new().z(0).x(2).y(4);
    let mut hamiltonian = PauliHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    ApplyConstantPauliHamiltonian::new(hamiltonian, 1.0.into())
}

fn create_apply_timedependent_spin_hamiltonian<T>(p: T) -> ApplyTimeDependentPauliHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = PauliHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    ApplyTimeDependentPauliHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

fn create_apply_timedependent_spin_hamiltonian_spin_test() -> ApplyTimeDependentPauliHamiltonian {
    let pp = PauliProduct::new().z(0).x(2).y(4);
    let mut hamiltonian = PauliHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from("omega"))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    ApplyTimeDependentPauliHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

fn new_system(py: Python) -> Bound<PauliHamiltonianWrapper> {
    let system_type = py.get_type::<PauliHamiltonianWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PauliHamiltonianWrapper>()
        .unwrap()
        .to_owned()
}

/// Test new() function for ApplyConstantPauliHamiltonian
#[test]
fn test_new_constantspinhamiltionian() {
    let input_operation = Operation::from(create_apply_constant_spin_hamiltonian(1.0));
    let method = "__eq__";
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let new_system = new_system(py);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system.extract::<PauliHamiltonianWrapper>().unwrap();

        let operation_type = py.get_type::<ApplyConstantPauliHamiltonianWrapper>();
        let binding = operation_type.call1((system_wrapper.clone(), 1.0)).unwrap();
        let operation_py = binding
            .downcast::<ApplyConstantPauliHamiltonianWrapper>()
            .unwrap();

        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<ApplyConstantPauliHamiltonianWrapper>()
            .unwrap();
        let binding = operation_type.call1((system_wrapper.clone(), 0.0)).unwrap();
        let new_op_diff = binding
            .downcast::<ApplyConstantPauliHamiltonianWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyConstantPauliHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "ApplyConstantPauliHamiltonianWrapper { internal: ApplyConstantPauliHamiltonian { hamiltonian: PauliHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(0.0) } }"
        );
    })
}

/// Test new() function for ApplyTimeDepenendentPauliHamiltonian
#[test]
fn test_new_timedependentspinhamiltionian() {
    let input_operation = Operation::from(create_apply_timedependent_spin_hamiltonian(1.0));
    let method = "__eq__";
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let new_system = new_system(py);
        new_system
            .call_method1("add_operator_product", ("0Z", 1.))
            .unwrap();
        let system_wrapper = new_system.extract::<PauliHamiltonianWrapper>().unwrap();

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![1.0]);

        let operation_type = py.get_type::<ApplyTimeDependentPauliHamiltonianWrapper>();
        let binding = operation_type
            .call1((system_wrapper.clone(), vec![1.0], values.clone()))
            .unwrap();
        let operation_py = binding
            .downcast::<ApplyTimeDependentPauliHamiltonianWrapper>()
            .unwrap();

        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![0.1]);

        let def_wrapper = operation_py
            .extract::<ApplyTimeDependentPauliHamiltonianWrapper>()
            .unwrap();
        let binding = operation_type
            .call1((system_wrapper.clone(), vec![0.1], values.clone()))
            .unwrap();
        let new_op_diff = binding
            .downcast::<ApplyTimeDependentPauliHamiltonianWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ApplyTimeDependentPauliHamiltonianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "ApplyTimeDependentPauliHamiltonianWrapper { internal: ApplyTimeDependentPauliHamiltonian { hamiltonian: PauliHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: [0.1], values: {\"omega\": [0.1]} } }"
        );
    })
}

/// Test is_parametrized() function for Analog Operations
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "constant_spin_hamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "time_depenent")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        assert!(operation
            .call_method0("is_parametrized")
            .unwrap()
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
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        assert!(!operation
            .call_method0("is_parametrized")
            .unwrap()
            .extract::<bool>()
            .unwrap());
    })
}

/// Test hqslang() function for Analog Operations
#[test_case("ApplyConstantPauliHamiltonian", Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantPauliHamiltonian")]
#[test_case("ApplyTimeDependentPauliHamiltonian", Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentPauliHamiltonian")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let name_op: String = operation
            .call_method0("hqslang")
            .unwrap()
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
        "ApplyConstantPauliHamiltonian",
        ];
    "ApplyConstantPauliHamiltonian")]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega")),
    vec![
        "Operation",
        "SpinsAnalogOperation",
        "ApplyTimeDependentPauliHamiltonian",
        ];
    "ApplyTimeDependentPauliHamiltonian")]
/// Test tags() function for Analog Operations
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let tags_op: Vec<String> = operation.call_method0("tags").unwrap().extract().unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantPauliHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentPauliHamiltonian")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let copy_op = operation.call_method0("__copy__").unwrap();
        let deepcopy_op = operation.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .call_method1("__eq__", (copy_deepcopy_param.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test format and repr functions
#[test_case(
    "ApplyConstantPauliHamiltonian { hamiltonian: PauliHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Float(1.0)} }, time: Float(1.0) }",
    Operation::from(create_apply_constant_spin_hamiltonian(1.0));
    "ApplyConstantPauliHamiltonian")]
#[test_case(
    "ApplyTimeDependentPauliHamiltonian { hamiltonian: PauliHamiltonian { internal_map: {PauliProduct { items: [(0, Z)] }: Str(\"omega\")} }, time: [1.0], values: {\"omega\": [1.0]} }",
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega"));
    "ApplyTimeDependentPauliHamiltonian")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let to_format = operation.call_method1("__format__", ("",)).unwrap();
        let format_op: String = to_format.extract().unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0("__repr__").unwrap();
        let repr_op: String = to_repr.extract().unwrap();
        assert_eq!(repr_op, format_repr);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantPauliHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentPauliHamiltonian_theta")]
fn test_pyo3_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1("substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();

        let test_operation = convert_operation_to_pyobject(substitute_param, py).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();

        assert!(comparison);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantPauliHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentPauliHamiltonian_theta")]
fn test_pyo3_substitute_params_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1("substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param, py).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian("theta")); "ApplyConstantPauliHamiltonian_theta")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("theta")); "ApplyTimeDependentPauliHamiltonian_theta")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    Python::with_gil(|py| {
        pyo3::prepare_freethreaded_python();
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1("substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let e = binding.value(py);
        assert_eq!(format!("{:?}", e), "RuntimeError('Parameter Substitution failed: CalculatorError(VariableNotSet { name: \"theta\" })')");
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(0.1)), vec![0]; "ApplyConstantPauliHamiltonian_0")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")), vec![0]; "ApplyTimeDependentPauliHamiltonian_0")]
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian_spin_test()), vec![0,2,4]; "ApplyConstantPauliHamiltonian_024")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian_spin_test()), vec![0,2,4]; "ApplyTimeDependentPauliHamiltonian_024")]
fn test_spin(input_operation: Operation, test_result: Vec<usize>) {
    Python::with_gil(|py| {
        pyo3::prepare_freethreaded_python();
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let result: Vec<usize> = operation
            .call_method1("spin", ())
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(result, test_result);
    })
}

#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantPauliHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian(1.0)); "ApplyTimeDependentPauliHamiltonian_theta")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 0.0);
        let substitute_op = operation
            .call_method1("substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let comparison =
            bool::extract_bound(&substitute_op.call_method1("__eq__", (operation,)).unwrap())
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
        let mut hamiltonian = PauliHamiltonian::new();
        hamiltonian
            .add_operator_product(pp1, CalculatorFloat::from(1.0))
            .unwrap();
        hamiltonian
            .add_operator_product(pp2, CalculatorFloat::from(2.0))
            .unwrap();

        let pp1 = PauliProduct::new().z(2).x(1);
        let pp2 = PauliProduct::new().z(0).x(3);
        let mut test_hamiltonian = PauliHamiltonian::new();
        test_hamiltonian
            .add_operator_product(pp1, CalculatorFloat::from(1.0))
            .unwrap();
        test_hamiltonian
            .add_operator_product(pp2, CalculatorFloat::from(2.0))
            .unwrap();

        let input_op = Operation::from(ApplyConstantPauliHamiltonian::new(
            hamiltonian.clone(),
            1.0.into(),
        ));
        let test_op = Operation::from(ApplyConstantPauliHamiltonian::new(
            test_hamiltonian.clone(),
            1.0.into(),
        ));
        let operation = convert_operation_to_pyobject(input_op, py).unwrap();
        let operation_2 = convert_operation_to_pyobject(test_op, py).unwrap();
        // remap qubits
        let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
        qubit_mapping_test.insert(0, 2);
        qubit_mapping_test.insert(2, 0);
        let result = operation
            .call_method1("remap_qubits", (qubit_mapping_test.clone(),))
            .unwrap();

        let comparison = bool::extract_bound(
            &result
                .call_method1("__eq__", (operation_2.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let mut values = HashMap::new();
        values.insert("omega".to_string(), vec![1.0]);
        let input_op = Operation::from(ApplyTimeDependentPauliHamiltonian::new(
            hamiltonian,
            vec![1.0],
            values.clone(),
        ));
        let test_op = Operation::from(ApplyTimeDependentPauliHamiltonian::new(
            test_hamiltonian,
            vec![1.0],
            values.clone(),
        ));
        let operation = convert_operation_to_pyobject(input_op, py).unwrap();
        let operation_2 = convert_operation_to_pyobject(test_op, py).unwrap();

        let result = operation
            .call_method1("remap_qubits", (qubit_mapping_test,))
            .unwrap();

        let comparison = bool::extract_bound(
            &result
                .call_method1("__eq__", (operation_2.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(
    Operation::from(create_apply_constant_spin_hamiltonian(1.0)),
    Operation::from(create_apply_constant_spin_hamiltonian(2.0)); "ApplyConstantPauliHamiltonian")]
#[test_case(
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega1")),
    Operation::from(create_apply_timedependent_spin_hamiltonian("omega2")); "ApplyTimeDependentPauliHamiltonian")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1, py).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2, py).unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1("__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1("__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

#[cfg(feature = "json_schema")]
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantPauliHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentPauliHamiltonian")]
fn test_pyo3_json_schema(operation: Operation) {
    let rust_schema = match operation {
        Operation::ApplyConstantPauliHamiltonian(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ApplyConstantPauliHamiltonian))
                .unwrap()
        }
        Operation::ApplyTimeDependentPauliHamiltonian(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ApplyTimeDependentPauliHamiltonian))
                .unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = "1.11.0".to_string();
        let pyobject = convert_operation_to_pyobject(operation, py).unwrap();
        let operation = pyobject;

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
