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
"""Implementing the PRAGMA operation classes"""
from qoqo.operations._operations_base_classes import (
    Pragma,
)
import numpy as np
from copy import copy
from typing import (
    Union,
    List,
    Any,
    Dict,
    Set,
    Optional,
    cast,
    Tuple
)
from hqsbase.calculator import (
    parse_string,
    CalculatorFloat,
    IntoCalculatorFloat
)
from hqsbase.qonfig import Qonfig


class PragmaSetNumberOfMeasurements(Pragma):
    """Implements SetNumberOfMeasurements PRAGMA.

    This PRAGMA operation sets the number of measurements of the circuit. This is used
    for backends that allow setting the number of tries. However, setting the number of
    measurements  does not allow access to the underlying wavefunction or density matrix.
    For instance, this would be useful for backends where real quantum hardware is used,
    as well as in backends such as pyquil and cirq.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaSetNumberOfMeasurements')
    _hqs_lang_name = 'PragmaSetNumberOfMeasurements'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'number_measurements': {'doc': 'The number of measurements',
                                'default': 1},
        'readout': {'doc': 'The register for the readout',
                    'default': 'ro'},
    }

    def __init__(
        self,
        number_measurements: int = 1,
        readout: str = 'ro'
    ) -> None:
        """Initialize 'set number measurements' PRAGMA

        Args:
            number_measurements: The number of measurements
            readout: Readout register of the measurement gates. PragmaSetNumberOfMeasurements
                     is only applied to the measurement gates that have this readout register.
        """
        self._number_measurements = number_measurements
        self._involved_qubits: Set[Union[int, str]] = set()
        self._parametrized = False
        self._readout = readout

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaSetNumberOfMeasurements']
                    ) -> 'PragmaSetNumberOfMeasurements':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaSetNumberOfMeasurements
        """
        return cls(number_measurements=config['number_measurements'],
                   readout=config['readout'])

    def to_qonfig(self) -> 'Qonfig[PragmaSetNumberOfMeasurements]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaSetNumberOfMeasurements]
        """
        config = Qonfig(self.__class__)
        config['number_measurements'] = self._number_measurements
        config['readout'] = self._readout

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._readout == other._readout:
            return False
        if not self._number_measurements == other._number_measurements:
            return False
        return True

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
            override (Optional[Dict[str, Any]]): Override of arguments in backend __init__
        """
        backend_instruction = None
        if backend is None:
            backend_instruction = None
        elif backend.startswith('pyquest_cffi'):
            backend_instruction = {'number_measurements': self._number_measurements}
        elif backend.startswith('braket'):
            backend_instruction = {'number_measurements': self._number_measurements}
        elif backend == 'pyquil':
            backend_instruction = {'number_measurements': self._number_measurements}
        elif backend == 'aqt':
            backend_instruction = {'number_measurements': self._number_measurements}
        elif backend == 'cirq':
            backend_instruction = {'number_measurements': self._number_measurements}
        elif backend == 'cirq_code':
            backend_instruction = {'number_measurements': self._number_measurements}
        return backend_instruction

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += '({})'.format(self._number_measurements)
        string += ' {}'.format(self._readout)
        return string


class PragmaSetStateVector(Pragma):
    """Implements the SetStateVector PRAGMA.

    This PRAGMA operation sets the statevector of a quantum register.
    The Circuit() module autmatically initializes the qubits in the |0> state, so this PRAGMA
    operation allows you to set the state of the qubits to a state of your choosing.
    For instance, to initialise the psi-minus Bell state, we pass the following vector to
    the PRAGMA:
        vector = np.array([0, 1 / np.sqrt(2), -1 / np.sqrt(2), 0])

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaSetStateVector')
    _hqs_lang_name = 'PragmaSetStateVector'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'statevec': {'doc': 'The statevector that is initialized',
                     'default': None},
    }

    def __init__(self,
                 statevec: np.ndarray,
                 ) -> None:
        """Initialize 'set state vector' PRAGMA

        Args:
            statevec: The statevector that is initialized
        """
        self._statevec = statevec
        self._involved_qubits: Set[Union[str, int]] = set(['ALL'])
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaSetStateVector']
                    ) -> 'PragmaSetStateVector':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaSetStateVector
        """
        return cls(statevec=config['statevec'])

    def to_qonfig(self) -> 'Qonfig[PragmaSetStateVector]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaSetStateVector]
        """
        config = Qonfig(self.__class__)
        config['statevec'] = self._statevec

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._statevec == other._statevec:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices

        Raises:
            NotImplementedError: Not implemented
        """
        raise NotImplementedError('Not implemented')

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        for _, amp in enumerate(self._statevec):
            string += ' {}'.format(amp)
        return string


class PragmaSetDensityMatrix(Pragma):
    """Implements the SetDensityMatrix PRAGMA.

    This PRAGMA operation sets the density matrix of a quantum register.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaSetDensityMatrix')
    _hqs_lang_name = 'PragmaSetDensityMatrix'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'density_matrix': {'doc': 'The density matrix that is initialized',
                           'default': None},
    }

    def __init__(self,
                 density_matrix: np.ndarray,
                 ) -> None:
        """Initialize 'set density matrix' PRAGMA

        Args:
            density_matrix: The density matrix that is initialized
        """
        self._density_matrix = density_matrix
        self._involved_qubits: Set[Union[str, int]] = set(['ALL'])
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaSetDensityMatrix']
                    ) -> 'PragmaSetDensityMatrix':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaSetDensityMatrix
        """
        return cls(density_matrix=config['density_matrix'])

    def to_qonfig(self) -> 'Qonfig[PragmaSetDensityMatrix]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaSetDensityMatrix]
        """
        config = Qonfig(self.__class__)
        config['density_matrix'] = self._density_matrix

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._density_matrix == other._density_matrix:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices

        Raises:
            NotImplementedError: Not implemented
        """
        raise NotImplementedError('Not implemented')

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        for _, amp in enumerate(self._density_matrix):
            string += ' {}'.format(amp)
        return string


