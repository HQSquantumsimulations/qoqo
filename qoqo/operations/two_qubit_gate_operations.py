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
"""Collection of qoqo two qubit gate operations"""
from qoqo.operations._operations_base_classes import (
    GateOperation,
)
import numpy as np
from cmath import phase
from qoqo.operations.single_qubit_gate_operations import (
    RotateX,
    RotateY,
    RotateZ,
    SingleQubitGateOperation
)
from hqsbase.calculator import (
    CalculatorFloat,
    CalculatorComplex,
)
from typing import (
    Optional,
    Union,
    Any,
    Dict,
    Sequence,
    Tuple,
)


class KakDecomposition(object):
    """Base Class for KAK decompostion.

    This is a temporary class to store the information for the KAK decomposition of a two-qubit
    gate. It already defines the interface, but it will be replaced by a proper version of the
    class. This will be outsourced to a different Python file soon.

    """

    def __init__(self,
                 g: CalculatorFloat,
                 B0: Optional[Union[SingleQubitGateOperation, Any]],
                 B1: Optional[Union[SingleQubitGateOperation, Any]],
                 k: Sequence[CalculatorFloat],
                 A0: Optional[Union[SingleQubitGateOperation, Any]],
                 A1: Optional[Union[SingleQubitGateOperation, Any]],
                 ) -> None:
        """Initialize KakDecomposition

        Args:
            g: Global phase of KAK decomposition
            B0: Single qubit operation acting on qubit 0 before two-qubit decomposition
                Can be SingleQubitOperation of circuit (annotated as Any to avoid circular import)
            B1: Single qubit operation acting on qubit 1 before two-qubit decomposition
                Can be SingleQubitOperation of circuit (annotated as Any to avoid circular import)
            k: Three component vektor of the KAK decompostion
            A0: Single qubit operation acting on qubit 0 after two-qubit decomposition
                Can be SingleQubitOperation of circuit (annotated as Any to avoid circular import)
            A1: Single qubit operation acting on qubit 1 after two-qubit decomposition
                Can be SingleQubitOperation of circuit (annotated as Any to avoid circular import)
        """
        if g is None:
            g0 = CalculatorFloat(0)
        else:
            g0 = CalculatorFloat(g)
        k0 = [CalculatorFloat(k[0]), CalculatorFloat(k[1]), CalculatorFloat(k[2])]
        (self.g, self.B0, self.B1, self.k, self.A0, self.A1) = (g0, B0, B1, k0, A0, A1)


