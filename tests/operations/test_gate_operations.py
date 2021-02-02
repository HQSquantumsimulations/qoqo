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
"""Test qoqo gate operations"""
import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from qoqo.operations import OperationNotInBackendError
from typing import (
    Dict,
    Optional
)
from hqsbase.calculator import Calculator
from copy import copy
from hqsbase.qonfig import Qonfig


@pytest.mark.parametrize("init", [
    (ops.Hadamard, 'Hadamard 0'),
    (ops.PauliX, 'PauliX 0'),
    (ops.PauliY, 'PauliY 0'),
    (ops.PauliZ, 'PauliZ 0'),
    (ops.SGate, 'SGate 0'),
    (ops.TGate, 'TGate 0'),
    (ops.SqrtPauliX, 'SqrtPauliX 0'),
    (ops.InvSqrtPauliX, 'InvSqrtPauliX 0'),
    (ops.CNOT, 'CNOT 1 0'),
    (ops.SWAP, 'SWAP 1 0'),
    (ops.ControlledPauliZ, 'ControlledPauliZ 1 0'),
    (ops.ControlledPauliY, 'ControlledPauliY 1 0'),
    (ops.MolmerSorensenXX, 'MolmerSorensenXX 1 0'),
    (ops.InvSqrtISwap, 'InvSqrtISwap 1 0'),
    (ops.SqrtISwap, 'SqrtISwap 1 0'),
])
def test_simple_gate_matrices(init):
    """Test gate operations without free parameters"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters()
    # Testing hqs_lang output functionality
    if op.number_of_qubits() == 1:
        q0 = 0
        operation = op(qubit=q0)
    else:
        q0, q1 = (0, 1)
        operation = op(control=q1, qubit=q0)
    assert(operation.to_hqs_lang() == string)
    assert(str(operation) == string)

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation

    assert(not operation.is_parameterized)
    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))

    # Testing power implementation
    with npt.assert_raises(AttributeError):
        operation**1.5


@pytest.mark.parametrize(
    "init",
    [(ops.SingleQubitGate,
      'SingleQubitGate(alpha_r, alpha_i, beta_r, beta_i, global_phase) 0'),
     ])
@pytest.mark.parametrize("a", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("b", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("c", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("d", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
def test_single_qubit_gate(init, a, b, c, d) -> None:
    """Test general single qubit gate operation"""
    op = init[0]
    string = init[1]
    alpha = np.exp(1j * a) * np.cos(b)
    beta = np.exp(1j * c) * np.sin(b)
    matrix_gate = op.unitary_matrix_from_parameters(alpha_r=np.real(
        alpha), alpha_i=np.imag(alpha), beta_r=np.real(beta), beta_i=np.imag(beta))
    Alpha_r, Alpha_i, Beta_r, Beta_i, Global_phase, q0 = (
        ('alpha_r', 'alpha_i', 'beta_r', 'beta_i', 'global_phase', 0))
    operation = op(qubit=q0, alpha_r=Alpha_r, alpha_i=Alpha_i,
                   beta_r=Beta_r, beta_i=Beta_i, global_phase=Global_phase)

    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    operation2 = _serialisation_convertion(operation)
    assert operation2 == operation

    alpha = np.exp(1j * a) * np.cos(b)
    beta = np.exp(1j * c) * np.sin(b)
    substitution_dict = {
        'alpha_r': np.real(alpha),
        'alpha_i': np.imag(alpha),
        'beta_r': np.real(beta),
        'beta_i': np.imag(beta),
        'global_phase': d}

    operation.substitute_parameters(substitution_dict)

    assert(not operation.is_parameterized)

    npt.assert_equal(set(operation.involved_qubits), set([0]))


@pytest.mark.parametrize("op", [
    ops.PauliY(qubit=0),
    ops.PauliZ(qubit=0),
    ops.TGate(qubit=0),
    ops.SGate(qubit=0)])
@pytest.mark.parametrize("a", list(np.arange(2 * np.pi / 3, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("b", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("c", list(np.arange(2 * np.pi / 3, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("d", list(np.arange(2 * np.pi / 3, np.pi, 2 * np.pi / 3)))
def test_single_qubit_multiplication(op, a, b, c, d):
    """Test single qubit gate mulitplication"""
    alpha = np.exp(1j * a) * np.cos(b)
    beta = np.exp(1j * c) * np.sin(b)
    op1 = ops.SingleQubitGate(qubit=0, alpha_r=np.real(alpha),
                              alpha_i=np.imag(alpha),
                              beta_r=np.real(beta),
                              beta_i=np.imag(beta),
                              global_phase=d)
    op2 = ops.SingleQubitGate(qubit=0, alpha_r=np.real(alpha),
                              alpha_i=np.imag(alpha),
                              beta_r=np.real(beta),
                              beta_i=np.imag(beta),
                              global_phase=d)
    opnew = op1 * op
    op2 *= op
    npt.assert_array_almost_equal(op1.unitary_matrix @ op.unitary_matrix, opnew.unitary_matrix)
    npt.assert_array_almost_equal(op1.unitary_matrix @ op.unitary_matrix, op2.unitary_matrix)

    op2 = _serialisation_convertion(op)
    assert op2 == op


@pytest.mark.parametrize("operation", [ops.RotateX,
                                       ops.RotateY,
                                       ops.RotateZ])
@pytest.mark.parametrize("a", list(np.arange(2 * np.pi / 3, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("b", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("c", list(np.arange(2 * np.pi / 3, 2 * np.pi, 2 * np.pi / 3)))
@pytest.mark.parametrize("d", [0])
@pytest.mark.parametrize("theta_p", list(np.arange(0, 2 * np.pi, 2 * np.pi / 3)))
def test_single_qubit_multiplication_parameter(operation, a, b, c, d, theta_p):
    """Test single qubit gate multiplication with constant parameters"""
    alpha = np.exp(1j * a) * np.cos(b)
    beta = np.exp(1j * c) * np.sin(b)
    op = operation(qubit=0, theta=theta_p)
    op1 = ops.SingleQubitGate(qubit=0, alpha_r=np.real(alpha),
                              alpha_i=np.imag(alpha),
                              beta_r=np.real(beta),
                              beta_i=np.imag(beta),
                              global_phase=d)
    op2 = ops.SingleQubitGate(qubit=0, alpha_r=np.real(alpha),
                              alpha_i=np.imag(alpha),
                              beta_r=np.real(beta),
                              beta_i=np.imag(beta),
                              global_phase=d)
    opnew = op1 * op
    op2 *= op
    npt.assert_array_almost_equal(op1.unitary_matrix @ op.unitary_matrix, opnew.unitary_matrix)
    npt.assert_array_almost_equal(op1.unitary_matrix @ op.unitary_matrix, op2.unitary_matrix)

    op2 = _serialisation_convertion(op)
    assert op2 == op


@pytest.mark.parametrize("init", [(ops.RotateX, 'RotateX(theta) 0'),
                                  (ops.RotateY, 'RotateY(theta) 0'),
                                  (ops.RotateZ, 'RotateZ(theta) 0'),
                                  (ops.ControlledPhaseShift, 'ControlledPhaseShift(theta) 1 0')
                                  ])
@pytest.mark.parametrize("theta_p", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
def test_single_parameter_gate_matrices(init, theta_p) -> None:
    """Test gate operations with single parameter"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(theta=theta_p)
    if op.number_of_qubits() == 1:
        theta, q0 = ('theta', 0)
        operation = op(qubit=q0, theta=theta)
    else:
        (theta, q0, q1) = ('theta', 0, 1)
        operation = op(control=q1, qubit=q0, theta=theta)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    substitution_dict = {'theta': theta_p}
    operation.substitute_parameters(substitution_dict)
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['theta'].isclose(
        1.5 * operation._ordered_parameter_dict['theta'])