class PragmaNoise(Pragma):
    """PRAGMA base class for all Noise gates.

    The following Noise gates are noise PRAGMAs:
        - PragmaDamping
        - PragmaDepolarise
        - PragmaDephasing
        - PragmaRandomNoise

    The necessary arguments of the initialization are provided by as keyword
    arguments; init then automatically compares with the default qubit_dict and
    parameter_dict of the gate and updates the values that where provided at
    initialization. If the keyword arguments have string type, they are
    considered symbolic and is_parametrized is true.

    """

    _operation_tags: Tuple[str, ...] = ('Operation', 'Pragma', 'PragmaNoise')

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': dict()},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_qubits_dict: Dict[str, int]
    _ordered_parameter_dict: Dict[str, CalculatorFloat]

    _ordered_qubits_dict_default = dict()
    _ordered_parameter_dict_default = dict()
    _ordered_qubits_dict = dict()
    _ordered_parameter_dict = dict()
    _is_self_inverse = False

    def __init__(self,
                 for_copy: bool = False,
                 **kwargs: Union[IntoCalculatorFloat, int]
                 ) -> None:
        """Initialize a Noise PRAGMA

        Args:
            for_copy: Initialize for creating a copy of the gate operation
            kwargs: The necessary arguments of the initialization
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
                    config: Qonfig['PragmaNoise']
                    ) -> 'PragmaNoise':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaNoise
        """
        kwargs = dict()
        for key, val in config['ordered_qubits_dict'].items():
            kwargs[key] = val
        for key, val in config['ordered_parameter_dict'].items():
            kwargs[key] = val

        return cls(**kwargs)

    def to_qonfig(self) -> 'Qonfig[PragmaNoise]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaNoise]
        """
        config = Qonfig(self.__class__)
        ordered_parameter_dict: Dict[str, Union[str, float]] = dict()
        # Converting from CalculatorFloat to Union[str, float]
        for key, val in self._ordered_parameter_dict.items():
            ordered_parameter_dict[key] = val.value
        config['ordered_qubits_dict'] = self._ordered_qubits_dict
        config['ordered_parameter_dict'] = ordered_parameter_dict

        return config

    def __pow__(self, other: IntoCalculatorFloat) -> 'PragmaNoise':
        """Return the gate to the other-th power

        Implements the power magic method for gates where the power operation can be applied easily
        (gates that have a rotation_strength_parameter).

        Args:
            other: exponent

        Returns:
            PragmaNoise

        Raises:
            AttributeError: Gate cannot be exponentiated
        """
        return_operation = copy(self)
        if self._rotation_strength_parameters is None:
            raise AttributeError("Gate {} cannot be exponentiated".format(type(self).__name__))
        for parameter in self._rotation_strength_parameters:
            return_operation._ordered_parameter_dict[parameter] = (
                (self._ordered_parameter_dict[parameter] * other)
            )
        if any([not p.is_float for p in return_operation._ordered_parameter_dict.values()]):
            return_operation._parametrized = True
        return return_operation

    def __and__(self, other: object) -> bool:
        r"""Return True when two gates act on same qubits

        We overload "&" to implement a comparison operation between two gates, yielding
        True when both gates are acting on the same qubits (but with potentially
        different parameters).

        Args:
            other: The object self is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        other = cast('PragmaNoise', other)
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
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

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

    def __copy__(self) -> 'PragmaNoise':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaNoise
        """
        self_copy = self.__class__(for_copy=True)
        self_copy._ordered_qubits_dict = copy(self._ordered_qubits_dict)
        self_copy._ordered_parameter_dict = copy(self._ordered_parameter_dict)
        self_copy._parametrized = copy(self._parametrized)
        self_copy._involved_qubits = copy(self._involved_qubits)
        return self_copy

    @classmethod
    def number_of_qubits(cls) -> int:
        """Return the number of qubits involved in the PRAGMA

        Returns:
            int
        """
        return len(cls._ordered_qubits_dict_default)

    def substitute_parameters(self,
                              substitution_dict: Dict[str, CalculatorFloat],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
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

    @staticmethod
    def superoperator_from_parameters(gate_time: float, rate: float) -> np.ndarray:
        """Return the superoperator representation of the noise gate

        Args:
            gate_time: The gate time
            rate: The damping rate

        Returns:
            np.ndarray
        """
        return np.array([])

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Raise an error, as Noise cannot be represented by a unitary matrix

        Args:
            kwargs: Keyword arguments

        Returns:
            np.ndarray

        Raises:
            TypeError: Noise cannot be represented by a unitary matrix
        """
        raise TypeError('Noise cannot be represented by a unitary matrix')
        return np.array([])

    @property
    def probability(self) -> CalculatorFloat:
        """Return the probability of the noise channel affecting the qubit

        Returns:
            CalculatorFloat
        """
        prob = 0.5 * (1 - (-2 * self._ordered_parameter_dict['gate_time']
                           * self._ordered_parameter_dict['rate']).exp())
        return prob

    @property
    def superoperator(self) -> np.ndarray:
        """Return the superoperator representation of the noise channel with operation parameters

        Returns:
            np.ndarray

        Raises:
            ValueError: Parametrized noise PRAGMA cannot be returned as superoperator
        """
        if self.is_parametrized:
            raise ValueError("Parametrized noise PRAGMA cannot be returned as superoperator")
        parameter_dict: Dict[str, float] = dict()
        for key, val in self._ordered_parameter_dict.items():
            parameter_dict[key] = float(val)
        return self.superoperator_from_parameters(**parameter_dict)

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
            override (Optional[Dict[str, Any]]): Override of arguments in backend __init__
        """
        if backend is None:
            return None
        elif backend.startswith('pyquest_cffi'):
            return {'use_density_matrix': True}
        else:
            return None
        return None


class PragmaDamping(PragmaNoise):
    r"""Implements the Damping PRAGMA.

    This PRAGMA operation applies a pure damping error corresponding to zero
    temperature environments.

    .. math::
        \rho &= \mathcal{K} \rho \mathcal{K}\\

    """

    _operation_tags = ('Operation', 'Pragma', 'GateOperation', 'PragmaNoise', 'PragmaDamping')
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['gate_time'] = 0
    _ordered_parameter_dict_default['rate'] = 0
    _rotation_strength_parameters = ['gate_time']

    _hqs_lang_name = 'PragmaDamping'
    _rotation_strength_parameters = ['gate_time']

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'gate_time': CalculatorFloat(0),
                                               'rate': CalculatorFloat(0)}},
    }

    @staticmethod
    def superoperator_from_parameters(gate_time: float, rate: float,) -> np.ndarray:
        """Return the superoperator representation of the noise gate

        Args:
            gate_time: The gate time
            rate: The damping rate

        Returns:
            np.ndarray
        """
        prob = (
            1 - np.exp(-float(gate_time)
                       * float(rate)))
        sqmp = np.sqrt(1 - prob)
        matrix = np.array([[1, 0, 0, prob],
                           [0, sqmp, 0, 0],
                           [0, 0, sqmp, 0],
                           [0, 0, 0, 1 - prob]], dtype=complex)
        return matrix

    @property
    def probability(self) -> CalculatorFloat:
        """Return the probability of the noise channel affecting the qubit

        Returns:
            CalculatorFloat
        """
        prob = (1 - (-self._ordered_parameter_dict['gate_time']
                     * self._ordered_parameter_dict['rate']).exp())
        return prob


class PragmaDepolarise(PragmaNoise):
    r"""Implements the Depolarise PRAGMA.

    This PRAGMA operation applies a depolarisation error corresponding to infinite
    temperature environments.

    .. math::
        \rho &= \mathcal{K} \rho \mathcal{K}\\

    """

    _operation_tags = ('Operation', 'Pragma', 'GateOperation', 'PragmaNoise', 'PragmaDepolarise')
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['gate_time'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['rate'] = CalculatorFloat(0)

    _hqs_lang_name = 'PragmaDepolarise'
    _rotation_strength_parameters = ['gate_time']

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'gate_time': CalculatorFloat(0),
                                               'rate': CalculatorFloat(0)}},
    }

    @staticmethod
    def superoperator_from_parameters(gate_time: float, rate: float,) -> np.ndarray:
        """Return the superoperator representation of the noise gate

        Args:
            gate_time: The gate time
            rate: The damping rate

        Returns:
            np.ndarray
        """
        probability = 3 / 4 * (
            1 - np.exp(-float(gate_time)
                       * float(rate)))
        one_plus = 1 - 2 / 3 * probability
        one_minus = 1 - 4 / 3 * probability
        two_three = 2 / 3 * probability
        matrix = np.array([[one_plus, 0, 0, two_three],
                           [0, one_minus, 0, 0],
                           [0, 0, one_minus, 0],
                           [two_three, 0, 0, one_plus]], dtype=complex)
        return matrix

    @property
    def probability(self) -> CalculatorFloat:
        """Return the probability of the noise channel affecting the qubit

        Returns:
            CalculatorFloat
        """
        gate_time = (self._ordered_parameter_dict['gate_time'])
        rate = (self._ordered_parameter_dict['rate'])
        prob = 3 / 4 * (
            1 - (-gate_time * rate).exp())
        return prob


class PragmaDephasing(PragmaNoise):
    r"""Implements the Dephasing PRAGMA.

    This PRAGMA operation applies a pure dephasing error.

    .. math::
        \rho &= \mathcal{K} \rho \mathcal{K}\\

    """

    _operation_tags = ('Operation', 'Pragma', 'GateOperation', 'PragmaNoise', 'PragmaDephasing')
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['gate_time'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['rate'] = CalculatorFloat(0)

    _hqs_lang_name = 'PragmaDephasing'
    _rotation_strength_parameters = ['gate_time']

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'gate_time': CalculatorFloat(0),
                                               'rate': CalculatorFloat(0)}},
    }

    @staticmethod
    def superoperator_from_parameters(gate_time: float, rate: float) -> np.ndarray:
        """Return the superoperator representation of the noise gate

        Args:
            gate_time: The gate time
            rate: The damping rate

        Returns:
            np.ndarray
        """
        probability = 1 / 2 * (
            1 - np.exp(-2 * float(gate_time)
                       * float(rate)))
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1 - 2 * probability, 0, 0],
                           [0, 0, 1 - 2 * probability, 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def probability(self) -> CalculatorFloat:
        """Return the probability of the noise channel affecting the qubit

        Returns:
            CalculatorFloat

        """
        prob = 1 / 2 * (1 - (-2 * self._ordered_parameter_dict['gate_time']
                             * self._ordered_parameter_dict['rate']).exp())
        return prob


class PragmaRandomNoise(PragmaNoise):
    r"""Implements the RandomNoise PRAGMA.

    This PRAGMA operation applies a stochastically unravelled combination
    of dephasing and depolarisation.

    .. math::
        \rho &= \mathcal{K} \rho \mathcal{K}\\

    """

    _operation_tags = ('Operation', 'Pragma', 'GateOperation', 'PragmaNoise', 'PragmaRandomNoise')
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['gate_time'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['depolarisation_rate'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['dephasing_rate'] = CalculatorFloat(0)

    _hqs_lang_name = 'PragmaRandomNoise'
    _rotation_strength_parameters = ['gate_time']

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'gate_time': CalculatorFloat(0),
                                               'depolarisation_rate': CalculatorFloat(0),
                                               'dephasing_rate': CalculatorFloat(0)}},
    }

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
            override (Optional[Dict[str, Any]]): Override of arguments in backend __init__
        """
        if backend is None:
            return None
        elif backend.startswith('pyquest_cffi'):
            return {'random_pauli_errors': True}
        else:
            return None
        return None

    def pauli_operation(self, left: int, right: int) -> int:
        """Return the product of two pauli matrices (absolute of Levi-Civita symbol)

        Pauli operators are encoded as:
            0: identity
            1: Pauli X
            2: Pauli Y
            3: Pauli Z

        Args:
            left: left Pauli operator
            right: right Pauli operator

        Returns:
            integer symbolising Pauli operator
        """
        op_matrix = np.array([[0, 1, 2, 3],
                              [1, 0, 3, 2],
                              [2, 3, 0, 1],
                              [3, 2, 1, 0]], dtype=int)
        return op_matrix[left, right]


