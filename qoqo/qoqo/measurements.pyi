# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

from .qoqo import Circuit, Operation  # noqa: F401
import numpy as np  # noqa: F401
from typing import Tuple, List, Optional, Set, Dict, Union, Self, Sequence  # noqa: F401

class PauliZProductInput:
    """
Provides Necessary Information to run a [roqoqo::measurements::PauliZProduct] measurement.

The PauliZProductInput starts with just the number of qubtis and flipped measurements set.
The pauli_poduct_qubit_masks and measured_exp_vals start empty
and can be extended with [PauliZProductInput::add_pauliz_product]
[PauliZProductInput::add_linear_exp_val] and [PauliZProductInput::add_symbolic_exp_val]

Args:
    number_qubits (int): The number of qubits in the PauliZProduct measurement.
    use_flipped_measurement (bool): Whether or not to use flipped measurements.

Returns:
    self: The new instance of PauliZProductInput with pauli_product_qubit_masks = an empty dictionary, the
          specified number of qubits in input, number_pauli_products = 0, measured_exp_vals = an empty
          dictionary, and whether to use flipped measurements as specified in input.
"""

    def __init__(self, number_qubits: int, use_flipped_measurement: bool):
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
    def add_pauliz_product(self, readout: str, pauli_product_mask: List[int]) -> int: # type: ignore
        """
Add measured Pauli product to PauliZProductInput and returns index of Pauli product.

When the pauli product is already in the measurement input the function only returns
it index.

Args:
    readout (str): The name of the readout register the pauli_product is defined on.
    pauli_product_mask (List[int]): List of the qubits involved in the Pauli produc measurement.

Returns:
    int: The index of the added Pauli product in the list of all Pauli products.

Raises:
    RuntimeError: Failed to add pauli product.
"""

    @classmethod
    def add_linear_exp_val(self, name: str, linear: Dict[int, float]): # type: ignore
        """
Add linear definition of expectation value to measurement input.

Adds an expectation value that is defined by a linear combination
of expectation values of Pauli products.

Args:
    name (str): The name of the expectation value.
    linear (Dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.

Raises:
    RuntimeError: Failed to add linear expectation value.
"""

    @classmethod
    def add_symbolic_exp_val(self, name: str, symbolic: str, given): # type: ignore
        """
Add symbolic definition of expectation value to measurement input.

Adds an expectation value that is defined by a symbolic combination
of expectation values of Pauli products.

Args:
    name (str): The name of the expectation value.
    symbolic (str): The symbolic expression for the expectation values
                    given by [qoqo_calculator::CalculatorFloat].

Raises:
    RuntimeError: Failed to add symbolic expectation value.

The i-th PauliProducts are hardcoded as variables pauli_product_i
in the string expression of CalculatorFloat.
"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Serialize the PauliZProductInput to json form.

Returns:
    str: The serialized PauliZProductInput.

Raises:
    PyRuntimeError: Unexpected error serializing PauliZProductInput.
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
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the PauliZProductInput using the [bincode] crate.

Returns:
    ByteArray: The serialized PauliZProductInput (in [bincode] form).

Raises:
    ValueError: Cannot serialize PauliZProductInput to bytes.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class CheatedPauliZProductInput:
    """
Collected information for executing a cheated basis rotation measurement.

The CheatedPauliZProductInput starts with just the number of qubtis and flipped measurements set.
The pauli_poduct_qubit_masks and measured_exp_vals start empty
and can be extended with [CheatedPauliZProductInput::add_linear_exp_val] and
[CheatedPauliZProductInput::add_symbolic_exp_val].

