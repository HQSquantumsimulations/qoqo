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
"""Testing the basis rotation measurements"""

import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from qoqo import measurements
from hqsbase import qonfig
from hqsbase.qonfig import Qonfig
from typing import Any

PAULI_PRODUCT_QUBIT_MASKS = dict()
PAULI_PRODUCT_QUBIT_MASKS['test1'] = {0: list(), 1: [0,1]}
PAULI_PRODUCT_QUBIT_MASKS['test2'] = {2: [0], 3: [1]}
PP_TO_EXP_VAL_ΜΑΤRIX = np.zeros((4, 4), dtype=complex)
PP_TO_EXP_VAL_ΜΑΤRIX[0, 0] = 0.5
PP_TO_EXP_VAL_ΜΑΤRIX[1, 0] = 1j
PP_TO_EXP_VAL_ΜΑΤRIX[2, 0] = 1
PP_TO_EXP_VAL_ΜΑΤRIX[3, 0] = 1
PP_TO_EXP_VAL_ΜΑΤRIX[1, 1] = 1
PP_TO_EXP_VAL_ΜΑΤRIX[2, 2] = 2j
PP_TO_EXP_VAL_ΜΑΤRIX[3, 3] = 3j
# Should give expectation values of a = 0.5, b= 1+1j, c=1+2j, d = 1+3j for system in state 0000
NUMBER_PAULI_PRODUCTS = 4
NUMBER_QUBITS = 2
USE_FLIPPED_MEASUREMENT = False
MEASURED_EXP_VALS = ['a', 'b', 'c', 'd']


def test_br_measurement_input_init():
    """Test basis rotation measurement input using the init function"""
    measurement_input = measurements.BRMeasurementInput()
    assert measurement_input._pauli_product_qubit_masks == dict()
    assert np.array_equal(measurement_input._pp_to_exp_val_matrix, np.zeros((0, 0)))
    assert measurement_input._number_qubits == 0
    assert measurement_input._number_pauli_products == 0
    assert measurement_input._measured_exp_vals == []
    assert measurement_input._use_flipped_measurement == False


def test_br_measurement_input_qonfig():
    """Test basis rotation measurement input using Qonfig"""
    measurement_input = measurements.BRMeasurementInput(
        pauli_product_qubit_masks=PAULI_PRODUCT_QUBIT_MASKS,
        pp_to_exp_val_matrix=PP_TO_EXP_VAL_ΜΑΤRIX,
        number_pauli_products=NUMBER_PAULI_PRODUCTS,
        number_qubits=NUMBER_QUBITS,
        use_flipped_measurement=USE_FLIPPED_MEASUREMENT,
        measured_exp_vals=MEASURED_EXP_VALS
    )
    measurement_qonfig = measurement_input.to_qonfig()
    measurement_qonfig2 = qonfig.Qonfig.from_json(measurement_qonfig.to_json())
    measurement_input2 = measurement_qonfig2.to_instance()
    assert (measurement_input._pauli_product_qubit_masks.keys()
            == measurement_input2._pauli_product_qubit_masks.keys())
    for key, val in measurement_input._pauli_product_qubit_masks.items():
        val2 = measurement_input2._pauli_product_qubit_masks[key]
        assert val.keys() == val2.keys()
        for key2, val3 in val.items():
            val4 = val2[key2]
            assert val3 == val4
    assert np.array_equal(measurement_input._pp_to_exp_val_matrix,
                          measurement_input2._pp_to_exp_val_matrix)
    assert measurement_input._number_qubits == measurement_input2._number_qubits
    assert measurement_input._number_pauli_products == measurement_input2._number_pauli_products
    assert measurement_input._measured_exp_vals == measurement_input2._measured_exp_vals
    assert measurement_input._use_flipped_measurement == measurement_input._use_flipped_measurement