class PragmaGeneralNoise(Pragma):
    r"""Implements the GeneralNoise PRAGMA.

    This PRAGMA operation applies a stochastically unravelled combination
    of dephasing and depolarisation.

    .. math::
        \rho &= \mathcal{K} \rho \mathcal{K}\\

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaGeneralNoise')
    _hqs_lang_name = 'PragmaGeneralNoise'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubit': {'doc': 'Qubit involved in general noise PRAGMA',
                  'default': 0},
        'gate_time': {'doc': 'The gate time of the GeneralNoise PRAGMA',
                      'default': CalculatorFloat(0)},
        'rate': {'doc': 'The rate of the GeneralNoise PRAGMA',
                 'default': CalculatorFloat(0)},
        'operators': {'doc': 'The operators representing the general noise',
                      'default': np.zeros(shape=(3, 3))},
    }

    def __init__(self,
                 qubit: int = 0,
                 gate_time: IntoCalculatorFloat = 0,
                 rate: IntoCalculatorFloat = 0,
                 operators: np.ndarray = None) -> None:
        """Initialize General Noise PRAGMA

        Args:
            qubit: Qubits involved in general noise PRAGMA
            gate_time: The gate time of the GeneralNoise PRAGMA
            rate: The rate of the GeneralNoise PRAGMA
            operators: The operators representing the general noise
        """
        if operators is None:
            operators = np.zeros((3, 3))
        self._qubit = qubit
        self._gate_time = CalculatorFloat(gate_time)
        self._rate = CalculatorFloat(rate)
        self._operators = operators
        self._parametrized = not (self._gate_time.is_float & self._rate.is_float)
        self._involved_qubits: Set[Union[str, int]] = set([self._qubit])

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGeneralNoise']
                    ) -> 'PragmaGeneralNoise':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGeneralNoise
        """
        return cls(qubit=config['qubit'],
                   gate_time=config['gate_time'],
                   rate=config['rate'],
                   operators=config['operators'])

    def to_qonfig(self) -> 'Qonfig[PragmaGeneralNoise]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGeneralNoise]
        """
        config = Qonfig(self.__class__)
        config['qubit'] = self._qubit
        config['gate_time'] = self._gate_time
        config['rate'] = self._rate
        config['operators'] = self._operators

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._gate_time == other._gate_time:
            return False
        if not self._rate == other._rate:
            return False
        if not np.array_equal(self._operators, other._operators):
            return False
        if not self._qubit == other._qubit:
            return False
        return True

    def substitute_parameters(self,
                              substitution_dict: Dict[str, float],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)

            parameter = self._gate_time
            if parameter.is_float is True:
                parameter = parameter.__float__()
            else:
                parameter = parameter.__str__()
            if isinstance(parameter, str):
                parameter = parse_string(substitution_string + '; ' + parameter)
                self._gate_time = CalculatorFloat(parameter)

            parameter = self._rate
            if parameter.is_float is True:
                parameter = parameter.__float__()
            else:
                parameter = parameter.__str__()
            if isinstance(parameter, str):
                parameter = parse_string(substitution_string + '; ' + parameter)
                self._rate = CalculatorFloat(parameter)

            self._parametrized = False

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubit in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        self._qubit = mapping_dict[self._qubit]
        self._involved_qubits = set([self._qubit])

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += '({}, '.format(self._gate_time)
        string += '{}, '.format(self._rate)
        string += '{})'.format(self._operators)
        string += " {}".format(self._qubit)
        return string


class PragmaRepeatGate(Pragma):
    """Implements the RepeatedGate PRAGMA.

    This PRAGMA operation repeats the following gate to increase
    the error rate for mitigation.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaRepeatGate')
    _hqs_lang_name = 'PragmaRepeatGate'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'repetition_coefficient': {'doc': 'Number of times the following gate is repeated',
                                   'default': 1},
    }

    def __init__(self,
                 repetition_coefficient: IntoCalculatorFloat = 1) -> None:
        """Initialize 'repeat gate' PRAGMA

        Args:
            repetition_coefficient: Number of times the following gate is repeated
        """
        self._coefficient = CalculatorFloat(repetition_coefficient)
        self._parametrized = not (self._coefficient.is_float)
        self._involved_qubits: Set[Union[str, int]] = set(['ALL'])

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaRepeatGate']
                    ) -> 'PragmaRepeatGate':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaRepeatGate
        """
        return cls(repetition_coefficient=config['repetition_coefficient'])

    def to_qonfig(self) -> 'Qonfig[PragmaRepeatGate]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaRepeatGate]
        """
        config = Qonfig(self.__class__)
        config['repetition_coefficient'] = self._coefficient

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._coefficient == other._coefficient:
            return False
        return True

    def substitute_parameters(self,
                              substitution_dict: Dict[str, float],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)
            parameter = self._coefficient.__str__()
            parameter = parse_string(substitution_string + '; ' + parameter)
            self._coefficient = CalculatorFloat(parameter)
            self._parametrized = False

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += '({})'.format(self._coefficient)
        return string


