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
"""Collection of qoqo measurement operations"""
from qoqo.operations._operations_base_classes import (
    Operation,
    Pragma,
)
from typing import (
    Union,
    Optional,
    List,
    Set,
    Any,
    Dict,
    cast,
    Sequence
)
import numpy as np
from copy import copy
from hqsbase.qonfig import Qonfig


class MeasureQubit(Operation):
    """Implements the Measurement gate operation.

    This Operation acts on one qubit, writing the result of the measurement into a
    readout, which has been previously defined using a Definition operation
    (see qoqo.operations.define_operations.py).

    """

    _operation_tags = ('Operation', 'Measurement', 'MeasureQubit')
    _hqs_lang_name = 'MeasureQubit'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubit': {'doc': 'The measured qubit',
                  'default': 0},
        'readout': {'doc': 'The register for the readout',
                    'default': 'ro'},
        'readout_index': {'doc': 'The index in the readout the result is saved to',
                          'default': None},
    }

    def __init__(self,
                 qubit: int = 0,
                 readout: str = 'ro',
                 readout_index: Optional[int] = None) -> None:
        """Initialize a MeasureQubit measurement operation

        Args:
            qubit: The measured qubit
            readout: The register for the readout
            readout_index: The index in the readout the result is saved to
        """
        self._qubit = qubit
        self._involved_qubits: Set[Union[str, int]] = set([self._qubit])
        if readout_index is None:
            readout_index = qubit
        self._readout_index = readout_index
        self._readout = readout
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['MeasureQubit']
                    ) -> 'MeasureQubit':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            MeasureQubit
        """
        return cls(qubit=int(config['qubit']),
                   readout=config['readout'],
                   readout_index=int(config['readout_index']))

    def to_qonfig(self) -> 'Qonfig[MeasureQubit]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[MeasureQubit]
        """
        config = Qonfig(self.__class__)
        config['qubit'] = self._qubit
        config['readout'] = self._readout
        config['readout_index'] = self._readout_index

        return config

    def __eq__(self, other: object) -> bool:
        """Compare the operation with the other Python object and returns True when equal

        Args:
            other: Object operation is compared to

        Returns:
            bool
        """
        if not isinstance(other, self.__class__):
            return False
        if not self._readout == other._readout:
            return False
        if not self._readout_index == other._readout_index:
            return False
        if not self._qubit == other._qubit:
            return False
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

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
        string += ' {}'.format(self._qubit)
        string += ' {}[{}]'.format(self._readout,
                                   self._readout_index)
        return string


class PragmaGetStateVector(Pragma):
    r"""Implements the GetStateVector PRAGMA.

    This PRAGMA operation returns the statevector of a quantum register.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma', 'PragmaGetStateVector')
    _hqs_lang_name = 'PragmaGetStateVector'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'qubit_mapping': {'doc': 'Mapping of qubits to indices in readout register',
                          'default': None},
        'circuit': {'doc': 'Optional Circuit applied on copy of quest register before measurement',
                    'default': None},
    }

    def __init__(self,
                 readout: str = 'ro',
                 qubit_mapping: Optional[Dict[int, int]] = None,
                 circuit: Any = None
                 ) -> None:
        """Initialize 'get state vector' PRAGMA

        Args:
            readout: Name of the readout register
            qubit_mapping: Mapping of qubits to indices in readout register
            circuit: Optional Circuit applied on copy of quest register before measurement. Needs
                     to be type annotated as Any to avoid circular import but is (qoqo.Circuit).
        """
        self._readout = readout
        self._qubit_mapping = qubit_mapping
        self._involved_qubits = set(['ALL'])
        self._parametrized = False
        self._circuit = circuit

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGetStateVector']
                    ) -> 'PragmaGetStateVector':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGetStateVector
        """
        if config['qubit_mapping'] is not None:
            for key, _ in config['qubit_mapping'].items():
                config['qubit_mapping'][float(key)] = config['qubit_mapping'].pop(key)
        if config['circuit'] is not None:
            circuit = config['circuit'].to_instance()
        else:
            circuit = config['circuit']
        return cls(readout=config['readout'],
                   qubit_mapping=config['qubit_mapping'],
                   circuit=circuit)

    def to_qonfig(self) -> 'Qonfig[PragmaGetStateVector]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGetStateVector]
        """
        config = Qonfig(self.__class__)
        config['readout'] = self._readout
        config['qubit_mapping'] = self._qubit_mapping
        if self._circuit is not None:
            config['circuit'] = self._circuit.to_qonfig()
        else:
            config['circuit'] = self._circuit

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
        if not self._circuit == other._circuit:
            return False
        return True

    def __copy__(self) -> 'PragmaGetStateVector':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaGetStateVector
        """
        self_copy = self.__class__()
        self_copy._readout = copy(self._readout)
        self_copy._circuit = copy(self._circuit)
        return self_copy

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        new_qubit_mapping: Optional[Dict[int, int]]
        if self._qubit_mapping is not None:
            new_qubit_mapping = dict()
            for key, val in self._qubit_mapping.items():
                new_qubit_mapping[mapping_dict[key]] = val
            self._qubit_mapping = new_qubit_mapping
            if self._circuit is not None:
                self._circuit.remap_qubits(mapping_dict=mapping_dict)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        if self._qubit_mapping is not None:
            string += '('
            for key in self._qubit_mapping.keys():
                string += '{}:{},'.format(key, self._qubit_mapping[key])
            string += ')'
        string += ' {}'.format(self._readout)
        return string


