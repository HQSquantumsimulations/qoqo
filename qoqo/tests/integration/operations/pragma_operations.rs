// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved
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

use ndarray::{arr2, array, Array1, Array2};
use num_complex::Complex64;
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::Python;
use qoqo::operations::*;
use qoqo::CircuitWrapper;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::*;
use roqoqo::Circuit;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

use super::convert_cf_to_pyobject;

fn reordering() -> HashMap<usize, usize> {
    let mut reordering: HashMap<usize, usize> = HashMap::new();
    reordering.insert(0, 0);
    reordering
}

fn reordering_remapped() -> HashMap<usize, usize> {
    let mut reordering: HashMap<usize, usize> = HashMap::new();
    reordering.insert(2, 2);
    reordering
}

fn qubit_remapping() -> HashMap<usize, usize> {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 2);
    qubit_mapping.insert(2, 0);
    qubit_mapping
}

fn qubits_remapped() -> HashMap<usize, usize> {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(2, 1);
    qubit_mapping
}

fn statevector() -> Array1<Complex64> {
    let statevector: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    statevector
}

fn densitymatrix() -> Array2<Complex64> {
    let densitymatrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    densitymatrix
}

fn operators() -> Array2<f64> {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    operators
}

fn create_circuit() -> Circuit {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit
}

fn circuit_remapped() -> Circuit {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    circuit
}

pub(crate) fn new_circuit(py: Python) -> Bound<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    circuit_type
        .call0()
        .unwrap()
        .downcast::<CircuitWrapper>()
        .unwrap()
        .to_owned()
}

#[cfg(feature = "unstable_simulation_repetitions")]
/// Test inputs of PragmaSimulationRepetitions
#[test]
fn test_pyo3_inputs_simulation_repetitions() {
    let input_pragma = Operation::from(PragmaSimulationRepetitions::new(100));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let repetitions: &usize = &operation
            .call_method0(py, "repetitions")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let repetitions_param: &usize = &100_usize;
        assert_eq!(repetitions, repetitions_param);
    })
}

/// Test inputs of PragmaSetNumberOfMeasurements
#[test]
fn test_pyo3_inputs_setnumbermeasurements() {
    let input_pragma = Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro")));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let nbr_meas_op: &usize = &operation
            .call_method0(py, "number_measurements")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let nbr_meas_param: &usize = &1_usize;
        assert_eq!(nbr_meas_op, nbr_meas_param);

        let readout_op: &String = &operation
            .call_method0(py, "readout")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let readout_param: &String = &String::from("ro");
        assert_eq!(readout_op, readout_param);
    })
}

/// Test inputs of PragmaLoop
#[test]
fn test_pyo3_inputs_loop() {
    let input_pragma = Operation::from(PragmaLoop::new(
        CalculatorFloat::from("number_t"),
        Circuit::new(),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let nbr_meas_op: &CalculatorFloatWrapper = &operation
            .call_method0(py, "repetitions")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let nbr_meas_param: &CalculatorFloatWrapper = &CalculatorFloatWrapper {
            internal: CalculatorFloat::from("number_t"),
        };
        assert_eq!(nbr_meas_op.internal, nbr_meas_param.internal);

        let readout_op: &CircuitWrapper = &operation
            .call_method0(py, "circuit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let readout_param: &CircuitWrapper = &CircuitWrapper {
            internal: Circuit::new(),
        };
        assert_eq!(readout_op, readout_param);
    })
}

/// Test inputs of PragmaSetStateVector
#[test]
fn test_pyo3_inputs_setstatevector() {
    let input_pragma = Operation::from(PragmaSetStateVector::new(statevector()));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let to_op: Py<PyAny> = operation.call_method0(py, "statevector").unwrap();

        let to_statevector_op: PyReadonlyArray1<Complex64> = to_op.bind(py).extract().unwrap();
        let statevector_op: Array1<Complex64> = to_statevector_op.as_array().to_owned();
        assert_eq!(statevector_op, statevector());
    })
}

/// Test inputs of PragmaSetDensityMatrix
#[test]
fn test_pyo3_inputs_setdensitymatrix() {
    let input_pragma = Operation::from(PragmaSetDensityMatrix::new(densitymatrix()));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();
        let to_operators_py = operation.call_method0(py, "density_matrix").unwrap();
        let to_operators_op = to_operators_py
            .bind(py)
            .extract::<PyReadonlyArray2<Complex64>>()
            .unwrap();
        let densmat_array = to_operators_op.as_array().to_owned();

        assert_eq!(densmat_array, densitymatrix());
    })
}

/// Test inputs of PragmaRepeatGate
#[test]
fn test_pyo3_inputs_repeatgate() {
    let input_pragma = Operation::from(PragmaRepeatGate::new(3));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let repeat_op: &usize = &operation
            .call_method0(py, "repetition_coefficient")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(repeat_op, &3_usize);
    })
}

/// Test inputs of PragmaOverrotation
#[test]
fn test_pyo3_inputs_overrotation() {
    let input_pragma = Operation::from(PragmaOverrotation::new(
        "RotateX".to_string(),
        vec![0],
        0.03,
        0.001,
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let string_op: &String = &operation
            .call_method0(py, "gate_hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(string_op, &"RotateX".to_string());
        let qubits_op: &Vec<usize> = &operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(qubits_op, &vec![0]);
        let amp_op: &f64 = &operation
            .call_method0(py, "amplitude")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(amp_op, &0.03);
        let var_op: &f64 = &operation
            .call_method0(py, "variance")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(var_op, &0.001);
    })
}

/// Test inputs of PragmaBoostNoise
#[test]
fn test_pyo3_inputs_boostnoise() {
    let input_pragma = Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003)));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let boost_op: &f64 = &operation
            .call_method0(py, "noise_coefficient")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(boost_op),
            CalculatorFloat::from(0.003),
        );
    })
}

/// Test inputs of PragmaStopParallelBlock
#[test]
fn test_pyo3_inputs_stop() {
    let input_pragma = Operation::from(PragmaStopParallelBlock::new(
        vec![0, 1],
        CalculatorFloat::from(0.0000001),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubits_op: &Vec<usize> = &operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubits_param: Vec<usize> = vec![0, 1];
        assert_eq!(qubits_op, &qubits_param);

        let boost_op: &f64 = &operation
            .call_method0(py, "execution_time")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(boost_op),
            CalculatorFloat::from(0.0000001),
        );
    })
}

/// Test inputs of PragmaGlobalPhase
#[test]
fn test_pyo3_inputs_globalphase() {
    let input_pragma = Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05)));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let boost_op: &f64 = &operation
            .call_method0(py, "phase")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(CalculatorFloat::from(boost_op), CalculatorFloat::from(0.05));
    })
}

/// Test inputs of PragmaSleep
#[test]
fn test_pyo3_inputs_sleep() {
    let input_pragma = Operation::from(PragmaSleep::new(
        vec![0, 1],
        CalculatorFloat::from(0.0000001),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubits_op: &Vec<usize> = &operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubits_param: Vec<usize> = vec![0, 1];
        assert_eq!(qubits_op, &qubits_param);

        let boost_op: &f64 = &operation
            .call_method0(py, "sleep_time")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(boost_op),
            CalculatorFloat::from(0.0000001),
        );
    })
}

/// Test inputs of PragmaActiveReset
#[test]
fn test_pyo3_inputs_activereset() {
    let input_pragma = Operation::from(PragmaActiveReset::new(0));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubit_op: &usize = &operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubit_param: &usize = &0_usize;
        assert_eq!(qubit_op, qubit_param);
    })
}

