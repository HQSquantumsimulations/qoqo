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
"""Abstract Base Class for all Operations used in qoqo"""
from copy import copy
from typing import (
    Optional,
    Dict,
    List,
    Union,
    cast,
    Set,
    Callable,
    Tuple,
    Any
)  # noqa: D100

import numpy as np

from hqsbase.calculator import (
    CalculatorFloat,
    IntoCalculatorFloat,
    parse_string,
)
from hqsbase.qonfig import Qonfig


class OperationNotInBackendError(Exception):
    """Exception raised when an operation is missing in the backend.

    Exception raised when a 'qoqo_xyz' interface is called for an operation
    where backend xyz does not support the operation.

    """

    def __init__(self,
                 message: str = '',
                 operation: Optional[str] = None,
                 backend: Optional[str] = None) -> None:
        """Initialize the OperationNotInBackendError exception

        Args:
            message: explanation of the error
            operation: input expression in which the error occurred
            backend: the backend xyz
        """
        self.operation = operation
        self.backend = backend
        self.message = message

    def __str__(self) -> str:
        """Represents the OperationNotInBackendError Exception as a string.

        Returns:
            str
        """
        return self.message + "operation: " + str(self.operation)


class BackendNotAvailableError(Exception):
    """Exception raised when backend xyz is not available"""

    def __init__(self,
                 message: str = '',
                 backend: Optional[str] = None
                 ) -> None:
        """Initialize the BackendNotAvailableError Exception

        Args:
            message: explanation of the error
            backend: the backend xyz
        """
        self.backend = backend
        self.message = message


