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
"""Testing qoqo Definition operation"""
import pytest
import sys
import numpy.testing as npt
from qoqo import operations as ops
from hqsbase.qonfig import Qonfig


@pytest.mark.parametrize("name", ['a', 'b', 'c'])
@pytest.mark.parametrize("vartype", ['float', 'bit', 'int'])
@pytest.mark.parametrize("length", [1, 2, 3])
@pytest.mark.parametrize("is_input", [True, False])
@pytest.mark.parametrize("is_output", [True, False])
def test_hqs_lang_define(name, vartype, length, is_input, is_output):
    """Test exporting/importing Define to HQS-Lang"""
    operation = ops.Definition
    assert operation.get_hqs_lang_name() == 'Definition'
    op = operation(name=name,
                   vartype=vartype,
                   length=length,
                   is_input=is_input,
                   is_output=is_output)
    if is_input or is_output:
        if vartype == 'float':
            string = "Definition({},{}) {} REAL[{}]".format(is_input, is_output, name, length)
        elif vartype == 'bit':
            string = "Definition({},{}) {} BIT[{}]".format(is_input, is_output, name, length)
        elif vartype == 'int':
            string = "Definition({},{}) {} INT[{}]".format(is_input, is_output, name, length)
    else:
        if vartype == 'float':
            string = "Definition {} REAL[{}]".format(name, length)
        elif vartype == 'bit':
            string = "Definition {} BIT[{}]".format(name, length)
        elif vartype == 'int':
            string = "Definition {} INT[{}]".format(name, length)

    assert op.to_hqs_lang() == string
    assert(op.involved_qubits == set())

    op2 = _serialisation_convertion(op)
    assert op2 == op


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
