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
"""Collection of qoqo single qubit gate operations"""
from qoqo.operations._operations_base_classes import (
    GateOperation,
)
import numpy as np
from copy import copy
from typing import (
    Dict,
    Tuple,
    List,
)
from hqsbase.calculator import (
    CalculatorFloat,
    CalculatorComplex,
)


class SingleQubitGateOperation(GateOperation):
    r"""Base Class for all single qubit unitary gates that can be brought into the form:

    .. math::
        U =e^{i \phi}\begin{pmatrix}
        \alpha_r+i \alpha_i & -\beta_r+i \beta_i\\
        \beta_r+i \beta_i & \alpha_r-i\alpha_i
        \end{pmatrix}

    These are the single qubit gates that are performed in the Circuit(), and are then translated
    to quantum hardware through the relevant backend. Two-qubit gates are also available
    (see qoqo.operations.two_qubit_gates.py).

    """

    _operation_tags: Tuple[str, ...] = ('Operation', 'GateOperation',
                                        'SingleQubitGateOperation')

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': dict()},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    def __eq__(self, other: object) -> bool:
        """Compare the operation with the other Python object and returns True when equal

        Args:
            other: Object operation is compared to

        Returns:
            bool
        """
        if not isinstance(other, SingleQubitGateOperation):
            return False
        for atr in ['alpha_r', 'alpha_i', 'beta_r', 'beta_i']:
            if not CalculatorFloat(getattr(self, atr)).isclose(getattr(other, atr)):
                return False
        for key, val in self._ordered_qubits_dict.items():
            if not (val
                    == other._ordered_qubits_dict[key]):
                return False
        return True

    @property
    def alpha_r(self) -> CalculatorFloat:
        """Real part of the on-diagonal elements of the single-qubit unitary

        Returns:
            CalculatorFloat
        """
        return self._alpha_r()

    @property
    def alpha_i(self) -> CalculatorFloat:
        """Imaginary part of the on-diagonal elements of the single-qubit unitary

        Returns:
            CalculatorFloat
        """
        return self._alpha_i()

    @property
    def beta_r(self) -> CalculatorFloat:
        """Real part of the off-diagonal elements of the single-qubit unitary

        Returns:
            CalculatorFloat
        """
        return self._beta_r()

    @property
    def beta_i(self) -> CalculatorFloat:
        """Imaginary part of the off-diagonal elements of the single-qubit unitary

        Returns:
            CalculatorFloat
        """
        return self._beta_i()

    @property
    def global_phase(self) -> CalculatorFloat:
        """Global phase phi of the single-qubit unitary

        Returns:
            CalculatorFloat
        """
        return self._global_phase()

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    @property
    def single_qubit_gate(self) -> 'SingleQubitGate':
        """Return the qoqo version of the universal single qubit gate

        It is used for intermediate steps.

        Returns:
            SingleQubitGate
        """
        return SingleQubitGate(qubit=self._ordered_qubits_dict['qubit'],
                               alpha_r=self.alpha_r,
                               alpha_i=self.alpha_i,
                               beta_r=self.beta_r,
                               beta_i=self.beta_i,
                               global_phase=self.global_phase)

    def __mul__(self,
                other: 'SingleQubitGateOperation'
                ) -> 'SingleQubitGate':
        """Multiply self with other GateOperation (which is a SingleQubitGateOperation)

        Args:
            other: the SingleQubitGateOperation that self is multiplied with

        Returns:
            SingleQubitGate

        Raises:
            TypeError: Multiplication only available between two single qubit gates
            ValueError: Multiplication of single qubit gates
                        operating on different qubits not possible
        """
        if not isinstance(other, SingleQubitGateOperation):
            raise TypeError('Multiplication only available between two single qubit gates')
        if self.involved_qubits == other.involved_qubits:
            if not isinstance(other, SingleQubitGate):
                other = other.single_qubit_gate
            if not isinstance(self, SingleQubitGate):
                s = self.single_qubit_gate
            else:
                s = self
            alpha = CalculatorComplex.from_pair(s.alpha_r, s.alpha_i)
            beta = CalculatorComplex.from_pair(s.beta_r, s.beta_i)
            oalpha = CalculatorComplex.from_pair(other.alpha_r, other.alpha_i)
            obeta = CalculatorComplex.from_pair(other.beta_r, other.beta_i)
            new_alpha = alpha * oalpha - obeta * beta.conj()
            new_beta = beta * oalpha + obeta * alpha.conj()
            global_phase_new = s.global_phase + other.global_phase
            alpha_r_new = new_alpha.real
            alpha_i_new = new_alpha.imag
            beta_r_new = new_beta.real
            beta_i_new = new_beta.imag
            global_phase_new = s.global_phase + other.global_phase

            return SingleQubitGate(qubit=self._ordered_qubits_dict['qubit'],
                                   alpha_r=alpha_r_new,
                                   alpha_i=alpha_i_new,
                                   beta_r=beta_r_new,
                                   beta_i=beta_i_new,
                                   global_phase=global_phase_new)
        else:
            raise ValueError(
                'Multiplication of single qubit gates operating on different qubits not possible')


