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
"""Testing the qoqo Circuit class"""

import pytest
import sys
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from typing import Any
from hqsbase.qonfig import Qonfig
from copy import deepcopy


def test_to_hqs_lang() -> None:
    """Test export import to HQS-Quil"""
    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.Definition(name='test', vartype='float', length=3)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')
    lines = circuit.to_hqs_lang()
    lines_test = ["Definition ro BIT[1]",
                  "Definition test REAL[3]",
                  "Hadamard 0",
                  "MeasureQubit 0 ro[0]"]
    assert lines ==  lines_test
    npt.assert_equal(circuit.count_occurences(), 4)
    npt.assert_equal(circuit.count_occurences(operations_list=[ops.GateOperation]), 1)


def test_reversed_order() -> None:
    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.PauliX(0)
    circuit += ops.PauliY(0)
    test_circuit = Circuit()
    test_circuit += ops.Definition(name='ro', vartype='bit', length=1)
    test_circuit += ops.PauliY(0)
    test_circuit += ops.PauliX(0)
    circuit2 = Circuit() + circuit[::-1]
    assert test_circuit == circuit2
    circuit2 = Circuit() + reversed(circuit)
    assert test_circuit == circuit2


def test_getitem_setitem_delitem() -> None:
    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.RotateX(qubit=0, theta='theta')
    circuit += ops.Definition(name='test', vartype='float', length=3)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')
    assert circuit[0] == ops.Definition(name='ro', vartype='bit', length=1)
    assert circuit[1] == ops.Definition(name='test', vartype='float', length=3)
    assert circuit[2] == ops.Hadamard(qubit=0)
    assert circuit[0:3] == [ops.Definition(name='ro', vartype='bit', length=1),
                            ops.Definition(name='test', vartype='float', length=3),
                            ops.Hadamard(qubit=0)]
    assert circuit[:3] == [ops.Definition(name='ro', vartype='bit', length=1),
                           ops.Definition(name='test', vartype='float', length=3),
                           ops.Hadamard(qubit=0)]
    assert circuit[2:5] == [
        ops.Hadamard(qubit=0),
        ops.RotateX(qubit=0, theta='theta'),
        ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')]
    assert circuit[2:] == [
        ops.Hadamard(qubit=0),
        ops.RotateX(qubit=0, theta='theta'),
        ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')]
    del circuit[0]
    assert circuit[0] == ops.Definition(name='test', vartype='float', length=3)
    assert len(circuit) == 4
    circuit[0] = ops.Definition(name='ro', vartype='bit', length=1)
    assert circuit[0] == ops.Definition(name='ro', vartype='bit', length=1)
    assert len(circuit) == 4
    del circuit[1]
    assert circuit[1] == ops.RotateX(qubit=0, theta='theta')
    assert len(circuit) == 3
    circuit[1] = ops.Hadamard(qubit=0)
    assert circuit[1] == ops.Hadamard(qubit=0)
    assert len(circuit) == 3
 
    circuit_copy = deepcopy(circuit)
    assert circuit == circuit_copy

    circuit.insert(1, ops.Definition('test', 'float', 1, False, False))
    assert circuit[1] == ops.Definition('test', 'float', 1, False, False)


def test_circuit_sequence_methods() -> None:
    """Test magick methods implementing sequence interface in circuit"""
    circuit1 = Circuit()
    circuit2 = Circuit()

    circuit1 += ops.Definition(name='ro', vartype='bit', length=1)
    circuit1 += ops.Hadamard(qubit=0)
    circuit1 += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')
    npt.assert_equal(circuit1[1], ops.Hadamard(qubit=0))
    circuit2 += ops.Definition(name='ro', vartype='bit', length=1)
    circuit2 += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')
    assert not circuit1 == circuit2
    del circuit1[1]
    npt.assert_equal(circuit1, circuit2)

    npt.assert_equal(len(circuit1), 2)


@pytest.mark.parametrize("gates_only", [True, False])
def test_get_operation_types(gates_only: bool) -> None:
    """Test getting type of operations in circuit"""
    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.CNOT(qubit=0, control=1)
    circuit += ops.GivensRotationLittleEndian(qubit=0, control=1)
    circuit += ops.PMInteraction(i=0, j=1)
    circuit += ops.Bogoliubov(i=0, j=1)
    circuit += ops.Fsim()
    circuit += ops.Definition(name='test', vartype='float', length=3)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')
    circuit += ops.PragmaSetNumberOfMeasurements(number_measurements=100)

    list_types = circuit.get_operation_types(gates_only)
    reference_list_types = ['Hadamard', 'CNOT',
                            'GivensRotationLittleEndian', 'PMInteraction', 'Fsim']
    reference_list_types_with = ['Definition', 'MeasureQubit', 'PragmaSetNumberOfMeasurements']

    for t in reference_list_types:
        assert t in list_types
    if not gates_only:
        for t in reference_list_types_with:
            assert t in list_types


def test_substitution() -> None:
    """Test parameter substitution in circuit"""

    Theta = 'theta'
    Theta2 = 'theta2'
    Delta = 'delta'

    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.CNOT(qubit=0, control=1)
    circuit += ops.GivensRotationLittleEndian(qubit=0, control=1, theta=Theta)
    circuit += ops.PMInteraction(i=0, j=1, theta=Theta2)
    circuit += ops.Bogoliubov(i=0, j=1, Delta=Delta)
    circuit += ops.Fsim()
    circuit += ops.Definition(name='test', vartype='float', length=3)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index=0)
    circuit += ops.PragmaSetNumberOfMeasurements(number_measurements=100)

    circuit2 = Circuit()
    circuit2 += ops.Definition(name='ro', vartype='bit', length=1)
    circuit2 += ops.Hadamard(qubit=0)
    circuit2 += ops.CNOT(qubit=0, control=1)
    circuit2 += ops.GivensRotationLittleEndian(qubit=0, control=1, theta=0)
    circuit2 += ops.PMInteraction(i=0, j=1, theta=1)
    circuit2 += ops.Bogoliubov(i=0, j=1, Delta=2)
    circuit2 += ops.Fsim()
    circuit2 += ops.Definition(name='test', vartype='float', length=3)
    circuit2 += ops.MeasureQubit(qubit=0, readout='ro', readout_index=0)
    circuit2 += ops.PragmaSetNumberOfMeasurements(number_measurements=100)
    circuit.substitute_parameters({'theta': 0, 'theta2': 1, 'delta': 2})

    assert circuit == circuit2

    circuit3 = _serialisation_convertion(circuit)
    assert circuit3 == circuit


def test_remap_qubits() -> None:
    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=1)
    circuit += ops.Hadamard(qubit=0)
    circuit += ops.CNOT(qubit=0, control=1)
    circuit += ops.GivensRotationLittleEndian(qubit=0, control=1, theta=0)
    circuit += ops.PMInteraction(i=0, j=1, theta=0)
    circuit += ops.Bogoliubov(i=0, j=1, Delta=0)
    circuit += ops.Fsim(qubit=0, control=1)
    circuit += ops.Definition(name='test', vartype='float', length=3)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index='0')

    circuit_test = Circuit()
    circuit_test += ops.Definition(name='ro', vartype='bit', length=1)
    circuit_test += ops.Hadamard(qubit=2)
    circuit_test += ops.CNOT(qubit=2, control=3)
    circuit_test += ops.GivensRotationLittleEndian(qubit=2, control=3, theta=0)
    circuit_test += ops.PMInteraction(i=2, j=3, theta=0)
    circuit_test += ops.Bogoliubov(i=2, j=3, Delta=0)
    circuit_test += ops.Fsim(qubit=2, control=3)
    circuit_test += ops.Definition(name='test', vartype='float', length=3)
    circuit_test += ops.MeasureQubit(qubit=2, readout='ro', readout_index='0')

    mapping = {0: 2, 1: 3}

    circuit.remap_qubits(mapping)
    assert circuit == circuit_test


def _serialisation_convertion(to_conv: Qonfig[Any]) -> Any:
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
