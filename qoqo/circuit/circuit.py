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
"""Circuit class for qoqo"""

from qoqo.operations._operations_base_classes import (
    Operation,
)
from copy import copy
from qoqo.operations.define_operations import Definition
from typing import (
    Sequence,
    Optional,
    List,
    Iterable,
    Union,
    Iterator,
    Dict,
    cast,
)
from hqsbase.qonfig import Qonfig


class Circuit(object):
    """Represents quantum circuits in qoqo.

    In qoqo, single operations are collected in a circuit to build up a quantum program.
    Qoqo circuits are strictly linear sequences of operations.
    The circuit class is closely related to Python lists and provides several standard
    functions of the Python Sequence class, such as iterators, getitem, delitem and length.

    Similarly to single operations, circuits can be translated to other frameworks via interfaces.

    For circuits, the following properties are defined:
        - to_/from_qonfig: circuits can be serialised with the HQS Qonfig package
        - is_parametrized: used in circuits with varying parameters, defined with symbolic
                           paramters. Goes hand in hand with the "substitute_parameters" function.

    Additionally, the following magic methods are defined:
        - getitem, setitem, delitem
        - length, equal, iterator, string
        - add, iadd
        - copy, deepcopy

    Lastly, the following functions can be called:
        - substitute_parameters: substitutes symbolic parameters in the circuit
        - get_operation_types: returns a list of all operation types in circuit
        - remap qubits: remaps the qubits in the operation according to a dictionary
        - to_hqs_lang: translates the operation to hqs_lang dialect

    """

    # Defining the methods Circuit needs to provide

    _qonfig_defaults_dict = {
        'definitions': {'doc': 'List of definitions in the circuit',
                        'default': None},
        'operations': {'doc': 'List of operations in the circuit',
                       'default': None},
    }
    _qonfig_never_receives_values = True

    def __init__(self) -> None:
        """Initialize qoqo circuit"""
        self._operations: List[Operation]
        self._operations = list()
        self._definitions: List[Definition]
        self._definitions = list()

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['Circuit']
                    ) -> 'Circuit':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            Circuit
        """
        circuit = cls()
        for definition in config['definitions']:
            circuit += definition.to_instance()
        for operation in config['operations']:
            circuit += operation.to_instance()
        return circuit

    def to_qonfig(self) -> 'Qonfig[Circuit]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[Circuit]
        """
        config = Qonfig(self.__class__)
        definitions_list = []
        for definition in self._definitions:
            definitions_list.append(definition.to_qonfig())
        operations_list = []
        for operation in self._operations:
            operations_list.append(operation.to_qonfig())
        config['definitions'] = definitions_list
        config['operations'] = operations_list

        return config

    def __getitem__(self, keyslice: Union[int, slice]
                    ) -> Union[Operation, List[Operation]]:
        """Get a specific operation from the circuit

        Args:
            keyslice: Index of the operation (can be slice object)

        Returns:
            Union[Operation, List[Operation]]
        """
        if isinstance(keyslice, int):
            key = cast(int, keyslice)
            if key < len(self._definitions):
                return (self._definitions[key])
            return_key = key - len(self._definitions)
            return (self._operations[return_key])
        elif isinstance(keyslice, slice):
            step = 1 if keyslice.step is None else keyslice.step
            if step < 0:
                start = keyslice.start if keyslice.start is not None else self.__len__() - 1
                stop = keyslice.stop if keyslice.stop is not None else -1
            else:
                start = keyslice.start if keyslice.start is not None else 0
                stop = keyslice.stop if keyslice.stop is not None else self.__len__()
        return_list: List[Operation] = list()
        for key in range(start, stop, step):
            if key < len(self._definitions):
                return_list.append(self._definitions[key])
            else:
                return_key = (key - len(self._definitions))
                return_list.append(self._operations[return_key])
        return return_list

    def __delitem__(self, keyslice: Union[int, slice]) -> None:
        """Delete a specific operation from the circuit

        Args:
            keyslice: Index of operation can be slice
        """
        if isinstance(keyslice, int):
            key = cast(int, keyslice)
            if key < len(self._definitions):
                del self._definitions[key]
            else:
                del_key = (key - len(self._definitions))
                del self._operations[del_key]
        elif isinstance(keyslice, slice):
            start = keyslice.start
            stop = keyslice.stop
            step = 1 if keyslice.step is None else keyslice.step
            for key in range(start, stop, step):
                if key < len(self._definitions):
                    del self._definitions[key]
                else:
                    del_key = (key - len(self._definitions))
                    del self._operations[del_key]

    def __setitem__(self, key: Union[int, slice],
                    val: Operation) -> None:
        """Set a specific operation in the circuit

        Args:
            key: Index of operation
            val: Set value

        Raises:
            KeyError: Trying to set Definition with no corresponding definition index
        """
        if isinstance(key, int):
            if isinstance(val, (Definition, type(None))) and key < len(self._definitions):
                self._definitions[key] = val
            elif key < len(self._definitions):
                raise KeyError(
                    'Trying to set Definition with no corresponding definition index {}'.format(
                        key))
            elif key >= len(self._definitions):
                set_key = (key - len(self._definitions))
                self._operations[set_key] = val

    def __len__(self) -> int:
        """Return the length of the circuit

        Returns:
            int
        """
        return (len(self._definitions) + len(self._operations))

    def __eq__(self, other: object) -> bool:
        """Compare the operation with other Python object and returns True when equal

        Args:
            other: object circuit is compared to

        Returns:
            bool
        """
        if not isinstance(other, Circuit):
            return False
        return (self._operations == other._operations
                and self._definitions == other._definitions)

    def __iter__(self) -> Iterator[Operation]:
        """Return an iterator over circuit

        Yields:
            Operation
        """
        yield from self._definitions
        yield from self._operations

    def __str__(self) -> str:
        """Return string representation of circuit

        Returns:
            str
        """
        string = ''
        for op in self._definitions:
            string += (op.__repr__() + '\n')
        for oop in self._operations:
            string += (oop.__repr__() + '\n')
        return string

    def __iadd__(self,
                 other: Optional[Union['Circuit',
                                       Operation,
                                       Iterable[Operation]]]
                 ) -> 'Circuit':
        """Add an operation or other Circuit to the Circuit

        Args:
            other: Operations that are added to the circuit

        Returns:
            Circuit

        Raises:
            TypeError: Only other Circuits and operations can be added to Circuit
        """
        if self is other:
            other = copy(other)
        if isinstance(other, Circuit):
            for op in other._definitions:
                self._append_operation(op)
            self._operations.extend(other._operations)
        elif isinstance(other, Operation):
            self._append_operation(other)
        elif hasattr(other, '__iter__'):
            other = cast(Iterable, other)
            try:
                other = cast(Iterable[Operation], other)
                for oop in other:
                    self._append_operation(oop)
            except Exception:
                raise TypeError("Only other Circuits and operations can be added to Circuit")
        elif other is None:
            pass
        else:
            raise TypeError("Only other Circuits and operations can be added to Circuit")
        return self

    def __add__(self,
                other: Optional[Union['Circuit',
                                      Operation,
                                      Iterable[Operation]]],
                ) -> 'Circuit':
        """Add two Circuits or a Circuit and an Operation

        Args:
            other: Operations that are added to the circuit

        Returns:
            Circuit

        Raises:
            TypeError: Only other Circuits and operations can be added to Circuit
        """
        new_circuit = Circuit()
        for op in self:
            new_circuit._append_operation(op)
        if isinstance(other, Circuit):
            for op in other._definitions:
                new_circuit._append_operation(op)
            new_circuit._operations.extend(other._operations)
        elif isinstance(other, Operation):
            new_circuit._append_operation(other)
        elif hasattr(other, '__iter__'):
            other = cast(Iterable, other)
            try:
                other = cast(Iterable[Operation], other)
                for oop in other:
                    new_circuit._append_operation(oop)
            except Exception:
                raise TypeError("Only other Circuits and operations can be added to Circuit")
        elif other is None:
            pass
        else:
            raise TypeError("Only other Circuits and operations can be added to Circuit")
        return new_circuit

    def insert(self, index: int, other: Operation) -> None:
        """Implement insert_operation on the circuit for high performace insertion operations

        Args:
            index: index where the operation is inserted
            other: Inserted operation

        Raises:
            TypeError: Only operations or sequences of operations can be added to circuits
        """
        if isinstance(other, Operation):
            self._insert_operation(index - len(self._definitions), other)
        elif (hasattr(other, '__iter__')
              and all([isinstance(op, Operation) for op in other])):
            for op in reversed(list(other)):
                self._insert_operation(index - len(self._definitions), op)
        else:
            raise TypeError(
                "Only operations or iterables of operations can be added to circuits")

    def count_occurences(
            self,
            operations_list: Optional[Sequence[type]] = None) -> int:
        """Count the number of occurences of a set of Operations in the circuit

         Args:
            operations_list: List of operations that should be counted.
                             Default is Operation so that all Operations are counted

        Returns:
            int
        """
        if operations_list is None:
            operations_list = [Operation]
        number_occurences = 0
        for dop in self._definitions:
            if any([isinstance(dop, counted_ops) for counted_ops in operations_list]):
                number_occurences += 1
        for op in self._operations:
            if any([isinstance(op, counted_ops) for counted_ops in operations_list]):
                number_occurences += 1
        return number_occurences

    def __copy__(self) -> 'Circuit':
        """Return a shallow copy of the circuit

        For a somewhat reasonable behaviour even a shallow copy needs to be a
        'deepcopy for the first level' Or more precisely the _operations attribute
        of the copied list has to be a copy of the original _operations attribute

        Returns:
            Circuit
        """
        return_circuit = self.__class__()
        return_circuit._operations = copy(self._operations)
        return_circuit._definitions = copy(self._definitions)
        return return_circuit

    def __deepcopy__(self, memodict: Optional[dict] = None) -> 'Circuit':
        """Return deep copy of circuit

        Args:
            memodict: memodict required by deepcopy operations

        Returns:
            Circuit
        """
        if memodict is None:
            memodict = dict()
        return_circuit = self.__class__()
        operations_new = list()
        for op in self._operations:
            operations_new.append(copy(op))
        definitions_new = list()
        for op in self._definitions:
            definitions_new.append(copy(op))
        return_circuit._operations = operations_new
        return_circuit._definitions = definitions_new
        return return_circuit

    def _append_operation(self, operation: Operation) -> None:
        """Append an operation to the circuit

        Appends an operation while respecting qoqo convention, adding definitions to the start

        Args:
            operation: appended operation

        Raises:
            TypeError: Circuit can only contain Operations
        """
        if isinstance(operation, Definition):
            if operation not in self._definitions:
                self._definitions.append(operation)
        elif isinstance(operation, Operation):
            self._operations.append(operation)
        else:
            raise TypeError('Circuit can only contain Operations')

    def _insert_operation(self, index: int, operation: Operation) -> None:
        """Insert an operation in the circuit

        Inserts an operation while respecting qoqo convention, adding definitions to the start

        Args:
            index: Index of insertion in self._operations
            operation: appended operation

        Raises:
            TypeError: Circuit can only contain Operations
        """
        if not isinstance(operation, Operation):
            raise TypeError('Circuit can only contain Operations')
        if isinstance(operation, Definition):
            if operation not in self._definitions:
                self._definitions.append(operation)
        else:
            self._operations.insert(index, operation)

    @property
    def is_parametrized(self) -> bool:
        """Return True if the circuit has operations with symbolic parameters

        Returns:
            bool
        """
        return any([op.is_parametrized for op in self._operations])

    def substitute_parameters(self, substitution_dict: Dict[str, float]
                              ) -> None:
        r"""Substitute the symbolic parameters in the circuit.

        Substitutes the symbols in the parameters and qubits of the gate
        according to a provided substitution_dict

        Args:
            substitution_dict: Dict of the form {'name': new_value}.
                 Where 'name' is the name of the symbol to be substituted
                 and new_value is the substituted value (can be another symbol)
        """
        for op in self._definitions:
            op.substitute_parameters(substitution_dict)
        for oop in self._operations:
            oop.substitute_parameters(substitution_dict)

    def get_operation_types(self, gates_only: bool = True) -> List[str]:
        """Return a list of all operation types in circuit

        Args:
            gates_only: Only return unitary gates (True) or all operations (False)

        Returns:
            List[str]
        """
        ll: List[str] = []
        for op in self._definitions:
            if not gates_only or isinstance(op, Operation):
                name = op.__class__.__name__
                if name not in ll:
                    ll.append(name)
        for oop in self._operations:
            if not gates_only or isinstance(oop, Operation):
                name = oop.__class__.__name__
                if name not in ll:
                    ll.append(name)
        return ll

    def remap_qubits(
            self,
            mapping_dict: Dict[int, int]) -> None:
        r"""Remap the qubits in the circuit

        Args:
            mapping_dict: Dict containing mapping old qubit indices to new
                          qubit indices
        """
        for op in self._operations:
            op.remap_qubits(mapping_dict)

    def to_hqs_lang(self) -> List[str]:
        r"""Translate the operation to an hqs_lang dialect expression

        Returns:
            List[str]
        """
        lines = list()
        for op in self._definitions:
            lines.append(op.to_hqs_lang())
        for oop in self._operations:
            lines.append(oop.to_hqs_lang())
        return lines