class SingleQubitGate(SingleQubitGateOperation):
    r"""Implements the general single qubit unitary gates.

    .. math::
       U =\begin{pmatrix} \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\
            \beta_r+i \beta_i & \alpha_r-i\alpha_i \end{pmatrix}

    Args:
       qubit: the qubit that the unitary gate is applied to.
       alpha_r: real part of the on-diagonal elements of the single-qubit unitary.
       alpha_i: imaginary part of the on-diagonal elements of the single-qubit unitary.
       beta_r: real part of the off-diagonal elements of the single-qubit unitary.
       beta_i: imaginary part of the off-diagonal elements of the single-qubit unitary.
       global_phase: the global phase of the single-qubit unitary.

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'SingleQubitGate')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['alpha_r'] = CalculatorFloat(1)
    _ordered_parameter_dict_default['alpha_i'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['beta_r'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['beta_i'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['global_phase'] = CalculatorFloat(0)

    _hqs_lang_name = 'SingleQubitGate'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'alpha_r': CalculatorFloat(1),
                                               'alpha_i': CalculatorFloat(0),
                                               'beta_r': CalculatorFloat(0),
                                               'beta_i': CalculatorFloat(0),
                                               'global_phase': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args,
                                       alpha_r: float,
                                       alpha_i: float,
                                       beta_r: float,
                                       beta_i: float,
                                       global_phase: float = 0,
                                       **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            alpha_r: real part of on-diagonal elements of the single-qubit unitary.
            alpha_i: imaginary part of on-diagonal elements of the single-qubit unitary.
            beta_r: real part of off-diagonal elements of the single-qubit unitary.
            beta_i: imaginary part of off-diagonal elements of the single-qubit unitary.
            global_phase: The global phase of the operation
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        alpha = alpha_r + 1j * alpha_i
        beta = beta_r + 1j * beta_i
        matrix = np.exp(1j * global_phase) * np.array([[alpha, -np.conj(beta)],
                                                       [beta, np.conj(alpha)]], dtype=complex)
        return matrix

    def __imul__(self, other: 'SingleQubitGateOperation') -> 'SingleQubitGate':
        """Multiply self (a single qubit operation) with the other single qubit operation

        Args:
            other: the other SingleQubitGate that self is multiplied with

        Returns:
            SingleQubitGate

        Raises:
            ValueError: Multiplication of single qubit gates
                        operating on different qubits not possible
        """
        if self.involved_qubits == other.involved_qubits:
            if not isinstance(other, SingleQubitGate):
                other = other.single_qubit_gate
            s = self
            alpha = CalculatorComplex.from_pair(s.alpha_r, s.alpha_i)
            beta = CalculatorComplex.from_pair(s.beta_r, s.beta_i)
            oalpha = CalculatorComplex.from_pair(other.alpha_r, other.alpha_i)
            obeta = CalculatorComplex.from_pair(other.beta_r, other.beta_i)
            new_alpha = alpha * oalpha - obeta * beta.conj()
            new_beta = beta * oalpha + obeta * alpha.conj()
            global_phase_new = s.global_phase + other.global_phase
            alpha_r_new = new_alpha.real
            alpha_i_new = new_alpha.imag
            beta_r_new = new_beta.real
            beta_i_new = new_beta.imag
            global_phase_new = s.global_phase + other.global_phase

            self._ordered_parameter_dict['alpha_r'] = alpha_r_new
            self._ordered_parameter_dict['alpha_i'] = alpha_i_new
            self._ordered_parameter_dict['beta_r'] = beta_r_new
            self._ordered_parameter_dict['beta_i'] = beta_i_new
            self._ordered_parameter_dict['global_phase'] = global_phase_new
            return self
        else:
            raise ValueError(
                'Multiplication of single qubit gates operating on different qubits not possible')

    def _alpha_r(self) -> CalculatorFloat:
        return self._ordered_parameter_dict['alpha_r']

    def _alpha_i(self) -> CalculatorFloat:
        return self._ordered_parameter_dict['alpha_i']

    def _beta_r(self) -> CalculatorFloat:
        return self._ordered_parameter_dict['beta_r']

    def _beta_i(self) -> CalculatorFloat:
        return self._ordered_parameter_dict['beta_i']

    def _global_phase(self) -> CalculatorFloat:
        return self._ordered_parameter_dict['global_phase']

    @property
    def single_qubit_gate(self) -> 'SingleQubitGate':
        """Return the qoqo version of the universal single qubit gate U3

        It is used for intermediate steps.

        Returns:
            SingleQubitGate
        """
        return copy(self)


class Hadamard(SingleQubitGateOperation):
    r"""Implements the Hadamard gate.

    .. math::
        U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 1\\
        1 & -1
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'Hadamard')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _rotation_strength_parameters: List[str]
    _rotation_strength_modulo: Dict[str, float]

    _hqs_lang_name = 'Hadamard'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = 1 / np.sqrt(2) * np.array([[1, 1],
                                            [1, -1]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(-1 / np.sqrt(2))

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(-1 / np.sqrt(2))

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 2)


class PauliX(SingleQubitGateOperation):
    r"""Implements the Pauli X gate.

    .. math::
        U = \begin{pmatrix}
        0 & 1\\
        1 & 0
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'PauliX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'PauliX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[0, 1],
                           [1, 0]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(-1)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 2)


class PauliY(SingleQubitGateOperation):
    r"""Implements the Pauli Y gate.

    .. math::
        U = \begin{pmatrix}
        0 & -i\\
        i & 0
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'PauliY')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'PauliY'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[0, -1j],
                           [1j, 0]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(1)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 2)


