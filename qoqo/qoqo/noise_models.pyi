# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

from .qoqo import Circuit, Operation  # noqa: F401
import numpy as np  # noqa: F401
from typing import Tuple, List, Optional, Set, Dict, Union, Self, Sequence  # noqa: F401

class ContinuousDecoherenceModel:
    """
Noise model representing a continuous decoherence process on qubits.

This noise model assumes that all qubits are constantly experiencing
decoherence over time (e.g. due to coupling to the environment).
The noise for each qubit can be different but only single qubit noise is
included in the model.

Noise is given by the rates of the Lindblad equation.
The Lindblad equation is a so-called master equation for the time evolution of a density matrix.
For the example of a single qubit the non-coherent part of the Lindblad equation can take the following form:

d/dt * ρ = Σ Mij * Li * ρ * Lj† - 1/2 * ( Lj† * Li * ρ + ρ * Lj† * Li),

where the indices i and j run from 0 to 2

with L0 = σ+, L1 = σ- and L3 = σz.

Here the genreal incoherent part of the Lindblad equation is internally represented by a [struqture::spins::PlusMinusLindbladNoiseOperator].

To create a complex decoherence model first create the Lindblad noise and then turn it into a ContinuousDecoherenceModel.
For a simple decoherence model, use new to create an empty model and use the add_damping, add_excitation and add_dephasing methods.
For more fine control access the internal lindblad_noise directly and modify it.

Args:

    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): Optional initialisation of Noise Model with given Lindblad operator.
"""

    def __init__(self, noise_operator):
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
    def get_noise_operator(self) -> PlusMinusLindbladNoiseOperator: # type: ignore
        """
Return the internal Lindblad noise operator of the continuous noise model.

Returns:
    PlusMinusLindbladNoiseOperator: The internal Lindblad noise operator of the continuous noise
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
    def json_schema(self): # type: ignore
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
    def add_damping_rate(self, qubits: List[int], rate: float) -> ContinuousDecoherenceModel: # type: ignore
        """
Convenience function to add damping to several qubits

Args:
    qubits (list[int]): The qubits to add damping to.
    rate (float): The damping rate.

Returns:
    ContinuousDecoherenceModel: The model with the damping added.
"""

    @classmethod
    def add_dephasing_rate(self, qubits: List[int], rate: float) -> ContinuousDecoherenceModel: # type: ignore
        """
Convenience function to add dephasing to several qubits

Args:
    qubits (list[int]): The qubits to add dephasing to.
    rate (float): The dephasing rate.

Returns:
    ContinuousDecoherenceModel: The model with the dephasing added.
"""

    @classmethod
    def add_depolarising_rate(self, qubits: List[int], rate: float) -> ContinuousDecoherenceModel: # type: ignore
        """
Convenience function to add depolarising to several qubits

Args:
    qubits (list[int]): The qubits to add depolarising to.
    rate (float): The depolarising rate.

Returns:
    ContinuousDecoherenceModel: The model with the depolarising added.
"""

    @classmethod
    def add_excitation_rate(self, qubits: List[int], rate: float) -> ContinuousDecoherenceModel: # type: ignore
        """
Convenience function to add excitation to several qubits

Args:
    qubits (list[int]): The qubits to add excitation to.
    rate (float): The excitation rate.

Returns:
    ContinuousDecoherenceModel: The model with the excitation added.
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
Creates deep copy of Noise-Model.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Noise-Model using the bincode crate.

Returns:
    ByteArray: The serialized Noise-Model (in bincode form).

Raises:
    ValueError: Cannot serialize Noise-Model to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Noise-Model.

Returns:
    str: The serialized form of Noise-Model.

Raises:
    ValueError: Cannot serialize Noise-Model to json.

"""

    @classmethod
    def current_version(self): # type: ignore
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
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

class ImperfectReadoutModel:
    """
Noise model representing readout errors.

Readout errors are modeled by two probabilities in this simple model.
One probability to detect a 1 instead of a 0 when the quantum measurement gives 0 and
one probability to detect a 0 instead of a 1 when the quantum measurement gives 1.

# Example

