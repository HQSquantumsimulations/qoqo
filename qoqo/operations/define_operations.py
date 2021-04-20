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
"""Definition class for qoqo"""
from qoqo.operations._operations_base_classes import (
    Operation,
)
from typing import (
    Set,
    Union
)
from hqsbase.qonfig import Qonfig


class Definition(Operation):
    """Definition Operation that defines the classical registers and variables in the Circuit.

    In general, every circuit will have at least one Definition operation. The main
    reasons one would need a Definition operation in a circuit is for measurements.
    If you need to measure something in a circuit using a MeasureQubit operation
    (see qoqo.operations.measurememnt_operations.py), you need to define the classical
    register used to store the measurement result.

    You need to add the Define operation to the circuit before what you are defining is used.
    Therefore, qoqo uses the convention that Definition operations are added first to the circuit,
    before you add any other operations.

    """

    _operation_tags = ('Operation', 'Definition')

    _hqs_lang_name = 'Definition'

    _quest_typedict = {
        'float': 'float',
        'bit': 'int',
        'int': 'int',
        'complex': 'Complex'
    }

    _quil_typedict = {
        'float': 'REAL',
        'bit': 'BIT',
        'int': 'INT',
        'complex': 'COMPLEX'
    }

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'register_name': {'doc': 'The name of the register that is defined',
                          'default': 'ro'},
        'vartype': {'doc': 'The type of the register that is defined',
                    'default': 'float'},
        'length': {'doc': 'The length of the register that is defined',
                   'default': 1},
        'is_input': {'doc': 'Is variable an input to the program',
                     'default': False},
        'is_output': {'doc': 'Is variable an output to the program',
                      'default': False},
    }

    # Defining the methods the Operations need to provide

    def __init__(self,
                 name: str,
                 vartype: str = 'float',
                 length: int = 1,
                 is_input: bool = False,
                 is_output: bool = False) -> None:
        """Initialize the Definition operation

        Args:
            name: The name of the register that is defined
            vartype: The type of the register that is defined
            length: The length of the register that is defined, usually the number of qubits to
                    be measured
            is_input: T/F Is the variable an input to the program
            is_output: T/F Is the variable an output to the program
        """
        self._involved_qubits: Set[Union[int, str]]
        self._name = name
        self._vartype = vartype
        self._length = length
        self._is_input = is_input
        self._is_output = is_output
        self._involved_qubits = set()
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['Definition']
                    ) -> 'Definition':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            Definition
        """
        return cls(name=config['register_name'],
                   vartype=config['vartype'],
                   length=config['length'],
                   is_input=config['is_input'],
                   is_output=config['is_output'])

    def to_qonfig(self) -> 'Qonfig[Definition]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[Definition]
        """
        config = Qonfig(self.__class__)
        config['register_name'] = self._name
        config['vartype'] = self._vartype
        config['length'] = self._length
        config['is_input'] = self._is_input
        config['is_output'] = self._is_output

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the operation with the other Python object and returns True when equal

        Args:
            other: object Definition is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._name == other._name:
            return False
        if not self._length == other._length:
            return False
        if not self._vartype == other._vartype:
            return False
        return True

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        if self._is_input or self._is_output:
            return "Definition({input},{output}) {name} {vartype}[{length}]".format(
                input=self._is_input,
                output=self._is_output,
                name=self._name,
                vartype=self._quil_typedict[self._vartype],
                length=self._length)
        else:
            return "Definition {name} {vartype}[{length}]".format(
                name=self._name,
                vartype=self._quil_typedict[self._vartype],
                length=self._length)