class PauliZ(SingleQubitGateOperation):
    r"""Implements the Pauli Z gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0\\
        0 & -1
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'PauliZ')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'PauliZ'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0],
                           [0, -1]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(-1)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 2)


class SGate(SingleQubitGateOperation):
    r"""Implements the S gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0\\
        0 & i
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'SGate')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'SGate'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0],
                           [0, 1j]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(1 / np.sqrt(2))

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(-1 / np.sqrt(2))

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 4)


class TGate(SingleQubitGateOperation):
    r"""Implements the T gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0\\
        0 & e^{i \frac{\pi}{4}}
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'TGate')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'TGate'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0],
                           [0, np.exp(1j * np.pi / 4)]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(np.cos(np.pi / 8))

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(-np.sin(np.pi / 8))

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(np.pi / 8)


class SqrtPauliX(SingleQubitGateOperation):
    r"""Implements the :math:`e^{-i \frac{\pi}{4} \sigma^x}` XPower gate.

    .. math::
        U = \frac{1}{\sqrt(2)}\begin{pmatrix}
        1 & -i\\
        -i & 1
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'SqrtPauliX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'SqrtPauliX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        theta = np.pi / 2
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        matrix = np.array([[c, -1j * s], [-1j * s, c]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(np.cos(np.pi / 4))

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(-np.sin(np.pi / 4))

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class InvSqrtPauliX(SingleQubitGateOperation):
    r"""Implements the :math:`e^{i \frac{\pi}{2} \sigma^x}` XPower gate.

    .. math::
        U = \frac{1}{\sqrt(2)}\begin{pmatrix}
        1 & i\\
        i & 1
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'InvSqrtPauliX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'InvSqrtPauliX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        theta = -np.pi / 2
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        matrix = np.array([[c, -1j * s], [-1j * s, c]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return CalculatorFloat(np.cos(np.pi / 4))

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(np.sin(np.pi / 4))

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class RotateX(SingleQubitGateOperation):
    r"""Implements the :math:`e^{-i \frac{\theta}{2} \sigma^x}` XPower gate.

    .. math::
        U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        0  &  -i \sin(\frac{\theta}{2})   \\
        -i \sin(\frac{\theta}{2})  & 0
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to
        theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'RotateX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'RotateX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, theta: float, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        matrix = np.array([[c, -1j * s], [-1j * s, c]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).cos()

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return -1 * (self._ordered_parameter_dict['theta'] / 2).sin()

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class RotateY(SingleQubitGateOperation):
    r"""Implements the :math:`e^{-i \frac{\theta}{2} \sigma^y}` YPower gate.

    .. math::
        U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        0 & - \sin(\frac{\theta}{2})\\
        \sin(\frac{\theta}{2}) & 0
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to
        theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'RotateY')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = 0
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'RotateY'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, theta: float, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        matrix = np.array([[c, -s], [s, c]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).cos()

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).sin()

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class RotateZ(SingleQubitGateOperation):
    r"""Implements the :math:`e^{-i \frac{\theta}{2} \sigma^z}` ZPower gate.

    .. math::
        U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        - i \sin(\frac{\theta}{2}) & 0\\
        0 & i \sin(\frac{\theta}{2})
        \end{pmatrix}

    Args:
        qubit: qubit the unitary gate is applied to
        theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'RotateZ')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = 0
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'RotateZ'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, theta: float, **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            theta: Angle theta of the rotation, in interval 0 to 2 :math:`2 \pi`
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        matrix = np.array([[c - 1j * s, 0], [0, c + 1j * s]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).cos()

    def _alpha_i(self) -> CalculatorFloat:
        return -1 * (self._ordered_parameter_dict['theta'] / 2).sin()

    def _beta_r(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class RotateAroundSphericalAxis(SingleQubitGateOperation):
    r"""Implements a rotation around an axis given in spherical coordinates.

    .. math::
        U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        -i \sin(\frac{\theta}{2}) v_z  &  \sin(\frac{\theta}{2}) \left(-i v_x - v_y \right) \\
        \sin(\frac{\theta}{2}) \left(-i v_x + v_y \right) & i \sin(\frac{\theta}{2}) v_z)
        \end{pmatrix}

        v_x = \sin(\theta_{sph}) \cos(\phi_{sph})
        v_y = \sin(\theta_{sph}) \sin(\phi_{sph})
        v_z = \cos(\theta_{sph})

    Args:
        qubit: qubit the unitary gate is applied to
        theta: Angle theta of the rotation
        spherical_theta: Rotation axis, unit-vector spherical coordinates phi
        spherical_phi: Rotation axis, unit-vector spherical coordinates phi

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'RotateAroundSphericalAxis')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['spherical_theta'] = CalculatorFloat(np.pi / 2)
    _ordered_parameter_dict_default['spherical_phi'] = CalculatorFloat(0)

    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'RotateAroundSphericalAxis'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0),
                                               'spherical_theta': CalculatorFloat(np.pi / 2),
                                               'spherical_phi': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, theta: float,
                                       spherical_theta: float,
                                       spherical_phi: float,
                                       **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            theta: Angle theta of the rotation
            spherical_theta: Rotation axis, unit-vector spherical coordinates phi
            spherical_phi: Rotation axis, unit-vector spherical coordinates phi
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        vx = np.sin(spherical_theta) * np.cos(spherical_phi)
        vy = np.sin(spherical_theta) * np.sin(spherical_phi)
        vz = np.cos(spherical_theta)
        matrix = np.array([[c - 1j * s * vz, s * (-1j * vx - vy)],
                           [s * (-1j * vx + vy), c + 1j * s * vz]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).cos()

    def _alpha_i(self) -> CalculatorFloat:
        s = (self._ordered_parameter_dict['theta'] / 2).sin()
        vz = self._ordered_parameter_dict['spherical_theta'].cos()
        return -1 * s * vz  # CHECK sign

    def _beta_r(self) -> CalculatorFloat:
        s = (self._ordered_parameter_dict['theta'] / 2).sin()
        vy = self._ordered_parameter_dict['spherical_phi'].sin()
        st = self._ordered_parameter_dict['spherical_theta'].sin()
        return s * vy * st

    def _beta_i(self) -> CalculatorFloat:
        s = (self._ordered_parameter_dict['theta'] / 2).sin()
        vx = self._ordered_parameter_dict['spherical_phi'].cos()
        st = self._ordered_parameter_dict['spherical_theta'].sin()
        return -1 * s * vx * st

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)


class W(RotateAroundSphericalAxis):
    r"""Implements a rotation around an axis in the x-y plane in spherical coordinates.

    .. math::
        U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        -i \sin(\frac{\theta}{2}) v_z  &  \sin(\frac{\theta}{2}) \left(-i v_x - v_y \right) \\
        \sin(\frac{\theta}{2}) \left(-i v_x + v_y \right) & i \sin(\frac{\theta}{2}) v_z)
        \end{pmatrix}

        v_x =  \cos(\phi_{sph})
        v_y =  \sin(\phi_{sph})
        v_z = 0

    Args:
        qubit: qubit the unitary gate is applied to
        theta: Angle theta of the rotation
        spherical_phi: Rotation axis, unit-vector spherical coordinates phi
                        gives the angle in the x-y plane

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'SingleQubitGateOperation',
                       'W')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0

    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['spherical_phi'] = CalculatorFloat(0)

    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'W'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0),
                                               'spherical_phi': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(*args, theta: float,
                                       spherical_phi: float,
                                       **kwargs: float) -> np.ndarray:
        r"""Return the unitary matrix of gate

        Args:
            args: Additional positional arguments should be empty
            theta: Angle theta of the rotation
            spherical_phi: Rotation axis, unit-vector spherical coordinates phi
                           gives the angle in the x-y plane
            kwargs: Additional keyword args

        Returns:
            np.ndarray
        """
        c = np.cos(theta / 2)
        s = np.sin(theta / 2)
        vx = np.cos(spherical_phi)
        vy = np.sin(spherical_phi)
        vz = 0
        matrix = np.array([[c - 1j * s * vz, s * (-1j * vx - vy)],
                           [s * (-1j * vx + vy), c + 1j * s * vz]], dtype=complex)
        return matrix

    def _alpha_r(self) -> CalculatorFloat:
        return (self._ordered_parameter_dict['theta'] / 2).cos()

    def _alpha_i(self) -> CalculatorFloat:
        return CalculatorFloat(0)

    def _beta_r(self) -> CalculatorFloat:
        s = (self._ordered_parameter_dict['theta'] / 2).sin()
        vy = self._ordered_parameter_dict['spherical_phi'].sin()
        return s * vy

    def _beta_i(self) -> CalculatorFloat:
        s = (self._ordered_parameter_dict['theta'] / 2).sin()
        vx = self._ordered_parameter_dict['spherical_phi'].cos()
        return -1 * s * vx

    def _global_phase(self) -> CalculatorFloat:
        return CalculatorFloat(0)