/// Test inputs of PragmaStartDecompositionBlock
#[test]
fn test_pyo3_inputs_startdecompblock() {
    let input_pragma =
        Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering()));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubits_op: &Vec<usize> = &operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubits_param: Vec<usize> = vec![0, 1];
        assert_eq!(qubits_op, &qubits_param);

        let boost_op: HashMap<usize, usize> = operation
            .call_method0(py, "reordering_dictionary")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(boost_op, reordering());
    })
}

/// Test inputs of PragmaStopDecompositionBlock
#[test]
fn test_pyo3_inputs_stopdecompblock() {
    let input_pragma = Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1]));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubits_op: &Vec<usize> = &operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubits_param: Vec<usize> = vec![0, 1];
        assert_eq!(qubits_op, &qubits_param);
    })
}

/// Test inputs of Noise Pragmas (except RandomNoise)
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
fn test_pyo3_inputs_noise(input_pragma: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubit_op: &usize = &operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubit_param: &usize = &0_usize;
        assert_eq!(qubit_op, qubit_param);

        let gate_time_op: &f64 = &operation
            .call_method0(py, "gate_time")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(gate_time_op),
            CalculatorFloat::from(0.005),
        );

        let rate_op: &f64 = &operation
            .call_method0(py, "rate")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(CalculatorFloat::from(rate_op), CalculatorFloat::from(0.02));
    })
}

/// Test inputs of PragmaRandomNoise
#[test]
fn test_pyo3_inputs_randomnoise() {
    let input_pragma = Operation::from(PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubit_op: &usize = &operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubit_param: &usize = &0_usize;
        assert_eq!(qubit_op, qubit_param);

        let gate_time_op: &f64 = &operation
            .call_method0(py, "gate_time")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(gate_time_op),
            CalculatorFloat::from(0.005),
        );

        let depol_rate_op: &f64 = &operation
            .call_method0(py, "depolarising_rate")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(depol_rate_op),
            CalculatorFloat::from(0.02)
        );

        let dephas_rate_op: &f64 = &operation
            .call_method0(py, "dephasing_rate")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(dephas_rate_op),
            CalculatorFloat::from(0.01)
        );
    })
}

/// Test inputs of PragmaGeneralNoise
#[test]
fn test_pyo3_inputs_generalnoise() {
    let input_pragma = Operation::from(PragmaGeneralNoise::new(
        0,
        CalculatorFloat::from(0.005),
        operators(),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let qubit_op: &usize = &operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let qubit_param: &usize = &0_usize;
        assert_eq!(qubit_op, qubit_param);

        let gate_time_op: &f64 = &operation
            .call_method0(py, "gate_time")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(gate_time_op),
            CalculatorFloat::from(0.005),
        );

        let to_operators_py = operation.call_method0(py, "rates").unwrap();
        let to_operators_op = to_operators_py
            .bind(py)
            .extract::<PyReadonlyArray2<f64>>()
            .unwrap();
        let operators_op = to_operators_op.as_array().to_owned();
        assert_eq!(operators_op, operators());
    })
}

/// Test inputs of PragmaConditional
#[test]
fn test_pyo3_inputs_conditional() {
    let input_pragma = Operation::from(PragmaConditional::new(
        String::from("ro"),
        1,
        create_circuit(),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let condition_register_op: &String = &operation
            .call_method0(py, "condition_register")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let condition_register_param: &String = &String::from("ro");
        assert_eq!(condition_register_op, condition_register_param);

        let condition_index_op: &usize = &operation
            .call_method0(py, "condition_index")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(condition_index_op, &1_usize);

        let to_circuit = operation.call_method0(py, "circuit").unwrap();
        let circuit_op = to_circuit.bind(py);
        let circuit = new_circuit(py);
        let paulix = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
        circuit.call_method1("add", (paulix,)).unwrap();
        let comparison_circuit =
            bool::extract_bound(&circuit_op.call_method1("__eq__", (circuit,)).unwrap()).unwrap();
        assert!(comparison_circuit);
    })
}

/// Test inputs of PragmaControlledCircuit
#[test]
fn test_pyo3_inputs_controlled_circuit() {
    let input_pragma = Operation::from(PragmaControlledCircuit::new(1, create_circuit()));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let condition_index_op: &usize = &operation
            .call_method0(py, "controlling_qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(condition_index_op, &1_usize);

        let to_circuit = operation.call_method0(py, "circuit").unwrap();
        let circuit_op = to_circuit.bind(py);
        let circuit = new_circuit(py);
        let paulix = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
        circuit.call_method1("add", (paulix,)).unwrap();
        let comparison_circuit =
            bool::extract_bound(&circuit_op.call_method1("__eq__", (circuit,)).unwrap()).unwrap();
        assert!(comparison_circuit);
    })
}

/// Test inputs of PragmaAnnotatedOp
#[test]
fn test_pyo3_inputs_annotated_op() {
    let input_op = Operation::from(PauliX::new(0));
    let input_pragma =
        Operation::from(PragmaAnnotatedOp::new(input_op.clone(), "test".to_string()));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();

        let op = operation.call_method0(py, "operation").unwrap();
        let op_ref = op.bind(py);
        let comparison_op = bool::extract_bound(
            &op_ref
                .call_method1(
                    "__eq__",
                    (convert_operation_to_pyobject(input_op.clone()).unwrap(),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_op);

        let annotation: String = operation
            .call_method0(py, "annotation")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(annotation, "test".to_string());
    })
}

/// Test involved_qubits function for Pragmas with None
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::new())); "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.005))), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_involved_qubits_none(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_involved = operation.call_method0(py, "involved_qubits").unwrap();
        let involved_op: HashSet<usize> = to_involved.bind(py).extract().unwrap();
        let involved_param: HashSet<usize> = HashSet::new();
        assert_eq!(involved_op, involved_param);
    })
}

/// Test involved_qubits function for Pragmas with All
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())); "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())); "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PragmaSetStateVector::new(statevector())), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_involved_qubits_all(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_involved = operation.call_method0(py, "involved_qubits").unwrap();
        let involved_op: HashSet<String> = to_involved.bind(py).extract().unwrap();
        let mut involved_param: HashSet<String> = HashSet::new();
        involved_param.insert("All".to_owned());
        assert_eq!(involved_op, involved_param);
    })
}

/// Test involved_qubits function for Pragmas with qubit 0
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaSleep::new(vec![0], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators())); "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new(0, create_circuit())); "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_involved_qubits_qubit(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_involved = operation.call_method0(py, "involved_qubits").unwrap();
        let involved_op: HashSet<usize> = to_involved.bind(py).extract().unwrap();
        let mut involved_param: HashSet<usize> = HashSet::new();
        involved_param.insert(0);
        assert_eq!(involved_op, involved_param);
    })
}

#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)); "PragmaOverrotation")]
fn test_pyo3_involved_qubits_qubit_overrotation(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_involved = operation.call_method0(py, "involved_qubits").unwrap();
        let involved_op: HashSet<usize> = to_involved.bind(py).extract().unwrap();
        let mut involved_param: HashSet<usize> = HashSet::new();
        involved_param.insert(0);
        assert_eq!(involved_op, involved_param);
    })
}

/// Test format and repr functions
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))),
            "PragmaSetNumberOfMeasurements { number_measurements: 1, readout: \"ro\" }"; "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())),
            "PragmaSetStateVector { statevector: [Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }], shape=[4], strides=[1], layout=CFcf (0xf), const ndim=1 }"; "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())),
            "PragmaSetDensityMatrix { density_matrix: [[Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],\n [Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2 }"; "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)),
            "PragmaRepeatGate { repetition_coefficient: 3 }"; "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))),
            "PragmaBoostNoise { noise_coefficient: Float(0.003) }"; "PragmaBoostNoise")]
