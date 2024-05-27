# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

from .qoqo import Circuit, Operation  # noqa: F401
import numpy as np  # noqa: F401
from typing import Tuple, List, Optional, Set, Dict, Union, Self, Sequence  # noqa: F401

class AllToAllDevice:
    """
A generic device with all-to-all connectivity.

Args:
    number_qubits (int): Fixed number of qubits.
    single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    default_gate_time (float): The default startig gate time.
"""

    def __init__(self, number_qubits: int, single_qubit_gates: List[str], two_qubit_gates: List[str], default_gate_time: float):
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
    def set_all_two_qubit_gate_times(self, gate: str, gate_time: float): # type: ignore
        """
Set gate time of all single-qubit gates of specific type

Args:
    gate (str): The hqslang name of the two-qubit-gate.
    gate_time (float): New gate time.

Returns:
    AllToAllDevice.

"""

    @classmethod
    def set_all_single_qubit_gate_times(self, gate: str, gate_time: float): # type: ignore
        """
Set gate time of all single-qubit gates of specific type

Args:
    gate (str): The hqslang name of the single-qubit-gate.
    gate_time (float): New gate time.

Returns:
    AllToAllDevice

"""

    @classmethod
    def set_all_qubit_decoherence_rates(self, rates): # type: ignore
        """
Function to set the decoherence rates for all qubits in the AllToAllDevice device.

Args:
    rates (2darray):: Decoherence rates provided as (3x3)-matrix for all qubits in the device.

Returns:
    AllToAllDevice

Raises:
    PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
"""

    @classmethod
    def add_damping_all(self, damping: float): # type: ignore
        """
Adds qubit damping to noise rates.

Args:
    damping (float): The damping rates.

Returns:
    AllToAllDevice
"""

    @classmethod
    def add_dephasing_all(self, dephasing: float): # type: ignore
        """
Adds qubit dephasing to noise rates.

Args:
    dephasing (float): The dephasing rates.

Returns:
    AllToAllDevice
"""

    @classmethod
    def add_depolarising_all(self, depolarising: float): # type: ignore
        """
Adds qubit depolarising to noise rates.

Args:
    depolarising (float): The depolarising rates.

Returns:
    AllToAllDevice
"""

    @classmethod
    def number_qubits(self) -> int: # type: ignore
        """
Return number of qubits in device.

Returns:
    int: The number of qubits.
"""

    @classmethod
    def two_qubit_edges(self) -> Sequence[(int, int)]: # type: ignore
        """
Return the list of pairs of qubits linked by a native two-qubit-gate in the device.

A pair of qubits is considered linked by a native two-qubit-gate if the device
can implement a two-qubit-gate between the two qubits without decomposing it
into a sequence of gates that involves a third qubit of the device.
The two-qubit-gate also has to form a universal set together with the available
single qubit gates.

The returned vectors is a simple, graph-library independent, representation of
the undirected connectivity graph of the device.
It can be used to construct the connectivity graph in a graph library of the user's
choice from a list of edges and can be used for applications like routing in quantum algorithms.

Returns:
    Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph

"""

    @classmethod
    def single_qubit_gate_time(self, hqslang: str, qubit: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a single qubit operation if the single qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    qubit (int): The qubit the gate acts on

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def two_qubit_gate_time(self, hqslang: str, control: int, target: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a two qubit operation if the two qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control (int): The control qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def three_qubit_gate_time(self, hqslang: str, control_0: int, control_1: int, target: int) -> Optional[float]: # type: ignore
        """
three_qubit_gate_time(gate, control_0, control_1, target
--

Returns the gate time of a three qubit operation if the three qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control_0 (int): The control_0 qubit the gate acts on.
    control_1 (int): The control_1 qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def multi_qubit_gate_time(self, hqslang: str, qubits: List[int]) -> Optional[float]: # type: ignore
        """
Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a multi qubit gate.
    qubits (List[int]): The qubits the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_single_qubit_gate_time(self, gate: str, qubit: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubit (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_two_qubit_gate_time(self, gate: str, control: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a two qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control (int): The control qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_three_qubit_gate_time(self, gate: str, control_0: int, control_1: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a three qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control_0 (int): The control_0 qubit for which the gate time is set
    control_1 (int): The control_1 qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_qubit_decoherence_rates(self, gate: str, qubits: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_multi_qubit_gate_time(self, gate: str, qubits: List[int], gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (List[int]): The qubits for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubits not in device
"""

    @classmethod
    def qubit_decoherence_rates(self, qubit: int) -> np.array: # type: ignore
        """
Return the matrix of the decoherence rates of the Lindblad equation.

Args:
    qubit (int): The qubit for which the rate matrix M is returned

Returns:
    np.array: 3 by 3 numpy array of decoherence rates

"""

    @classmethod
    def add_damping(self, qubit: int, damping: float): # type: ignore
        """
Adds single qubit damping to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    damping (float): The damping rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_dephasing(self, qubit: int, dephasing: float): # type: ignore
        """
Adds single qubit dephasing to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    dephasing (float): The dephasing rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_depolarising(self, qubit: int, depolarising: float): # type: ignore
        """
Adds single qubit depolarising to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    depolarising (float): The depolarising rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def to_generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def single_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a single qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def two_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a two qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def multi_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a mutli qubit operations available on the device.

The list of names also includes the three qubit gate operations.

Returns:
    List[str]: The list of gate names.

"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Returns a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.

"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Device.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Device using the bincode crate.

Returns:
    ByteArray: The serialized Device (in bincode form).

Raises:
    ValueError: Cannot serialize Device to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Device.

Returns:
    str: The serialized form of Device.

Raises:
    ValueError: Cannot serialize Device to json.

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

class GenericDevice:
    """
A generic device assuming all-to-all connectivity between all involved qubits.

Args:
    number_qubits (int): The number of qubits in the device

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
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
    def number_qubits(self) -> int: # type: ignore
        """
Return number of qubits in device.

Returns:
    int: The number of qubits.
"""

    @classmethod
    def two_qubit_edges(self) -> Sequence[(int, int)]: # type: ignore
        """
Return the list of pairs of qubits linked by a native two-qubit-gate in the device.

A pair of qubits is considered linked by a native two-qubit-gate if the device
can implement a two-qubit-gate between the two qubits without decomposing it
into a sequence of gates that involves a third qubit of the device.
The two-qubit-gate also has to form a universal set together with the available
single qubit gates.

The returned vectors is a simple, graph-library independent, representation of
the undirected connectivity graph of the device.
It can be used to construct the connectivity graph in a graph library of the user's
choice from a list of edges and can be used for applications like routing in quantum algorithms.

Returns:
    Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph

"""

    @classmethod
    def single_qubit_gate_time(self, hqslang: str, qubit: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a single qubit operation if the single qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    qubit (int): The qubit the gate acts on

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def two_qubit_gate_time(self, hqslang: str, control: int, target: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a two qubit operation if the two qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control (int): The control qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def three_qubit_gate_time(self, hqslang: str, control_0: int, control_1: int, target: int) -> Optional[float]: # type: ignore
        """
three_qubit_gate_time(gate, control_0, control_1, target
--

Returns the gate time of a three qubit operation if the three qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control_0 (int): The control_0 qubit the gate acts on.
    control_1 (int): The control_1 qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def multi_qubit_gate_time(self, hqslang: str, qubits: List[int]) -> Optional[float]: # type: ignore
        """
Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a multi qubit gate.
    qubits (List[int]): The qubits the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_single_qubit_gate_time(self, gate: str, qubit: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubit (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_two_qubit_gate_time(self, gate: str, control: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a two qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control (int): The control qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_three_qubit_gate_time(self, gate: str, control_0: int, control_1: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a three qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control_0 (int): The control_0 qubit for which the gate time is set
    control_1 (int): The control_1 qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_qubit_decoherence_rates(self, gate: str, qubits: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_multi_qubit_gate_time(self, gate: str, qubits: List[int], gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (List[int]): The qubits for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubits not in device
"""

    @classmethod
    def qubit_decoherence_rates(self, qubit: int) -> np.array: # type: ignore
        """
Return the matrix of the decoherence rates of the Lindblad equation.

Args:
    qubit (int): The qubit for which the rate matrix M is returned

Returns:
    np.array: 3 by 3 numpy array of decoherence rates

"""

    @classmethod
    def add_damping(self, qubit: int, damping: float): # type: ignore
        """
Adds single qubit damping to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    damping (float): The damping rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_dephasing(self, qubit: int, dephasing: float): # type: ignore
        """
Adds single qubit dephasing to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    dephasing (float): The dephasing rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_depolarising(self, qubit: int, depolarising: float): # type: ignore
        """
Adds single qubit depolarising to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    depolarising (float): The depolarising rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def to_generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def single_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a single qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def two_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a two qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def multi_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a mutli qubit operations available on the device.

The list of names also includes the three qubit gate operations.

Returns:
    List[str]: The list of gate names.

"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Returns a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.

"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Device.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Device using the bincode crate.

Returns:
    ByteArray: The serialized Device (in bincode form).

Raises:
    ValueError: Cannot serialize Device to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Device.

Returns:
    str: The serialized form of Device.

Raises:
    ValueError: Cannot serialize Device to json.

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

class SquareLatticeDevice:
    """
A generic square lattice device with only next-neighbours-connectivity.

Args:
    number_rows (int): The fixed number of rows in device..
    number_columns (int): Fixed number of columns in device.
    single_qubit_gates (List[str]): A list of 'hqslang' names of single-qubit-gates supported by the device.
    two_qubit_gates (List[str]): A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    default_gate_time (float): The default startig gate time.
"""

    def __init__(self, number_rows: int, number_columns: int, single_qubit_gates: List[str], two_qubit_gates: List[str], default_gate_time: float):
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
    def number_rows(self) -> int: # type: ignore
        """
Return the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.

Returns:
    int: The number of rows.

"""

    @classmethod
    def number_columns(self) -> int: # type: ignore
        """
Return number of columns in device.

Returns:
    int: The number of columns.

"""

    @classmethod
    def set_all_two_qubit_gate_times(self, gate: str, gate_time): # type: ignore
        """
Set gate time of all two-qubit gates of specific type

Args:
    gate (str): The hqslang name of the two-qubit-gate.
    gate_time (f64): Gate time for the given gate, valid for all qubits in the device.

Returns:
    A qoqo Device with updated gate times.

"""

    @classmethod
    def set_all_single_qubit_gate_times(self, gate: str, gate_time): # type: ignore
        """
Set gate time of all single-qubit gates of specific type

Args:
    gate (str): The hqslang name of the single-qubit-gate.
    gate_time (f64): New gate time.

Returns:
    A qoqo Device with updated gate times.

"""

    @classmethod
    def set_all_qubit_decoherence_rates(self, rates): # type: ignore
        """
Set the decoherence rates for all qubits in the SquareLatticeDevice device.

Args:
    rates (2darray):: Decoherence rates provided as (3x3)-matrix for all qubits in the device.

Returns:
    SquareLatticeDevice

Raises:
    PyValueError: The input parameter `rates` needs to be a (3x3)-matrix.
"""

    @classmethod
    def add_damping_all(self, damping: float): # type: ignore
        """
Adds qubit damping to noise rates.

Args:
    damping (float): The damping rates.

Returns:
    SquareLatticeDevice
"""

    @classmethod
    def add_dephasing_all(self, dephasing: float): # type: ignore
        """
Adds qubit dephasing to noise rates.

Args:
    dephasing (float): The dephasing rates.

Returns:
    SquareLatticeDevice
"""

    @classmethod
    def add_depolarising_all(self, depolarising: float): # type: ignore
        """
Adds qubit depolarising to noise rates.

Args:
    depolarising (float): The depolarising rates.

Returns:
    SquareLatticeDevice
"""

    @classmethod
    def number_qubits(self) -> int: # type: ignore
        """
Return number of qubits in device.

Returns:
    int: The number of qubits.
"""

    @classmethod
    def two_qubit_edges(self) -> Sequence[(int, int)]: # type: ignore
        """
Return the list of pairs of qubits linked by a native two-qubit-gate in the device.

A pair of qubits is considered linked by a native two-qubit-gate if the device
can implement a two-qubit-gate between the two qubits without decomposing it
into a sequence of gates that involves a third qubit of the device.
The two-qubit-gate also has to form a universal set together with the available
single qubit gates.

The returned vectors is a simple, graph-library independent, representation of
the undirected connectivity graph of the device.
It can be used to construct the connectivity graph in a graph library of the user's
choice from a list of edges and can be used for applications like routing in quantum algorithms.

Returns:
    Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph

"""

    @classmethod
    def single_qubit_gate_time(self, hqslang: str, qubit: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a single qubit operation if the single qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    qubit (int): The qubit the gate acts on

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def two_qubit_gate_time(self, hqslang: str, control: int, target: int) -> Optional[float]: # type: ignore
        """
Returns the gate time of a two qubit operation if the two qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control (int): The control qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def three_qubit_gate_time(self, hqslang: str, control_0: int, control_1: int, target: int) -> Optional[float]: # type: ignore
        """
three_qubit_gate_time(gate, control_0, control_1, target
--

Returns the gate time of a three qubit operation if the three qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a single qubit gate.
    control_0 (int): The control_0 qubit the gate acts on.
    control_1 (int): The control_1 qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device

"""

    @classmethod
    def multi_qubit_gate_time(self, hqslang: str, qubits: List[int]) -> Optional[float]: # type: ignore
        """
Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.

Args:
    hqslang (str): The hqslang name of a multi qubit gate.
    qubits (List[int]): The qubits the gate acts on.

Returns:
    Option[float]: None if gate is not available

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_single_qubit_gate_time(self, gate: str, qubit: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubit (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_two_qubit_gate_time(self, gate: str, control: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a two qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control (int): The control qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_three_qubit_gate_time(self, gate: str, control_0: int, control_1: int, target: int, gate_time: float): # type: ignore
        """
Set the gate time of a three qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    control_0 (int): The control_0 qubit for which the gate time is set
    control_1 (int): The control_1 qubit for which the gate time is set
    target (int): The control qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_qubit_decoherence_rates(self, gate: str, qubits: int, gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (int): The qubit for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def set_multi_qubit_gate_time(self, gate: str, qubits: List[int], gate_time: float): # type: ignore
        """
Set the gate time of a single qubit gate.

Args:
    gate (str): hqslang name of the single-qubit-gate.
    qubits (List[int]): The qubits for which the gate time is set
    gate_time (float): The gate time for the given gate.

Raises:
    PyValueError: Qubits not in device
"""

    @classmethod
    def qubit_decoherence_rates(self, qubit: int) -> np.array: # type: ignore
        """
Return the matrix of the decoherence rates of the Lindblad equation.

Args:
    qubit (int): The qubit for which the rate matrix M is returned

Returns:
    np.array: 3 by 3 numpy array of decoherence rates

"""

    @classmethod
    def add_damping(self, qubit: int, damping: float): # type: ignore
        """
Adds single qubit damping to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    damping (float): The damping rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_dephasing(self, qubit: int, dephasing: float): # type: ignore
        """
Adds single qubit dephasing to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    dephasing (float): The dephasing rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def add_depolarising(self, qubit: int, depolarising: float): # type: ignore
        """
Adds single qubit depolarising to noise rates.

Args:
    qubit (int): The qubit for which the decoherence is added
    depolarising (float): The depolarising rates.

Raises:
    PyValueError: Qubit is not in device
"""

    @classmethod
    def generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def to_generic_device(self) -> GenericDevice: # type: ignore
        """
Turns Device into GenericDevice

Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
(for example when the interface needs to be serialized)

Returns:
    GenericDevice: The device in generic representation

Note:
    GenericDevice uses nested HashMaps to represent the most general device connectivity.
    The memory usage will be inefficient for devices with large qubit numbers.
"""

    @classmethod
    def single_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a single qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def two_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a two qubit operations available on the device.

Returns:
    List[str]: The list of gate names.
"""

    @classmethod
    def multi_qubit_gate_names(self) -> List[str]: # type: ignore
        """
Returns the names of a mutli qubit operations available on the device.

The list of names also includes the three qubit gate operations.

Returns:
    List[str]: The list of gate names.

"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Returns a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.

"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Device.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Device using the bincode crate.

Returns:
    ByteArray: The serialized Device (in bincode form).

Raises:
    ValueError: Cannot serialize Device to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Device.

Returns:
    str: The serialized form of Device.

Raises:
    ValueError: Cannot serialize Device to json.

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