@pytest.mark.parametrize("init", [(ops.W, 'W(theta, spherical_phi) 0'),
                                  ])
@pytest.mark.parametrize("theta_p", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
@pytest.mark.parametrize("phi_p", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
def test_parameter_gate_matrices(init, theta_p, phi_p) -> None:
    """Test gate operations with two parameters"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(theta=theta_p, spherical_phi=phi_p)
    if op.number_of_qubits() == 1:
        (theta, spherical_phi, q0) = ('theta', 'spherical_phi', 0)
        operation = op(qubit=q0,
                       theta=theta, spherical_phi=spherical_phi)
    else:
        (theta, spherical_phi, q0, q1) = ('theta', 'spherical_phi', 0, 1)
        operation = op(control=q1, qubit=q0,
                       theta=theta, spherical_phi=spherical_phi)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'theta': theta_p,
                                         'spherical_phi': phi_p})
    else:
        operation.substitute_parameters({'theta': theta_p,
                                         'spherical_phi': phi_p})
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['theta'].isclose(
        1.5 * operation._ordered_parameter_dict['theta'])


@pytest.mark.parametrize("init", [(ops.PMInteraction, 'PMInteraction(theta) 1 0'),
                                  ])
@pytest.mark.parametrize("theta", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
def test_PM(init, theta) -> None:
    """Test plus-minus gate operation"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(theta)
    if op.number_of_qubits() == 1:
        (Theta, q0) = ('theta', 0)
        operation = op(qubit=q0,
                       theta=Theta,)
    else:
        (Theta, q0, q1) = ('theta', 0, 1)
        operation = op(i=q1, j=q0,
                       theta=Theta)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'theta': theta,
                                         })
    else:
        operation.substitute_parameters({'theta': theta,
                                         })
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['theta'].isclose(
        1.5 * operation._ordered_parameter_dict['theta'])