```Python
form qoqo.noise_models import ImperfectReadoutModel

model = ImperfectReadoutModel.new_with_uniform_error(3, 0.5, 0.5);
model = model.set_error_probabilites(2, 0.3, 0.7)
uniform_prob = model.prob_detect_0_as_1(0)
assert uniform_prob == 0.5
lower_prob = model.prob_detect_0_as_1(2)
assert lower_prob == 0.3
higher_prob = model.prob_detect_1_as_0(2)
assert higher_prob == 0.7
```
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
    def new_with_uniform_error(self): # type: ignore
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
    def json_schema(self): # type: ignore
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
    def set_error_probabilites(self, qubit: int, prob_detect_0_as_1: float, prob_detect_1_as_0: float) -> Self: # type: ignore
        """
Set and overwrite the measurement error probabilities

Args:
    qubit (int): The qubitsfor which error rates is set.
    prob_detect_0_as_1 (float): The error probability to detect a 1 instead of a 0 when measuring 0
    prob_detect_1_as_0 (float): The error probability to detect a 0 instead of a 1 when measuring 1

Returns:
    Self: The updated error model

Raises:
    ValueError: Raised if the error probabilities are not valid (< 0 or > 1).
"""

    @classmethod
    def prob_detect_0_as_1(self) -> float: # type: ignore
        """
Return probability to detect 0 as 1 for a qubit

Args:
    qubit (int) The qubit for which the probability is returned.

Returns:
    float: The probability to detect 0 as 1 for the qubit
"""

    @classmethod
    def prob_detect_1_as_0(self) -> float: # type: ignore
        """
Return probability to detect 1 as 0 for a qubit

Args:
    qubit (int) The qubit for which the probability is returned.

Returns:
    float: The probability to detect 1 as 0 for the qubit
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
Creates deep copy of Noise-Model.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Noise-Model using the bincode crate.

Returns:
    ByteArray: The serialized Noise-Model (in bincode form).

Raises:
    ValueError: Cannot serialize Noise-Model to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Noise-Model.

Returns:
    str: The serialized form of Noise-Model.

Raises:
    ValueError: Cannot serialize Noise-Model to json.

"""

    @classmethod
    def current_version(self): # type: ignore
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
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

class DecoherenceOnGateModel:
    """
Error model for noise that is only present on gate executions.

Adds additional noise when specific gates (identified by hqslang name and qubits acted on) are executed.
The noise is given in the form of a struqture.spins.PlusMinusLindbladNoiseOperator the same way it
is for the ContinuousDecoherence model.

Example:

```
from qoqo.noise_models import DecoherenceOnGateModel
from struqture_py.spins import (PlusMinusLindbladNoiseOperator, PlusMinusProduct)

noise_model = DecoherenceOnGateModel()
lindblad_noise = PlusMinusLindbladNoiseOperator()
lindblad_noise.add_operator_product(
   (PlusMinusProduct().z(0), PlusMinusProduct().z(0)),
   0.9)
lindblad_noise.add_operator_product(
   (PlusMinusProduct().z(1), PlusMinusProduct().z(1)),
   0.9)

noise_model = noise_model.set_two_qubit_term(
"CNOT", 0,1,
lindblad_noise
)
```
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
    def set_single_qubit_gate_error(self, gate: str, qubit: int, noise_operator) -> Self: # type: ignore
        """
Set extra noise for a single qubit gate.

Args:
    gate (str): The name of the gate.
    qubit (int): The qubit the gate acts on.
    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.

Returns:
    Self: The error model with the new noise on gate set.

Raises:
    PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
"""

    @classmethod
    def get_single_qubit_gate_error(self, gate: str, qubit: int): # type: ignore
        """
Return the extra noise for a single qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    qubit (int): The qubit the gate acts on.

Returns
    Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
"""

    @classmethod
    def set_two_qubit_gate_error(self, gate: str, control: int, target: int, noise_operator) -> Self: # type: ignore
        """
Set extra noise for a two qubit gate.

Args:
    gate (str): The name of the gate.
    control (int): The control qubit the gate acts on.
    target (int): The target qubit the gate acts on.
    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.

Returns:
    Self: The error model with the new noise on gate set.

Raises:
    PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
"""

    @classmethod
    def get_two_qubit_gate_error(self, gate: str): # type: ignore
        """
Return the extra noise for a single qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    control (int) - The control qubit the gate acts on.
    target (int) - The target qubit the gate acts on.

Returns
    Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
"""

    @classmethod
    def set_three_qubit_gate_error(self, gate: str, control0: int, control1: int, target: int, noise_operator) -> Self: # type: ignore
        """
Set extra noise for a single qubit gate.

Args:
    gate (str): The name of the gate.
    control0 (int): The first control qubit the gate acts on.
    control1 (int): The second control qubit the gate acts on.
    target (int): The target qubit the gate acts on.
    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.

Returns:
    Self: The error model with the new noise on gate set.

Raises:
    PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
"""

    @classmethod
    def get_three_qubit_gate_error(self, gate: str, control0: int, control1: int, target: int): # type: ignore
        """
Return the extra noise for a three qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    control0 (int): The first control qubit the gate acts on.
    control1 (int): The second control qubit the gate acts on.
    target (int): The target qubit the gate acts on.

Returns
    Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
"""

    @classmethod
    def set_multi_qubit_gate_error(self, gate: str, qubits: List, noise_operator) -> Self: # type: ignore
        """
Set extra noise for a multi qubit gate.

Args:
    gate (str): The name of the gate.
    qubits (list): The qubits the gate acts on.
    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): The noise affecting system when gate is applied.