class PragmaBoostNoise(Pragma):
    """Implements the BoostNoise PRAGMA.

    This PRAGMA operation boosts noise and overrotations in the circuit.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaBoostNoise')
    _hqs_lang_name = 'PragmaBoostNoise'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'noise_coefficient': {'doc': 'The coefficient by which the noise is boosted',
                              'default': 1},
    }

    def __init__(self, noise_coefficient: IntoCalculatorFloat = 1) -> None:
        """Initialize 'boost noise levels' PRAGMA

        Args:
            noise_coefficient: The coefficient by which the noise is boosted
        """
        self._coefficient = CalculatorFloat(noise_coefficient)
        self._parametrized = not self._coefficient.is_float
        self._involved_qubits: Set[Union[int, str]] = set()

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaBoostNoise']
                    ) -> 'PragmaBoostNoise':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaBoostNoise
        """
        return cls(noise_coefficient=config['noise_coefficient'])

    def to_qonfig(self) -> 'Qonfig[PragmaBoostNoise]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaBoostNoise]
        """
        config = Qonfig(self.__class__)
        config['noise_coefficient'] = self._coefficient

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._coefficient == other._coefficient:
            return False
        return True

    def substitute_parameters(self,
                              substitution_dict: Dict[str, float],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)
            parameter = self._coefficient.__str__()
            parameter = parse_string(substitution_string + '; ' + parameter)
            self._coefficient = CalculatorFloat(parameter)
            self._parametrized = False

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += '({})'.format(self._coefficient)
        return string


class PragmaOverrotation(Pragma):
    """Implements the Overrotation PRAGMA.

    This PRAGMA operation applies statistic overrotation to one gate of a given type.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaOverrotation')
    _hqs_lang_name = 'PragmaOverrotation'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'gate': {'doc': 'Name of the gate that is overrotated',
                 'default': None},
        'statistic_type': {'doc': 'Type of the overrotation, static or statistic',
                           'default': "static"},
        'ordered_qubits_dict': {'doc': 'Qubits in the gate',
                                'default': None},
        'parameter': {'doc': 'Name of the overrotated parameter',
                      'default': ''},
        'overrotation_parameter': {'doc': 'String form of overrotation parameter',
                                   'default': ''},
        'variance': {'doc': 'Variance of the overrotation',
                     'default': 0},
        'mean': {'doc': 'Mean of the overrotation',
                 'default': 0},
    }

    def __init__(self, gate: Optional[str] = None,
                 statistic_type: str = "static",
                 ordered_qubits_dict: Optional[Dict[str, int]] = None,
                 parameter: str = '',
                 overrotation_parameter: str = '',
                 variance: IntoCalculatorFloat = 0,
                 mean: IntoCalculatorFloat = 0) -> None:
        """Initialize 'statistic overrotation' PRAGMA

        Args:
            gate: Name of the gate that is overrotated
            statistic_type: Type of the overrotation, static or statistic
            ordered_qubits_dict: Qubits in the gate
            parameter: Name of the overrotated parameter
            overrotation_parameter: String form of overrotation parameter
            variance: Variance of the overrotation
            mean: Mean of the overrotation
        """
        self._gate = gate
        self._type = statistic_type
        self._overrotation_parameter = overrotation_parameter
        if ordered_qubits_dict is None:
            ordered_qubits_dict = dict()
        self._ordered_qubits_dict = ordered_qubits_dict
        self._involved_qubits: Set[Union[int, str]] = set(self._ordered_qubits_dict.values())
        self._parameter = CalculatorFloat(parameter)
        self._variance = CalculatorFloat(variance)
        self._mean = CalculatorFloat(mean)
        self._parametrized = not (self._mean.is_float and self._variance.is_float)

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaOverrotation']
                    ) -> 'PragmaOverrotation':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaOverrotation
        """
        return cls(gate=config['gate'],
                   statistic_type=config['statistic_type'],
                   ordered_qubits_dict=config['ordered_qubits_dict'],
                   parameter=config['parameter'],
                   overrotation_parameter=config['overrotation_parameter'],
                   variance=config['variance'],
                   mean=config['mean'])

    def to_qonfig(self) -> 'Qonfig[PragmaOverrotation]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaOverrotation]
        """
        config = Qonfig(self.__class__)
        config['gate'] = self._gate
        config['statistic_type'] = self._type
        config['ordered_qubits_dict'] = self._ordered_qubits_dict
        config['parameter'] = self._parameter
        config['overrotation_parameter'] = self._overrotation_parameter
        config['variance'] = self._variance
        config['mean'] = self._mean

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._type == other._type:
            return False
        if not self._overrotation_parameter == other._overrotation_parameter:
            return False
        if not self._gate == other._gate:
            return False
        if not self._ordered_qubits_dict == other._ordered_qubits_dict:
            return False
        if not self._parameter == other._parameter:
            return False
        if not self._variance == other._variance:
            return False
        if not self._mean == other._mean:
            return False
        return True

    def __copy__(self) -> 'PragmaOverrotation':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaOverrotation
        """
        self_copy = self.__class__()
        self_copy._gate = copy(self._gate)
        self_copy._type = self._type
        self_copy._overrotation_parameter = self._overrotation_parameter
        self_copy._ordered_qubits_dict = copy(self._ordered_qubits_dict)
        self_copy._involved_qubits = copy(self._involved_qubits)
        self_copy._parameter = copy(self._parameter)
        self_copy._variance = copy(self._variance)
        self_copy._mean = copy(self._mean)
        self_copy._parametrized = copy(self._parametrized)
        return self_copy

    def __pow__(self, other: IntoCalculatorFloat) -> 'PragmaOverrotation':
        """Return the gate to the other-th power

        Implements the power magic method for gates where the power operation can be applied easily
        (gates that have a rotation_strength_parameter).

        Args:
            other: exponent

        Returns:
            PragmaOverrotation
        """
        return_operation = copy(self)
        return_operation._variance = (return_operation._variance * other)
        return_operation._mean = (return_operation._mean * other)
        if not (return_operation._variance.is_float and return_operation._mean.is_float):
            return_operation._parametrized = True
        return return_operation

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

    def substitute_parameters(self,
                              substitution_dict: Dict[str, float],
                              ) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)

            parameter = self._variance
            if parameter.is_float is True:
                parameter = parameter.__float__()
            else:
                parameter = parameter.__str__()
            if isinstance(parameter, str):
                parameter = parse_string(substitution_string + '; ' + parameter)
                self._variance = CalculatorFloat(parameter)

            parameter = self._mean
            if parameter.is_float is True:
                parameter = parameter.__float__()
            else:
                parameter = parameter.__str__()
            if isinstance(parameter, str):
                parameter = parse_string(substitution_string + '; ' + parameter)
                self._mean = CalculatorFloat(parameter)
            self._parametrized = False

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += ' {} ({},{},{},{})'.format(
            self._type,
            self._gate,
            self._parameter,
            self._mean,
            self._variance)
        for key in self._ordered_qubits_dict.keys():
            string += ' {}'.format(self._ordered_qubits_dict[key])
        return string


class PragmaStop(Pragma):
    """Implements the Stop PRAGMA.

    This PRAGMA operation signals the STOP of a parallel execution block.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaStop')
    _hqs_lang_name = 'PragmaStop'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubits': {'doc': 'Qubits involved in parallel execution block',
                   'default': None},
        'execution_time': {'doc': 'Optional time for the execution of the block in seconds',
                           'default': None},
    }

    def __init__(self,
                 qubits: Optional[List[Union[int, str]]] = None,
                 execution_time: Optional[float] = None
                 ) -> None:
        """Initialize 'STOP' PRAGMA

        Args:
            qubits: Qubits involved in parallel execution block
            execution_time: Optional time for the execution of the block in seconds
        """
        if qubits is None:
            qubits = ['ALL']
        self._qubits = qubits
        self._involved_qubits: Set[Union[int, str]] = set(self._qubits)
        self.execution_time = execution_time
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaStop']
                    ) -> 'PragmaStop':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaStop
        """
        return cls(qubits=config['qubits'],
                   execution_time=config['execution_time'])

    def to_qonfig(self) -> 'Qonfig[PragmaStop]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaStop]
        """
        config = Qonfig(self.__class__)
        config['qubits'] = self._qubits
        config['execution_time'] = self.execution_time

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not set(self._qubits) == set(other._qubits):
            return False
        if not self.execution_time == other.execution_time:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        if self._qubits != ['ALL']:
            new_qubits: List[Union[int, str]] = list()
            qubits = cast(List[int], self._qubits)
            for qubit in qubits:
                new_qubits.append(mapping_dict[qubit])
            self._qubits = new_qubits
            self._involved_qubits = set(self._qubits)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = "PragmaStop"
        if self.execution_time is not None:
            string += '({})'.format(self.execution_time)
        if self._qubits == ['ALL']:
            string += ' ALL'
        else:
            for qubit in self._qubits:
                string += " {}".format(qubit)
        return string