class TwoQubitGateOperation(GateOperation):
    r"""Base Class for all two-qubit gates.

    Provides the kak_decompostion property, which two-qubit gates have to supply.

    For the qubit names we use the following convention:

        - When the unitary matrix is not symmetric we always call the most significant
          qubit in the unitary matrix 'control' and the least significant qubit 'qubit'.

        - When the unitary matrix of the two qubit gate is symmetric under the exchange
          of both qubits we can either use control and qubit or use i and j as qubit names.

    For instance, for the CNOT gate: The most significant qubit in little
    Endian encoding is qubit 1, the least significant qubit is qubit 0.
    In our convention, when the control qubit is qubit 1 and the controlled
    qubit is qubit 0, the CNOT gate takes the matrix form:

    .. math::
        U =\begin{pmatrix} 1 & 0 & 0 & 0 \\
                           0 & 1 & 0 & 0 \\
                           0 & 0 & 0 & 1 \\
                           0 & 0 & 1 & 0  \end{pmatrix}

    Args:
        control: most significant qubit in the two qubit unitary matrix
        qubit: least significant qubit in the two qubit unitary matrix

    """

    _operation_tags: Tuple[str, ...] = ('Operation', 'GateOperation',
                                        'TwoQubitGateOperation',
                                        )

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': dict()},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    # TODO: Include KakDecompostion class and define structure.
    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Raises:
            NotImplementedError: Not implemented
        """
        raise NotImplementedError()


class CNOT(TwoQubitGateOperation):
    r"""Implements the controlled-NOT gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 0 & 1\\
        0 & 0 & 1 & 0
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'CNOT')

    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'CNOT'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1, 0, 0],
                           [0, 0, 0, 1],
                           [0, 0, 1, 0]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        from qoqo.circuit.circuit import Circuit
        g = CalculatorFloat(np.pi / 4)
        B0 = Circuit()
        B0 += RotateZ(qubit=self._ordered_qubits_dict["control"], theta=np.pi / 2)
        B0 += RotateY(qubit=self._ordered_qubits_dict["control"], theta=np.pi / 2)
        B1 = RotateX(qubit=self._ordered_qubits_dict["qubit"], theta=np.pi / 2)
        k = [np.pi / 4, 0, 0]
        A0 = RotateY(qubit=self._ordered_qubits_dict["control"], theta=-np.pi / 2)
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class ISwap(TwoQubitGateOperation):
    r"""Implements the ISwap gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 0 & i & 0\\
        0 & i & 0 & 0\\
        0 & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'ISwap')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'ISwap'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 0, 1j, 0],
                           [0, 1j, 0, 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = None
        B0 = None
        B1 = None
        k = [np.pi / 4, np.pi / 4, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class FSwap(TwoQubitGateOperation):
    r"""Implements the fermionic SWAP gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 0 & 1 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 0 & -1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'FSwap')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    # TODO: Change control/qubit to i/j? Also below in kak_decomposition()
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'FSwap'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 0, 1, 0],
                           [0, 1, 0, 0],
                           [0, 0, 0, -1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = -np.pi / 2
        B0 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=-np.pi / 2)
        B1 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=-np.pi / 2)
        k = [np.pi / 4, np.pi / 4, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class SqrtISwap(TwoQubitGateOperation):
    r"""Implements the square root ISwap gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & \frac{1}{\sqrt{2}} & \frac{i}{\sqrt{2}} & 0\\
        0 & \frac{i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0\\
        0 & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'SqrtISwap')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'SqrtISwap'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1 / np.sqrt(2), 1j / np.sqrt(2), 0],
                           [0, 1j / np.sqrt(2), 1 / np.sqrt(2), 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = None
        B0 = None
        B1 = None
        k = [np.pi / 8, np.pi / 8, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class InvSqrtISwap(TwoQubitGateOperation):
    r"""Implements the inverse square root ISwap gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & \frac{1}{\sqrt{2}} & \frac{-i}{\sqrt{2}} & 0\\
        0 & \frac{-i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0\\
        0 & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'InvSqrtISwap')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'InvSqrtISwap'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1 / np.sqrt(2), -1j / np.sqrt(2), 0],
                           [0, -1j / np.sqrt(2), 1 / np.sqrt(2), 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = None
        B0 = None
        B1 = None
        k = [-np.pi / 8, -np.pi / 8, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class MolmerSorensenXX(TwoQubitGateOperation):
    r"""Implements the fixed phase MolmerSorensen XX gate. (http://arxiv.org/abs/1705.02771)

    .. math::
        U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 0 & 0 & -i\\
        0 & 1 & -i & 0\\
        0 & -i & 1 & 0\\
        -i & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'MolmerSorensenXX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()

    _hqs_lang_name = 'MolmerSorensenXX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        # matrix = np.array([[1, 0, 0, 1j],
        #                    [0, 1, 1j, 0],
        #                    [0, 1j, 1, 0],
        #                    [1j, 0, 0, 1]], dtype=complex) * (1 - 1j) / 2
        matrix = np.array([[1, 0, 0, -1j],
                           [0, 1, -1j, 0],
                           [0, -1j, 1, 0],
                           [-1j, 0, 0, 1]], dtype=complex) / np.sqrt(2)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = None
        B0 = None
        B1 = None
        k = [-np.pi / 4, 0, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class VariableMSXX(TwoQubitGateOperation):
    r"""Implements the variable-angle MolmerSorensen XX gate.

    .. math::
        U = \begin{pmatrix}
        cos(theta/2) & 0 & 0 & -isin(theta/2)\\
        0 & cos(theta/2) & -isin(theta/2) & 0\\
        0 & -isin(theta/2) & cos(theta/2) & 0\\
        -isin(theta/2) & 0 & 0 & cos(theta/2)
        \end{pmatrix}

    Args:
        theta: rotation angle
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'VariableMSXX')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = 0
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'VariableMSXX'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta: rotation angle
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        cos = np.cos(theta / 2)
        sin = 1j * np.sin(theta / 2)
        matrix = np.array([[cos, 0, 0, -sin],
                           [0, cos, -sin, 0],
                           [0, -sin, cos, 0],
                           [-sin, 0, 0, cos]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = self._ordered_parameter_dict["theta"]
        g = None
        B0 = None
        B1 = None
        k = [-theta, 0, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class SWAP(TwoQubitGateOperation):
    r"""Implements the SWAP gate

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 0 & 1 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'SWAP')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    # TODO: i/j instead of control/qubit?
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'SWAP'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 0, 1, 0],
                           [0, 1, 0, 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = CalculatorFloat(-np.pi / 4)
        B0 = None
        B1 = None
        k = 3 * [np.pi / 4]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class ControlledPhaseShift(TwoQubitGateOperation):
    r"""Implements the controlled-PhaseShift gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 1 & 0\\
        0 & 0 & 0 & e^{i \theta}
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to
        theta: rotation angle

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'ControlledPhaseShift')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 2 * np.pi}

    _hqs_lang_name = 'ControlledPhaseShift'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta: rotation angle
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1, 0, 0],
                           [0, 0, 1, 0],
                           [0, 0, 0, np.exp(1j * theta)]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = self._ordered_parameter_dict["theta"]
        g = CalculatorFloat(theta / 4)
        B0 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=(theta / 2))
        B1 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=(theta / 2))
        k = [0, 0, theta / 4]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class ControlledPauliY(TwoQubitGateOperation):
    r"""Implements the controlled-PauliY gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 0 & -i\\
        0 & 0 & i & 0
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'ControlledPauliY')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'ControlledPauliY'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1, 0, 0],
                           [0, 0, 0, -1j],
                           [0, 0, 1j, 0]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        from qoqo.circuit.circuit import Circuit
        g = CalculatorFloat(np.pi / 4)
        B0 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=np.pi / 2)
        B1 = Circuit()
        B1 += RotateY(qubit=self._ordered_qubits_dict["qubit"], theta=np.pi / 2)
        B1 += RotateX(qubit=self._ordered_qubits_dict["qubit"], theta=np.pi / 2)
        k = [0, 0, np.pi / 4]
        A0 = None
        A1 = RotateX(qubit=self._ordered_qubits_dict["qubit"], theta=-np.pi / 2)
        return KakDecomposition(g, B0, B1, k, A0, A1)


class ControlledPauliZ(TwoQubitGateOperation):
    r"""Implements the controlled-phase flip gate (also known as controlled-Z gate).

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & 1 & 0 & 0\\
        0 & 0 & 1 & 0\\
        0 & 0 & 0 & -1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'ControlledPauliZ')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _is_self_inverse = True

    _hqs_lang_name = 'ControlledPauliZ'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': dict()},
    }

    @staticmethod
    def unitary_matrix_from_parameters(**kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, 1, 0, 0],
                           [0, 0, 1, 0],
                           [0, 0, 0, -1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        g = CalculatorFloat(np.pi / 4)
        B0 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=np.pi / 2)
        B1 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=np.pi / 2)
        k = [0, 0, np.pi / 4]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class Fsim(TwoQubitGateOperation):
    r"""Implements the FSIM gate.

    Implements the fermionic simulation gate

     .. math::
        \begin{pmatrix}
        \cos(\Delta) & 0 & 0 & i \sin(\Delta)\\
        0 & -i \sin(t) & \cos(t) & 0\\
        0 & \cos(t) & -i \sin(t) & 0\\
        -\sin(\Delta)*\sexp(-i U) & 0 & 0 & -\cos(\Delta)*\exp(-i U)
        \end{pmatrix}

    Args:
        t: hopping strength
        U: interaction strength
        Delta: Bogoliubov interactions strength
        control: index of the most significant qubit in the unitary representation
        qubit: index of the least significant qubit in the unitary representation

    Note:
        The qubits have to be adjacent, i.e., :math:`|i-j|=1` has to hold. This is the only case
        in which the gate is valid as a two-qubit gate (due to the Jordan-Wigner transformation).

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'Fsim')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_qubits_dict_default['control'] = 1
    # TODO: Check that abs(i-j) == 1?
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['U'] = 0
    _ordered_parameter_dict_default['t'] = 0
    # TODO: Delta has to be real. Split into real/imag parts as in Bogoliubov. Adjust kak_decomp.
    _ordered_parameter_dict_default['Delta'] = CalculatorFloat(0)

    _hqs_lang_name = 'Fsim'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'U': 0, 't': 0, 'Delta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(U: float, t: float, Delta: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            t: Hopping strength
            U: interaction strength
            Delta: Bogoliubov interactions strength
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array([[np.cos(Delta), 0, 0, 1j * np.sin(Delta)],
                         [0, -1j * np.sin(t), np.cos(t), 0],
                         [0, np.cos(t), -1j * np.sin(t), 0],
                         [-1j * np.exp(-1j * U) * np.sin(Delta), 0, 0,
                          -np.exp(-1j * U) * np.cos(Delta)]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        U = self._ordered_parameter_dict["U"]
        t = self._ordered_parameter_dict["t"]
        Delta = self._ordered_parameter_dict["Delta"]
        Upi = np.pi
        tDpi = np.pi
        g = Upi / -2 + U / -4
        theta = Upi / -2 + U / -2
        B0 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=theta)
        B1 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=theta)
        k0 = tDpi / 4 + t / -2 + Delta / 2
        k1 = tDpi / 4 + t / -2 + Delta / -2
        k = [k0, k1, U / -4]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class Qsim(TwoQubitGateOperation):
    r"""Implements the Qsim gate.

    Implements the qubit simulation gate

    .. math::
        \begin{pmatrix}
        \cos(x-y)*\exp(-i * z) & 0 & 0 & i \sin(x-y)*\exp(-i * z)\\
        0 & -i \sin(x+y)*\exp(i * z) & \cos(x+y)*\exp(i * z) & 0\\
        0 & \cos(x+y)*\exp(i * z) & -i \sin(x+y)*\exp(i * z) & 0\\
        -\sin(x-y)*\sexp(-i *z) & 0 & 0 & -\cos(x-y)*\exp(-i *z)
        \end{pmatrix}

    Args:
        x: Prefactor of the XX interaction
        y: Prefactor of the YY interaction
        z: Prefactor of the ZZ interaction
        control: index of the most significant qubit in the unitary representation
        qubit: index of the least significant qubit in the unitary representation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'Qsim')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_qubits_dict_default['control'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['x'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['y'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['z'] = CalculatorFloat(0)

    _hqs_lang_name = 'Qsim'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'x': CalculatorFloat(0),
                                               'y': CalculatorFloat(0),
                                               'z': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(x: float, y: float, z: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            x: Prefactor of the XX interaction
            y: Prefactor of the YY interaction
            z: Prefactor of the ZZ interaction
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array(
            [[np.cos(x - y) * np.exp(-1j * z), 0, 0, -1j * np.sin(x - y) * np.exp(-1j * z)],
             [0, -1j * np.sin(x + y) * np.exp(1j * z), np.cos(x + y) * np.exp(1j * z), 0],
             [0, np.cos(x + y) * np.exp(1j * z), -1j * np.sin(x + y) * np.exp(1j * z), 0],
             [-1j * np.sin(x - y) * np.exp(-1j * z), 0, 0, np.cos(x - y) * np.exp(-1j * z)]]
        )

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        mx = -1 * self._ordered_parameter_dict["x"]
        my = -1 * self._ordered_parameter_dict["y"]
        mz = -1 * self._ordered_parameter_dict["z"]
        pifourth_x = np.pi / 4
        pifourth_y = np.pi / 4
        pifourth_z = np.pi / 4
        g = -np.pi / 4
        B0 = None
        B1 = None
        k = [mx + pifourth_x, my + pifourth_y, mz + pifourth_z]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class SpinInteraction(TwoQubitGateOperation):
    r"""Implements the generalized, anisotropic XYZ Heisenberg interaction between spins, i.e.:

    .. math::
        \exp(-\mathrm{i} (x XX + y YY + z ZZ)),

    where x, y, z are prefactors of the XX, YY, ZZ Pauliproducts.

    Args:
        x: Prefactor of the XX interaction
        y: Prefactor of the YY interaction
        z: Prefactor of the ZZ interaction
        control: index of the most significant qubit in the unitary representation
        qubit: index of the least significant qubit in the unitary representation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'SpinInteraction')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_qubits_dict_default['control'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['x'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['y'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['z'] = CalculatorFloat(0)

    _hqs_lang_name = 'SpinInteraction'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'x': CalculatorFloat(0),
                                               'y': CalculatorFloat(0),
                                               'z': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(x: float, y: float, z: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            x: Prefactor of the XX interaction
            y: Prefactor of the YY interaction
            z: Prefactor of the ZZ interaction
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array(
            [[np.cos(x - y) * np.exp(-1j * z), 0, 0, -1j * np.sin(x - y) * np.exp(-1j * z)],
             [0, np.cos(x + y) * np.exp(1j * z), -1j * np.sin(x + y) * np.exp(1j * z), 0],
             [0, -1j * np.sin(x + y) * np.exp(1j * z), np.cos(x + y) * np.exp(1j * z), 0],
             [-1j * np.sin(x - y) * np.exp(-1j * z), 0, 0, np.cos(x - y) * np.exp(-1j * z)]]
        )

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        x = self._ordered_parameter_dict["x"]
        y = self._ordered_parameter_dict["y"]
        z = self._ordered_parameter_dict["z"]
        g = None
        B0 = None
        B1 = None
        k = [-x, -y, -z]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class Bogoliubov(TwoQubitGateOperation):
    r"""Implements the Bogoliubov DeGennes interaction gate.

    .. math:

        \exp(-\mathrm{i} Re(\Delta) (XX - YY)/2 + Im(\Delta) (XY+YX)/2)

    .. math:

        \begin{pmatrix}
        cos(|\Delta|) & 0 & 0 & \sin(|\Delta|) (1j* e^{i \textrm{angle(\Delta)})\\
        0 & 1 & 0 & 0\\
        0 & 0 & 1 & 0\\
        \sin(|\Delta|) (1j* e^{-i \textrm{angle}(\Delta)}) & 0 & 0 & cos(|\Delta|)
        \end{pmatrix}

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'Bogoliubov')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['i'] = 0
    _ordered_qubits_dict_default['j'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['Delta_real'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['Delta_imag'] = CalculatorFloat(0)
    _rotation_strength_parameters = ['Delta_real', 'Delta_imag']
    _rotation_strength_modulo = {'Delta_real': 1, 'Delta_imag': 1}

    _hqs_lang_name = 'Bogoliubov'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'i': 0, 'j': 1}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'Delta_real': CalculatorFloat(0),
                                               'Delta_imag': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(Delta_real: float,
                                       Delta_imag: float,
                                       **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            Delta_real: Bogoliubov interactions real part of gap
            Delta_imag: Bogoliubov interactions imaginary part of gap
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        Delta = complex(Delta_real + 1j * Delta_imag)
        D = np.abs(Delta)
        arg = np.angle(Delta)
        return np.array([[np.cos(D), 0, 0, np.sin(D) * (1j * np.exp(1j * arg))],
                         [0, 1, 0, 0],
                         [0, 0, 1, 0],
                         [np.sin(D) * (1j * np.exp(-1j * arg)), 0, 0, np.cos(D)]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        Delta = CalculatorComplex.from_pair(self._ordered_parameter_dict["Delta_real"],
                                            self._ordered_parameter_dict["Delta_imag"])
        Delta_abs = Delta.__abs__()
        Delta_arg = Delta.arg()
        g = None
        B0 = RotateZ(qubit=self._ordered_parameter_dict["i"], theta=-1 * Delta_arg)
        B1 = None
        k = [Delta_abs / -2, Delta_abs / 2, 0]
        A0 = RotateZ(qubit=self._ordered_parameter_dict["i"], theta=Delta_arg)
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class GivensRotation(TwoQubitGateOperation):
    r"""Implements the Givens rotation interaction gate in big endian notation.

    .. math:

        \exp(-\mathrm{i} t (XY -YX)

    \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & \cos(\theta)e^{i \phi} & \sin(\theta)  & 0\\
        0 & -\sin(\theta)e^{i \phi} & \cos(\theta)  & 0\\
        0 & 0 & 0 & e^{i \phi}
        \end{pmatrix}

    Args:
        qubit: index of the least significant qubit in the unitary representation
        control: index of the most significant qubit in the unitary representation
        theta:  Angle of the rotation
        phi: Phase of the rotation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'GivensRotation')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_qubits_dict_default['control'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['phi'] = CalculatorFloat(0)

    _hqs_lang_name = 'GivensRotation'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0),
                                               'phi': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float,
                                       phi: float,
                                       **kwargs,) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta:  Angle of the rotation
            phi: Phase of the rotation
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array([[1, 0, 0, 0],
                         [0, np.cos(theta) * np.exp(1j * phi), np.sin(theta), 0],
                         [0, -np.sin(theta) * np.exp(1j * phi), np.cos(theta), 0],
                         [0, 0, 0, np.exp(1j * phi)]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = self._ordered_parameter_dict["theta"]
        phi = self._ordered_parameter_dict["phi"]
        g = phi / 2
        # TODO: check qubit/control notation! Switch qubit to control?
        B0 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=np.pi / 2 * phi)
        B1 = None
        k = [theta / 2, theta / 2, 0]
        A0 = RotateZ(qubit=self._ordered_qubits_dict["qubit"], theta=-np.pi / 2)
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class GivensRotationLittleEndian(TwoQubitGateOperation):
    r"""Implements the Givens rotation interaction gate in little endian notation.

    .. math:

        \exp(-\mathrm{i} t (XY -YX)

    \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & \cos(\theta) & \sin(\theta)  & 0\\
        0 & -\sin(\theta)e^{i \phi} & \cos(\theta) e^{i \phi} & 0\\
        0 & 0 & 0 & e^{i \phi}
        \end{pmatrix}

    Args:
        qubit: index of the least significant qubit in the unitary representation
        control: index of the most significant qubit in the unitary representation
        theta:  Angle of the rotation
        phi: Phase of the rotation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'GivensRotationLittleEndian')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_qubits_dict_default['control'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['phi'] = CalculatorFloat(0)

    _hqs_lang_name = 'GivensRotationLittleEndian'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0),
                                               'phi': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float,
                                       phi: float,
                                       **kwargs,
                                       ) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta:  Angle of the rotation
            phi: Phase of the rotation
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array([[1, 0, 0, 0],
                         [0, np.cos(theta), np.sin(theta), 0],
                         [0, -np.sin(theta) * np.exp(1j * phi),
                          np.cos(theta) * np.exp(1j * phi), 0],
                         [0, 0, 0, np.exp(1j * phi)]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = self._ordered_parameter_dict["theta"]
        phi = self._ordered_parameter_dict["phi"]
        g = phi / 2
        # TODO: check qubit/control notation! Switch control to qubit?
        B0 = None
        B1 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=-np.pi / 2)
        k = [theta / 2, theta / 2, 0]
        A0 = None
        A1 = RotateZ(qubit=self._ordered_qubits_dict["control"], theta=np.pi / 2 + phi)
        return KakDecomposition(g, B0, B1, k, A0, A1)


class PMInteraction(TwoQubitGateOperation):
    r"""Implements the transversal interaction gate.

    .. math:

        \exp(-\mathrm{i} \theta (X_i X_j + Y_i Y_j))
        = \exp(-\mathrm{i} \theta (\sigma^+_i \sigma^-_j + \sigma^-_i \sigma^+_j))

    Args:
        i: first qubit the gate acts on
        j: second qubit the gate acts on
        theta: strength of the rotation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'PMInteraction')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['i'] = 0
    _ordered_qubits_dict_default['j'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 2 * np.pi}

    _hqs_lang_name = 'PMInteraction'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'i': 0, 'j': 1}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta: Angle of the rotation
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        return np.array([[1, 0, 0, 0],
                         [0, np.cos(theta), -1j * np.sin(theta), 0],
                         [0, -1j * np.sin(theta), np.cos(theta), 0],
                         [0, 0, 0, 1]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        t = self._ordered_parameter_dict["theta"]
        g = None
        B0 = None
        B1 = None
        k = [t / -2, t / -2, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class ComplexPMInteraction(TwoQubitGateOperation):
    r"""Implements the complex hopping gate.

    .. math:

        \exp(-\mathrm{i} (\theta_real (X_i X_j + Y_i Y_j) - \theta_real (X_i Y_j - Y_i X_j)))

    Args:
        i: first qubit the gate acts on
        j: second qubit the gate acts on
        theta: strength of the rotation

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'ComplexPMInteraction')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['i'] = 0
    _ordered_qubits_dict_default['j'] = 1
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta_real'] = CalculatorFloat(0)
    _ordered_parameter_dict_default['theta_imag'] = CalculatorFloat(0)

    _rotation_strength_parameters = ['theta_real', 'theta_imag']
    _rotation_strength_modulo = {'theta_real': 2 * np.pi, 'theta_imag': 2 * np.pi}

    _hqs_lang_name = 'ComplexPMInteraction'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'i': 0, 'j': 1}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta_real': CalculatorFloat(0),
                                               'theta_imag': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta_real: float,
                                       theta_imag: float,
                                       **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta_real: Real part of the angle of rotation
            theta_imag: Imaginary part of the angle of rotation
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        theta = complex(theta_real + 1j * theta_imag)
        theta_abs = abs(theta)
        theta_arg = phase(theta)

        return np.array([[1, 0, 0, 0],
                         [0, np.cos(theta_abs),
                          -1j * np.exp(-1j * theta_arg) * np.sin(theta_abs), 0],
                         [0, -1j * np.exp(1j * theta_arg) * np.sin(theta_abs),
                          np.cos(theta_abs), 0],
                         [0, 0, 0, 1]])

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = CalculatorComplex.from_pair(self._ordered_parameter_dict["theta_real"],
                                            self._ordered_parameter_dict["theta_imag"])
        theta_abs = theta.__abs__()
        theta_arg = theta.arg()

        g = None
        B0 = RotateZ(qubit=self._ordered_parameter_dict["i"], theta=theta_arg)
        B1 = None
        k = [theta_abs / -2, theta_abs / -2, 0]
        A0 = RotateZ(qubit=self._ordered_parameter_dict["i"], theta=-1 * theta_arg)
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)


class XY(TwoQubitGateOperation):
    r"""Implements the XY gate.

    .. math::
        U = \begin{pmatrix}
        1 & 0 & 0 & 0\\
        0 & cos(theta/2) & i*sin(theta/2) & 0\\
        0 & i*sin(theta/2) & cos(theta/2) & 0\\
        0 & 0 & 0 & 1
        \end{pmatrix}

    Args:
        control: qubit that controls the application of the unitary
        qubit: qubit the unitary gate is applied to
        theta: rotation angle

    """

    _operation_tags = ('Operation', 'GateOperation',
                       'TwoQubitGateOperation',
                       'XY')
    _ordered_qubits_dict_default: Dict[str, int]
    _ordered_qubits_dict_default = dict()
    _ordered_qubits_dict_default['control'] = 1
    _ordered_qubits_dict_default['qubit'] = 0
    _ordered_parameter_dict_default: Dict[str, CalculatorFloat]
    _ordered_parameter_dict_default = dict()
    _ordered_parameter_dict_default['theta'] = CalculatorFloat(0)
    _rotation_strength_parameters = ['theta']
    _rotation_strength_modulo = {'theta': 4 * np.pi}

    _hqs_lang_name = 'XY'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'ordered_qubits_dict': {'doc': 'Ordered qubit dictionary for this gate operation',
                                'default': {'control': 1, 'qubit': 0}},
        'ordered_parameter_dict': {'doc': 'Ordered parameter dictionary for this gate operation',
                                   'default': {'theta': CalculatorFloat(0)}},
    }

    @staticmethod
    def unitary_matrix_from_parameters(theta: float, **kwargs) -> np.ndarray:
        """Return the unitary matrix of the gate

        Args:
            theta: rotation angle
            kwargs: Additional keyword arguments

        Returns:
            np.ndarray
        """
        matrix = np.array([[1, 0, 0, 0],
                           [0, np.cos(theta / 2), 1j * np.sin(theta / 2), 0],
                           [0, 1j * np.sin(theta / 2), np.cos(theta / 2), 0],
                           [0, 0, 0, 1]], dtype=complex)
        return matrix

    @property
    def kak_decomposition(self) -> KakDecomposition:
        """Return the KAK decomposition of the gate

        Returns:
            KakDecomposition
        """
        theta = self._ordered_parameter_dict["theta"]
        g = None
        B0 = None
        B1 = None
        k = [theta / 4, theta / 4, 0]
        A0 = None
        A1 = None
        return KakDecomposition(g, B0, B1, k, A0, A1)
