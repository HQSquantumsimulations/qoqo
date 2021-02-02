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
"""Testing the qoqo DoUnitary class"""

import pytest
import sys
import numpy.testing as npt
from qoqo.measurements import BRMeasurementInput, BasisRotationMeasurement
from qoqo import DoUnitary
from qoqo import Circuit
from qoqo import operations as ops
import numpy as np
import scipy.sparse as sp
from typing import Any
from hqsbase.qonfig import Qonfig


def test_do_unitary():
    number_measurements = 100000

    circuit = Circuit()
    circuit += ops.Definition(name='ro', vartype='bit', length=2, is_output=True)
    circuit += ops.RotateX(qubit=0, theta='3*time+offset')
    circuit += ops.PragmaRepeatedMeasurement(readout='ro', number_measurements=number_measurements)

    measurement_input = BRMeasurementInput(
        pauli_product_qubit_masks={'ro': {0: [0]}}, # From readout 'ro' measure two pauli products 0: < Z0 > 
        pp_to_exp_val_matrix= np.array([[1]]), # One expectation value: 3* pauli_product0 + 1* pauli_product1
        number_qubits=1,
        number_pauli_products=1,
        measured_exp_vals=['example'])

    measurement = BasisRotationMeasurement(measurement_input=measurement_input, circuit_list=[circuit], verbose=False)

    try:
        from qoqo_pyquest import PyQuestBackend
        backend = PyQuestBackend(circuit=circuit, number_qubits=2)

        do_unitary = DoUnitary(measurement=measurement, backend=backend, free_parameters=['time', 'offset']) # The symbolic parameter is the free parameter
        results = do_unitary([0.5, 0])

        assert np.isclose(results[0], 0.06, atol=0.02)
        assert results[1] == 0.5
        assert results[2] == 0.0

        do_unitary2 = _serialisation_convertion(do_unitary)

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