// #[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))),
//             "PragmaStopParallelBlock { qubits: [0, 1], execution_time: Float(0.0000001) }"; "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))),
            "PragmaGlobalPhase { phase: Float(0.05) }"; "PragmaGlobalPhase")]
// #[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))),
//             "PragmaSleep { qubits: [0, 1], sleep_time: Float(0.0000001) }"; "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)),
            "PragmaActiveReset { qubit: 0 }"; "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())),
            "PragmaStartDecompositionBlock { qubits: [0, 1], reordering_dictionary: {0: 0} }"; "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])),
            "PragmaStopDecompositionBlock { qubits: [0, 1] }"; "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            "PragmaDamping { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"; "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            "PragmaDepolarising { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"; "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            "PragmaDephasing { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"; "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))),
            "PragmaRandomNoise { qubit: 0, gate_time: Float(0.005), depolarising_rate: Float(0.02), dephasing_rate: Float(0.01) }"; "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators())),
            "PragmaGeneralNoise { qubit: 0, gate_time: Float(0.005), rates: [[1.0, 0.0, 0.0],\n [0.0, 1.0, 0.0],\n [0.0, 0.0, 1.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 }"; "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, Circuit::default())),
            "PragmaConditional { condition_register: \"ro\", condition_index: 1, circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } }"; "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, Circuit::default())),
            "PragmaControlledCircuit { controlling_qubit: 1, circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } }"; "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::default())),
            "PragmaLoop { repetitions: Str(\"number_t\"), circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } }"; "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())),
            "PragmaAnnotatedOp { operation: PauliX(PauliX { qubit: 0 }), annotation: \"test\" }"; "PragmaAnnotatedOp")]
fn test_pyo3_format_repr(input_measurement: Operation, format_repr: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)),
            "PragmaOverrotation { gate_hqslang: \"RotateX\", qubits: [0], amplitude: 0.03, variance: 0.001 }"; "PragmaOverrotation")]
fn test_pyo3_format_repr_overrotation(input_measurement: Operation, format_repr: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())); "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())); "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())); "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())); "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::default())); "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_copy_deepcopy(input_measurement: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
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

#[test]
fn test_pyo3_copy_deepcopy_overrotation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(PragmaOverrotation::new(
            "RotateX".to_string(),
            vec![0],
            0.03,
            0.001,
        )))
        .unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();

        let extracted_copy: PragmaOverrotationWrapper = copy_op.bind(py).extract().unwrap();
        assert_eq!(
            extracted_copy.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)
        );
        let comparison_copy = bool::extract_bound(
            &copy_op
                .bind(py)
                .call_method1("__eq__", (operation.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let extracted_copy: PragmaOverrotationWrapper = copy_op.bind(py).extract().unwrap();
        assert_eq!(
            extracted_copy.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)
        );
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .bind(py)
                .call_method1("__eq__", (operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test tags function for simple Pragmas
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))), "PragmaSetNumberOfMeasurements"; "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())), "PragmaSetStateVector"; "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())), "PragmaSetDensityMatrix"; "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)), "PragmaRepeatGate"; "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))), "PragmaBoostNoise"; "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))), "PragmaGlobalPhase"; "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::default())), "PragmaLoop"; "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())), "PragmaAnnotatedOp"; "PragmaAnnotatedOp")]
fn test_pyo3_tags_simple(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &["Operation", "PragmaOperation", tag_name];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test tags function for Pragmas that are also MultiQubitGates
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))), "PragmaStopParallelBlock"; "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))), "PragmaSleep"; "PragmaSleep")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())), "PragmaStartDecompositionBlock"; "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])), "PragmaStopDecompositionBlock"; "PragmaStopDecompositionBlock")]
fn test_pyo3_tags_multi(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &[
            "Operation",
            "MultiQubitOperation",
            "PragmaOperation",
            tag_name,
        ];
        assert_eq!(tags_op, tags_param);
    })
}

#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)), "PragmaOverrotation"; "PragmaOverrotation")]
fn test_pyo3_tags_multi_overrotation(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &[
            "Operation",
            "MultiQubitOperation",
            "PragmaOperation",
            tag_name,
        ];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test tags function for Pragmas that are also SingleQubitGates
#[test_case(Operation::from(PragmaActiveReset::new(0)), "PragmaActiveReset"; "PragmaActiveReset")]
fn test_pyo3_tags_single(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &[
            "Operation",
            "SingleQubitOperation",
            "PragmaOperation",
            tag_name,
        ];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test tags function for Pragmas that are also SingleQubitGates
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())), "PragmaConditional"; "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())), "PragmaControlledCircuit"; "PragmaControlledCircuit")]
fn test_pyo3_tags_conditional(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &["Operation", "PragmaOperation", tag_name];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test tags function for PragmaGeneralNoise
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators())), "PragmaGeneralNoise"; "PragmaGeneralNoise")]
fn test_pyo3_tags_general_noise(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &[
            "Operation",
            "SingleQubitOperation",
            "PragmaOperation",
            "PragmaNoiseOperation",
            tag_name,
        ];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test tags function for Noise Pragmas
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDamping"; "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDepolarising"; "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDephasing"; "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))), "PragmaRandomNoise"; "PragmaRandomNoise")]
fn test_pyo3_tags_noise(input_measurement: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &[
            "Operation",
            "SingleQubitOperation",
            "PragmaOperation",
            "PragmaNoiseOperation",
            "PragmaNoiseProbaOperation",
            tag_name,
        ];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test hqslang function
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))), "PragmaSetNumberOfMeasurements"; "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())), "PragmaSetStateVector"; "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())), "PragmaSetDensityMatrix"; "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)), "PragmaRepeatGate"; "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))), "PragmaBoostNoise"; "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))), "PragmaStopParallelBlock"; "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))), "PragmaGlobalPhase"; "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))), "PragmaSleep"; "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)), "PragmaActiveReset"; "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())), "PragmaStartDecompositionBlock"; "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])), "PragmaStopDecompositionBlock"; "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDamping"; "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDepolarising"; "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), "PragmaDephasing"; "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))), "PragmaRandomNoise"; "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())), "PragmaGeneralNoise"; "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())), "PragmaConditional"; "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())), "PragmaControlledCircuit"; "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::default())), "PragmaLoop"; "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())), "PragmaAnnotatedOp"; "PragmaAnnotatedOp")]
fn test_pyo3_hqslang(input_measurement: Operation, hqslang_param: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let hqslang_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(hqslang_op, hqslang_param.to_string());
    })
}

#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)), "PragmaOverrotation"; "PragmaOverrotation")]
fn test_pyo3_hqslang_overrotation(input_measurement: Operation, hqslang_param: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        let hqslang_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(hqslang_op, hqslang_param.to_string());
    })
}

/// Test is_parametrized function (false)
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())); "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())); "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())); "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())); "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from(1.0), Circuit::default())); "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_is_parametrized_false(input_pragma: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();
        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test is_parametrized function (true)
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(0, CalculatorFloat::Str("theta".to_string()))), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_is_parametrized_true(input_pragma: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_pragma).unwrap();
        assert!(operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)); "PragmaOverrotation")]
