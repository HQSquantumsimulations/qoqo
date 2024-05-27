# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

from typing import Optional, List, Tuple, Dict, Set  # noqa: F401
from qoqo_quest import Backend

def qoqo():
    """
Quantum Operation Quantum Operation (qoqo)

Yes, we use reduplication.

qoqo is the HQS python package to represent quantum circuits.

.. autosummary::
    :toctree: generated/

    Circuit
    CircuitDag
    QuantumProgram
    operations
    measurements
    devices
    noise_models

"""

class Circuit:
    """
Circuit of Operations.

A quantum program is represented as a linear sequence of Operations.
"""

    def __init__(self):
       return

    @classmethod
    def __repr__(self): # type: ignore
        """
Return repr(self).
"""

    @classmethod
    def __lt__(self): # type: ignore
        """
Return self<value.
"""

    @classmethod
    def __le__(self): # type: ignore
        """
Return self<=value.
"""

    @classmethod
    def __eq__(self): # type: ignore
        """
Return self==value.
"""

    @classmethod
    def __ne__(self): # type: ignore
        """
Return self!=value.
"""

    @classmethod
    def __gt__(self): # type: ignore
        """
Return self>value.
"""

    @classmethod
    def __ge__(self): # type: ignore
        """
Return self>=value.
"""

    @classmethod
    def __iter__(self): # type: ignore
        """
Implement iter(self).
"""

    @classmethod
    def __add__(self, rhs: Operation | Circuit) -> Circuit: # type: ignore
        """
Implement the `+` (__add__) magic method to add two Circuits.

Args:
    rhs (Operation | Circuit): The second Circuit object in this operation.

Returns:
    Circuit: self + rhs the two Circuits added together.

    Raises:
    TypeError: Left hand side can not be converted to Circuit.
    TypeError: Right hand side cannot be converted to Operation or Circuit.
"""

    @classmethod
    def __radd__(self): # type: ignore
        """
Return value+self.
"""

    @classmethod
    def __iadd__(self, other: Operation | Circuit) -> Circuit: # type: ignore
        """
Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.

Args:
    other (Operation | Circuit): The Operation object to be added to self.

Returns:
    Circuit: self + other the two Circuits added together as the first one.

Raises:
    TypeError: Right hand side cannot be converted to Operation or Circuit.
"""

    @classmethod
    def __len__(self): # type: ignore
        """
Return len(self).
"""

    @classmethod
    def __getitem__(self): # type: ignore
        """
Return self[key].
"""

    @classmethod
    def __setitem__(self): # type: ignore
        """
Set self[key] to value.
"""

    @classmethod
    def __delitem__(self): # type: ignore
        """
Delete self[key].
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Circuit: # type: ignore
        """
Substitute the symbolic parameters in a clone of the Circuit according to the substitution_parameters input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

Returns:
    self: The Circuit with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Circuit: # type: ignore
        """
Remap qubits in operations in clone of Circuit.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the Circuit.

Returns:
    self: The Circuit with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def overrotate(self) -> Circuit: # type: ignore
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

    @classmethod
    def count_occurences(self, operations: List[str]) -> int: # type: ignore
        """
Count the number of occurences of a set of operation tags in the circuit.

Args:
    operations (List[str]): List of operation tags that should be counted.

Returns:
    int: The number of occurences of these operation tags.
"""

    @classmethod
    def get_operation_types(self) -> Set[str]: # type: ignore
        """
Return a list of the hqslang names of all operations occuring in the circuit.

Returns:
    Set[str]: The operation types in the Circuit.
"""

    @classmethod
    def __copy__(self) -> Circuit: # type: ignore
        """
Return a copy of the Circuit (copy here produces a deepcopy).