def test_basis_rotation_pyquest():
    """Test basis rotation measurement"""
    measurement_input = measurements.BRMeasurementInput(
        pauli_product_qubit_masks=PAULI_PRODUCT_QUBIT_MASKS,
        pp_to_exp_val_matrix=PP_TO_EXP_VAL_ΜΑΤRIX,
        number_pauli_products=NUMBER_PAULI_PRODUCTS,
        number_qubits=NUMBER_QUBITS,
        use_flipped_measurement=USE_FLIPPED_MEASUREMENT,
        measured_exp_vals=MEASURED_EXP_VALS
    )
    circuit = Circuit()
    circuit += ops.Definition(name='test1', vartype='bit', length=2, is_output=True)
    circuit += ops.MeasureQubit(qubit=0, readout='test1', readout_index=0)
    circuit += ops.MeasureQubit(qubit=1, readout='test1', readout_index=1)
    circuit2 = Circuit()
    circuit2 += ops.Definition(name='test2', vartype='bit', length=2, is_output=True)
    circuit2 += ops.MeasureQubit(qubit=0, readout='test2', readout_index=0)
    circuit2 += ops.MeasureQubit(qubit=1, readout='test2', readout_index=1)

    circuit_list = [circuit, circuit2]
    try:
        from qoqo_pyquest import PyQuestBackend
        backend = PyQuestBackend(number_qubits=2,)
        measurement = measurements.BasisRotationMeasurement(
            measurement_input=measurement_input,
            circuit_list=circuit_list,
            backend=backend,
            )
        expectation_values = measurement()
        assert set(expectation_values.keys()) == set(
            ['exp_val_a', 'exp_val_b', 'exp_val_c', 'exp_val_d', ])
        assert expectation_values['exp_val_a'] == 0.5
        assert expectation_values['exp_val_b'] == 1 + 1j
        assert expectation_values['exp_val_c'] == 1 + 2j
        assert expectation_values['exp_val_d'] == 1 + 3j
    except ImportError:
        pass


def test_basis_rotation_none():
    """Test basis rotation None return"""
    measurement = measurements.BasisRotationMeasurement()
    expectation_values = measurement()
    assert list(expectation_values.keys()) == list()


def test_basis_rotation_pyquest_w_readout_errors():
    """Test basis rotation measurement with readout errors"""
    masks = PAULI_PRODUCT_QUBIT_MASKS
    masks['test1_flipped'] = {0: list(), 1: [0, 1]}
    masks['test2_flipped'] = {2: [0], 3: [1]}
    measurement_input = measurements.BRMeasurementInput(
        pauli_product_qubit_masks=masks,
        pp_to_exp_val_matrix=PP_TO_EXP_VAL_ΜΑΤRIX,
        number_pauli_products=NUMBER_PAULI_PRODUCTS,
        number_qubits=NUMBER_QUBITS,
        use_flipped_measurement=True,
        measured_exp_vals=MEASURED_EXP_VALS
    )
    circuit = Circuit()
    circuit += ops.Definition(name='test1', vartype='bit', length=2, is_output=True)
    circuit += ops.MeasureQubit(qubit=0, readout='test1', readout_index=0)
    circuit += ops.MeasureQubit(qubit=1, readout='test1', readout_index=1)
    circuit2 = Circuit()
    circuit2 += ops.Definition(name='test2', vartype='bit', length=2, is_output=True)
    circuit2 += ops.MeasureQubit(qubit=0, readout='test2', readout_index=0)
    circuit2 += ops.MeasureQubit(qubit=1, readout='test2', readout_index=1)
    circuit3 = Circuit()
    circuit3 += ops.Definition(name='test1_flipped', vartype='bit', length=2, is_output=True)
    circuit3 += ops.PauliX(qubit=0)
    circuit3 += ops.PauliX(qubit=1)
    circuit3 += ops.MeasureQubit(qubit=0, readout='test1_flipped', readout_index=0)
    circuit3 += ops.MeasureQubit(qubit=1, readout='test1_flipped', readout_index=1)
    circuit4 = Circuit()
    circuit4 += ops.Definition(name='test2_flipped', vartype='bit', length=2, is_output=True)
    circuit4 += ops.PauliX(qubit=0)
    circuit4 += ops.PauliX(qubit=1)
    circuit4 += ops.MeasureQubit(qubit=0, readout='test2_flipped', readout_index=0)
    circuit4 += ops.MeasureQubit(qubit=1, readout='test2_flipped', readout_index=1)

    circuit_list = [circuit, circuit2, circuit3, circuit4]
    try:
        from qoqo_pyquest import PyQuestBackend
        backend = PyQuestBackend(number_qubits=2)
        measurement = measurements.BasisRotationMeasurement(
            measurement_input=measurement_input,
            circuit_list=circuit_list,
            backend=backend,
            )
        measurement2 = _serialisation_convertion(measurement)
        expectation_values = measurement()
        assert set(expectation_values.keys()) == set(
            ['exp_val_a', 'exp_val_b', 'exp_val_c', 'exp_val_d', ])
        assert expectation_values['exp_val_a'] == 0.5
        assert expectation_values['exp_val_b'] == 1 + 1j
        assert expectation_values['exp_val_c'] == 1 + 2j
        assert expectation_values['exp_val_d'] == 1 + 3j
    except ImportError:
        pass


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