class PragmaGetDensityMatrix(Pragma):
    r"""Implements the GetDensityMatrix PRAGMA.

    This PRAGMA operation returns the density matrix of a quantum register.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma', 'PragmaGetDensityMatrix')
    _hqs_lang_name = 'PragmaGetDensityMatrix'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'qubit_mapping': {'doc': 'Mapping of qubits to indices in readout register',
                          'default': None},
        'circuit': {'doc': 'Optional Circuit applied on copy of quest register before measurement',
                    'default': None},
    }

    def __init__(self,
                 readout: str = 'ro',
                 qubit_mapping: Optional[Dict[int, int]] = None,
                 circuit: Any = None
                 ) -> None:
        """Initialize 'get density matrix' PRAGMA

        Args:
            readout: Name of the readout register
            qubit_mapping: Mapping of qubits to indices in readout register
            circuit: Optional Circuit applied on copy of quest register before measurement. Needs
                     to be type annotated as Any to avoid circular import but is (qoqo.Circuit).
        """
        self._readout = readout
        self._qubit_mapping = qubit_mapping
        self._involved_qubits = set(['ALL'])
        self._parametrized = False
        self._circuit = circuit

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGetDensityMatrix']
                    ) -> 'PragmaGetDensityMatrix':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGetDensityMatrix
        """
        if config['qubit_mapping'] is not None:
            for key, _ in config['qubit_mapping'].items():
                config['qubit_mapping'][float(key)] = config['qubit_mapping'].pop(key)
        if config['circuit'] is not None:
            circuit = config['circuit'].to_instance()
        else:
            circuit = config['circuit']
        return cls(readout=config['readout'],
                   qubit_mapping=config['qubit_mapping'],
                   circuit=circuit)

    def to_qonfig(self) -> 'Qonfig[PragmaGetDensityMatrix]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGetDensityMatrix]
        """
        config = Qonfig(self.__class__)
        config['readout'] = self._readout
        config['qubit_mapping'] = self._qubit_mapping
        if self._circuit is not None:
            config['circuit'] = self._circuit.to_qonfig()
        else:
            config['circuit'] = self._circuit

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
        if not self._circuit == other._circuit:
            return False
        return True

    def __copy__(self) -> 'PragmaGetDensityMatrix':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaGetDensityMatrix
        """
        self_copy = self.__class__()
        self_copy._readout = copy(self._readout)
        self_copy._circuit = copy(self._circuit)
        return self_copy

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        new_qubit_mapping: Optional[Dict[int, int]]
        if self._qubit_mapping is not None:
            new_qubit_mapping = dict()
            for key, val in self._qubit_mapping.items():
                new_qubit_mapping[mapping_dict[key]] = val
            self._qubit_mapping = new_qubit_mapping
            if self._circuit is not None:
                self._circuit.remap_qubits(mapping_dict=mapping_dict)

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        if self._qubit_mapping is not None:
            string += '('
            for key in self._qubit_mapping.keys():
                string += '{}:{},'.format(key, self._qubit_mapping[key])
            string += ')'
        string += ' {}'.format(self._readout)
        return string


