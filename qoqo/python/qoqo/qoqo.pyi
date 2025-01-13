# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

"""
Qoqo

| Quantum Operation Quantum Operation
| Yes we use `reduplication <https://en.wikipedia.org/wiki/Reduplication>`_.

qoqo is the HQS python package to represent quantum circuits.

.. autosummary::
    :toctree: generated/

    Circuit
    CircuitDag
    operations
    QuantumProgram
    measurements
    devices
    noise_models

"""

from typing import Optional, List, Tuple, Dict, Set

class Circuit:
    """
    Circuit of Operations.

    A quantum program is represented as a linear sequence of Operations."""

    def __init__(self):
        return
    def __add__(self, rhs: Operation | Circuit) -> Circuit:
        """
        Implement the `+` (__add__) magic method to add two Circuits.

        Args:
            rhs (Operation | Circuit): The second Circuit object in this operation.

        Returns:
            Circuit: self + rhs the two Circuits added together.

            Raises:
            TypeError: Left hand side can not be converted to Circuit.
            TypeError: Right hand side cannot be converted to Operation or Circuit."""
    def __iadd__(self, other: Operation | Circuit) -> Circuit:
        """
        Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.

        Args:
            other (Operation | Circuit): The Operation object to be added to self.

        Returns:
            Circuit: self + other the two Circuits added together as the first one.

        Raises:
            TypeError: Right hand side cannot be converted to Operation or Circuit."""
    def substitute_parameters(
        self, substitution_parameters: Dict[str, float]
    ) -> Circuit:
        """
        Substitute the symbolic parameters in a clone of the Circuit according to the substitution_parameters input.

        Args:
            substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

        Returns:
            self: The Circuit with the parameters substituted.

        Raises:
            RuntimeError: The parameter substitution failed."""
    def remap_qubits(self, mapping: Dict[int, int]) -> Circuit:
        """
        Remap qubits in operations in clone of Circuit.

        Args:
            mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the Circuit.

        Returns:
            self: The Circuit with the qubits remapped.

        Raises:
            RuntimeError: The qubit remapping failed."""
    def overrotate(self) -> Circuit:
        """
        Return clone of the circuit with all overrotation Pragmas applied.

        Returns:
            Circuit: Circuit with the overrotation applied

        Raises:
            RuntimeError: Error applying PragmaOverrotation in circuit.
        Example:

        >>> circuit = Circuit()
        >>> circuit += PragmaOverrotation("RotateY", [1,], 20.0, 30.0)
        >>> circuit += RotateX(0, 0.0)
        >>> circuit += RotateY(0, 1.0)
        >>> circuit += RotateY(1, 2.0)
        >>> circuit += RotateY(1, 3.0)
        >>> circuit_overrotated = circuit.overrotate()
        print(circuit)
        print(circuit_overrotated)
        """
    def count_occurences(self, operations: List[str]) -> int:
        """
        Count the number of occurences of a set of operation tags in the circuit.

        Args:
            operations (List[str]): List of operation tags that should be counted.

        Returns:
            int: The number of occurences of these operation tags."""
    def get_operation_types(self) -> Set[str]:
        """
        Return a list of the hqslang names of all operations occuring in the circuit.

        Returns:
            Set[str]: The operation types in the Circuit."""
    def _qoqo_versions(self) -> Tuple[str, str]:
        """
        Return the roqoqo and qoqo versions from when the code was compiled.

        Returns:
            Tuple[str, str]: The roqoqo and qoqo versions."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Circuit using the [bincode] crate.

        Returns:
            ByteArray: The serialized Circuit (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Circuit to bytes."""
    def from_bincode(self, input: bytearray) -> Circuit:
        """
        Convert the bincode representation of the Circuit to a Circuit using the [bincode] crate.

        Args:
            input (ByteArray): The serialized Circuit (in [bincode] form).

        Returns:
            Circuit: The deserialized Circuit.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to Circuit."""
    def to_json(self) -> str:
        """
        Return the json representation of the Circuit.

        Returns:
            str: The serialized form of Circuit.

        Raises:
            ValueError: Cannot serialize Circuit to json."""
    def json_schema(self) -> str:
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json"""
    def current_version(self) -> str:
        """
        Returns the current version of the qoqo library .

        Returns:
            str: The current version of the library."""
    def min_supported_version(self) -> str:
        """
        Return the minimum version of qoqo that supports this object.

        Returns:
            str: The minimum version of the qoqo library to deserialize this object."""
    def from_json(self, input: str) -> Circuit:
        """
        Convert the json representation of a Circuit to a Circuit.

        Args:
            input (str): The serialized Circuit in json form.

        Returns:
            Circuit: The deserialized Circuit.

        Raises:
            ValueError: Input cannot be deserialized to Circuit."""
    def get(self, index: int) -> Operation:
        """
        Return a copy of the Operation at a certain index of the Circuit.

        Args:
            index (int): The index of the Operation to get in the Circuit.

        Returns:
            Operation: The operation at the given index (if it exists).

        Raises:
            IndexError: Index out of range."""
    def get_slice(self, start: Optional[int], stop: Optional[int]) -> Circuit:
        """
        Return the copy of a slice of the Circuit.

        Args:
            start (Optional[int]): The starting index of the slice (inclusive).
            stop (Optional[int]): The stopping index of the slice (exclusive).

        Returns:
            Circuit: The slice of the operations in the Circuit with the specified indices.

        Raises:
            IndexError: Stop index smaller than start index.
            IndexError: Stop index out of range.
            IndexError: Start index out of range."""
    def definitions(self) -> List[Operation]:
        """
        Return a list of definitions in the Circuit.

        Definitions need to be unique.

        Returns:
            List[Operation]: A vector of the definitions in the Circuit."""
    def operations(self) -> List[Operation]:
        """
        Return a list of all operations in the Circuit.

        Returns:
            List[Operation]: A vector of the operations in the Circuit."""
    def filter_by_tag(self, tag: str) -> List[Operation]:
        """
        Return a list of operations with given tag.

        Args:
            tag (str): tag by which to filter operations.

        Returns:
            List[Operation]: A vector of the operations with the specified tag in the Circuit.
        """
    def add(self, op: Operation):
        """
        Add an Operation to Circuit.

        Args:
            op (Operation): The Operation to add to the Circuit."""

class QuantumProgram:
    """
    Represents a quantum program evaluating measurements based on a one or more free float parameters.

    The main use of QuantumProgram is to contain a Measurements implementing [crate::measurements::Measure]
    that measures expectation values or output registers of [crate::Circuit] quantum circuits that contain
    symbolic parameters. Circuit with symbolic parameters can not be simulated or executed on real hardware.
    The symbolic parameters need to be replaced with real floating point numbers first.
    A QuantumProgram contains a list of the free parameters (`input_parameter_names`) and can automatically
    replace the parameters with its `run` methods and return the result.

    The QuantumProgram should correspond as closely as possible to a normal multi-parameter function
    in classical computing that can be called with a set of parameters and returns a result.
    It is the intended way to interface between normal program code and roqoqo based quantum programs.
    """

    def __init__(self):
        return
    def measurement(self):
        """
        Returns the measurement attribute of the QuantumProgram as Python object.

        Returns:
            PyObject corresponding to the qoqo measurement type of the QuantumProgram,
            i.e. PauliZProduct, CheatedPauliZProduct, Cheated or ClassicalRegister."""
    def input_parameter_names(self):
        """
        Returns the input_parameter_names attribute of the qoqo QuantumProgram.

        Returns:
            List of input parameter names."""
    def run(self, backend: Backend, parameters: Optional[List[float]]):
        """
        Runs the QuantumProgram and returns expectation values.

        Runs the quantum programm for a given set of parameters passed in the same order as the parameters
        listed in `input_parameter_names` and returns expectation values.

        Args:
            backend (Backend): The backend the program is executed on.
            parameters (Optional[List[float]]): List of float  parameters of the function call in order of `input_parameter_names`
        """
    def run_registers(self, backend: Backend, parameters: Optional[List[float]]):
        """
        Runs the QuantumProgram and returns the classical registers of the quantum program.

        Runs the quantum programm for a given set of parameters passed in the same order as the parameters
        listed in `input_parameter_names` and returns the classical register output.
        The classical registers usually contain a record of measurement values for the repeated execution
        of a [crate::Circuit] quantum circuit for real quantum hardware
        or the readout of the statevector or the density matrix for simulators.

        Args:
            backend (Backend): The backend the program is executed on.
            parameters (Optional[List[float]]): List of float  parameters of the function call in order of `input_parameter_names`
        """
    def _qoqo_versions(self) -> Tuple[str, str]:
        """
        Return the roqoqo and qoqo versions from when the code was compiled.

        Returns:
            Tuple[str, str]: The roqoqo and qoqo versions."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the QuantumProgram using the [bincode] crate.

        Returns:
            ByteArray: The serialized QuantumProgram (in [bincode] form).

        Raises:
            ValueError: Cannot serialize QuantumProgram to bytes."""
    def from_bincode(self, input: bytearray) -> QuantumProgram:
        """
        Convert the bincode representation of the QuantumProgram to a QuantumProgram using the [bincode] crate.

        Args:
            input (ByteArray): The serialized QuantumProgram (in [bincode] form).

        Returns:
            QuantumProgram: The deserialized QuantumProgram.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to QuantumProgram."""
    def to_json(self) -> str:
        """
        Return the json representation of the QuantumProgram.

        Returns:
            str: The serialized form of QuantumProgram.

        Raises:
            ValueError: Cannot serialize QuantumProgram to json."""
    def from_json(self, input: str) -> QuantumProgram:
        """
        Convert the json representation of a QuantumProgram to a QuantumProgram.

        Args:
            input (str): The serialized QuantumProgram in json form.

        Returns:
            QuantumProgram: The deserialized QuantumProgram.

        Raises:
            ValueError: Input cannot be deserialized to QuantumProgram."""
    def json_schema(self) -> str:
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json"""
    def current_version(self) -> str:
        """
        Returns the current version of the qoqo library .

        Returns:
            str: The current version of the library."""
    def min_supported_version(self) -> str:
        """
        Return the minimum version of qoqo that supports this object.

        Returns:
            str: The minimum version of the qoqo library to deserialize this object."""