class PragmaGlobalPhase(Pragma):
    """Implements the GlobalPhase PRAGMA.

    This PRAGMA operation signals that the quantum register picks up a global phase.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaGlobalPhase')
    _hqs_lang_name = 'PragmaGlobalPhase'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'phase': {'doc': 'Picked up phase',
                  'default': 0},
    }

    def __init__(self,
                 phase: IntoCalculatorFloat = 0
                 ) -> None:
        """Initialize 'global phase' PRAGMA

        Args:
            phase: Picked up phase
        """
        self._involved_qubits: Set[Union[int, str]] = set()
        self.phase = CalculatorFloat(phase)
        self._parametrized = not (self.phase.is_float)

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGlobalPhase']
                    ) -> 'PragmaGlobalPhase':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGlobalPhase
        """
        return cls(phase=config['phase'])

    def to_qonfig(self) -> 'Qonfig[PragmaGlobalPhase]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGlobalPhase]
        """
        config = Qonfig(self.__class__)
        config['phase'] = self.phase

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self.phase == other.phase:
            return False
        return True

    def substitute_parameters(
            self,
            substitution_dict: Dict[str, float]) -> None:
        r"""Substitute the symbolic parameters in the operation

        Substitutes the symbols in the parameters and qubits of the gate
        according to the provided substitution_dict.

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (must be CalculatorFloat)
        """
        if self.is_parametrized:
            substitution_string = ''
            for key, val in substitution_dict.items():
                substitution_string += '{}={}; '.format(key, val)
            parameter = self.phase
            if parameter.is_float is True:
                parameter = parameter.__float__()
            else:
                parameter = parameter.__str__()
            if isinstance(parameter, str):
                new_parameter = parse_string(substitution_string + '; ' + parameter)
                self.phase = CalculatorFloat(new_parameter)
            self._parametrized = False

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = "PragmaGlobalPhase"
        string += " {}".format(self.phase)
        return string


class PragmaParameterSubstitution(Pragma):
    """Implements ParameterSubstitution PRAGMA.

    This PRAGMA operation tells the backend to substitute parameters
    in to_backend functions.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaParameterSubstitution')
    _hqs_lang_name = 'PragmaParameterSubstitution'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'substitution_dict': {'doc': 'Qubits involved in parallel execution block',
                              'default': None},
    }

    def __init__(self,
                 substitution_dict: Optional[Dict[str, float]] = None,
                 ) -> None:
        """Initialize 'parameter substitution' PRAGMA

        Args:
            substitution_dict: Qubits involved in parallel execution block
        """
        if substitution_dict is None:
            substitution_dict = cast(Dict[str, float], dict())
        self._substitution_dict = substitution_dict
        self._involved_qubits: Set[Union[int, str]] = set()
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaParameterSubstitution']
                    ) -> 'PragmaParameterSubstitution':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaParameterSubstitution
        """
        return cls(substitution_dict=config['substitution_dict'])

    def to_qonfig(self) -> 'Qonfig[PragmaParameterSubstitution]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaParameterSubstitution]
        """
        config = Qonfig(self.__class__)
        config['substitution_dict'] = self._substitution_dict

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._substitution_dict == other._substitution_dict:
            return False
        return True

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = "PragmaParameterSubstitution"
        for name, val in self._substitution_dict.items():
            string += ' {}={};'.format(name, val)
        return string

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
            override (Optional[Dict[str, Any]]): Override of arguments in backend __init__
        """
        if backend is None:
            return None
        return {'substitution_dict': self._substitution_dict}


class PragmaSleep(Pragma):
    """Implements the Sleep PRAGMA.

    This PRAGMA operation makes the quantum hardware wait a certain amount of time.
    It is used to boost noise, for instance for mitigation reasons.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaSleep')
    _hqs_lang_name = 'PragmaSleep'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubits': {'doc': 'Qubits involved in sleep block',
                   'default': None},
        'execution_time': {'doc': 'Optional time for the execution of the block in seconds',
                           'default': None},
    }

    def __init__(self,
                 qubits: Optional[List[Union[int, str]]] = None,
                 execution_time: Optional[float] = None
                 ) -> None:
        """Initialize 'sleep' PRAGMA

        Args:
            qubits: Qubits involved in sleep block
            execution_time: Optional time for the execution of the block in seconds
        """
        if qubits is None:
            qubits = ['ALL']
        self._qubits = qubits
        self._involved_qubits: Set[Union[int, str]] = set(self._qubits)
        self.execution_time = execution_time
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaSleep']
                    ) -> 'PragmaSleep':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaSleep
        """
        return cls(qubits=config['qubits'],
                   execution_time=config['execution_time'])

    def to_qonfig(self) -> 'Qonfig[PragmaSleep]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaSleep]
        """
        config = Qonfig(self.__class__)
        config['qubits'] = self._qubits
        config['execution_time'] = self.execution_time

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not set(self._qubits) == set(other._qubits):
            return False
        if not self.execution_time == other.execution_time:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        if self._qubits != ['ALL']:
            new_qubits: List[Union[int, str]] = list()
            qubits = cast(List[int], self._qubits)
            for qubit in qubits:
                new_qubits.append(mapping_dict[qubit])
            self._qubits = new_qubits
            self._involved_qubits = set(self._qubits)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        if self.execution_time is not None:
            string += '({})'.format(self.execution_time)
        if self._qubits == ['ALL']:
            string += ' ALL'
        else:
            for qubit in self._qubits:
                string += " {}".format(qubit)
        return string


