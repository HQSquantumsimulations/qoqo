"""Test qoqo mocked backend"""

# Copyright © 2019-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
import pytest
import sys
from qoqo import operations as ops
from qoqo import Circuit


@pytest.mark.parametrize(
    "measurement",
    [
        (ops.MeasureQubit(qubit=0, readout="ro", readout_index=0), int, 0),
        (ops.PragmaRepeatedMeasurement(readout="ro", number_measurements=10), int, 0),
        (
            ops.PragmaGetPauliProduct(
                qubit_paulis={0: 1, 1: 2}, readout="ro", circuit=Circuit()
            ),
            float,
            1,
        ),
        (ops.PragmaGetOccupationProbability(readout="ro", circuit=Circuit()), float, 1),
        (ops.PragmaGetStateVector(readout="ro", circuit=Circuit()), complex, 2),
        (ops.PragmaGetDensityMatrix(readout="ro", circuit=Circuit()), complex, 2),
    ],
)
def test_circuit(measurement):
    """Test building a circuit"""
    circuit = Circuit()
    circuit += ops.DefinitionFloat(name="ro", length=1, is_output=True)
    circuit += ops.DefinitionComplex(name="ro", length=1, is_output=True)
    circuit += ops.DefinitionBit(name="ro", length=1, is_output=True)
    circuit += ops.PauliX(qubit=0)
    circuit += measurement[0]
    assert len(circuit) > 0


if __name__ == "__main__":
    pytest.main(sys.argv)
