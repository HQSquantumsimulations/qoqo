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
"""Testing cheated measurements"""

import pytest
import sys
import numpy as np
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from qoqo import measurements
from hqsbase import qonfig
from hqsbase.qonfig import Qonfig
from typing import Dict, List, Any
import pandas as pd
import scipy.sparse as sp

OPERATOR_MATRICES: Dict[str, Dict[str, List[float]]] = dict()
OPERATOR_MATRICES['test1'] = dict()
OPERATOR_MATRICES['test2'] = dict()
OPERATOR_MATRICES['test1']['a'] = sp.csr_matrix(np.zeros((4, 4), dtype=complex))
OPERATOR_MATRICES['test1']['a'][0, 0] = 0.5
OPERATOR_MATRICES['test1']['a'][1, 1] = 0.5j
OPERATOR_MATRICES['test1']['a'][2, 2] = 0.5j
OPERATOR_MATRICES['test1']['b'] = sp.csr_matrix(np.zeros((4, 4), dtype=complex))
OPERATOR_MATRICES['test1']['b'][0, 0] = 1 + 1j
OPERATOR_MATRICES['test2']['c'] = sp.csr_matrix(np.zeros((4, 4), dtype=complex))
OPERATOR_MATRICES['test2']['c'][0, 0] = 1 + 2j
OPERATOR_MATRICES['test2']['d'] = sp.csr_matrix(np.zeros((4, 4), dtype=complex))
OPERATOR_MATRICES['test2']['d'][0, 0] = 1 + 3j
# Should give expectation values of a = 0.5, b= 1+1j, c=1+2j, d = 1+3j for system in state 0000
USE_DENSITY_MATRIX = False


def test_cheated_measurement_input_init():
    """Test cheated measurement input using the init function"""
    measurement_input = measurements.PurePragmaMeasurementInput()
    assert measurement_input.operator_matrices == dict()
    assert measurement_input.use_density_matrix == False


def test_cheated_measurement_input_qonfig():
    """Test cheated measurement input using Qonfig"""
    measurement_input = measurements.PurePragmaMeasurementInput(
        operator_matrices=OPERATOR_MATRICES,
        use_density_matrix=USE_DENSITY_MATRIX
    )
    measurement_qonfig = measurement_input.to_qonfig()
    measurement_qonfig2 = qonfig.Qonfig.from_json(measurement_qonfig.to_json())
    measurement_input2 = measurement_qonfig2.to_instance()
    assert measurement_input.operator_matrices.keys() == measurement_input2.operator_matrices.keys()
    for readout_key, readout_val in measurement_input.operator_matrices.items():
        readout_val2 = measurement_input2.operator_matrices[readout_key]
        assert readout_val.keys() == readout_val2.keys()
        for key, val in readout_val.items():
            val2 = readout_val2[key]
            assert np.array_equal(val.todense(), val2.todense())
    assert measurement_input.use_density_matrix == measurement_input2.use_density_matrix


def test_cheated_none():
    """Test none return"""
    measurement = measurements.PurePragmaMeasurement()
    expectation_values = measurement()
    assert list(expectation_values.keys()) == list()


def test_cheated_measurement_pyquest():
    """Test cheated measurement"""
    measurement_input = measurements.PurePragmaMeasurementInput(
        operator_matrices=OPERATOR_MATRICES,
        use_density_matrix=USE_DENSITY_MATRIX
    )
    circuit = Circuit()
    circuit += ops.Definition(name='test1', vartype='complex',
                              length=2**2,
                              is_output=True)
    circuit += ops.PragmaGetStateVector(readout='test1',
                                        )
    circuit2 = Circuit()
    circuit2 += ops.Definition(name='test2', vartype='complex',
                               length=2**2,
                               is_output=True)
    circuit2 += ops.PragmaGetStateVector(readout='test2',
                                         )

    circuit_list = [circuit, circuit2]
    try:
        from qoqo_pyquest import PyQuestBackend
        backend = PyQuestBackend(number_qubits=2)
        measurement = measurements.PurePragmaMeasurement(
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