Returns:
    Circuit: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Circuit: # type: ignore
        """
Return a deep copy of the Circuit.

Returns:
    Circuit: A deep copy of self.
"""

    @classmethod
    def _qoqo_versions(self) -> Tuple[str, str]: # type: ignore
        """
Return the roqoqo and qoqo versions from when the code was compiled.

Returns:
    Tuple[str, str]: The roqoqo and qoqo versions.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Circuit using the [bincode] crate.

Returns:
    ByteArray: The serialized Circuit (in [bincode] form).

Raises:
    ValueError: Cannot serialize Circuit to bytes.
"""

    @classmethod
    def from_bincode(self): # type: ignore
        """
staticmethod(function) -> method

Convert a function to be a static method.

A static method does not receive an implicit first argument.
To declare a static method, use this idiom:

     class C:
         @staticmethod
         def f(arg1, arg2, argN):
             ...

It can be called either on the class (e.g. C.f()) or on an instance
(e.g. C().f()). Both the class and the instance are ignored, and
neither is passed implicitly as the first argument to the method.

Static methods in Python are similar to those found in Java or C++.
For a more advanced concept, see the classmethod builtin.
"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Circuit.

Returns:
    str: The serialized form of Circuit.

Raises:
    ValueError: Cannot serialize Circuit to json.
"""

    @classmethod
    def from_json(self): # type: ignore
        """
staticmethod(function) -> method

Convert a function to be a static method.

A static method does not receive an implicit first argument.
To declare a static method, use this idiom:

     class C:
         @staticmethod
         def f(arg1, arg2, argN):
             ...

It can be called either on the class (e.g. C.f()) or on an instance
(e.g. C().f()). Both the class and the instance are ignored, and
neither is passed implicitly as the first argument to the method.

Static methods in Python are similar to those found in Java or C++.
For a more advanced concept, see the classmethod builtin.
"""

    @classmethod
    def get(self, index: int) -> Operation: # type: ignore
        """
Return a copy of the Operation at a certain index of the Circuit.

Args:
    index (int): The index of the Operation to get in the Circuit.

Returns:
    Operation: The operation at the given index (if it exists).

Raises:
    IndexError: Index out of range.
"""

    @classmethod
    def get_slice(self, start: Optional[int], stop: Optional[int]) -> Circuit: # type: ignore
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
    IndexError: Start index out of range.
"""

    @classmethod
    def definitions(self) -> List[Operation]: # type: ignore
        """
Return a list of definitions in the Circuit.

Definitions need to be unique.

Returns:
    List[Operation]: A vector of the definitions in the Circuit.
"""

    @classmethod
    def operations(self) -> List[Operation]: # type: ignore
        """
Return a list of all operations in the Circuit.

Returns:
    List[Operation]: A vector of the operations in the Circuit.
"""

    @classmethod
    def filter_by_tag(self, tag: str) -> List[Operation]: # type: ignore
        """
Return a list of operations with given tag.

Args:
    tag (str): tag by which to filter operations.

Returns:
    List[Operation]: A vector of the operations with the specified tag in the Circuit.
"""

    @classmethod
    def add(self, op: Operation): # type: ignore
        """
Add an Operation to Circuit.

Args:
    op (Operation): The Operation to add to the Circuit.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the Circuit.

Returns:
    str: The string representation of the Circuit.
"""

    @classmethod
    def __doc__(self): # type: ignore
        """
str(object='') -> str
str(bytes_or_buffer[, encoding[, errors]]) -> str

Create a new string object from the given object. If encoding or
errors is specified, then the object must expose a data buffer
that will be decoded using the given encoding and error handler.
Otherwise, returns the result of object.__str__() (if defined)
or repr(object).
encoding defaults to sys.getdefaultencoding().
errors defaults to 'strict'.
"""

    @classmethod
    def __module__(self): # type: ignore
        """
str(object='') -> str
str(bytes_or_buffer[, encoding[, errors]]) -> str

Create a new string object from the given object. If encoding or
errors is specified, then the object must expose a data buffer
that will be decoded using the given encoding and error handler.
Otherwise, returns the result of object.__str__() (if defined)
or repr(object).
encoding defaults to sys.getdefaultencoding().
errors defaults to 'strict'.
"""

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

    @classmethod
    def __lt__(self): # type: ignore
        """
Return self<value.
"""

    @classmethod
    def __le__(self): # type: ignore
        """
Return self<=value.
"""

    @classmethod
    def __eq__(self): # type: ignore
        """
Return self==value.
"""

    @classmethod
    def __ne__(self): # type: ignore
        """
Return self!=value.
"""

    @classmethod
    def __gt__(self): # type: ignore
        """
Return self>value.
"""

    @classmethod
    def __ge__(self): # type: ignore
        """
Return self>=value.
"""

    @classmethod
    def measurement(self): # type: ignore
        """
Returns the measurement attribute of the QuantumProgram as Python object.

Returns:
    PyObject corresponding to the qoqo measurement type of the QuantumProgram,
    i.e. PauliZProduct, CheatedPauliZProduct, Cheated or ClassicalRegister.
"""

    @classmethod
    def input_parameter_names(self): # type: ignore
        """