Returns:
    self: The new instance of CheatedPauliZProductInput with measured_exp_vals = an empty
           HashMap and pauli_product_keys = an empty HashMap.
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
    def add_pauliz_product(self, readout: str) -> int: # type: ignore
        """
Add measured Pauli product to CheatedPauliZProductInput and returns index of Pauli product.

When the pauli product is already in the measurement input the function only returns
its index.

Args:
    readout (str): The name of the readout register containing the pauli_product expectation value.

Returns:
    int: The index of the added Pauli product in the list of all Pauli products.
"""

    @classmethod
    def add_linear_exp_val(self, name: str, linear: Dict[int, float]): # type: ignore
        """
Add linear definition of expectation value to measurement input.

Adds an expectation value that is defined by a linear combination
of expectation values of Pauli products.

Args:
    name (str): The name of the expectation value.
    linear (Dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.

Raises:
    RuntimeError: Failed to add linear expectation value.
"""

    @classmethod
    def add_symbolic_exp_val(self, name: str, symbolic: str, given): # type: ignore
        """
Add symbolic definition of expectation value to measurement input.

Adds an expectation value that is defined by a symbolic combination
of expectation values of Pauli products.

Args:
    name (str): The name of the expectation value.
    symbolic (str): The symbolic expression for the expectation values
                    given by [qoqo_calculator::CalculatorFloat].

Raises:
    RuntimeError: Failed to add symbolic expectation value.

The i-th PauliProducts are hardcoded as variables pauli_product_i
in the string expression of CalculatorFloat.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Serialize the CheatedPauliZProductInput to json form.

Returns:
    str: The serialized CheatedPauliZProductInput.

Raises:
    PyRuntimeError: Unexpected error serializing CheatedPauliZProductInput.
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
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the CheatedPauliZProductInput using the [bincode] crate.

Returns:
    ByteArray: The serialized CheatedPauliZProductInput (in [bincode] form).

Raises:
    ValueError: Cannot serialize CheatedPauliZProductInput to bytes.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class CheatedInput:
    """
Provides Necessary Information to run a cheated measurement.

The CheatedInput stores the number of qubits that are measured
and a dictionary mapping expectation value names to operators on the Hilbert space
of the qubits. The operators are represented by sparse lists of non-zero entry triples
of an operator matrix.

Args:
    number_qubits (int): The number of qubits in the PauliZProduct measurement.

Returns:
    CheatedInput: The new instance of CheatedInput with the specified number of qubits in input,
                  and an empty dictionay of expectation values.
"""

    def __init__(self, number_qubits: int):
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
    def add_operator_exp_val(self, name: str, operator: int, int, complex, readout: str): # type: ignore
        """
Add operator based expectation value to measurement input.

Adds an expectation value that is defined by an operator on the Hilbert space.

Args:
    name (str): The name of the expectation value.
    operator (List[(int, int, complex)]): The measured operator on the Hilbert space,
                                          given as a list of sparse matrix entries of the form (row, col, value).
    readout (str): The mame of the readout register that contains the density matrix or satevector.

Raises:
    RuntimeError: Failed to add operator based expectation value.
"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Serialize the CheatedInput to json form.

Returns:
    str: The serialized CheatedInput.

Raises:
    PyRuntimeError: Unexpected error serializing CheatedInput.
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
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the CheatedInput using the [bincode] crate.

Returns:
    ByteArray: The serialized CheatedInput (in [bincode] form).

Raises:
    ValueError: Cannot serialize CheatedInput to bytes.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class PauliZProduct:
    """
Collected information for executing a measurement of PauliZ product.

Args:
    constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    circuits (List[Circuit]): The collection of quantum circuits for the separate basis rotations.
    input (PauliZProductInput): The additional input information required for measurement.

Returns:
    PauliZProduct: The PauliZProduct containing the new PauliZ product measurement.
"""

    def __init__(self, constant_circuit: Optional[Circuit], circuits: List[Circuit], input: PauliZProductInput):
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
    def evaluate(self, input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]], float_registers: Dict[str, List[List[float]]], complex_registers: Dict[str, List[List[complex]]]) -> Optional[Dict[str, float]]: # type: ignore
        """
Execute the PauliZ product measurement.

Args:
    input_bit_registers (Dict[str, Union[List[List[int]], List[List[bool]]]]): The classical bit registers with the register name as key
    float_registers (Dict[str, List[List[float]]]): The classical float registers as a dictionary with the register name as key
    complex_registers (Dict[str, List[List[complex]]]): The classical complex registers as a dictionary with the register name as key

Returns:
    Optional[Dict[str, float]]: The evaluated measurement.

Raises:
    RuntimeError: Unexpected repetition of key in bit_register.
    RuntimeError: Error evaluating PauliZ product measurement.
"""

    @classmethod
    def circuits(self) -> List[Circuit]: # type: ignore
        """
Return the collection of quantum circuits for the separate basis rotations.

Returns:
    List[Circuit]: The quantum circuits.
"""

    @classmethod
    def constant_circuit(self) -> Optional[Circuit]: # type: ignore
        """
Return constant circuit that is executed before any Circuit in circuits.

Returns:
    Optional[Circuit]: The constant Circuit (None if not defined).
"""

    @classmethod
    def input(self) -> PauliZProductInput: # type: ignore
        """
Returns the measurement input data defining how to construct expectation values from measurements.

Returns:
    PauliZProductInput: The measurment input of PauliZProduct.
"""

    @classmethod
    def measurement_type(self) -> str: # type: ignore
        """
Returns the type of the measurement in string form.

Returns:
   str: The type of the measurement.
"""

    @classmethod
    def substitute_parameters(self, substituted_parameters: Dict[str, float]): # type: ignore
        """
Return clone of Measurement with symbolic parameters replaced.

Args:
    substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
"""

    @classmethod
    def _internal_to_bincode(self): # type: ignore
        """
Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

Returns:
    (str, ByteArray): Name and serialized measurement (in [bincode] form).

Raises:
    ValueError: Cannot serialize Measurement to bytes.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the PauliZProduct using the [bincode] crate.

Returns:
    ByteArray: The serialized PauliZProduct (in [bincode] form).

Raises:
    ValueError: Cannot serialize PauliZProduct to bytes.
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
Serialize the PauliZProduct to json form using the [serde_json] crate.

Returns:
    str: The serialized PauliZProduct.

Raises:
    RuntimeError: Unexpected error serializing PauliZProduct.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class CheatedPauliZProduct:
    """