fn test_pyo3_is_parametrized_overrotation(input_measurement: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_measurement).unwrap();
        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test substitute_parameters function
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))),
            Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro")));
            "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())),
            Operation::from(PragmaSetStateVector::new(statevector()));
            "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())),
            Operation::from(PragmaSetDensityMatrix::new(densitymatrix()));
            "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)),
            Operation::from(PragmaRepeatGate::new(3));
            "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from("test"))),
            Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(1.0)));
            "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from("test"))),
            Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(1.0)));
            "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from("test"))),
            Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(1.0)));
            "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from("test"))),
            Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(1.0)));
            "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)),
            Operation::from(PragmaActiveReset::new(0));
            "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())),
            Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering()));
            "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])),
            Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1]));
            "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDamping::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.02)));
            "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.02)));
            "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.02)));
            "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))),
            Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01)));
            "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from("test"), operators())),
            Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(1.0),  operators()));
            "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())),
            Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit()));
            "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new(1, create_circuit())),
            Operation::from(PragmaControlledCircuit::new(1, create_circuit()));
            "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("test"), Circuit::default())),
            Operation::from(PragmaLoop::new(CalculatorFloat::from(1.0), Circuit::default()));
            "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(0, CalculatorFloat::from("test"))), "test".to_string())),
            Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(0, CalculatorFloat::from(1.0))), "test".to_string()));
            "PragmaAnnotatedOp")]
fn test_pyo3_substitute_parameters(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();
        let mut substitution_dict: HashMap<String, f64> = HashMap::new();
        substitution_dict.insert("test".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = convert_operation_to_pyobject(second_op).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test]
fn test_pyo3_substitute_parameters_overrotation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(PragmaOverrotation::new(
            "RotateX".to_string(),
            vec![0],
            0.03,
            0.001,
        )))
        .unwrap();

        let mut substitution_dict: HashMap<String, f64> = HashMap::new();
        substitution_dict.insert("test".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();

        let extracted: PragmaOverrotationWrapper = substitute_op.bind(py).extract().unwrap();
        assert_eq!(
            extracted.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)
        );

        let comparison = substitute_op
            .bind(py)
            .call_method1("__eq__", (operation,))
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters() causing an error `None`
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from("test")));
            "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from("test")));
            "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from("test")));
            "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from("test")));
            "PragmaSleep")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02)));
            "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02)));
            "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02)));
            "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from("test"), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01)));
            "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from("test"),  operators()));
            "PragmaGeneralNoise")]
#[test_case(
    Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(
        0,
        CalculatorFloat::from("test")
    )), "test".to_string()));
    "PragmaAnnotatedOp"
)]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test substitute_parameters() causing an error `not-a-real-number`
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())); "PragmaControlledCircuit")]
#[test_case(
    Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(
        0,
        CalculatorFloat::from("test")
    )), "test".to_string()));
    "PragmaAnnotatedOp"
)]
fn test_pyo3_substituteparameters_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mut substitution_dict: HashMap<&str, &str> = HashMap::new();
        substitution_dict.insert("ro", "test");
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test remap_qubits function
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))),
            Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro")));
            "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())),
            Operation::from(PragmaSetStateVector::new(statevector()));
            "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())),
            Operation::from(PragmaSetDensityMatrix::new(densitymatrix()));
            "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)),
            Operation::from(PragmaRepeatGate::new(3));
            "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))),
            Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003)));
            "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0], CalculatorFloat::from(0.0000001))),
            Operation::from(PragmaStopParallelBlock::new(vec![2], CalculatorFloat::from(0.0000001)));
            "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))),
            Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05)));
            "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0], CalculatorFloat::from(0.0000001))),
            Operation::from(PragmaSleep::new(vec![2], CalculatorFloat::from(0.0000001)));
            "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)),
            Operation::from(PragmaActiveReset::new(2));
            "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0], reordering())),
            Operation::from(PragmaStartDecompositionBlock::new(vec![2], reordering_remapped()));
            "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0])),
            Operation::from(PragmaStopDecompositionBlock::new(vec![2]));
            "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDamping::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDepolarising::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDephasing::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))),
            Operation::from(PragmaRandomNoise::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01)));
            "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())),
            Operation::from(PragmaGeneralNoise::new(2, CalculatorFloat::from(0.005),  operators()));
            "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())),
            Operation::from(PragmaConditional::new(String::from("ro"), 1, circuit_remapped()));
            "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new(20, create_circuit())),
            Operation::from(PragmaControlledCircuit::new(20, circuit_remapped()));
            "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), create_circuit())),
            Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), circuit_remapped()));
            "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(0, CalculatorFloat::from("test"))), "test".to_string())),
            Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(2, CalculatorFloat::from("test"))), "test".to_string()));
            "PragmaAnnotatedOp")]
fn test_pyo3_remap_qubits(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();

        let remapped_op = operation
            .call_method1(py, "remap_qubits", (qubit_remapping(),))
            .unwrap();
        let comparison_op = convert_operation_to_pyobject(second_op).unwrap();

        let comparison = remapped_op
            .call_method1(py, "__eq__", (comparison_op,))
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap();
        assert!(comparison);
    })
}

#[test]
fn test_pyo3_remap_qubits_overrotation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(PragmaOverrotation::new(
            "RotateX".to_string(),
            vec![0],
            0.03,
            0.001,
        )))
        .unwrap();

        let remapped_op = operation
            .call_method1(py, "remap_qubits", (qubit_remapping(),))
            .unwrap();
        let extracted: PragmaOverrotationWrapper = remapped_op.bind(py).extract().unwrap();
        assert_eq!(
            extracted.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![2], 0.03, 0.001)
        );

        let operation_two = convert_operation_to_pyobject(Operation::from(
            PragmaOverrotation::new("RotateX".to_string(), vec![2], 0.03, 0.001),
        ))
        .unwrap();
        let comparison = remapped_op
            .bind(py)
            .call_method1("__eq__", (operation_two,))
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert!(comparison);

        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
        assert!(result.is_err());
    })
}

/// Test superoperator of PragmaDamping
#[test]
fn test_pyo3_noise_superoperator_damping() {
    let pragma_op =
        PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let noise_pragma = Operation::from(pragma_op.clone());
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(noise_pragma).unwrap();

        let superop_prob: f64 = f64::try_from(pragma_op.probability()).unwrap();
        let superop_sqrt: f64 = (1.0 - superop_prob).sqrt();
        let superop_param: Array2<f64> = arr2(&[
            [1.0, 0.0, 0.0, superop_prob],
            [0.0, superop_sqrt, 0.0, 0.0],
            [0.0, 0.0, superop_sqrt, 0.0],
            [0.0, 0.0, 0.0, 1.0 - superop_prob],
        ]);

        let to_superop_op = operation.call_method0(py, "superoperator").unwrap();
        let superop_op = to_superop_op
            .extract::<PyReadonlyArray2<f64>>(py)
            .unwrap()
            .as_array()
            .to_owned();
        assert_eq!(superop_op, superop_param);
    })
}

/// Test superoperator of PragmaDamping
#[test]
fn test_pyo3_noise_superoperator_depolarising() {
    let noise_pragma = Operation::from(PragmaDepolarising::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(noise_pragma).unwrap();

        let superop_pre_exp: f64 = -1.0 * 0.005 * 0.02;
        let superop_prob: f64 = 0.75 * (1.0 - superop_pre_exp.exp());
        let superop_proba1: f64 = 1.0 - (2.0 / 3.0) * superop_prob;
        let superop_proba2: f64 = 1.0 - (4.0 / 3.0) * superop_prob;
        let superop_proba3: f64 = (2.0 / 3.0) * superop_prob;
        let superop_param: Array2<f64> = arr2(&[
            [superop_proba1, 0.0, 0.0, superop_proba3],
            [0.0, superop_proba2, 0.0, 0.0],
            [0.0, 0.0, superop_proba2, 0.0],
            [superop_proba3, 0.0, 0.0, superop_proba1],
        ]);

        let to_superop_op = operation.call_method0(py, "superoperator").unwrap();
        let superop_op = to_superop_op
            .extract::<PyReadonlyArray2<f64>>(py)
            .unwrap()
            .as_array()
            .to_owned();

        assert_eq!(superop_op, superop_param);
    })
}