class PragmaActiveReset(Pragma):
    """Implements the ActiveReset PRAGMA.

    This PRAGMA operation resets the chosen qubit to the zero state.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaActiveReset')
    _hqs_lang_name = 'PragmaActiveReset'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubit': {'doc': 'Qubit to be reset',
                  'default': None},
    }

    def __init__(self,
                 qubit: int,
                 ) -> None:
        """Initialize 'sleep' PRAGMA

        Args:
            qubit: Qubit to be reset
        """
        self._qubit = qubit
        self._involved_qubits: Set[Union[int, str]] = set([self._qubit])
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaActiveReset']
                    ) -> 'PragmaActiveReset':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaActiveReset
        """
        return cls(qubit=config['qubit'])

    def to_qonfig(self) -> 'Qonfig[PragmaActiveReset]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaActiveReset]
        """
        config = Qonfig(self.__class__)
        config['qubit'] = self._qubit

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._qubit == other._qubit:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices

        Raises:
            NotImplementedError: Not implemented
        """
        raise NotImplementedError('Not implemented')

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += " {}".format(self._qubit)
        return string


class PragmaStartDecompositionBlock(Pragma):
    """Implements the StartDecompositionBlock PRAGMA.

    This PRAGMA operation signals the START of a decomposition block.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaStartDecompositionBlock')
    _hqs_lang_name = 'PragmaStartDecompositionBlock'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubits': {'doc': 'Qubits involved in the block',
                   'default': None},
        'reordering_dictionary': {'doc': 'Reordering dictionary of the block',
                                  'default': None},
    }

    def __init__(self,
                 qubits: Optional[List[Union[int, str]]] = None,
                 reordering_dictionary: Optional[Dict[int, int]] = None,
                 ) -> None:
        """Initialize 'start decomposition block' PRAGMA

        Args:
            qubits: Qubits involved in the block
            reordering_dictionary: Reordering dictionary of the block
        """
        if qubits is None:
            qubits = ['ALL']
        self._qubits = qubits
        self._involved_qubits: Set[Union[int, str]] = set(self._qubits)
        self.reordering_dictionary = reordering_dictionary

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaStartDecompositionBlock']
                    ) -> 'PragmaStartDecompositionBlock':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaStartDecompositionBlock
        """
        instance_rd: Dict[int, int] = dict()
        if config['reordering_dictionary'] is not None:
            for key, value in config['reordering_dictionary'].items():
                instance_rd[int(key)] = value
        return cls(qubits=config['qubits'],
                   reordering_dictionary=(
                       instance_rd if config['reordering_dictionary'] is not None else None))

    def to_qonfig(self) -> 'Qonfig[PragmaStartDecompositionBlock]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaStartDecompositionBlock]
        """
        config = Qonfig(self.__class__)
        config['qubits'] = self._qubits
        config['reordering_dictionary'] = self.reordering_dictionary

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not set(self._qubits) == set(other._qubits):
            return False
        if not self.reordering_dictionary == other.reordering_dictionary:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        if self._qubits != ['ALL']:
            new_qubits: List[Union[int, str]] = list()
            qubits = cast(List[int], self._qubits)
            for qubit in qubits:
                new_qubits.append(mapping_dict[qubit])
            self._qubits = new_qubits
            self._involved_qubits = set(self._qubits)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = "PragmaStartDecompositionBlock"
        string += '({})'.format(self.reordering_dictionary)
        if self._qubits == ['ALL']:
            string += ' ALL'
        else:
            for qubit in self._qubits:
                string += " {}".format(qubit)
        return string


class PragmaStopDecompositionBlock(Pragma):
    """Implements the StopDecompositionBlock PRAGMA.

    This PRAGMA operation signals the STOP of a decomposition block.

    """

    _operation_tags = ('Operation', 'Pragma', 'PragmaStopDecompositionBlock')
    _hqs_lang_name = 'PragmaStopDecompositionBlock'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubits': {'doc': 'Qubits involved in the block',
                   'default': None},
    }

    def __init__(self,
                 qubits: Optional[List[Union[int, str]]] = None,
                 ) -> None:
        """Initialize 'stop decomposition block' PRAGMA

        Args:
            qubits: Qubits involved in the block
        """
        if qubits is None:
            qubits = ['ALL']
        self._qubits = qubits
        self._involved_qubits: Set[Union[int, str]] = set(self._qubits)

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaStopDecompositionBlock']
                    ) -> 'PragmaStopDecompositionBlock':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaStopDecompositionBlock
        """
        return cls(qubits=config['qubits'])

    def to_qonfig(self) -> 'Qonfig[PragmaStopDecompositionBlock]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaStopDecompositionBlock]
        """
        config = Qonfig(self.__class__)
        config['qubits'] = self._qubits

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the PRAGMA with the other Python object and returns True when equal

        Args:
            other: Object the PRAGMA is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not set(self._qubits) == set(other._qubits):
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        if self._qubits != ['ALL']:
            new_qubits: List[Union[int, str]] = list()
            qubits = cast(List[int], self._qubits)
            for qubit in qubits:
                new_qubits.append(mapping_dict[qubit])
            self._qubits = new_qubits
            self._involved_qubits = set(self._qubits)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = "PragmaStopDecompositionBlock"
        if self._qubits == ['ALL']:
            string += ' ALL'
        else:
            for qubit in self._qubits:
                string += " {}".format(qubit)
        return string