Returns:
    Self: The error model with the new noise on gate set.

Raises:
    PyTypeError: Noise operator is not a struqture.spins.PlusMinusLindbladNoiseOperator.
"""

    @classmethod
    def get_multi_qubit_gate_error(self, gate: str, qubits: List[int]): # type: ignore
        """
Return the extra noise for a multi qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    qubits (list[int]): The qubits the gate acts on.

Returns
    Optional[struqture_py.spins.PlusMinusLindbladNoiseOperator]: The error model applied when gate is applied.
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
    def json_schema(self): # type: ignore
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
Returns a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.

"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Noise-Model.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Noise-Model using the bincode crate.

Returns:
    ByteArray: The serialized Noise-Model (in bincode form).

Raises:
    ValueError: Cannot serialize Noise-Model to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Noise-Model.

Returns:
    str: The serialized form of Noise-Model.

Raises:
    ValueError: Cannot serialize Noise-Model to json.

"""

    @classmethod
    def current_version(self): # type: ignore
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
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

class SingleQubitOverrotationDescription:
    """
Description of single qubit overrotation noise model, [roqoqo::noise_models::SingleQubitOverrotationOnGate].

Consists of the raw data needed to construct a rotation gate that adds
overrotation: gate name and statistics (mean and standard deviation) of a Gaussian distribution
from which the overrotation angle is sampled.
Example:

```
from qoqo.noise_models import SingleQubitOverrotationDescription;
gate = "RotateX";
theta_mean = 0.0;
theta_std = 1.0;
noise_desc = SingleQubitOverrotationDescription(gate, theta_mean, theta_std);
```
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
    def __copy__(self): # type: ignore
        """
Return a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Create deep copy of Noise-Model.

Returns:
    A deep copy of self.
"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of SingleQubitOverrotationDescription using the bincode crate.

Returns:
    ByteArray: The serialized SingleQubitOverrotationDescription (in bincode form).

Raises:
    ValueError: Cannot serialize SingleQubitOverrotationDescription to bytes.
"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the SingleQubitOverrotationDescription.

Returns:
    str: The serialized form of SingleQubitOverrotationDescription.

Raises:
    ValueError: Cannot serialize SingleQubitOverrotationDescription.
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
"""

    @classmethod
    def current_version(self): # type: ignore
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
    def json_schema(self): # type: ignore
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

class SingleQubitOverrotationOnGate:
    """
Single qubit overrotation noise model on gate.

Adds a rotation gate with a randomly distributed rotation angle after specified gates in a quantum circuit.
Example:

```
from qoqo.noise_models import SingleQubitOverrotationDescription
from qoqo.noise_models import SingleQubitOverrotationOnGate
gate = "RotateX"
theta_mean = 0.0
theta_std = 1.0
noise_desc = SingleQubitOverrotationDescription(gate, theta_mean, theta_std)

noise = SingleQubitOverrotationOnGate();
circuit_gate_with_noise = "RotateZ";
qubit = 0;
noise.set_single_qubit_overrotation(circuit_gate_with_noise, qubit, noise_desc);
```
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
    def set_single_qubit_overrotation(self, gate: str, qubit: int) -> Self: # type: ignore
        """
Set overrotation for a single qubit gate.

Args:
    gate (str): The name of the gate.
    qubit (int): The qubit the gate acts on.
    noise_description (SingleQubitOverrotationDescription) - overrotation description for gate.

Returns:
    Self: The overotation model with the new overrotation on gate set.

Raises:
    PyTypeError: Noise description is not a SingleQubitOverrotationDescription.
"""

    @classmethod
    def get_single_qubit_overrotation(self, gate: str, qubit: int): # type: ignore
        """
Return the overrotation description for a single qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    qubit (int): The qubit the gate acts on.

Returns
    Optional[SingleQubitOverrotationDescription]: The overrotation applied when gate is applied.
"""

    @classmethod
    def set_two_qubit_overrotation(self, gate: str, control: int, target: int) -> Self: # type: ignore
        """
Set extra noise for a two qubit gate.

Args:
    gate (str): The name of the gate.
    control (int): The control qubit the gate acts on.
    target (int): The target qubit the gate acts on.
    noise_description ((SingleQubitOverrotationDescription, SingleQubitOverrotationDescription)) - overrotation description for gate.

Returns:
    Self: The overrotation model with the new overrotation on gate set.

Raises:
    PyTypeError: Noise description is not a (SingleQubitOverrotationDescription, SingleQubitOverrotationDescription).
"""

    @classmethod
    def get_two_qubit_overrotation(self, gate: str): # type: ignore
        """
Return the extra noise for a single qubit gate, if it exists.

Args:
    gate (str): The name of the gate.
    control (int) - The control qubit the gate acts on.
    target (int) - The target qubit the gate acts on.