/// Test superoperator of PragmaDamping
#[test]
fn test_pyo3_noise_superoperator_dephasing() {
    let noise_pragma = Operation::from(PragmaDephasing::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(noise_pragma).unwrap();

        let superop_pre_exp: f64 = -2.0 * 0.005 * 0.02;
        let superop_prob: f64 = 0.5 * (1.0 - superop_pre_exp.exp());
        let superop_proba1: f64 = 1.0 - 2.0 * superop_prob;
        let superop_param: Array2<f64> = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, superop_proba1, 0.0, 0.0],
            [0.0, 0.0, superop_proba1, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let to_superop_op = operation.call_method0(py, "superoperator").unwrap();
        let superop_op = to_superop_op
            .extract::<PyReadonlyArray2<f64>>(py)
            .unwrap()
            .as_array()
            .to_owned();

        assert_eq!(superop_op, superop_param);
    })
}

/// Test superoperator of PragmaDamping
#[test]
fn test_pyo3_noise_superoperator_randomnoise() {
    let noise_pragma = Operation::from(PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    ));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(noise_pragma).unwrap();

        let superop_pre_exp: f64 = -2.0 * 0.005 * 0.01;
        let superop_prob: f64 = 0.5 * (1.0 - superop_pre_exp.exp());
        let superop_proba1: f64 = 1.0 - 2.0 * superop_prob;
        let superop_param: Array2<f64> = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, superop_proba1, 0.0, 0.0],
            [0.0, 0.0, superop_proba1, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let to_superop_op = operation.call_method0(py, "superoperator").unwrap();
        let superop_op = to_superop_op
            .extract::<PyReadonlyArray2<f64>>(py)
            .unwrap()
            .as_array()
            .to_owned();
        assert_eq!(superop_op, superop_param);
    })
}

/// Test probability function of Noise Pragmas
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), 0.00009999500016666385; "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), 0.00007499625012499789; "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))), 0.00009999000066662767; "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))), 0.000125; "PragmaRandomNoise")]
fn test_pyo3_noise_proba(noise_pragma: Operation, proba: f64) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(noise_pragma).unwrap();

        let gate_time_op: &f64 = &operation
            .call_method0(py, "probability")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(
            CalculatorFloat::from(gate_time_op),
            CalculatorFloat::from(proba),
        );
    })
}

/// Test powercf function of Noise Pragmas
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02)));
            "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02)));
            "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02)));
            "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))),
            Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01)));
            "PragmaRandomNoise")]
fn test_pyo3_noise_powercf(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();

        let power = convert_cf_to_pyobject(py, CalculatorFloat::from(1.5));
        let comparison_op = convert_operation_to_pyobject(second_op).unwrap();

        let remapped_op = operation.call_method1(py, "powercf", (power,)).unwrap();
        let comparison = &remapped_op
            .call_method1(py, "__eq__", (comparison_op,))
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap();
        assert!(comparison);
    })
}

/// Test the __richcmp__ function
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))),
            Operation::from(PragmaSetNumberOfMeasurements::new(2, String::from("ro")));
            "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())),
            Operation::from(PragmaSetStateVector::new(statevector() + 1.0));
            "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())),
            Operation::from(PragmaSetDensityMatrix::new(densitymatrix() + 1.0));
            "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)),
            Operation::from(PragmaRepeatGate::new(1));
            "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))),
            Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.001)));
            "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0], CalculatorFloat::from(0.0000001))),
            Operation::from(PragmaStopParallelBlock::new(vec![2], CalculatorFloat::from(0.0000001)));
            "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))),
            Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.02)));
            "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0], CalculatorFloat::from(0.0000001))),
            Operation::from(PragmaSleep::new(vec![2], CalculatorFloat::from(0.0000001)));
            "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)),
            Operation::from(PragmaActiveReset::new(2));
            "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0], reordering())),
            Operation::from(PragmaStartDecompositionBlock::new(vec![0], qubits_remapped()));
            "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0])),
            Operation::from(PragmaStopDecompositionBlock::new(vec![2]));
            "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDamping::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDepolarising::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(PragmaDephasing::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02)));
            "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))),
            Operation::from(PragmaRandomNoise::new(2, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01)));
            "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())),
            Operation::from(PragmaGeneralNoise::new(2, CalculatorFloat::from(0.005), operators()));
            "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())),
            Operation::from(PragmaConditional::new(String::from("ro"), 1, circuit_remapped()));
            "PragmaConditional")]
#[test_case(Operation::from(PragmaControlledCircuit::new( 1, create_circuit())),
            Operation::from(PragmaControlledCircuit::new( 1, circuit_remapped()));
            "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), create_circuit())),
            Operation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), circuit_remapped()));
            "PragmaLoop")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(0, CalculatorFloat::from("test"))), "test".to_string())),
            Operation::from(PragmaAnnotatedOp::new(Operation::from(RotateX::new(2, CalculatorFloat::from("test"))), "test".to_string()));
            "PragmaAnnotatedOp")]
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

#[test]
fn test_pyo3_richcmp_overrotation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(Operation::from(
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001),
        ))
        .unwrap();
        let operation_two = convert_operation_to_pyobject(Operation::from(
            PragmaOverrotation::new("RotateX".to_string(), vec![1], 0.03, 0.001),
        ))
        .unwrap();

        let extracted_one: PragmaOverrotationWrapper = operation_one.bind(py).extract().unwrap();
        assert_eq!(
            extracted_one.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)
        );

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

/// Test PragmaSetNumberOfMeasurements new() function
#[test]
fn test_pyo3_new_set_number_of_measurements() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaSetNumberOfMeasurementsWrapper>();
        let binding = operation.call1((1, "ro".to_string())).unwrap();
        let new_op = binding
            .downcast::<PragmaSetNumberOfMeasurementsWrapper>()
            .unwrap();

        let input_definition =
            Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro")));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op
            .extract::<PragmaSetNumberOfMeasurementsWrapper>()
            .unwrap();
        let binding = operation.call1((2, "ro".to_string())).unwrap();
        let new_op_diff = binding
            .downcast::<PragmaSetNumberOfMeasurementsWrapper>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaSetNumberOfMeasurementsWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaSetNumberOfMeasurementsWrapper { internal: PragmaSetNumberOfMeasurements { number_measurements: 1, readout: \"ro\" } }"
        );
    })
}

/// Test PragmaSetStateVector new() function
#[test]
fn test_pyo3_new_set_statevector() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaSetStateVectorWrapper>();

        let pylist = PyList::new(py, vec![1.0, 0.0]).unwrap();
        let binding_from_pylist = operation.call1((pylist,));
        assert!(binding_from_pylist.is_ok());

        let to_get_statevec_0 = Operation::from(PragmaSetStateVector::new(statevector()));
        let convert_to_get_statevec_0 = convert_operation_to_pyobject(to_get_statevec_0).unwrap();
        let statevector_op_0 = convert_to_get_statevec_0
            .call_method0(py, "statevector")
            .unwrap();
        let binding = operation.call1((statevector_op_0,)).unwrap();
        let new_op = binding.downcast::<PragmaSetStateVectorWrapper>().unwrap();

        let comparison_copy = bool::extract_bound(
            &new_op
                .call_method1("__eq__", (convert_to_get_statevec_0,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let to_get_statevec_1 = Operation::from(PragmaSetStateVector::new(statevector() + 1.0));
        let convert_to_get_statevec_1 = convert_operation_to_pyobject(to_get_statevec_1).unwrap();
        let statevector_op_1 = convert_to_get_statevec_1
            .call_method0(py, "statevector")
            .unwrap();

        let pragma_wrapper = new_op.extract::<PragmaSetStateVectorWrapper>().unwrap();
        let binding = operation.call1((statevector_op_1,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaSetStateVectorWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaSetStateVectorWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaSetStateVectorWrapper { internal: PragmaSetStateVector { statevector: [Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }], shape=[4], strides=[1], layout=CFcf (0xf), const ndim=1 } }"
        );
    })
}

