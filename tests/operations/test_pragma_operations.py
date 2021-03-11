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
"""Test qoqo PRAGMA operations"""
import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from hqsbase.calculator import (
    Calculator,
    CalculatorFloat,
)
from copy import copy
from hqsbase.qonfig import Qonfig


@pytest.mark.parametrize("init", [(ops.PragmaDamping,
                                   'PragmaDamping(gate_time, rate) 0'),
                                  (ops.PragmaDepolarise,
                                   'PragmaDepolarise(gate_time, rate) 0'),
                                  (ops.PragmaDephasing,
                                   'PragmaDephasing(gate_time, rate) 0')
                                  ])
@pytest.mark.parametrize("gate_time", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("rate", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
def test_noise_operators(init, gate_time, rate):
    """Test PRAGMA operators applying noise"""
    op = init[0]
    string = init[1]

    (Gate_time, Rate, q0) = ('gate_time', 'rate', 0)
    operation = op(qubit=q0, gate_time=Gate_time, rate=Rate)

    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)
    substitution_dict = {'gate_time': gate_time, 'rate': rate}
    calculator = Calculator()
    for name, val in substitution_dict.items():
        calculator.set(name, val)

    operation.substitute_parameters(substitution_dict)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set([0]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['gate_time'].isclose(
        1.5 * operation._ordered_parameter_dict['gate_time'])


@pytest.mark.parametrize("gate_time", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("depolarisation_rate", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("dephasing_rate", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
def test_random_noise_operator(gate_time, depolarisation_rate, dephasing_rate):
    """Test PRAGMA operators applying random noise (stochastic unravelling)"""
    op = ops.PragmaRandomNoise
    string = 'PragmaRandomNoise(gate_time, depolarisation_rate, dephasing_rate) 0'

    (Gate_time, depol_Rate, dephasing_Rate, q0) = ('gate_time',
                                                   'depolarisation_rate',
                                                   'dephasing_rate',
                                                   0)
    operation = op(gate_time=Gate_time,
                   depolarisation_rate=depol_Rate, dephasing_rate=dephasing_Rate)

    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)
    substitution_dict = {'gate_time': gate_time,
                         'depolarisation_rate': depolarisation_rate,
                         'dephasing_rate': dephasing_rate}
    calculator = Calculator()
    for name, val in substitution_dict.items():
        calculator.set(name, val)

    operation.substitute_parameters(substitution_dict)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set([0]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['gate_time'].isclose(
        1.5 * operation._ordered_parameter_dict['gate_time'])


@pytest.mark.parametrize("gate_time", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("rate", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("operators", [
    np.array([[0, 3, 1], [2, 0, 0], [0, 0, 1]]),
    np.array([[2, 1, 4], [0, 2, 0], [1, 6, 7]]),
    np.array([[0, 0, 0], [2, 4, 6], [0, 5, 0]]),
])
def test_general_noise_operator(gate_time, rate, operators):
    """Test PRAGMA operators applying general noise"""
    op = ops.PragmaGeneralNoise
    string = 'PragmaGeneralNoise(gate_time, rate, operators) 0'

    (Gate_time, Rate, Operators, q0) = ('gate_time', 'rate', 'operators', 0)
    operation = op(qubits=[q0], gate_time=Gate_time, rate=Rate, operators=Operators)

    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    operation = op(qubits=[q0], gate_time=gate_time, rate=rate, operators=operators)
    substitution_dict = {'gate_time': gate_time, 'rate': rate, 'operators': operators}
    # calculator = Calculator()
    # for name, val in substitution_dict.items():
    #     calculator.set(name, val)

    # operation.substitute_parameters(substitution_dict)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set([0]))

    operation.remap_qubits({0: 2})
    assert operation.involved_qubits == set([2])
    assert operation._qubits == [2]

    assert operation != ops.PragmaStop()
    assert operation != op([2], 12, rate, operators)
    assert operation != op([2], gate_time, 25, operators)
    assert operation != op([2], gate_time, rate, np.zeros((2, 6)))
    assert operation != op([0], gate_time, rate, operators)

    operation = op(['ALL'], Gate_time, Rate, Operators)
    assert operation.to_hqs_lang() == 'PragmaGeneralNoise(gate_time, rate, operators) ALL'


@pytest.mark.parametrize("init", [
    ops.PragmaDamping, ops.PragmaDepolarise, ops.PragmaDephasing, ops.PragmaRandomNoise
])
def test_remap_qubits_single_qubit_gates(init):
    """Test remap qubits function of noise PRAGMAs"""
    gate = init()
    qubit_mapping = {0: 2}
    new_gate = copy(gate)
    new_gate.remap_qubits(qubit_mapping)
    for key, val in gate._ordered_qubits_dict.items():
        assert new_gate._ordered_qubits_dict[key] == val + 2
    assert new_gate.involved_qubits == set([2])

    new_gate2 = _serialisation_convertion(new_gate)
    assert new_gate2 == new_gate


@pytest.mark.parametrize("backend", [
    None, 'pyquest_cffi', 'braket', 'aqt', 'cirq', 'cirq_code', 'pyquil'
])
def test_set_number_measurements_pragma(backend):
    """Test PragmaSetNumberOfMeasurements"""
    op = ops.PragmaSetNumberOfMeasurements(number_measurements=10)
    if backend is None:
        assert op.backend_instruction(backend) == None
    else:
        assert op.backend_instruction(backend) == {'number_measurements': 10}

    assert not op == ops.Definition('ro')
    assert not op == ops.PragmaSetNumberOfMeasurements(number_measurements=10, readout='different')
    assert not op == ops.PragmaSetNumberOfMeasurements(number_measurements=1, readout='ro')   

    assert op.to_hqs_lang() == 'PragmaSetNumberOfMeasurements(10) ro'



def test_repeat_pragma():
    """Test PRAGMA repeating a certain gate"""
    op = ops.PragmaRepeatGate
    Coefficient = 'coefficient'
    string = "PragmaRepeatGate(coefficient)"
    operation = op(repetition_coefficient=Coefficient)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    operation.substitute_parameters({'coefficient': 2})

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))


@pytest.mark.parametrize("init", [(ops.PragmaDamping),
                                  (ops.PragmaDepolarise),
                                  (ops.PragmaDephasing)
                                  ])
def test_noise_boost_pragma(init):
    """Test PRAGMA boosting noise in the circuit"""
    op = ops.PragmaBoostNoise
    Coefficient = 'coefficient'
    string = "PragmaBoostNoise(coefficient)"
    operation = op(noise_coefficient=Coefficient)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    operation.substitute_parameters({'coefficient': 2})

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set())


@pytest.mark.parametrize("init", [('W',
                                   '(W,theta,mean,variance) 0',
                                   ),
                                  ('ControlledPhaseShift',
                                   '(ControlledPhaseShift,theta,mean,variance) 0 1',
                                   ),
                                  ('RotateZ',
                                   '(RotateZ,theta,mean,variance) 0',
                                   )
                                  ])
@pytest.mark.parametrize("stype", ['static', "statistic"])
def test_overrotation_pragma(init, stype):
    """Test PRAGMA for the overrotation of a gate"""
    op = ops.PragmaOverrotation
    (Theta0, Variance, Mean) = ('theta0', 'variance', 'mean')
    ordered_qubits_dict = {'control'}
    string = "PragmaOverrotation {} {}".format(stype, init[1])
    if init[0] in ['W', 'RotateZ']:
        q0 = 0
        ordered_qubits_dict = dict()
        ordered_qubits_dict['qubit'] = q0
    else:
        (q0, q1) = (0, 1)
        ordered_qubits_dict = dict()
        ordered_qubits_dict['control'] = q0
        ordered_qubits_dict['qubit'] = q1
    operation = op(gate=init[0],
                   statistic_type=stype,
                   ordered_qubits_dict=ordered_qubits_dict,
                   parameter='theta',
                   variance=Variance,
                   mean=Mean)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    if init[0] in ['W', 'RotateZ']:
        assert(operation.involved_qubits == set([q0]))
    else:
        assert(set(operation.involved_qubits) == set([q0, q1]))


def test_remap_qubits_overrotations():
    """Test remap qubits function of overrotation PRAGMA"""
    operation = ops.PragmaOverrotation(gate='RotateZ',
                                       statistic_type='static',
                                       ordered_qubits_dict={'qubit': 0},
                                       parameter='theta',
                                       overrotation_parameter='test',
                                       variance=0,
                                       mean=0)
    mapping = {0: 2}
    operation.remap_qubits(mapping)
    assert operation._ordered_qubits_dict['qubit'] == 2

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


def test_stop_pragma():
    """Test STOP pragma"""
    op = ops.PragmaStop
    string = "PragmaStop ALL"
    operation = op()
    assert(operation.to_hqs_lang() == string)
    assert(not operation.is_parameterized)
    assert(operation.involved_qubits == set(['ALL']))

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    string = "PragmaStop 1"
    Q1 = 1

    operation = op(qubits=[Q1])
    assert(operation.involved_qubits == set([Q1]))
    assert(operation.to_hqs_lang() == string)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    string = "PragmaStop(0.001) 1"
    Q1 = 1

    operation = op(qubits=[Q1], execution_time=0.001)
    assert(operation.involved_qubits == set([Q1]))
    assert(operation.to_hqs_lang() == string)

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


@pytest.mark.parametrize("gate", [ops.PragmaStop,
                                  ops.PragmaSleep])
def test_remap_qubits_stop(gate):
    """Test remap qubits function of STOP PRAGMA"""
    operation = gate(qubits=[0, 1])
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation.involved_qubits == set([2, 3])
    operation = ops.PragmaStop()
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation.involved_qubits == set(['ALL'])

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


@pytest.mark.parametrize("initial", [(None,
                                      {},
                                      'PragmaParameterSubstitution'),
                                     ({'test': 0.1},
                                      {'test': 0.1},
                                      'PragmaParameterSubstitution test=0.1;'
                                      ),
                                     ])
def test_stop_parameter_substitution(initial):
    """Test parameter substitution function of STOP PRAGMA"""
    gate = ops.PragmaParameterSubstitution(substitution_dict=initial[0])
    assert gate._substitution_dict == initial[1]
    gate_not_equal = ops.PragmaParameterSubstitution(substitution_dict={'not_equal': 3})
    assert gate != gate_not_equal
    gate_equal = ops.PragmaParameterSubstitution(substitution_dict=initial[0])
    assert gate == gate_equal
    test_string = gate.to_hqs_lang()
    assert test_string == initial[2]
    assert gate.backend_instruction() is None

    gate2 = _serialisation_convertion(gate)
    assert gate2 == gate


def test_global_phase_pragma():
    """Test global phase PRAGMA"""
    gate = ops.PragmaGlobalPhase(phase="test")
    assert gate.to_hqs_lang() == "PragmaGlobalPhase test"
    assert gate.is_parameterized is True
    substitution_dict = {'test': 0.1}
    gate.substitute_parameters(substitution_dict)
    assert not gate.is_parameterized
    assert gate.phase == 0.1

    gate2 = _serialisation_convertion(gate)
    assert gate2 == gate


def test_start_decomposition_block_pragma():
    """Test PragmaStartDecompositionBlock PRAGMA"""
    op = ops.PragmaStartDecompositionBlock

    string = "PragmaStartDecompositionBlock(None) ALL"
    operation = op()
    assert(operation.to_hqs_lang() == string)
    assert(operation.involved_qubits == set(['ALL']))
    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    string = "PragmaStartDecompositionBlock(None) 1"
    Q1 = 1
    operation = op(qubits=[Q1])
    assert(operation.involved_qubits == set([Q1]))
    assert(operation.to_hqs_lang() == string)
    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    string = "PragmaStartDecompositionBlock({1: 2, 0: 1}) 0 1 2"
    Q1 = [0, 1, 2]
    operation = op(qubits=Q1, reordering_dictionary={1: 2, 0: 1})
    assert(operation.involved_qubits == set(Q1))
    assert(operation.to_hqs_lang() == string)
    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


def test_stop_decomposition_block_pragma():
    """Test PragmaStopDecompositionBlock PRAGMA"""
    op = ops.PragmaStopDecompositionBlock

    string = "PragmaStopDecompositionBlock ALL"
    operation = op()
    assert(operation.to_hqs_lang() == string)
    assert(operation.involved_qubits == set(['ALL']))
    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation

    string = "PragmaStopDecompositionBlock 1"
    Q1 = 1
    operation = op(qubits=[Q1])
    assert(operation.involved_qubits == set([Q1]))
    assert(operation.to_hqs_lang() == string)
    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


@pytest.mark.parametrize("gate", [ops.PragmaStartDecompositionBlock,
                                  ops.PragmaStopDecompositionBlock])
def test_remap_qubits_stop(gate):
    """Test remap qubits function of Decomposition block PRAGMAs"""
    operation = gate(qubits=[0, 1])
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation.involved_qubits == set([2, 3])
    operation = ops.PragmaStop()
    mapping = {0: 2, 1: 3}
    operation.remap_qubits(mapping)
    assert operation.involved_qubits == set(['ALL'])

    operation3 = _serialisation_convertion(operation)
    assert operation3 == operation


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