class CircuitDag:
    """
    Represents the Direct Acyclic Graph (DAG) of a Circuit.
    """

    def __init__(self):
        return
    def from_circuit(self, circuit: Circuit) -> CircuitDag:
        """
        Create a CircuitDag from a given Circuit;

        Args:
            circuit (Circuit): The Circuit to build the new CircuitDag from.

        Returns:
            self: The new CircuitDag."""
    def to_circuit(self):
        """
        Transforms the CircuitDag into a Circuit.
        """
    def add_to_back(self, op: Operation):
        """
        Add an Operation to the back of the CircuitDag, if necessary.

        Args:
            op (Operation): The Operation to add to the back of the CircuitDag.

        Raises:
            TypeError: The Python Object cannot be converted to Operation."""
    def add_to_front(self, op: Operation):
        """
        Add an Operation to the front of the CircuitDag, if necessary.

        Args:
            op (Operation): The Operation to add to the front of the CircuitDag.

        Raises:
            TypeError: The Python Object cannot be converted to Operation."""
    def execution_blocked(
        self, already_executed: List[int], to_be_executed: int
    ) -> List[int]:
        """
        Checks if executing an operation is blocked by any not-yet executed operation.

        Args:
            already_executed (List[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
            to_be_executed (int): NodeIndex of the operation that should be executed next.

        Returns:
            List[int]: List containing the sorted blocking elements."""
    def blocking_predecessors(
        self, already_executed: List[int], to_be_executed: int
    ) -> List[int]:
        """
        Checks which of the direct predecessors of an Operation in the CircuitDag blocks the execution.

        Warning:
        This method can only be used to determine if an operation can be executed when `already_executed` is consistent.
        When the list  `already_executed` is inconsistent (a n operation is reported as executed that could not have been executed yet)
        this method returning an empty vector does not imply that the `to_be_executed` operation can be executed.

        Args:
            already_executed (List[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
            to_be_executed (int): NodeIndex of the Operation that should be executed next.

        Returns:
            List[int]: List containing the sorted blocking elements."""
    def new_front_layer(
        self,
        already_executed: List[int],
        current_front_layer: List[int],
        to_be_executed: int,
    ):
        """
        Returns a new front-layer after executing an operation from the current front layer.

        Returns an error if operation to be executed is not in the current front layer.

        Args:
            already_executed (List[int]): List of NodeIndices of Nodes that have already been executed in the Circuit.
            current_front_layer (List[int]): List of NodeIndices in the current front layer ready to be executed if physically possible.
            to_be_executed (int): NodeIndex of the operation that should be executed next.
        """
    def parallel_blocks(self):
        """
        Returns an iterator over the possible parallel blocks in circuit that can be executed simultaneously

        Returns an Iterator over Vectors of references to the NodeIndices in the parallel block as well
        as references to the Operation in the blocks"""
    def get(self, index: int) -> Operation:
        """
        Given a NodeIndex, returns the Operation contained in the node of
        the CircuitDag.

        Args:
            index (int): The index of the node to get from the CircuitDag.

        Returns:
            Operation: The Operation at the given index (if it exists).

        Raises:
            IndexError: Index out of range."""
    def _qoqo_versions(self) -> Tuple[str, str]:
        """
        Return the roqoqo and qoqo versions from when the code was compiled.

        Returns:
            Tuple[str, str]: The roqoqo and qoqo versions."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the CircuitDag using the [bincode] crate.

        Returns:
            ByteArray: The serialized CircuitDag (in [bincode] form).

        Raises:
            ValueError: Cannot serialize CircuitDag to bytes."""
    def from_bincode(self, input: bytearray) -> CircuitDag:
        """
        Convert the bincode representation of the CircuitDag to a CircuitDag using the [bincode] crate.

        Args:
            input (ByteArray): The serialized CircuitDag (in [bincode] form).

        Returns:
            CircuitDag: The deserialized CircuitDag.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to CircuitDag."""
    def successors(self):
        """
        Returns the list of the successors of a given node in the CircuitDag.
        """
    def commuting_operations(self) -> List[int]:
        """
        Returns the list of nodes of commuting operations in CircuitDag.

        Returns:
            List[int]: The list of nodes of commuting operations."""
    def first_parallel_block(self) -> Set[int]:
        """
        Returns a set containing the nodes in the first parallel block.

        Returns:
            Set[int]: The set of nodes in the first parallel block."""
    def last_parallel_block(self) -> Set[int]:
        """
        Returns a set containing the nodes in the last parallel block.

        Returns:
            Set[int]: The set of nodes in the last parallel block."""
    def first_operation_involving_qubit(self) -> Dict[int, int]:
        """
        Returns a dictionary where a key represents a qubit and its value represents
        the first node that involves that qubit.

        Returns:
            Dict[int, int]: The dictionary of {qubit: node} elements."""
    def last_operation_involving_qubit(self) -> Dict[int, int]:
        """
        Returns a dictionary where a key represents a qubit and its value represents
        the last node that involves that qubit.

        Returns:
            Dict[int, int]: The dictionary of {qubit: node} elements."""
    def first_operation_involving_classical(self) -> Dict[(str, int), int]:
        """
        Returns a dictionary where a key is composed by the name and the size
        of the classical register and its value represents the first node that involves that
        register.

        Returns:
            Dict[(str, int), int]: The dictionary of {(str, int), int} elements."""
    def last_operation_involving_classical(self) -> Dict[(str, int), int]:
        """
        Returns a dictionary where a key is composed by the name and the size
        of the classical register and its value represents the last node that involves that
        register.

        Returns:
            Dict[(str, int), int]: The dictionary of {(str, int), int} elements."""

class Operation:
    """
    Operations are the atomic instructions in any quantum program that can be represented by qoqo.

    Operations can be of various kinds: Definitions, GateOperations, PRAGMAs or measurement Operations.

    Operations:
        Definition operations define the classical registers and variables in the Circuit.
        GateOperations are single-, two- or multi-qubit gate operations that act on a set of qubits
        and can be executed on a quantum computing device.
        PRAGMAs are operations that can be used when running a simulation of a quantum computing program.
        Measurement Operations are operations that perform a measurement either on a quantum computing device (MeasureQubit)
        or on a simulation of a quantum computing program (PRAGMA measurement operations).
    """

    def __init__(self):
        return

class Backend:
    """
    Can be any backend from a qoqo interface such as qoqo-qiskit, qoqo-quest or qoqo-qasm.
    """