/// Test PragmaSetDensityMatrix new() function
#[test]
fn test_pyo3_new_set_densitymatrix() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaSetDensityMatrixWrapper>();

        let pylist = PyList::new(py, vec![vec![1.0, 0.0], vec![0.0, 0.0]]).unwrap();
        let binding_from_pylist = operation.call1((pylist,));
        assert!(binding_from_pylist.is_ok());

        let to_get_densmat_0 = Operation::from(PragmaSetDensityMatrix::new(densitymatrix()));
        let convert_to_get_densmat_0 = convert_operation_to_pyobject(to_get_densmat_0).unwrap();
        let densmat_op_0 = convert_to_get_densmat_0
            .call_method0(py, "density_matrix")
            .unwrap();
        let binding = operation.call1((densmat_op_0,)).unwrap();
        let new_op = binding.downcast::<PragmaSetDensityMatrixWrapper>().unwrap();

        let comparison_copy = bool::extract_bound(
            &new_op
                .call_method1("__eq__", (convert_to_get_densmat_0,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let to_get_densmat_1 = Operation::from(PragmaSetDensityMatrix::new(densitymatrix() + 1.0));
        let convert_to_get_densmat_1 = convert_operation_to_pyobject(to_get_densmat_1).unwrap();
        let densmat_op_1 = convert_to_get_densmat_1
            .call_method0(py, "density_matrix")
            .unwrap();

        let pragma_wrapper = new_op.extract::<PragmaSetDensityMatrixWrapper>().unwrap();
        let binding = operation.call1((densmat_op_1,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaSetDensityMatrixWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaSetDensityMatrixWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaSetDensityMatrixWrapper { internal: PragmaSetDensityMatrix { density_matrix: [[Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],\n [Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2 } }"
        );
    })
}

/// Test PragmaRepeatGate new() function
#[test]
fn test_pyo3_new_repeated_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaRepeatGateWrapper>();
        let binding = operation.call1((2,)).unwrap();
        let new_op = binding.downcast::<PragmaRepeatGateWrapper>().unwrap();

        let input_definition = Operation::from(PragmaRepeatGate::new(2));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaRepeatGateWrapper>().unwrap();
        let binding = operation.call1((3,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaRepeatGateWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaRepeatGateWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaRepeatGateWrapper { internal: PragmaRepeatGate { repetition_coefficient: 2 } }"
        );
    })
}

/// Test PragmaOverrotation new() function
#[test]
fn test_pyo3_new_overrotation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaOverrotationWrapper>();
        let binding = operation.call1(("RotateX", vec![0], 0.03, 0.001)).unwrap();
        let new_op = binding.downcast::<PragmaOverrotationWrapper>().unwrap();
        let input_definition = Operation::from(PragmaOverrotation::new(
            "RotateX".to_string(),
            vec![0],
            0.03,
            0.001,
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let extracted: PragmaOverrotationWrapper =
            PragmaOverrotationWrapper::extract_bound(new_op).unwrap();
        assert_eq!(
            extracted.internal,
            PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)
        );
        let comparison =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison);

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaOverrotationWrapper>().unwrap();
        let binding = operation.call1(("RotateX", vec![1], 0.03, 0.001)).unwrap();
        let new_op_diff = binding.downcast::<PragmaOverrotationWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaOverrotationWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaOverrotationWrapper { internal: PragmaOverrotation { gate_hqslang: \"RotateX\", qubits: [0], amplitude: 0.03, variance: 0.001 } }"
        );
    })
}

/// Test PragmaBoostNoise new() function
#[test]
fn test_pyo3_new_boost_noise() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaBoostNoiseWrapper>();
        let binding = operation.call1((0.003,)).unwrap();
        let new_op = binding.downcast::<PragmaBoostNoiseWrapper>().unwrap();
        let input_definition = Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003)));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((vec!["fails"],));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaBoostNoiseWrapper>().unwrap();
        let binding = operation.call1((0.001,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaBoostNoiseWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaBoostNoiseWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaBoostNoiseWrapper { internal: PragmaBoostNoise { noise_coefficient: Float(0.003) } }"
        );
    })
}

/// Test PragmaStopParallelBlock new() function
#[test]
fn test_pyo3_new_stop() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaStopParallelBlockWrapper>();
        let binding = operation.call1((vec![0], 0.0000001)).unwrap();
        let new_op = binding
            .downcast::<PragmaStopParallelBlockWrapper>()
            .unwrap();
        let input_definition = Operation::from(PragmaStopParallelBlock::new(
            vec![0],
            CalculatorFloat::from(0.0000001),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((vec![0], vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaStopParallelBlockWrapper>().unwrap();
        let binding = operation.call1((vec![1], 0.0000001)).unwrap();
        let new_op_diff = binding
            .downcast::<PragmaStopParallelBlockWrapper>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaStopParallelBlockWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        let string_comparison = (
            format!("{:?}", pragma_wrapper) == 
            "PragmaStopParallelBlockWrapper { internal: PragmaStopParallelBlock { qubits: [0], execution_time: Float(0.0000001) } }"
        ) || (
            format!("{:?}", pragma_wrapper) == 
            "PragmaStopParallelBlockWrapper { internal: PragmaStopParallelBlock { qubits: [0], execution_time: Float(1e-7) } }"
        );

        assert!(string_comparison)
    })
}

/// Test PragmaGlobalPhase new() function
#[test]
fn test_pyo3_new_global_phase() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaGlobalPhaseWrapper>();
        let binding = operation.call1((0.003,)).unwrap();
        let new_op = binding.downcast::<PragmaGlobalPhaseWrapper>().unwrap();
        let input_definition =
            Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.003)));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((vec!["fails"],));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaGlobalPhaseWrapper>().unwrap();
        let binding = operation.call1((0.001,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaGlobalPhaseWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaGlobalPhaseWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaGlobalPhaseWrapper { internal: PragmaGlobalPhase { phase: Float(0.003) } }"
        );
    })
}

/// Test PragmaSleep new() function
#[test]
fn test_pyo3_new_sleep() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaSleepWrapper>();
        let binding = operation.call1((vec![0], 0.0000001)).unwrap();
        let new_op = binding.downcast::<PragmaSleepWrapper>().unwrap();

        let input_definition =
            Operation::from(PragmaSleep::new(vec![0], CalculatorFloat::from(0.0000001)));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((vec![0], vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaSleepWrapper>().unwrap();
        let binding = operation.call1((vec![1], 0.0000001)).unwrap();
        let new_op_diff = binding.downcast::<PragmaSleepWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaSleepWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        let string_comparison = (
            format!("{:?}", pragma_wrapper) == 
            "PragmaSleepWrapper { internal: PragmaSleep { qubits: [0], sleep_time: Float(0.0000001) } }"
        ) || (
            format!("{:?}", pragma_wrapper) == 
            "PragmaSleepWrapper { internal: PragmaSleep { qubits: [0], sleep_time: Float(1e-7) } }"
        );

        assert!(string_comparison)
    })
}

