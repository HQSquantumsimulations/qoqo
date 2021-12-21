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

use nalgebra::{Complex, DMatrix, DVector};
use ndarray::Array2;
use rand::distributions::{Standard, Uniform};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use roqoqo::operations::{
    DefinitionBit, GateOperation, PauliZ, PragmaRepeatedMeasurement, TwoQubitGateOperation,
};
use roqoqo::prelude::*;
use roqoqo::{
    measurements::{BasisRotation, BasisRotationInput},
    operations::*,
    Circuit,
};

/// Provides input data to run a stochastic gate test.
///
/// # Arguments
///
/// * `gate` - roqoqo GateOperation to be measured.
/// * `preparation_gates` - List of roqoqo SingleQubitGateOperations for the randomly chosen initial state preparation.
/// * `basis_rotations_gates` - List of roqoqo SingleQubitGateOperations to perform randomly chosen basis rotation.
/// * `two_qubit_gate` - None or Some(TwoQubitGateOperation).
/// * `number_stochastic_tests` - Number of the test runs.
/// * `number_projective_measurement` - Number of the measurements.
///
/// # Returns
///
/// * Tuple `(measurement, expected_values)`
pub fn prepare_monte_carlo_gate_test(
    gate: GateOperation,
    preparation_gates: Vec<SingleQubitGateOperation>,
    basis_rotations_gates: Vec<SingleQubitGateOperation>,
    two_qubit_gate: Option<TwoQubitGateOperation>,
    number_stochastic_tests: usize,
    number_projective_measurement: usize,
) -> (BasisRotation, HashMap<String, f64>) {
    if let Some(x) = two_qubit_gate {
        if !(x.control() == &0 && x.target() == &1 || x.control() == &1 && x.target() == &0) {
            panic!("Provided two_qubit_gate does not act on qubits 0 and 1")
        }
    }

    let number_qubits = match gate.involved_qubits() {
        InvolvedQubits::Set(x) => x.len(),
        _ => panic!("Tested gate has no well defined number of qubits"),
    };

    // initialize variables
    let gate_matrix = ndarray_to_nalgebra(gate.unitary_matrix().unwrap());
    let id_matrix: DMatrix<Complex<f64>> = DMatrix::identity(2, 2);
    let mut starting_vec: DVector<Complex<f64>> = DVector::from_element(
        2_usize.pow(number_qubits as u32),
        Complex::<f64>::new(0.0, 0.0),
    );
    starting_vec[1] = Complex::<f64>::new(1.0, 0.0);

    let mut expected_values: HashMap<String, f64> = HashMap::new();
    let mut measurement_input = BasisRotationInput::new(number_qubits, false);
    let mut measurement_circuits: Vec<Circuit> = Vec::new();
    // for random number generation
    let mut rng = thread_rng();

    // loop over test runs
    for i in 0..number_stochastic_tests {
        let mut init_circuit = Circuit::new();
        let mut meas_circuit = Circuit::new();
        meas_circuit += DefinitionBit::new(format!("ro_{}", i), number_qubits, true);

        let mut pauli_product_mask: Vec<usize> = Vec::new();
        // randomly choose one of the provided preparation_gates for the initial state preparation.
        let prep = preparation_gates.choose(&mut rng).unwrap();
        // randomly choose one of the provided basis_rotations_gates for the measurement.
        let meas = basis_rotations_gates.choose(&mut rng).unwrap();
        let involve_qubit: bool = rand::random();
        let mut init_matrix: DMatrix<Complex<f64>> =
            ndarray_to_nalgebra(prep.unitary_matrix().unwrap());
        init_circuit += prep.clone();
        let mut basis_rot_matrix: DMatrix<Complex<f64>> = if involve_qubit {
            pauli_product_mask.push(0);
            meas_circuit += meas.clone();
            ndarray_to_nalgebra(meas.unitary_matrix().unwrap())
        } else {
            id_matrix.clone()
        };
        let mut measurement_matrix = if involve_qubit {
            ndarray_to_nalgebra(PauliZ::new(0).unitary_matrix().unwrap())
        } else {
            id_matrix.clone()
        };
        // loop over number of qubits in each test run
        for n in 1..number_qubits {
            let prep = preparation_gates.choose(&mut rng).unwrap();
            let meas = basis_rotations_gates.choose(&mut rng).unwrap();
            let involve_qubit: bool = rand::random();
            let mut mapping: HashMap<usize, usize> = HashMap::new();
            let _ = mapping.insert(0, n);
            init_matrix =
                ndarray_to_nalgebra(prep.unitary_matrix().unwrap()).kronecker(&init_matrix);
            init_circuit += prep.remap_qubits(&mapping).unwrap();
            if involve_qubit {
                pauli_product_mask.push(n);
                basis_rot_matrix = ndarray_to_nalgebra(meas.unitary_matrix().unwrap())
                    .kronecker(&basis_rot_matrix);
                meas_circuit += meas.remap_qubits(&mapping).unwrap();
                measurement_matrix = ndarray_to_nalgebra(PauliZ::new(0).unitary_matrix().unwrap())
                    .kronecker(&measurement_matrix);
            } else {
                basis_rot_matrix = id_matrix.kronecker(&basis_rot_matrix);
                measurement_matrix = id_matrix.kronecker(&measurement_matrix);
            }
        }
        meas_circuit += PragmaRepeatedMeasurement::new(
            format!("ro_{}", i),
            number_projective_measurement,
            None,
        );

        let j = measurement_input
            .add_pauli_product(format!("ro_{}", i), pauli_product_mask)
            .unwrap();
        let mut linear_map: HashMap<usize, f64> = HashMap::new();
        linear_map.insert(j, 1.0);
        measurement_input
            .add_linear_exp_val(format!("exp_val_{}", i), linear_map)
            .unwrap();
        let circuit = init_circuit + gate.clone() + meas_circuit;
        measurement_circuits.push(circuit);

        //  Expectation value <0|Matrix|0>
        let expected_value = (init_matrix.conjugate().transpose()
            * gate_matrix.clone().adjoint()
            * basis_rot_matrix.adjoint()
            * measurement_matrix
            * basis_rot_matrix
            * gate_matrix.clone()
            * init_matrix)[(0, 0)];
        let _ = expected_values.insert(format!("exp_val_{}", i), expected_value.re);
    }
    let measurement = BasisRotation {
        circuits: measurement_circuits,
        input: measurement_input,
        constant_circuit: None,
    };
    (measurement, expected_values)
}

