# Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Test qoqo measurement operations"""
import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from copy import copy
from hqsbase.qonfig import Qonfig


def test_measurement() -> None:
    """Test MeasureQubit operation"""
    op = ops.MeasureQubit
    string = 'MeasureQubit 0 ro[1]'
    (q0, ro, ri) = (0,
                    'ro',
                    1)
    operation = op(qubit=q0,
                   readout=ro,
                   readout_index=ri)

    assert(operation.to_hqs_lang() == string)
    assert(not operation.is_parameterized)
    assert(set(operation.involved_qubits) == set([0]))

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_remap_qubits_measurement() -> None:
    """Test remap qubits function of Measurement operation"""
    operation = ops.MeasureQubit(qubit=0,
                                 readout='ro',
                                 readout_index=0)
    mapping = {0: 2}
    operation.remap_qubits(mapping)
    assert operation._qubit == 2
    assert operation._readout_index == 0
    assert operation.involved_qubits == set([2])

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_pragma_repeated_measurement() -> None:
    """Test repeated measurement PRAGMA"""
    op = ops.PragmaRepeatedMeasurement
    string1 = 'PragmaRepeatedMeasurement(100) ALL ro'
    string2 = 'PragmaRepeatedMeasurement(100) 0 ro[0] '

    operation = op(readout='ro',
                   number_measurements=100)
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation

    operation = op(readout='ro',
                   qubit_mapping={0: 0},
                   number_measurements=100)
    assert(operation.to_hqs_lang() == string2)

    operation2 = _serialisation_convertion(operation)
    print(operation2._qubit_mapping, operation._qubit_mapping)
    assert operation2 == operation


def test_pragma_pauli_product_measurement() -> None:
    """Test Pauli product measurement PRAGMA"""
    op = ops.PragmaGetPauliProduct
    string1 = 'PragmaGetPauliProduct ro'

    operation = op(readout='ro',
                   pauli_product=[1, 0],
                   )
    assert(operation.to_hqs_lang() == string1)

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_pragma_get_statevec() -> None:
    """Test get state vector (measurement) PRAGMA"""
    op = ops.PragmaGetStateVector
    string1 = 'PragmaGetStateVector{} ro'.format('')

    operation = op(readout='ro',
                   qubit_mapping=None
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


@pytest.mark.parametrize("gate", [
    ops.PragmaGetStateVector,
    ops.PragmaGetDensityMatrix,
    ops.PragmaGetOccupationProbability,
    ops.PragmaRepeatedMeasurement
])
def test_remap_qubits_get_statevec(gate) -> None:
    """Test remap qubits function of get statevec PRAGMA"""
    operation = gate(readout='ro',
                     qubit_mapping={0: 1, 1: 0}
                     )
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation._qubit_mapping == {2: 1, 3: 0}

    operation = ops.PragmaGetStateVector(readout='ro',
                                         )
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation._qubit_mapping is None

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_pragma_get_densitymatrix() -> None:
    """Test get density matrix (measurement) PRAGMA"""
    op = ops.PragmaGetDensityMatrix
    string1 = 'PragmaGetDensityMatrix ro'
    operation = op(readout='ro',
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_pragma_get_occupation_probability() -> None:
    """Test get occupation probability (measurement) PRAGMA"""
    op = ops.PragmaGetOccupationProbability
    string1 = 'PragmaGetOccupationProbability{} ro'.format('')
    operation = op(readout='ro',
                   qubit_mapping=None
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation


def test_pragma_get_rotated_occupation_probability() -> None:
    """Test get rotated occupation probability (measurement) PRAGMA"""
    op = ops.PragmaGetRotatedOccupationProbability
    string1 = 'PragmaGetRotatedOccupationProbability ro'
    circuit = Circuit()
    circuit += ops.Hadamard(qubit=0)
    operation = op(readout='ro',
                   circuit=circuit
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation3 = _serialisation_convertion(operation)
    print(operation, operation3)
    assert operation3 == operation

    operation2 = copy(operation)
    assert operation2 == operation
    operation2._circuit = Circuit()
    assert not operation2 == operation
    operation2._readout_index = 20
    assert not operation2 == operation
    operation2._readout = 'test'
    assert not operation2 == operation

    operation4 = _serialisation_convertion(operation2)
    assert operation4 == operation2


def test_pragma_pauli_prod_measurement() -> None:
    """Test product of pauli operators measurement PRAGMA"""
    op = ops.PragmaPauliProdMeasurement
    string1 = 'PragmaPauliProdMeasurement 0, 1 ro[0]'
    circuit = Circuit()
    circuit += ops.Hadamard(qubit=0)
    operation = op(readout='ro',
                   readout_index=0,
                   qubits=[0],
                   paulis=[1]
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set([0]))

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    operation2 = copy(operation)
    assert operation2 == operation
    operation2._paulis = [3]
    assert not operation2 == operation
    operation2._qubits = [3]
    assert not operation2 == operation
    operation2._readout_index = 20
    assert not operation2 == operation
    operation2._readout = 'test'
    assert not operation2 == operation

    operation4 = _serialisation_convertion(operation2)
    assert operation4 == operation2


def test_pragma_get_pauli_prod_measurement() -> None:
    """Test get product of pauli operators (measurement) PRAGMA"""
    string1 = 'PragmaGetPauliProduct ro'

    op = ops.PragmaGetPauliProduct
    circuit = Circuit()
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.PauliX(qubit=1)
    circuit += ops.CNOT(control=0, qubit=2)
    operation = op(readout='ro',
                   pauli_product=[1, 0]
                   )
    assert(operation.to_hqs_lang() == string1)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    operation2 = copy(operation)
    assert operation2 == operation

    operation4 = _serialisation_convertion(operation2)
    assert operation4 == operation2


def _serialisation_convertion(to_conv):
    """Convertion function for all serialisation unittests

    Takes the object input, serialises and deserialises it, returning the deserialised
    version. The original object and the manipulated one are then compared in the unittest
    script to assert that the serialisation worked correctly.

    Args:
        to_conv: object to be serialised and deserialised

    Returns:
        Any: deserialised object
    """
    config = to_conv.to_qonfig()
    json = config.to_json()
    config2 = Qonfig.from_json(json)
    converted = config2.to_instance()

    return converted


if __name__ == '__main__':
    pytest.main(sys.argv)