class PragmaGetOccupationProbability(Pragma):
    r"""Implements the GetOccupationProbability PRAGMA.

    This PRAGMA operation returns the vector of the probabilities
    to find the quantum register in each :math:`\sigma^z` basis state.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma', 'PragmaGetOccupationProbability')
    _hqs_lang_name = 'PragmaGetOccupationProbability'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'qubit_mapping': {'doc': 'Mapping of qubits to indices in readout register',
                          'default': None},
    }

    def __init__(self,
                 readout: str = 'ro',
                 qubit_mapping: Optional[Dict[int, int]] = None
                 ) -> None:
        """Initialize 'get occupation probabilities' PRAGMA

        Args:
            readout: Name of the readout register
            qubit_mapping: Mapping of qubits to indices in readout register
        """
        self._readout = readout
        self._qubit_mapping = qubit_mapping
        self._involved_qubits = set(['ALL'])
        self._parametrized = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGetOccupationProbability']
                    ) -> 'PragmaGetOccupationProbability':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGetOccupationProbability
        """
        if config['qubit_mapping'] is not None:
            for key, _ in config['qubit_mapping'].items():
                config['qubit_mapping'][float(key)] = config['qubit_mapping'].pop(key)
        return cls(readout=config['readout'],
                   qubit_mapping=config['qubit_mapping'])

    def to_qonfig(self) -> 'Qonfig[PragmaGetOccupationProbability]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGetOccupationProbability]
        """
        config = Qonfig(self.__class__)
        config['readout'] = self._readout
        config['qubit_mapping'] = self._qubit_mapping

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
        return True

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        new_qubit_mapping: Optional[Dict[int, int]]
        if self._qubit_mapping is not None:
            new_qubit_mapping = dict()
            for key, val in self._qubit_mapping.items():
                new_qubit_mapping[mapping_dict[key]] = val
            self._qubit_mapping = new_qubit_mapping

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        if self._qubit_mapping is not None:
            string += '('
            for key in self._qubit_mapping.keys():
                string += '{}:{},'.format(key, self._qubit_mapping[key])
            string += ')'
        string += ' {}'.format(self._readout)
        return string


class PragmaGetRotatedOccupationProbability(PragmaGetOccupationProbability):
    r"""Implements the GetRotatedOccupationProbability PRAGMA.

    This PRAGMA operation returns the vector of the probabilities to find the quantum
    register in each basis state after applying a circuit to Rotate to another basis.
    It does not change the quantum register.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma',
                       'PragmaGetRotatedOccupationProbability')
    _hqs_lang_name = 'PragmaGetRotatedOccupationProbability'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'circuit': {'doc': 'The circuit used to rotate the qureg',
                    'default': None},
    }

    def __init__(self,
                 readout: str = 'ro',
                 circuit: Any = None
                 ) -> None:
        """Initialize 'get occupation probabilities' PRAGMA

        Args:
            readout: Name of the readout register
            circuit: The circuit used to rotate the qureg. Needs to be type annotated as Any
                     to avoid circular import but is (qoqo.Circuit).
        """
        self._readout = readout
        self._circuit = circuit
        self._parametrized = False
        self._involved_qubits = set(['ALL'])

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGetRotatedOccupationProbability']
                    ) -> 'PragmaGetRotatedOccupationProbability':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGetRotatedOccupationProbability
        """
        if config['circuit'] is not None:
            circuit = config['circuit'].to_instance()
        else:
            circuit = config['circuit']
        return cls(readout=config['readout'],
                   circuit=circuit)

    def to_qonfig(self) -> 'Qonfig[PragmaGetRotatedOccupationProbability]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGetRotatedOccupationProbability]
        """
        config = Qonfig(self.__class__)
        config['readout'] = self._readout
        if self._circuit is not None:
            config['circuit'] = self._circuit.to_qonfig()
        else:
            config['circuit'] = self._circuit

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
        if not (self._circuit
                == other._circuit):
            return False
        return True

    def __copy__(self) -> 'PragmaGetRotatedOccupationProbability':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaGetRotatedOccupationProbability
        """
        self_copy = self.__class__()
        self_copy._readout = copy(self._readout)
        self_copy._circuit = copy(self._circuit)
        return self_copy

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
        r"""Translate the operation to an hqs_lang dialect expression.

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += ' {}'.format(self._readout)
        return string