@pytest.mark.parametrize("init", [(ops.GivensRotation, 'GivensRotation(theta, phi) 0 1'),
                                  (ops.GivensRotationLittleEndian,
                                   'GivensRotationLittleEndian(theta, phi) 0 1'),
                                  ])
@pytest.mark.parametrize("theta", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
@pytest.mark.parametrize("phi", list(np.arange(2 * np.pi / 3, 2 * np.pi, 2 * np.pi / 3)))
def test_Givens(init, theta, phi) -> None:
    """Test Givens rotation gate operation"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(theta, phi)
    if op.number_of_qubits() == 1:
        (theta, phi, q0) = ('theta',
                            'phi', 0)
        operation = op(qubit=q0,
                       theta=theta, phi=phi)
    else:
        (Theta, Phi, q0, q1) = ('theta',
                                'phi', 0, 1)
        if init[0] == ops.GivensRotation:
            operation = op(qubit=q0, control=q1,
                           theta=Theta, phi=Phi)
        else:
            operation = op(qubit=q0, control=q1,
                           theta=Theta, phi=Phi)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'theta': theta,
                                         'phi': phi})
    else:
        operation.substitute_parameters({'theta': theta,
                                         'phi': phi})
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))


@pytest.mark.parametrize("init", [(ops.Bogoliubov, 'Bogoliubov(Delta_real, Delta_imag) 1 0'),
                                  ])
@pytest.mark.parametrize("delta", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
@pytest.mark.parametrize("delta_arg", list(np.arange(0, 2 * np.pi, 2 * np.pi / 10)))
def test_Bogoliubov(init, delta, delta_arg) -> None:
    """Test Bogoliubov-deGennes gate operation"""
    delta_real = np.real(delta * np.exp(1j * delta_arg))
    delta_imag = np.imag(delta * np.exp(1j * delta_arg))
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(delta_real, delta_imag)
    if op.number_of_qubits() == 1:
        (Delta_real, Delta_imag, q0) = ('Delta_real',
                                        'Delta_imag', 0)
        operation = op(qubit=q0,
                       Delta_real=Delta_real, Delta_imag=Delta_imag)
    else:
        (Delta_real, Delta_imag, q0, q1) = ('Delta_real',
                                            'Delta_imag', 0, 1)
        operation = op(i=q1, j=q0,
                       Delta_real=Delta_real, Delta_imag=Delta_imag)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'Delta_real': delta_real,
                                         'Delta_imag': delta_imag})
    else:
        operation.substitute_parameters({'Delta_real': delta_real,
                                         'Delta_imag': delta_imag})
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))

    operation2 = operation**1.5
    assert operation2._ordered_parameter_dict['Delta_imag'].isclose(
        1.5 * operation._ordered_parameter_dict['Delta_imag'])
    assert operation2._ordered_parameter_dict['Delta_real'].isclose(
        1.5 * operation._ordered_parameter_dict['Delta_real'])


parameter_list = [0, .1, np.pi, -np.pi, np.pi / 4,
                  2 * np.pi, -np.pi - .1, -.1, np.pi + .1, 2 * np.pi + .1]
parameter_list3 = [[p1, p2, p2] for p1 in parameter_list for p2 in parameter_list[:3]] + \
                  [[p2, p1, p2] for p1 in parameter_list[3:] for p2 in parameter_list[:3]] + \
                  [[p2, p2, p1] for p1 in parameter_list[3:] for p2 in parameter_list[:3]]


@pytest.mark.parametrize("init", [(ops.Fsim, 'Fsim(U, t, Delta) 0 1')])
@pytest.mark.parametrize("U, t, Delta", parameter_list3)
def test_Fsim(init, U, t, Delta) -> None:
    """Test fermionic simulation gate operation"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(U, t, Delta)
    if op.number_of_qubits() == 1:
        (Us, ts, Deltas, q0) = ('U', 't', 'Delta', 0)
        operation = op(qubit=q0,
                       U=Us, t=ts, Delta=Deltas)
    else:
        (Us, ts, Deltas, q0, q1) = ('U', 't', 'Delta', 0, 1)
        operation = op(qubit=q0, control=q1,
                       U=Us, t=ts, Delta=Deltas)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'U': U,
                                         't': t, 'Delta': Delta})
    else:
        operation.substitute_parameters({'U': U,
                                         't': t, 'Delta': Delta})
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))