/// Function to construct a random Circuit for stochastic gate tests.
///
/// # Arguments
///
/// * `circuit_length` - The number of gates to be added to the random Circuit.
/// * `number_qubits` - Number of qubits in the circuit.
/// * `seed` - Seed for the random number generator.
///
/// # Returns
///
/// * `circuit` - The constructed random roqoqo Circuit.
///
pub fn construct_random_circuit(circuit_length: usize, number_qubits: usize, seed: u64) -> Circuit {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut circuit = Circuit::new();
    for _ in 0..circuit_length {
        let tmp_seed: u64 = rng.sample(Standard);
        add_random_operation(&mut circuit, number_qubits, tmp_seed)
    }
    circuit
}

/// Function to add a random operation to the Circuit.
///
/// A random single- or two-qubit gate operation is added to the circuit.
/// All arguments for the gate operation, like qubit number, rotation angle, etc. are randomly sitributed.
///
/// # Arguments
///
/// * `circuit` - Mutable roqoqo Circuit, where the operation is added.
/// * `number_qubits` - Number of qubits in the circuit.
/// * `seed` - Seed for the random number generator.
///
pub fn add_random_operation(circuit: &mut Circuit, number_qubits: usize, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    let qubits_dist = Uniform::from(0..number_qubits);
    let two_qubits_dist = Uniform::from(0..number_qubits - 1);
    let gate_type_dist = Uniform::from(0..35);
    let new_op: Operation = match gate_type_dist.sample(&mut rng) {
        0 => PauliX::new(qubits_dist.sample(&mut rng)).into(),
        1 => PauliY::new(qubits_dist.sample(&mut rng)).into(),
        2 => PauliZ::new(qubits_dist.sample(&mut rng)).into(),
        3 => Hadamard::new(qubits_dist.sample(&mut rng)).into(),
        4 => SqrtPauliX::new(qubits_dist.sample(&mut rng)).into(),
        5 => InvSqrtPauliX::new(qubits_dist.sample(&mut rng)).into(),
        7 => {
            let theta: f64 = rng.sample(Standard);
            RotateX::new(qubits_dist.sample(&mut rng), theta.into()).into()
        }
        8 => {
            let theta: f64 = rng.sample(Standard);
            RotateY::new(qubits_dist.sample(&mut rng), theta.into()).into()
        }
        9 => {
            let theta: f64 = rng.sample(Standard);
            RotateZ::new(qubits_dist.sample(&mut rng), theta.into()).into()
        }
        10 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            CNOT::new(qubit, qubit + 1).into()
        }
        11 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            ControlledPauliY::new(qubit, qubit + 1).into()
        }
        12 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            ControlledPauliZ::new(qubit, qubit + 1).into()
        }
        13 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            MolmerSorensenXX::new(qubit, qubit + 1).into()
        }
        14 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            SqrtISwap::new(qubit, qubit + 1).into()
        }
        15 => {
            let theta: f64 = rng.sample(Standard);
            let qubit = two_qubits_dist.sample(&mut rng);
            VariableMSXX::new(qubit, qubit + 1, theta.into()).into()
        }
        16 => {
            let theta: f64 = rng.sample(Standard);
            let qubit = two_qubits_dist.sample(&mut rng);
            ControlledPhaseShift::new(qubit, qubit + 1, theta.into()).into()
        }
        17 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let delta_real: f64 = rng.sample(Standard);
            let delta_imag: f64 = rng.sample(Standard);
            Bogoliubov::new(qubit, qubit + 1, delta_real.into(), delta_imag.into()).into()
        }
        18 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let t_real: f64 = rng.sample(Standard);
            let t_imag: f64 = rng.sample(Standard);
            ComplexPMInteraction::new(qubit, qubit + 1, t_real.into(), t_imag.into()).into()
        }
        19 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            FSwap::new(qubit, qubit + 1).into()
        }
        20 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let t: f64 = rng.sample(Standard);
            let u: f64 = rng.sample(Standard);
            let delta: f64 = rng.sample(Standard);
            Fsim::new(qubit, qubit + 1, t.into(), u.into(), delta.into()).into()
        }
        21 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let theta: f64 = rng.sample(Standard);
            let phi: f64 = rng.sample(Standard);
            GivensRotation::new(qubit, qubit + 1, theta.into(), phi.into()).into()
        }
        22 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let theta: f64 = rng.sample(Standard);
            let phi: f64 = rng.sample(Standard);
            GivensRotationLittleEndian::new(qubit, qubit + 1, theta.into(), phi.into()).into()
        }
        23 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            ISwap::new(qubit, qubit + 1).into()
        }
        24 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            InvSqrtISwap::new(qubit, qubit + 1).into()
        }
        25 => {
            let t: f64 = rng.sample(Standard);
            let qubit = two_qubits_dist.sample(&mut rng);
            PMInteraction::new(qubit, qubit + 1, t.into()).into()
        }
        26 => {
            let theta: f64 = rng.sample(Standard);
            let qubit = qubits_dist.sample(&mut rng);
            PhaseShiftState0::new(qubit, theta.into()).into()
        }
        27 => {
            let theta: f64 = rng.sample(Standard);
            let qubit = qubits_dist.sample(&mut rng);
            PhaseShiftState1::new(qubit, theta.into()).into()
        }
        28 => {
            let phi: f64 = rng.sample(Standard);
            let qubit = two_qubits_dist.sample(&mut rng);
            PhaseShiftedControlledZ::new(qubit, qubit + 1, phi.into()).into()
        }
        29 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let x: f64 = rng.sample(Standard);
            let y: f64 = rng.sample(Standard);
            let z: f64 = rng.sample(Standard);
            Qsim::new(qubit, qubit + 1, x.into(), y.into(), z.into()).into()
        }
        30 => {
            let qubit = qubits_dist.sample(&mut rng);
            let theta: f64 = rng.sample(Standard);
            let spherical_theta: f64 = rng.sample(Standard);
            let spherical_phi: f64 = rng.sample(Standard);
            RotateAroundSphericalAxis::new(
                qubit,
                theta.into(),
                spherical_theta.into(),
                spherical_phi.into(),
            )
            .into()
        }
        31 => SGate::new(qubits_dist.sample(&mut rng)).into(),
        32 => TGate::new(qubits_dist.sample(&mut rng)).into(),
        33 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            SWAP::new(qubit, qubit + 1).into()
        }
        34 => {
            let qubit = two_qubits_dist.sample(&mut rng);
            let x: f64 = rng.sample(Standard);
            let y: f64 = rng.sample(Standard);
            let z: f64 = rng.sample(Standard);
            SpinInteraction::new(qubit, qubit + 1, x.into(), y.into(), z.into()).into()
        }
        35 => {
            let theta: f64 = rng.sample(Standard);
            let qubit = two_qubits_dist.sample(&mut rng);
            XY::new(qubit, qubit + 1, theta.into()).into()
        }
        _ => {
            let theta: f64 = rng.sample(Standard);
            RotateZ::new(qubits_dist.sample(&mut rng), theta.into()).into()
        }
    };
    circuit.add_operation(new_op);
}

// Helper conversion function
fn ndarray_to_nalgebra(input: Array2<Complex<f64>>) -> DMatrix<Complex<f64>> {
    let shape = input.shape();
    let matrix: DMatrix<Complex<f64>> =
        DMatrix::from_iterator(shape[0], shape[1], input.t().iter().cloned());
    matrix
}