Collected information for executing a cheated measurement of PauliZ product.

Args:
    constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    circuits (List[Circuit]): The collection of quantum circuits for the separate basis rotations.
    input (CheatedPauliZProductInput): The additional input information required for measurement.

Returns:
    self: The CheatedPauliZProduct containing the new cheated PauliZ product measurement.
"""

    def __init__(self, constant_circuit: Optional[Circuit], circuits: List[Circuit], input: CheatedPauliZProductInput):
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
    def evaluate(self, input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]], float_registers: Dict[str, List[List[float]]], complex_registers: Dict[str, List[List[complex]]]) -> Optional[Dict[str, float]]: # type: ignore
        """
Executes the cheated PauliZ product measurement.

Args:
    input_bit_registers (Dict[str, Union[List[List[int]], List[List[bool]]]]): The classical bit registers with the register name as key
    float_registers (Dict[str, List[List[float]]]): The classical float registers as a dictionary with the register name as key
    complex_registers (Dict[str, List[List[complex]]]): The classical complex registers as a dictionary with the register name as key

Returns:
    Optional[Dict[str, float]]: The evaluated measurement.

Raises:
    RuntimeError: Unexpected repetition of key in bit_register.
    RuntimeError: Error evaluating cheated PauliZ product measurement.
"""

    @classmethod
    def circuits(self) -> List[Circuit]: # type: ignore
        """
Returns the collection of quantum circuits for the separate basis rotations.

Returns:
    List[Circuit]: The quantum circuits.
"""

    @classmethod
    def constant_circuit(self) -> Optional[Circuit]: # type: ignore
        """
Returns constant circuit that is executed before any Circuit in circuits.

Returns:
    Optional[Circuit]: The constant Circuit (None if not defined).
"""

    @classmethod
    def input(self) -> CheatedPauliZProductInput: # type: ignore
        """
Returns the measurement input data defining how to construct expectation values from measurements.

Returns:
    CheatedPauliZProductInput: The measurment input of CheatedPauliZProduct.
"""

    @classmethod
    def measurement_type(self) -> str: # type: ignore
        """
Returns the type of the measurement in string form.

Returns:
   str: The type of the measurement.
"""

    @classmethod
    def substitute_parameters(self, substituted_parameters: Dict[str, float]): # type: ignore
        """
Returns clone of Measurement with symbolic parameters replaced

Args:
    substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
"""

    @classmethod
    def _internal_to_bincode(self): # type: ignore
        """
Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

Returns:
    (str, ByteArray): Name and serialized measurement (in [bincode] form).

Raises:
    ValueError: Cannot serialize Measurement to bytes.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the CheatedPauliZProduct using the [bincode] crate.

Returns:
    ByteArray: The serialized CheatedPauliZProduct (in [bincode] form).

Raises:
    ValueError: Cannot serialize CheatedPauliZProduct to bytes.
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
Serializes the CheatedPauliZProduct to json form using the [serde_json] crate.

Returns:
    str: The serialized CheatedPauliZProduct.