class PragmaGetPauliProduct(Pragma):
    r"""Implements the GetPauliProduct PRAGMA.

    This PRAGMA operation returns a Pauli product expectation value after applying
    a circuit to Rotate to another basis.
    It does not change the quantum register.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma',
                       'PragmaGetPauliProduct')
    _hqs_lang_name = 'PragmaGetPauliProduct'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'pauli_product': {'doc': 'Array of qubits involved in the PauliProduct i',
                          'default': 0},
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'circuit': {'doc': 'Optional Circuit applied on copy of quest register before measurement',
                    'default': None},
    }

    def __init__(self,
                 pauli_product: Sequence[int],
                 readout: str = 'ro',
                 circuit: Any = None
                 ) -> None:
        """Initialize 'get occupation probabilities' PRAGMA

        Args:
            readout: Name of the readout register
            pauli_product: Array of qubits involved in the PauliProduct i
                            1 -> Z, 0 -> Id
            circuit: The circuit used to rotate the qureg. Needs to be type annotated as Any
                     to avoid circular import but is (qoqo.Circuit).
        """
        self._readout = readout
        self._circuit = circuit
        self._pauli_product = pauli_product
        self._parametrized = False
        self._involved_qubits = set(['ALL'])

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaGetPauliProduct']
                    ) -> 'PragmaGetPauliProduct':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaGetPauliProduct
        """
        if config['circuit'] is not None:
            circuit = config['circuit'].to_instance()
        else:
            circuit = config['circuit']
        return cls(pauli_product=config['pauli_product'],
                   readout=config['readout'],
                   circuit=circuit)

    def to_qonfig(self) -> 'Qonfig[PragmaGetPauliProduct]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaGetPauliProduct]
        """
        config = Qonfig(self.__class__)
        config['pauli_product'] = self._pauli_product
        config['readout'] = self._readout
        if self._circuit is not None:
            config['circuit'] = self._circuit.to_qonfig()
        else:
            config['circuit'] = self._circuit

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
        if not np.array_equal(self._pauli_product, other._pauli_product):
            return False
        if not (self._circuit
                == other._circuit):
            return False
        return True

    def __copy__(self) -> 'PragmaGetPauliProduct':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaGetPauliProduct
        """
        self_copy = self.__class__(pauli_product=copy(self._pauli_product))
        self_copy._readout = copy(self._readout)
        self_copy._circuit = copy(self._circuit)
        return self_copy

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
        string += ' {}'.format(self._readout)
        return string