/// Test PragmaActiveReset new() function
#[test]
fn test_pyo3_new_active_reset() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaActiveResetWrapper>();
        let binding = operation.call1((0,)).unwrap();
        let new_op = binding.downcast::<PragmaActiveResetWrapper>().unwrap();

        let input_definition = Operation::from(PragmaActiveReset::new(0));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaActiveResetWrapper>().unwrap();
        let binding = operation.call1((1,)).unwrap();
        let new_op_diff = binding.downcast::<PragmaActiveResetWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaActiveResetWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaActiveResetWrapper { internal: PragmaActiveReset { qubit: 0 } }"
        );
    })
}

/// Test PragmaStartDecompositionBlock new() function
#[test]
fn test_pyo3_new_start_decomposition_block() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaStartDecompositionBlockWrapper>();
        let binding = operation.call1((vec![0], reordering())).unwrap();
        let new_op = binding
            .downcast::<PragmaStartDecompositionBlockWrapper>()
            .unwrap();

        let input_definition =
            Operation::from(PragmaStartDecompositionBlock::new(vec![0], reordering()));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op
            .extract::<PragmaStartDecompositionBlockWrapper>()
            .unwrap();
        let binding = operation.call1((vec![1], reordering())).unwrap();
        let new_op_diff = binding
            .downcast::<PragmaStartDecompositionBlockWrapper>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaStartDecompositionBlockWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaStartDecompositionBlockWrapper { internal: PragmaStartDecompositionBlock { qubits: [0], reordering_dictionary: {0: 0} } }"
        );
    })
}

/// Test PragmaStopDecompositionBlock new() function
#[test]
fn test_pyo3_new_stop_decomposition_block() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaStopDecompositionBlockWrapper>();
        let binding = operation.call1((vec![0],)).unwrap();
        let new_op = binding
            .downcast::<PragmaStopDecompositionBlockWrapper>()
            .unwrap();

        let input_definition = Operation::from(PragmaStopDecompositionBlock::new(vec![0]));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op
            .extract::<PragmaStopDecompositionBlockWrapper>()
            .unwrap();
        let binding = operation.call1((vec![1],)).unwrap();
        let new_op_diff = binding
            .downcast::<PragmaStopDecompositionBlockWrapper>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaStopDecompositionBlockWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaStopDecompositionBlockWrapper { internal: PragmaStopDecompositionBlock { qubits: [0] } }"
        );
    })
}

/// Test PragmaDamping new() function
#[test]
fn test_pyo3_new_damping() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaDampingWrapper>();
        let binding = operation.call1((0, 0.005, 0.02)).unwrap();
        let new_op = binding.downcast::<PragmaDampingWrapper>().unwrap();
        let input_definition = Operation::from(PragmaDamping::new(
            0,
            CalculatorFloat::from(0.005),
            CalculatorFloat::from(0.02),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation.call1((0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaDampingWrapper>().unwrap();
        let binding = operation.call1((1, 0.005, 0.02)).unwrap();
        let new_op_diff = binding.downcast::<PragmaDampingWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaDampingWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaDampingWrapper { internal: PragmaDamping { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) } }"
        );
    })
}

/// Test PragmaDepolarising new() function
#[test]
fn test_pyo3_new_depolarising() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaDepolarisingWrapper>();
        let binding = operation.call1((0, 0.005, 0.02)).unwrap();
        let new_op = binding.downcast::<PragmaDepolarisingWrapper>().unwrap();
        let input_definition = Operation::from(PragmaDepolarising::new(
            0,
            CalculatorFloat::from(0.005),
            CalculatorFloat::from(0.02),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation.call1((0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaDepolarisingWrapper>().unwrap();
        let binding = operation.call1((1, 0.005, 0.02)).unwrap();
        let new_op_diff = binding.downcast::<PragmaDepolarisingWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaDepolarisingWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaDepolarisingWrapper { internal: PragmaDepolarising { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) } }"
        );
    })
}

/// Test PragmaDephasing new() function
#[test]
fn test_pyo3_new_dephasing() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaDephasingWrapper>();
        let binding = operation.call1((0, 0.005, 0.02)).unwrap();
        let new_op = binding.downcast::<PragmaDephasingWrapper>().unwrap();
        let input_definition = Operation::from(PragmaDephasing::new(
            0,
            CalculatorFloat::from(0.005),
            CalculatorFloat::from(0.02),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation.call1((0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaDephasingWrapper>().unwrap();
        let binding = operation.call1((1, 0.005, 0.02)).unwrap();
        let new_op_diff = binding.downcast::<PragmaDephasingWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaDephasingWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaDephasingWrapper { internal: PragmaDephasing { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) } }"
        );
    })
}

/// Test PragmaRandomNoise new() function
#[test]
fn test_pyo3_new_randomnoise() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaRandomNoiseWrapper>();
        let binding = operation.call1((0, 0.005, 0.02, 0.01)).unwrap();
        let new_op = binding.downcast::<PragmaRandomNoiseWrapper>().unwrap();
        let input_definition = Operation::from(PragmaRandomNoise::new(
            0,
            CalculatorFloat::from(0.005),
            CalculatorFloat::from(0.02),
            CalculatorFloat::from(0.01),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((0, vec!["fails"], 0.0, 0.0));
        assert!(result.is_err());
        let result = operation.call1((0, 0.0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation.call1((0, 0.0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaRandomNoiseWrapper>().unwrap();
        let binding = operation.call1((1, 0.005, 0.02, 0.01)).unwrap();
        let new_op_diff = binding.downcast::<PragmaRandomNoiseWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaRandomNoiseWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaRandomNoiseWrapper { internal: PragmaRandomNoise { qubit: 0, gate_time: Float(0.005), depolarising_rate: Float(0.02), dephasing_rate: Float(0.01) } }"
        );
    })
}

/// Test PragmaGeneralNoise new() function
#[test]
fn test_pyo3_new_general_noise() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation = py.get_type::<PragmaGeneralNoiseWrapper>();

        let pylist = PyList::new(
            py,
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ],
        )
        .unwrap();
        let binding_from_pylist = operation.call1((0, 1.0, pylist));
        assert!(binding_from_pylist.is_ok());

        let to_get_operators = Operation::from(PragmaGeneralNoise::new(
            0,
            CalculatorFloat::from(0.005),
            operators(),
        ));
        let convert_to_get_operators = convert_operation_to_pyobject(to_get_operators).unwrap();
        let operators_op = convert_to_get_operators.call_method0(py, "rates").unwrap();

        let binding = operation
            .call1((0, 0.005, operators_op.clone_ref(py)))
            .unwrap();
        let new_op = binding.downcast::<PragmaGeneralNoiseWrapper>().unwrap();

        let comparison_copy = bool::extract_bound(
            &new_op
                .call_method1("__eq__", (convert_to_get_operators,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        // Error initialisation
        let result = operation.call1((0, vec!["fails"], 0.0, operators_op.clone_ref(py)));
        assert!(result.is_err());

        let result = operation.call1((0, 0.0, vec!["fails"], operators_op.clone_ref(py)));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let pragma_wrapper = new_op.extract::<PragmaGeneralNoiseWrapper>().unwrap();
        let binding = operation.call1((1, 0.005, operators_op)).unwrap();
        let new_op_diff = binding.downcast::<PragmaGeneralNoiseWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaGeneralNoiseWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaGeneralNoiseWrapper { internal: PragmaGeneralNoise { qubit: 0, gate_time: Float(0.005), rates: [[1.0, 0.0, 0.0],\n [0.0, 1.0, 0.0],\n [0.0, 0.0, 1.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 } }"
        );
    })
}

/// Test PragmaConditional new() function
#[test]
fn test_pyo3_new_conditional() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaConditionalWrapper>();
        let binding = operation
            .call1(("ro".to_string(), 0, new_circuit(py)))
            .unwrap();
        let new_op = binding.downcast::<PragmaConditionalWrapper>().unwrap();

        let input_definition =
            Operation::from(PragmaConditional::new("ro".to_string(), 0, Circuit::new()));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaConditionalWrapper>().unwrap();
        let binding = operation
            .call1(("ro".to_string(), 2, new_circuit(py)))
            .unwrap();
        let new_op_diff = binding.downcast::<PragmaConditionalWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaConditionalWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaConditionalWrapper { internal: PragmaConditional { condition_register: \"ro\", condition_index: 0, circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } } }"
        );
    })
}