@pytest.mark.parametrize("init", [(ops.Qsim, 'Qsim(x, y, z) 0 1')])
@pytest.mark.parametrize("x, y, z", parameter_list3)
def test_Qsim(init, x, y, z) -> None:
    """Test spin swap simulation gate operation"""
    op = init[0]
    string = init[1]
    matrix_gate = op.unitary_matrix_from_parameters(x, y, z)
    if op.number_of_qubits() == 1:
        (Us, ts, Deltas, q0) = ('x', 'y', 'z', 0)
        operation = op(qubit=q0,
                       U=Us, t=ts, Delta=Deltas)
    else:
        (xs, ys, zs, q0, q1) = ('x', 'y', 'z', 0, 1)
        operation = op(qubit=q0, control=q1,
                       x=xs, y=ys, z=zs)
    assert(operation.to_hqs_lang() == string)
    assert(operation.is_parameterized)

    if op.number_of_qubits() == 1:
        operation.substitute_parameters({'x': x,
                                         'y': y, 'z': z})
    else:
        operation.substitute_parameters({'x': x,
                                         'y': y, 'z': z})
    assert(not operation.is_parameterized)

    op2 = _serialisation_convertion(operation)
    assert op2 == operation

    if op.number_of_qubits() == 1:
        npt.assert_equal(set(operation.involved_qubits), set([0]))
    else:
        npt.assert_equal(set(operation.involved_qubits), set([0, 1]))


@pytest.mark.parametrize("init", [(ops.Bogoliubov, 0),
                                  (ops.CNOT, 'CNOT 1 0'),
                                  (ops.SWAP, 'SWAP 1 0'),
                                  (ops.ControlledPauliZ, 'ControlledPauliZ 1 0'),
                                  (ops.ControlledPauliY, 'ControlledPauliY 1 0'),
                                  (ops.MolmerSorensenXX, 'MolmerSorensenXX 1 0'),
                                  (ops.InvSqrtISwap, 'InvSqrtISwap 1 0'),
                                  (ops.SqrtISwap, 'SqrtISwap 1 0'),
                                  ])
def test_remap_qubits_two_qubit_gates(init):
    """Test remap qubits function of two qubit gates"""
    gate = init[0]()
    qubit_mapping = {0: 2, 1: 3}
    new_gate = copy(gate)
    new_gate.remap_qubits(qubit_mapping)
    for key, val in gate._ordered_qubits_dict.items():
        assert new_gate._ordered_qubits_dict[key] == val + 2
    assert new_gate.involved_qubits == set([2, 3])


@pytest.mark.parametrize("init", [(ops.Hadamard, 'Hadamard 0'),
                                  (ops.PauliX, 'PauliX 0'),
                                  (ops.PauliY, 'PauliY 0'),
                                  (ops.PauliZ, 'PauliZ 0'),
                                  (ops.SGate, 'SGate 0'),
                                  (ops.TGate, 'TGate 0'),
                                  (ops.SqrtPauliX, 'SqrtPauliX 0'),
                                  (ops.InvSqrtPauliX, 'InvSqrtPauliX 0'),
                                  ])
def test_remap_qubits_single_qubit_gates(init):
    """Test remap qubits function of single qubit gates"""
    gate = init[0]()
    qubit_mapping = {0: 2}
    new_gate = copy(gate)
    new_gate.remap_qubits(qubit_mapping)
    for key, val in gate._ordered_qubits_dict.items():
        assert new_gate._ordered_qubits_dict[key] == val + 2
    assert new_gate.involved_qubits == set([2])


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