class Operation():
    r"""Abstract Base Class for all operations used by qoqo.

    This class should not be instantiated itself.

    Operations are at the center of the qoqo paradigm.
    Operations are the atomic instructions in any quantum program
    that can be represented by qoqo.

    Operations include standard qubit gates such as PauliX, CNOT, etc,
    as well as qubit measurements and PRAGMA (or PRAGMA) operations
    that are not necessary for a valid quantum program.
    In this regard, qoqo follows the concept of Rigetti's pyquil.

    The PRAGMA operations include:
    1. Error operations: introduce decoherence when a circuit is simulated
    2. Measurement pragmas: extract information from simulation backends
       that is normally not accessible (like the full statevector)
    3. Circuit modification PRAGMAs: instructions that modify the circuit itself,
       e.g. PRAGMAs that introduce overrotations into the gates
    4. Backend instructions: instructions for specific backends,
       e.g. setting the number of measurement repetitions

    The operations can be translated to different quantum computing frameworks
    or representations with interfaces specific to the framework.

    Qoqo operations can be serialised to json or yaml using the HQS qonfig package.

    Operations can be parametrized, where one or more of its internal parameters can be
    string expressions instead of standard Python types.
    These parameters can be substituted with a substitute_parameters method or by
    adding a PragmaParameterSubstitution in the circuit.

    For all Operation objects, the following properties are defined:
        - to_/from_qonfig: Operations can be serialised with the HQS Qonfig package
        - get_hqs_lang_name: called by the to_hqs_lang function, and should not be used by itself.
        - is_parametrized: used in Operations with varying parameters, defined with symbolic
                           paramters. Goes hand in hand with the "substitute_parameters" function.
        - involved_qubits: returns the qubits involved in the operation. This is mainly used by
                           the interfaces to translate operations to xyz.

    Additionally, the following magic methods are defined:
        - str, repr
        - and, rand
        - deepcopy

    Lastly, the following functions can be called:
        - substitute_parameters: substitutes symbolic parameters in the Operation
        - remap qubits: remaps the qubits in the operation according to a dictionary
        - to_hqs_lang: translates the operation to hqs_lang dialect

    """

    _operation_tags: Tuple[str, ...] = ('Operation',)

    _ordered_qubits_dict: Dict[str, int]
    _ordered_parameter_dict: Dict[str, CalculatorFloat]
    _ordered_qubits_dict = dict()
    _ordered_parameter_dict = dict()
    _rotation_strength_parameters: Optional[List[str]] = None
    _rotation_strength_modulo: Optional[Dict[str, float]] = dict()
    _hqs_lang_name = 'Operation'
    # Defining class methods

    @classmethod
    def get_hqs_lang_name(cls) -> str:
        """Return the name of the gate

        Returns:
            str

        Raises:
            OperationNotInBackendError: Operation can not be translated
        """
        if cls._hqs_lang_name is None:
            raise OperationNotInBackendError()
        return cls._hqs_lang_name

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['Operation']
                    ) -> 'Operation':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Raises:
            NotImplementedError: Not yet implemented
        """
        raise NotImplementedError()

    def to_qonfig(self) -> 'Qonfig[Operation]':
        """Create a Qonfig from Instance

        Raises:
            NotImplementedError: Not yet implemented
        """
        raise NotImplementedError()

    # Defining the methods the Operations need to provide

    def __init__(self) -> None:
        """Initialize the Operation class"""
        self._involved_qubits: Set[Union[int, str]] = set()
        self._parametrized = False

    @property
    def is_parametrized(self) -> bool:
        """Return True if the operation has symbolic parameters

        True if any of the parameters of the gate is a symbol that can be replaced
        with the substitute_parameters function

        Returns:
            bool
        """
        return self._parametrized

    def substitute_parameters(
            self,
            substitution_dict: Dict[str, CalculatorFloat]) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to a provided substitution_dict

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                               Where 'name' is the name of the symbol to be substituted
                               and new_value is the substituted value (must be CalculatorFloat)
        """
        pass

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        pass

    @property
    def involved_qubits(self) -> Set[Union[int, str]]:
        """Return the qubits involved in the operation

        Returns:
            Set[Union[int, str]]
        """
        return self._involved_qubits

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        return ''

    def __repr__(self) -> str:
        """Return the unique representation of the operation

        Returns:
            str
        """
        return self.to_hqs_lang()

    def __str__(self) -> str:
        """Return the string representation of the operation

        Returns:
            str
        """
        return self.to_hqs_lang()

    def __and__(self, other: 'Operation') -> bool:
        r"""Return True when two gates act on same qubits

        We overload & to implement a comparison operation between
        two gates, yielding True when both gates are acting
        on the same qubits but with potentially different parameters

        Args:
            other: Operation self is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        return True

    def __rand__(self, other: 'Operation') -> bool:
        r"""Return True when two gates act on same qubits

        Args:
            other: Operation self is compared to

        Returns:
            bool
        """
        return self.__and__(other)

    def __deepcopy__(self,
                     memodict: Optional[dict] = None
                     ) -> 'Operation':
        """Return a deep copy of the operation

        Args:
            memodict: Required keyword argument for deepcopy method

        Returns:
            Operation
        """
        if memodict is None:
            memodict = dict()
        return copy(self)


class Pragma(Operation):
    r"""Abstract Base Class for all 'PRAGMA' operations.

    Base class for all 'PRAGMA' operations on quantum computers used by qoqo.
    PRAGMA operations are defined as additional directives
    for defining things like the application of noise in simulations or the parameter substitution
    for a quantum device.

    It implements two abstract methods:
        is_backend_instruction: this boolean returns True if the PRAGMA is a backend instruction
        backend_instruction: returns a backend instruction,
                             for example setting the number of measurements

    """

    _operation_tags: Tuple[str, ...] = ('Operation', 'Pragma')

    def __init__(self) -> None:
        """Initialize the PRAGMA class"""
        self._involved_qubits: Set[Union[str, int]] = set()

    def is_backend_instruction(self, backend: str = None, **kwargs) -> bool:
        """Determine if the PRAGMA operation is a backend instruction for a given backend

        Args:
            backend: Given backend
            kwargs: Additional keyword arguments

        Returns:
            bool
        """
        return_bool: bool = True if self.backend_instruction(backend=backend) is not None else False
        return return_bool

    def backend_instruction(self,
                            backend: Optional[str] = None,
                            **kwargs
                            ) -> Optional[Dict[str, Any]]:
        """Return the instruction to the quantum computing backend

        Backend instructions are returned as dicts
        that override keyword args in the __init__ of the backend

        Args:
            backend: Given backend
            kwargs: Additional keyword arguments

        Returns:
            override (Optional[Dict[str, Any]]): Override of
                                                 arguments in backend __init__
        """
        return None


class GateOperation(Operation):
    """Abstract Base Class for all gate operations.

    Base class for all gate and unitary operations used by qoqo.
    Gate and unitary operations are defined as all operations applying an arbitrary
    unitary operator to a number of qubits.

    Operations in this class need to implement the static method unitary_matrix_from_parameters,
    the class method number_of_qubits and the property unitary_matrix.

    Args:
        kwargs: The necessary arguments of the initialization are provided by as keyword
            arguments; init then automatically compares with the default qubit_dict and
            parameter_dict of the gate and updates the values that where provided at
            initialization. If the keyword arguments have string type, they are
            considered symbolic and is_parametrized is true.

    """

    _operation_tags: Tuple[str, ...] = ('Operation', 'GateOperation')

    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_qubits_dict: Dict[str, int]
    _ordered_parameter_dict: Dict[str, CalculatorFloat]

    _ordered_qubits_dict_default = dict()
    _ordered_parameter_dict_default = dict()
    _ordered_qubits_dict = dict()
    _ordered_parameter_dict = dict()
    _is_self_inverse = False

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': dict()},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    def __init__(self,
                 for_copy: bool = False,
                 **kwargs: Union[IntoCalculatorFloat, int]
                 ) -> None:
        """Initialize a gate or unitary operation

        Args:
            for_copy: Initialize for creating copy of gate operation
            kwargs: The necessary arguments of the initialization are provided by as keyword
                arguments; init then automatically compares with the default qubit_dict and
                parameter_dict of the gate and updates the values that where provided at
                initialization. If the keyword arguments have string type, they are
                considered symbolic and is_parametrized is true.
        """
        self._ordered_qubits_dict = dict()
        self._ordered_parameter_dict: Dict[str, CalculatorFloat]
        self._ordered_parameter_dict = dict()
        if not for_copy:
            self._parametrized = False
            for key in self._ordered_parameter_dict_default.keys():
                self._ordered_parameter_dict[key] = CalculatorFloat(
                    kwargs.get(key, self._ordered_parameter_dict_default[key]))
                if not self._ordered_parameter_dict[key].is_float:
                    self._parametrized = True
            for key in self._ordered_qubits_dict_default.keys():
                self._ordered_qubits_dict[key] = cast(
                    int,
                    kwargs.get(key, self._ordered_qubits_dict_default[key]))
            self._involved_qubits = cast(Set[Union[int, str]],
                                         set(self._ordered_qubits_dict.values()))

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['GateOperation']
                    ) -> 'GateOperation':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            GateOperation
        """
        kwargs = dict()
        for key, val in config['ordered_qubits_dict'].items():
            kwargs[key] = val
        for key, val in config['ordered_parameter_dict'].items():
            kwargs[key] = val
        return cls(**kwargs)

    def to_qonfig(self) -> 'Qonfig[GateOperation]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[GateOperation]
        """
        config = Qonfig(self.__class__)
        ordered_parameter_dict: Dict[str, Union[str, float]] = dict()
        # Converting from CalculatorFloat to Union[str, float]
        for key, val in self._ordered_parameter_dict.items():
            ordered_parameter_dict[key] = val.value
        config['ordered_qubits_dict'] = self._ordered_qubits_dict
        config['ordered_parameter_dict'] = ordered_parameter_dict

        return config

    def __pow__(self, other: IntoCalculatorFloat) -> 'GateOperation':
        """Return the gate to the other-th power

        Implements the power magic method for gates where the power operation can be
        applied easily (i.e. gates that have a rotation_strength_parameter)

        Args:
            other: exponent

        Returns:
            GateOperation

        Raises:
            AttributeError: Gate can not be exponentiated
        """
        return_operation = copy(self)
        if self._rotation_strength_parameters is None:
            raise AttributeError("Gate {} can not be exponentiated".format(type(self).__name__))
        for parameter in self._rotation_strength_parameters:
            return_operation._ordered_parameter_dict[parameter] = (
                (self._ordered_parameter_dict[parameter] * other)
            )
        if any([not p.is_float for p in return_operation._ordered_parameter_dict.values()]):
            return_operation._parametrized = True
        return return_operation

    def __and__(self, other: object) -> bool:
        r"""Return True when two gates act on same qubits

        We overload & to implement a comparison operation between
        two gates, yielding True when both gates are acting
        on the same qubits but with potentially different parameters

        Args:
            other: The object self is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        other = cast('GateOperation', other)
        for key, val in self._ordered_qubits_dict.items():
            if not (val
                    == other._ordered_qubits_dict[key]):
                return False
        return True

    def __rand__(self, other: object) -> bool:
        r"""Return True when two gates act on same qubits

        Args:
            other: The object self is compared to

        Returns:
            bool
        """
        return self.__and__(other)

    def __eq__(self, other: object) -> bool:
        """Compare the operation with the other Python object and returns True when equal

        Args:
            other: Object operation is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        for key in self._ordered_parameter_dict:
            if not (self._ordered_parameter_dict[key]
                    == other._ordered_parameter_dict[key]):
                return False
        for key in self._ordered_qubits_dict:
            if not (self._ordered_qubits_dict[key]
                    == other._ordered_qubits_dict[key]):
                return False
        return True

    def __copy__(self) -> 'GateOperation':
        """Return a shallow copy of the operation

        Creates a copy by copying the contents of the internal dicts

        Returns:
            GateOperation
        """
        self_copy = self.__class__(for_copy=True)
        self_copy._ordered_qubits_dict = copy(self._ordered_qubits_dict)
        self_copy._ordered_parameter_dict = copy(self._ordered_parameter_dict)
        self_copy._parametrized = copy(self._parametrized)
        self_copy._involved_qubits = copy(self._involved_qubits)
        return self_copy

    @classmethod
    def number_of_qubits(cls) -> int:
        """Return the number of qubits involved in the operation

        Returns:
            int
        """
        return len(cls._ordered_qubits_dict_default)

    def substitute_parameters(self,
                              substitution_dict: Dict[str, CalculatorFloat],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to a provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                               Where 'name' is the name of the symbol to be substituted
                               and new_value is the substituted value
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)
            for key in self._ordered_parameter_dict.keys():
                parameter = CalculatorFloat(self._ordered_parameter_dict[key])
                if not parameter.is_float:
                    new_parameter = parse_string(substitution_string + '; ' + parameter.value)
                    self._ordered_parameter_dict[key] = CalculatorFloat(new_parameter)
            self._parametrized = False

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        for key, val in self._ordered_qubits_dict.items():
            self._ordered_qubits_dict[key] = mapping_dict[val]
        self._involved_qubits = cast(
            Set[Union[int, str]],
            set(self._ordered_qubits_dict.values()))

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        if len(self._ordered_parameter_dict.keys()) > 0:
            keylist = list(self._ordered_parameter_dict.keys())
            string += '('
            key = keylist[0]
            string += '{}'.format(self._ordered_parameter_dict[key])
            for key in keylist[1:]:
                string += ', {}'.format(self._ordered_parameter_dict[key])
            string += ')'
        for key in self._ordered_qubits_dict.keys():
            string += ' {}'.format(self._ordered_qubits_dict[key])
        return string

    @property
    def unitary_matrix(self) -> np.ndarray:
        """Return the unitary matrix corresponding to the instance of the class

        Returns:
            np.ndarray

        Raises:
            ValueError: Parametrized gate can not be returned as unitary matrix
            AttributeError: Operation has no unitary matrix method
        """
        if self.is_parametrized:
            raise ValueError('Parametrized gate can not be returned as unitary matrix')
        parameters = {key: val.value for key, val in self._ordered_parameter_dict.items()}
        matrix_method = getattr(self, 'unitary_matrix_from_parameters', None)
        if matrix_method is None:
            raise AttributeError("Operation has no unitary matrix method")
        matrix_method = cast(Callable, matrix_method)
        return matrix_method(**parameters)
