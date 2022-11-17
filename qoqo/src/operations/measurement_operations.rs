// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use crate::{convert_into_circuit, CircuitWrapper};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_macros::*;
use roqoqo::operations::*;
use roqoqo::Circuit;
use std::collections::HashMap;

#[wrap(Operate, OperateSingleQubit)]
#[derive(Eq)]
/// Measurement gate operation.
///
/// This Operation acts on one qubit writing the result of the measurement into a readout.
/// The classical register for the readout needs to be defined in advance by using a Definition operation.
///
/// Args:
///     qubit (int): The measured qubit.
///     readout (string): The classical register for the readout.
///     readout_index (int): The index in the readout the result is saved to.
///
pub struct MeasureQubit {
    qubit: usize,
    readout: String,
    readout_index: usize,
}

#[wrap(Operate, OperatePragma)]
/// This PRAGMA measurement operation returns the statevector of a quantum register.
///
/// Args:
///     readout (string): The name of the classical readout register.
///     circuit (Optional[Circuit]): The measurement preparation Circuit, applied on a copy of the register before measurement.
///
pub struct PragmaGetStateVector {
    readout: String,
    circuit: Option<Circuit>,
}

#[wrap(Operate, OperatePragma)]
/// This PRAGMA measurement operation returns the density matrix of a quantum register.
///
/// Args:
///     readout (string): The name of the classical readout register.
///     circuit (Optional[Circuit]): The measurement preparation Circuit, applied on a copy of the register before measurement.
///
struct PragmaGetDensityMatrix {
    readout: String,
    circuit: Option<Circuit>,
}

#[wrap(Operate, OperatePragma)]
/// This PRAGMA measurement operation returns the vector of the occupation probabilities.
///
/// Occupation probabilities in the context of this PRAGMA operation are probabilities of finding the quantum
/// register in each :math:`\sigma_z` basis state. The quantum register remains unchanged by this PRAGMA measurement operation.
///
/// Args:
///     readout (string): The name of the classical readout register.
///     circuit (Optional[Circuit]): The Circuit used to rotate the qureg.
///
struct PragmaGetOccupationProbability {
    readout: String,
    circuit: Option<Circuit>,
}

#[wrap(Operate, OperatePragma)]
/// This PRAGMA measurement operation returns a Pauli product expectation value.
///
/// This PRAGMA operation returns a Pauli product expectation value after applying
/// a Rotate to another basis. It performs all of the operation on a clone of the quantum register,
/// sothat the actual quantum register remains unchanged.
///
/// Args:
///     qubit_paulis (dict[int, int]): The dictionary of the pauli matrix to apply to each qubit in the form
///                                    {qubit: pauli}. Allowed values to be provided for 'pauli' are: 0 = identity, 1 = PauliX, 2 = PauliY, 3 = PauliZ.
///     readout (string): The name of the classical readout register.
///     circuit (Circuit): The measurement preparation Circuit, applied on a copy of the register before measurement.
///
struct PragmaGetPauliProduct {
    qubit_paulis: std::collections::HashMap<usize, usize>,
    readout: String,
    circuit: Circuit,
}

#[wrap(Operate, OperatePragma)]
#[derive(Eq)]
/// This PRAGMA measurement operation returns a measurement record for N repeated measurements.
///
/// Args:
///     readout (string): The name of the classical readout register.
///     qubit_mapping (dict[int, int]): The mapping of qubits to indices in readout register.
///     number_measurements (int): The number of times to repeat the measurement.
///
struct PragmaRepeatedMeasurement {
    readout: String,
    number_measurements: usize,
    qubit_mapping: Option<std::collections::HashMap<usize, usize>>,
}
