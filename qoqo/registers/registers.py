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
"""Classical registers for qoqo"""
from qoqo.operations import (
    Definition
)
from typing import Union, List, Dict
import numpy as np


class BitRegister(object):
    """Bit register in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize Register

        Args:
            definition: Define operation creating register

        Raises:
            TypeError: BitRegister can only be initialized by bit Definition
        """
        if definition._vartype != 'bit':
            raise TypeError('BitRegister can only be initialized by Bit Definition')
        self.vartype: str = 'bit'
        self.name: str = definition._name
        self.length: int = definition._length
        self.is_output: bool = definition._is_output
        self.register: List[bool] = [False for i in range(self.length)]

    def reset(self) -> None:
        """Reset internal register"""
        self.register = [False for i in range(self.length)]


class FloatRegister(object):
    """Float register in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize Register

        Args:
            definition: Define operation creating register

        Raises:
            TypeError: FloatRegister can only be initialized by float Definition
        """
        if definition._vartype != 'float':
            raise TypeError('FloatRegister can only be initialized by float Definition')
        self.vartype: str = 'float'
        self.name: str = definition._name
        self.length: int = definition._length
        self.is_output: bool = definition._is_output
        self.register: List[float] = [0 for i in range(self.length)]

    def reset(self) -> None:
        """Reset internal register"""
        self.register = [0 for i in range(self.length)]


class ComplexRegister(object):
    """Complex register in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize Register

        Args:
            definition: Define operation creating register

        Raises:
            TypeError: ComplexRegister can only be initialized by complex Definition
        """
        if definition._vartype != 'complex':
            raise TypeError('ComplexRegister can only be initialized by complex Definition')
        self.vartype: str = 'complex'
        self.name: str = definition._name
        self.length: int = definition._length
        self.is_output: bool = definition._is_output
        self.register: np.ndarray = np.zeros((self.length,), dtype=complex)

    def reset(self) -> None:
        """Reset internal register"""
        self.register = np.zeros((self.length,), dtype=complex)


class RegisterOutput(object):
    """Output register in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize OutputRegister

        Args:
            definition: Define operation creating output register
        """
        self.name: str = ''
        self.vartype: str = ''
        self.length: int = 0
        self.register: List = list()
        pass

    def __len__(self) -> int:
        """Return length of OutputRegister

        Returns:
            int
        """
        return self.size

    @property
    def size(self) -> int:
        """Return size of OutputRegister

        Returns:
            int
        """
        return len(self.register)

    def append(self, register: Union[BitRegister, FloatRegister, ComplexRegister]) -> None:
        """Append content of register to output register

        Args:
            register: Register that is appended to the output register

        Raises:
            TypeError: Output register and appended register not compatible.
                       Name, vartype of length different.
        """
        if (self.name != register.name
                or self.vartype != register.vartype
                or self.length != register.length):
            raise TypeError(
                "Output register and appended register not compatible. "
                + "Name, vartype of length different.")
        self.register.append(register.register)

    def extend(self, register: Union[BitRegister, FloatRegister, ComplexRegister]) -> None:
        """Extend output register with content of register

        Args:
            register: Register that is appended to the output register

        Raises:
            TypeError: Output register and extended register not compatible.
                       Name, vartype of length different.
        """
        if (self.name != register.name
                or self.vartype != register.vartype
                or self.length != register.length):
            raise TypeError(
                "Output register and extended register not compatible. "
                + "Name, vartype of length different.")
        self.register.extend(register.register)


class BitRegisterOutput(RegisterOutput):
    """Output BitRegister in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize OutputRegister

        Args:
            definition: Define operation creating output register

        Raises:
            TypeError: BitRegister can only be initialized by bit Definition
        """
        if definition._vartype != 'bit':
            raise TypeError('BitRegister can only be initialized by Bit Definition')
        if not definition._is_output:
            raise TypeError('OutputRegister can only be created to output definition')
        self.vartype: str = 'bit'
        self.name: str = definition._name
        self.length: int = definition._length
        self.register: List[List[bool]] = list()


class FloatRegisterOutput(RegisterOutput):
    """Output FloatRegister in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize OutputRegister

        Args:
            definition: Define operation creating output register

        Raises:
            TypeError: FloatRegister can only be initialized by float Definition
        """
        if definition._vartype != 'float':
            raise TypeError('FloatRegister can only be initialized by float Definition')
        if not definition._is_output:
            raise TypeError('OutputRegister can only be created to output definition')
        self.vartype: str = 'float'
        self.name: str = definition._name
        self.length: int = definition._length
        self.register: List[List[float]] = list()


class ComplexRegisterOutput(RegisterOutput):
    """Output ComplexRegister in the qoqo backends"""

    def __init__(self, definition: Definition) -> None:
        """Initialize OutputRegister

        Args:
            definition: Define operation creating output register

        Raises:
            TypeError: ComplexRegister can only be initialized by complex Definition
        """
        if definition._vartype != 'complex':
            raise TypeError('ComplexRegister can only be initialized by complex Definition')
        if not definition._is_output:
            raise TypeError('OutputRegister can only be created to output definition')
        self.vartype: str = 'complex'
        self.name: str = definition._name
        self.length: int = definition._length
        self.register: List[List[float]] = list()


def add_register(register_dict: Dict[str, Union[BitRegister, FloatRegister, ComplexRegister]],
                 output_dict: Dict[str, RegisterOutput],
                 definition: Definition) -> None:
    """Add register to register dict and output register dict

    Args:
        register_dict: Dict of all internal registers in the calculator
        output_dict: Dict of all output registers
        definition: Define operation defining new register
    """
    if definition._vartype == 'bit':
        register_dict[definition._name] = BitRegister(definition)
        if definition._is_output:
            output_dict[definition._name] = BitRegisterOutput(definition)
    if definition._vartype == 'float':
        register_dict[definition._name] = FloatRegister(definition)
        if definition._is_output:
            output_dict[definition._name] = FloatRegisterOutput(definition)
    if definition._vartype == 'complex':
        register_dict[definition._name] = ComplexRegister(definition)
        if definition._is_output:
            output_dict[definition._name] = ComplexRegisterOutput(definition)