Returns
    Optional[(SingleQubitOverrotationDescription, SingleQubitOverrotationDescription)]: The overrotation applied when gate is applied.
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
    def json_schema(self): # type: ignore
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
Returns a copy of the device (copy here produces a deepcopy).

Returns:
    A deep copy of self.

"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Noise-Model.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Noise-Model using the bincode crate.

Returns:
    ByteArray: The serialized Noise-Model (in bincode form).

Raises:
    ValueError: Cannot serialize Noise-Model to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Noise-Model.

Returns:
    str: The serialized form of Noise-Model.

Raises:
    ValueError: Cannot serialize Noise-Model to json.

"""

    @classmethod
    def current_version(self): # type: ignore
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
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

class DecoherenceOnIdleModel:
    """
Noise model representing a continuous decoherence process on idle qubits.

The purpose of this noise model is to enable defining background noise models that exclude the qubits involved
in a gate, as these might be decoupled from the background noise.The noise for each qubit can be different but
only single qubit noise is included in the model.

Noise is given by the rates of the Lindblad equation.
The Lindblad equation is a so-called master equation for the time evolution of a density matrix.
For the example of a single qubit the non-coherent part of the Lindblad equation can take the following form:

d/dt * ρ = Σ Mij * Li * ρ * Lj† - 1/2 * ( Lj† * Li * ρ + ρ * Lj† * Li),

where the indices i and j run from 0 to 2

with L0 = σ+, L1 = σ- and L3 = σz.

Here the genreal incoherent part of the Lindblad equation is internally represented by a [struqture::spins::PlusMinusLindbladNoiseOperator].

To create a complex decoherence model first create the Lindblad noise and then turn it into a DecoherenceOnIdleModel.
For a simple decoherence model, use new to create an empty model and use the add_damping, add_excitation and add_dephasing methods.
For more fine control access the internal lindblad_noise directly and modify it.

Args:

    noise_operator (struqture_py.spins.PlusMinusLindbladNoiseOperator): Optional initialisation of Noise Model with given Lindblad operator.
"""

    def __init__(self, noise_operator):
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
    def get_noise_operator(self) -> PlusMinusLindbladNoiseOperator: # type: ignore
        """
Return the internal Lindblad noise operator of the continuous noise model.

Returns:
    PlusMinusLindbladNoiseOperator: The internal Lindblad noise operator of the continuous noise
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
    def json_schema(self): # type: ignore
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
    def add_damping_rate(self, qubits: List[int], rate: float) -> DecoherenceOnIdleModel: # type: ignore
        """
Convenience function to add damping to several qubits

Args:
    qubits (list[int]): The qubits to add damping to.
    rate (float): The damping rate.

Returns:
    DecoherenceOnIdleModel: The model with the damping added.
"""

    @classmethod
    def add_dephasing_rate(self, qubits: List[int], rate: float) -> DecoherenceOnIdleModel: # type: ignore
        """
Convenience function to add dephasing to several qubits

Args:
    qubits (list[int]): The qubits to add dephasing to.
    rate (float): The dephasing rate.

Returns:
    DecoherenceOnIdleModel: The model with the dephasing added.
"""

    @classmethod
    def add_depolarising_rate(self, qubits: List[int], rate: float) -> DecoherenceOnIdleModel: # type: ignore
        """
Convenience function to add depolarising to several qubits

Args:
    qubits (list[int]): The qubits to add depolarising to.
    rate (float): The depolarising rate.

Returns:
    DecoherenceOnIdleModel: The model with the depolarising added.
"""

    @classmethod
    def add_excitation_rate(self, qubits: List[int], rate: float) -> DecoherenceOnIdleModel: # type: ignore
        """
Convenience function to add excitation to several qubits

Args:
    qubits (list[int]): The qubits to add excitation to.
    rate (float): The excitation rate.

Returns:
    DecoherenceOnIdleModel: The model with the excitation added.
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
Creates deep copy of Noise-Model.

Returns:
    A deep copy of self.

"""

    @classmethod
    def to_bincode(self) -> bytearray: # type: ignore
        """
Return the bincode representation of the Noise-Model using the bincode crate.

Returns:
    ByteArray: The serialized Noise-Model (in bincode form).

Raises:
    ValueError: Cannot serialize Noise-Model to bytes.

"""

    @classmethod
    def to_json(self) -> str: # type: ignore
        """
Return the json representation of the Noise-Model.

Returns:
    str: The serialized form of Noise-Model.

Raises:
    ValueError: Cannot serialize Noise-Model to json.

"""

    @classmethod
    def current_version(self): # type: ignore
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
    def min_supported_version(self) -> str: # type: ignore
        """
Return the minimum version of qoqo that supports this object.

Returns:
    str: The minimum version of the qoqo library to deserialize this object.
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