/// Test PragmaControlledCircuit new() function
#[test]
fn test_pyo3_new_controlled_circuit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaControlledCircuitWrapper>();
        let binding = operation.call1((0, new_circuit(py))).unwrap();
        let new_op = binding
            .downcast::<PragmaControlledCircuitWrapper>()
            .unwrap();

        let input_definition = Operation::from(PragmaControlledCircuit::new(0, Circuit::new()));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaControlledCircuitWrapper>().unwrap();
        let binding = operation.call1((2, new_circuit(py))).unwrap();
        let new_op_diff = binding
            .downcast::<PragmaControlledCircuitWrapper>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaControlledCircuitWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaControlledCircuitWrapper { internal: PragmaControlledCircuit { controlling_qubit: 0, circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } } }"
        );
    })
}

/// Test PragmaLoop new() function
#[test]
fn test_pyo3_new_loop() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaLoopWrapper>();
        let binding = operation
            .call1(("number_t".to_string(), new_circuit(py)))
            .unwrap();
        let new_op = binding.downcast::<PragmaLoopWrapper>().unwrap();

        let input_definition = Operation::from(PragmaLoop::new("number_t".into(), Circuit::new()));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaLoopWrapper>().unwrap();
        let binding = operation
            .call1(("ro".to_string(), new_circuit(py)))
            .unwrap();
        let new_op_diff = binding.downcast::<PragmaLoopWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaLoopWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaLoopWrapper { internal: PragmaLoop { repetitions: Str(\"number_t\"), circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } } }"
        );
    })
}

/// Test PragmaAnnotatedOp new() function
#[test]
fn test_pyo3_new_annotated_op() {
    let internal_op_0 = Operation::from(PauliX::new(0));
    let internal_op_1 = Operation::from(PauliX::new(1));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaAnnotatedOpWrapper>();
        let binding = operation
            .call1((
                convert_operation_to_pyobject(internal_op_0).unwrap(),
                "test",
            ))
            .unwrap();
        let new_op = binding.downcast::<PragmaAnnotatedOpWrapper>().unwrap();

        let input_definition = Operation::from(PragmaAnnotatedOp::new(
            Operation::from(PauliX::new(0)),
            "test".to_string(),
        ));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaAnnotatedOpWrapper>().unwrap();
        let binding = operation
            .call1((
                convert_operation_to_pyobject(internal_op_1).unwrap(),
                "test",
            ))
            .unwrap();
        let new_op_diff = binding.downcast::<PragmaAnnotatedOpWrapper>().unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<PragmaAnnotatedOpWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaAnnotatedOpWrapper { internal: PragmaAnnotatedOp { operation: PauliX(PauliX { qubit: 0 }), annotation: \"test\" } }"
        );
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators())); "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaControlledCircuit::new(0, roqoqo::Circuit::new())); "PragmaControlledCircuit")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(Operation::from(PauliX::new(0)), "test".to_string())); "PragmaAnnotatedOp")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
        assert!(result.is_err());
    })
}

/// Test json_schema function for all pragma operations
#[cfg(feature = "json_schema")]
#[test_case(PragmaOperation::from(PragmaSetStateVector::new(statevector())); "PragmaSetStateVector")]
#[test_case(PragmaOperation::from(PragmaSetDensityMatrix::new(densitymatrix())); "PragmaSetDensityMatrix")]
#[test_case(PragmaOperation::from(PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(PragmaOperation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(PragmaOperation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(PragmaOperation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(PragmaOperation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(PragmaOperation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(PragmaOperation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(PragmaOperation::from(PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(PragmaOperation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(PragmaOperation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(PragmaOperation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(PragmaOperation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(PragmaOperation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005),  operators())); "PragmaGeneralNoise")]
#[test_case(PragmaOperation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(PragmaOperation::from(PragmaControlledCircuit::new( 1, create_circuit())); "PragmaControlledCircuit")]
#[test_case(PragmaOperation::from(PragmaLoop::new(CalculatorFloat::from("number_t"), Circuit::default())); "PragmaLoop")]
#[test_case(PragmaOperation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(PragmaOperation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)); "PragmaOverrotation")]
fn test_pyo3_json_schema(operation: PragmaOperation) {
    let rust_schema = match operation {
        PragmaOperation::PragmaSetNumberOfMeasurements(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaSetNumberOfMeasurements))
                .unwrap()
        }
        PragmaOperation::PragmaSetStateVector(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaSetStateVector)).unwrap()
        }
        PragmaOperation::PragmaSetDensityMatrix(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaSetDensityMatrix)).unwrap()
        }
        PragmaOperation::PragmaRepeatGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaRepeatGate)).unwrap()
        }
        PragmaOperation::PragmaOverrotation(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaOverrotation)).unwrap()
        }
        PragmaOperation::PragmaBoostNoise(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaBoostNoise)).unwrap()
        }
        PragmaOperation::PragmaStopParallelBlock(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaStopParallelBlock)).unwrap()
        }
        PragmaOperation::PragmaGlobalPhase(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaGlobalPhase)).unwrap()
        }
        PragmaOperation::PragmaSleep(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaSleep)).unwrap()
        }
        PragmaOperation::PragmaActiveReset(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaActiveReset)).unwrap()
        }
        PragmaOperation::PragmaStartDecompositionBlock(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaStartDecompositionBlock))
                .unwrap()
        }
        PragmaOperation::PragmaStopDecompositionBlock(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaStopDecompositionBlock))
                .unwrap()
        }
        PragmaOperation::PragmaDamping(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaDamping)).unwrap()
        }
        PragmaOperation::PragmaDepolarising(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaDepolarising)).unwrap()
        }
        PragmaOperation::PragmaDephasing(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaDephasing)).unwrap()
        }
        PragmaOperation::PragmaRandomNoise(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaRandomNoise)).unwrap()
        }
        PragmaOperation::PragmaGeneralNoise(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaGeneralNoise)).unwrap()
        }
        PragmaOperation::PragmaConditional(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaConditional)).unwrap()
        }
        PragmaOperation::PragmaLoop(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaLoop)).unwrap()
        }
        PragmaOperation::PragmaControlledCircuit(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PragmaControlledCircuit)).unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = match operation {
            PragmaOperation::PragmaLoop(_) => "1.1.0".to_string(),
            PragmaOperation::PragmaControlledCircuit(_) => "1.5.0".to_string(),
            _ => "1.0.0".to_string(),
        };
        let converted_op = Operation::from(operation);
        let pyobject = convert_operation_to_pyobject(converted_op).unwrap();
        let operation = pyobject.bind(py);

        let schema: String =
            String::extract_bound(&operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&operation.call_method0("min_supported_version").unwrap())
                .unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, minimum_version);
    });
}