class PragmaRepeatedMeasurement(Pragma):
    """Implements the RepeatedMeasurement PRAGMA.

    This PRAGMA operation returns a measurement record for N repeated measurements.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma',
                       'PragmaRepeatedMeasurement')
    _hqs_lang_name = 'PragmaRepeatedMeasurement'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'qubit_mapping': {'doc': 'Mapping of qubits to indices in readout register',
                          'default': None},
        'number_measurements': {'doc': 'The number of measurements',
                                'default': None},
    }

    def __init__(self,
                 readout: str = 'ro',
                 qubit_mapping: Optional[Dict[int, int]] = None,
                 number_measurements: int = 1) -> None:
        """Initialize 'repeated measurement' PRAGMA

        Args:
            qubit_mapping: A dict mapping qubit index to read-out index,
                           one to one mapping is assumed for none
            readout: Name of the readout register
            number_measurements: The number of measurements
        """
        self._number_measurements = number_measurements
        self._qubit_mapping = qubit_mapping
        self._involved_qubits = set(['ALL'])
        self._parametrized = False
        self._readout = readout

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaRepeatedMeasurement']
                    ) -> 'PragmaRepeatedMeasurement':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaRepeatedMeasurement
        """
        if config['qubit_mapping'] is not None:
            for key, _ in list(config['qubit_mapping'].items()):
                config['qubit_mapping'][int(key)] = config['qubit_mapping'].pop(key)
        return cls(readout=config['readout'],
                   qubit_mapping=config['qubit_mapping'],
                   number_measurements=config['number_measurements'])

    def to_qonfig(self) -> 'Qonfig[PragmaRepeatedMeasurement]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaRepeatedMeasurement]
        """
        config = Qonfig(self.__class__)
        config['readout'] = self._readout
        config['qubit_mapping'] = self._qubit_mapping
        config['number_measurements'] = self._number_measurements

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
        if not (self._qubit_mapping
                == other._qubit_mapping):
            print('here')
            return False
        return True

    def __copy__(self) -> 'PragmaRepeatedMeasurement':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaRepeatedMeasurement
        """
        self_copy = self.__class__()
        self_copy._readout = copy(self._readout)
        self_copy._number_measurements = copy(self._number_measurements)
        self_copy._qubit_mapping = copy(self._qubit_mapping)
        return self_copy

    def remap_qubits(self,
                     mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the operation

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new qubit indices
        """
        new_qubit_mapping: Optional[Dict[int, int]]
        if self._qubit_mapping is not None:
            new_qubit_mapping = dict()
            for key, val in self._qubit_mapping.items():
                new_qubit_mapping[mapping_dict[key]] = val
            self._qubit_mapping = new_qubit_mapping

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
        if backend == 'aqt':
            backend_instruction = {'readout': self._readout}
            return backend_instruction
        else:
            return None

    def to_hqs_lang(self) -> str:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            str
        """
        string = self.get_hqs_lang_name()
        string += '({}) '.format(self._number_measurements)
        if self._qubit_mapping is None:
            string += 'ALL {}'.format(self._readout)
        else:
            for qubit in self._qubit_mapping:
                string += '{} {}[{}] '.format(
                    qubit, self._readout, self._qubit_mapping[qubit])
        return string


class PragmaPauliProdMeasurement(Pragma):
    """Implements the PauliProdMeasurement PRAGMA.

    This PRAGMA operation returns a measurement a single cheated measurement
    for a product of Pauli operators.

    """

    _operation_tags = ('Operation', 'Measurement', 'Pragma',
                       'PragmaPauliProdMeasurement')
    _hqs_lang_name = 'PragmaPauliProdMeasurement'

    _qonfig_never_receives_values = True

    _qonfig_defaults_dict = {
        'qubits': {'doc': 'Target qubits',
                   'default': None},
        'paulis': {'doc': 'List of Pauli operators in the product encoded as int via '
                          + 'IDENTITY=0, PAULI_X=1, PAULI_Y=2, PAULI_Z=3',
                   'default': None},
        'readout': {'doc': 'Name of the readout register',
                    'default': 'ro'},
        'readout_index': {'doc': 'Name of the readout register',
                          'default': None},
    }

    def __init__(self,
                 qubits: Optional[List[Union[int, str]]] = None,
                 paulis: Optional[List[int]] = None,
                 readout: str = 'ro',
                 readout_index: int = 0) -> None:
        """Initialize 'Pauli product measurement' PRAGMA

        Args:
            qubits: Target qubits
            paulis: List of Pauli operators in the product
                    encoded as int via IDENTITY=0, PAULI_X=1, PAULI_Y=2, PAULI_Z=3
            readout: Name of the readout register
            readout_index: The index in the readout
        """
        if qubits is None:
            qubits = cast(List[Union[int, str]], list())
        if paulis is None:
            paulis = cast(List[int], list())
        self._qubits = qubits
        self._involved_qubits = set(self._qubits)
        self._parametrized = False
        self._paulis = paulis
        self._readout = readout
        self._readout_index = readout_index

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PragmaPauliProdMeasurement']
                    ) -> 'PragmaPauliProdMeasurement':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PragmaPauliProdMeasurement
        """
        return cls(qubits=config['qubits'],
                   paulis=config['paulis'],
                   readout=config['readout'],
                   readout_index=config['readout_index'])

    def to_qonfig(self) -> 'Qonfig[PragmaPauliProdMeasurement]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PragmaPauliProdMeasurement]
        """
        config = Qonfig(self.__class__)
        config['qubits'] = self._qubits
        config['paulis'] = self._paulis
        config['readout'] = self._readout
        config['readout_index'] = self._readout_index

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
        if not self._paulis == other._paulis:
            return False
        if not self._qubits == other._qubits:
            return False
        if not (self._readout_index
                == other._readout_index):
            return False
        if not self._readout == other._readout:
            return False
        return True

    def __copy__(self) -> 'PragmaPauliProdMeasurement':
        """Return a shallow copy of the PRAGMA

        Creates a copy by copying the contents of the internal dicts

        Returns:
            PragmaPauliProdMeasurement
        """
        self_copy = self.__class__()
        self_copy._readout = copy(self._readout)
        self_copy._readout_index = copy(self._readout_index)
        self_copy._qubits = copy(self._qubits)
        self_copy._involved_qubits = self._involved_qubits
        self_copy._paulis = copy(self._paulis)
        return self_copy

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
        string += ' '
        for qubit, pauli in zip(self._qubits, self._paulis):
            string += '{}, {} '.format(
                qubit, pauli)
        string += '{}[{}]'.format(self._readout, self._readout_index)
        return string