Raises:
    RuntimeError: Unexpected error serializing CheatedPauliZProduct.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class Cheated:
    """
Collected information for executing a cheated measurement.

Args:
    constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    circuits (List[Circuit]): The collection of quantum circuits executed for the measurement.
    input (CheatedInput): The additional input information required for measurement.

Returns:
    Cheated: The new measurement.
"""

    def __init__(self, constant_circuit: Optional[Circuit], circuits: List[Circuit], input: CheatedInput):
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
    def evaluate(self, input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]], float_registers: Dict[str, List[List[float]]], complex_registers: Dict[str, List[List[complex]]]) -> Optional[Dict[str, float]]: # type: ignore
        """
Execute the cheated measurement.

Args:
    input_bit_registers (Dict[str, Union[List[List[int]], List[List[bool]]]]): The classical bit registers with the register name as key.
    float_registers (Dict[str, List[List[float]]]): The classical float registers as a dictionary with the register name as key.
    complex_registers (Dict[str, List[List[complex]]]): The classical complex registers as a dictionary with the register name as key.

Returns:
    Optional[Dict[str, float]]: The evaluated expectation values.

Raises:
    RuntimeError: Unexpected repetition of key in bit_register.
    RuntimeError: Error evaluating cheated measurement.
"""

    @classmethod
    def circuits(self) -> List[Circuit]: # type: ignore
        """
Return the collection of quantum circuits for the separate cheated measurements.

Returns:
    List[Circuit]: The quantum circuits.
"""

    @classmethod
    def constant_circuit(self) -> Optional[Circuit]: # type: ignore
        """
Returns constant circuit that is executed before any Circuit in circuits.

Returns:
    Optional[Circuit]: The constant Circuit (None if not defined).
"""

    @classmethod
    def input(self) -> CheatedInput: # type: ignore
        """
Returns the measurement input data defining how to construct expectation values from measurements.

Returns:
    CheatedInput: The input of Cheated measurement
"""

    @classmethod
    def measurement_type(self) -> str: # type: ignore
        """
Returns the type of the measurement in string form.

Returns:
   str: The type of the measurement.
"""

    @classmethod
    def substitute_parameters(self): # type: ignore
        """
Return copy of Measurement with symbolic parameters replaced.

Arguments:
    substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

Raises:
    RuntimeError: Error substituting symbolic parameters.
"""

    @classmethod
    def _internal_to_bincode(self): # type: ignore
        """
Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

Returns:
    (str, ByteArray): Name and serialized measurement (in [bincode] form).

Raises:
    ValueError: Cannot serialize Measurement to bytes.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Cheated using the [bincode] crate.

Returns:
    ByteArray: The serialized Cheated (in [bincode] form).

Raises:
    ValueError: Cannot serialize Cheated to bytes.
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
Serialize the Cheated measurement to json form.

Returns:
    str: The serialized Cheated measurement.

Raises:
    RuntimeError: Unexpected error serializing Cheated.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

class ClassicalRegister:
    """
Collected information for executing a classical register.

Args:
    constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
    circuits (List[Circuit]): The collection of quantum circuits executed for the measurement.

Returns:
    ClassicalRegister: The new register.
"""

    def __init__(self, constant_circuit: Optional[Circuit], circuits: List[Circuit]):
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
    def circuits(self) -> List[Circuit]: # type: ignore
        """
Return the collection of quantum circuits that make up the total measurement.

Returns:
    List[Circuit]: The quantum circuits.
"""

    @classmethod
    def constant_circuit(self) -> Optional[Circuit]: # type: ignore
        """
Returns constant circuit that is executed before any Circuit in circuits.

Returns:
    Optional[Circuit]: The constant Circuit (None if not defined).
"""

    @classmethod
    def measurement_type(self) -> str: # type: ignore
        """
Returns the type of the measurement in string form.

Returns:
   str: The type of the measurement.
"""

    @classmethod
    def substitute_parameters(self, substituted_parameters: Dict[str, float]): # type: ignore
        """
Return copy of Measurement with symbolic parameters replaced.

Args:
    substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

Raises:
    RuntimeError: Error substituting symbolic parameters.
"""

    @classmethod
    def _internal_to_bincode(self): # type: ignore
        """
Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

Returns:
    (str, ByteArray): Name and serialized measurement (in [bincode] form).

Raises:
    ValueError: Cannot serialize Measurement to bytes.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the ClassicalRegister using the [bincode] crate.

Returns:
    ByteArray: The serialized ClassicalRegister (in [bincode] form).

Raises:
    ValueError: Cannot serialize ClassicalRegister to bytes.
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
Serialize the ClassicalRegister measurement to json form.

Returns:
    str: The serialized ClassicalRegister measurement.

Raises:
    PyRuntimeError: Unexpected error serializing ClassicalRegister.
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
    def __copy__(self): # type: ignore
        """
Return a copy of the Object (copy here produces a deepcopy).
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Return a deep copy of the Object.
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