Returns the input_parameter_names attribute of the qoqo QuantumProgram.

Returns:
    List of input parameter names.
"""

    @classmethod
    def run(self, backend: Backend, parameters: Optional[List[float]]): # type: ignore
        """
Runs the QuantumProgram and returns expectation values.

Runs the quantum programm for a given set of parameters passed in the same order as the parameters
listed in `input_parameter_names` and returns expectation values.

Args:
    backend (Backend): The backend the program is executed on.
    parameters (Optional[List[float]]): List of float  parameters of the function call in order of `input_parameter_names`
"""

    @classmethod
    def run_registers(self, backend: Backend, parameters: Optional[List[float]]): # type: ignore
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

    @classmethod
    def __copy__(self) -> QuantumProgram: # type: ignore
        """
Return a copy of the QuantumProgram (copy here produces a deepcopy).

Returns:
    QuantumProgram: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> QuantumProgram: # type: ignore
        """
Return a deep copy of the QuantumProgram.

Returns:
    QuantumProgram: A deep copy of self.
"""

    @classmethod
    def _qoqo_versions(self) -> Tuple[str, str]: # type: ignore
        """
Return the roqoqo and qoqo versions from when the code was compiled.

Returns:
    Tuple[str, str]: The roqoqo and qoqo versions.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the QuantumProgram using the [bincode] crate.

Returns:
    ByteArray: The serialized QuantumProgram (in [bincode] form).

Raises:
    ValueError: Cannot serialize QuantumProgram to bytes.
"""

    @classmethod
    def from_bincode(self): # type: ignore
        """
staticmethod(function) -> method

Convert a function to be a static method.

A static method does not receive an implicit first argument.
To declare a static method, use this idiom:

     class C:
         @staticmethod
         def f(arg1, arg2, argN):
             ...

It can be called either on the class (e.g. C.f()) or on an instance
(e.g. C().f()). Both the class and the instance are ignored, and
neither is passed implicitly as the first argument to the method.

Static methods in Python are similar to those found in Java or C++.
For a more advanced concept, see the classmethod builtin.
"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the QuantumProgram.

Returns:
    str: The serialized form of QuantumProgram.

Raises:
    ValueError: Cannot serialize QuantumProgram to json.
"""

    @classmethod
    def from_json(self): # type: ignore
        """
staticmethod(function) -> method

Convert a function to be a static method.

A static method does not receive an implicit first argument.
To declare a static method, use this idiom:

     class C:
         @staticmethod
         def f(arg1, arg2, argN):
             ...

It can be called either on the class (e.g. C.f()) or on an instance
(e.g. C().f()). Both the class and the instance are ignored, and
neither is passed implicitly as the first argument to the method.

Static methods in Python are similar to those found in Java or C++.
For a more advanced concept, see the classmethod builtin.
"""

    @classmethod
    def __doc__(self): # type: ignore
        """
str(object='') -> str
str(bytes_or_buffer[, encoding[, errors]]) -> str

Create a new string object from the given object. If encoding or
errors is specified, then the object must expose a data buffer
that will be decoded using the given encoding and error handler.
Otherwise, returns the result of object.__str__() (if defined)
or repr(object).
encoding defaults to sys.getdefaultencoding().
errors defaults to 'strict'.
"""

    @classmethod
    def __module__(self): # type: ignore
        """
str(object='') -> str
str(bytes_or_buffer[, encoding[, errors]]) -> str

Create a new string object from the given object. If encoding or
errors is specified, then the object must expose a data buffer
that will be decoded using the given encoding and error handler.
Otherwise, returns the result of object.__str__() (if defined)
or repr(object).
encoding defaults to sys.getdefaultencoding().
errors defaults to 'strict'.
"""

class Operation:
    """
Operations are the atomic instructions in any quantum program that can be represented by qoqo.

Operations can be of various kinds: Definitions, GateOperations, PRAGMAs or measurement Operations.

Operations:
    Definition operations define the classical registers and variables in the Circuit.
    GateOperations are single-, two- or multi-qubit gate operations that act on a set of qubits
    and can be executed on a quantum computing device.
    PRAGMAs are operations that can be used when running a simulation of a quantum computing program.
    Measurement Operations are operations that perform a measurement either on a quantum computing device (MeasuareQubit)
    or on a simulation of a quantum computing program (PRAGMA measurement operations).

"""

    def __init__(self):
       return

