# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

from .qoqo import Circuit, Operation  # noqa: F401
import numpy as np  # noqa: F401
from typing import Tuple, List, Optional, Set, Dict, Union, Self, Sequence  # noqa: F401

class SingleQubitGate(Operation):
    """
The general single qubit unitary gate.

.. math::
    U =\begin{pmatrix}
        \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
        \beta_r+i \beta_i & \alpha_r-i\alpha_i
        \end{pmatrix}

Args:
    qubit (int): The qubit that the unitary gate is applied to.
    alpha_r (CalculatorFloat): The real part of the on-diagonal elements of the single-qubit unitary.
    alpha_i (CalculatorFloat): The imaginary part of the on-diagonal elements of the single-qubit unitary.
    beta_r (CalculatorFloat): The real part of the off-diagonal elements of the single-qubit unitary.
    beta_i (CalculatorFloat): The imaginary part of the off-diagonal elements of the single-qubit unitary.
    global_phase (CalculatorFloat): The global phase of the single-qubit unitary.

"""

    def __init__(self, qubit: int, alpha_r: Union[float, str], alpha_i: Union[float, str], beta_r: Union[float, str], beta_i: Union[float, str], global_phase: Union[float, str]):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class RotateZ(Operation):
    """
The ZPower gate :math:`e^{-i \frac{\theta}{2} \sigma^z}`.

.. math::
    U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0 \\\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        - i \sin(\frac{\theta}{2}) & 0 \\\\
        0 & i \sin(\frac{\theta}{2})
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class RotateY(Operation):
    """
The YPower gate :math:`e^{-i \frac{\theta}{2} \sigma^y}`.

.. math::
    U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0 \\\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        0 & - \sin(\frac{\theta}{2}) \\\\
        \sin(\frac{\theta}{2}) & 0
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class RotateX(Operation):
    """
The XPower gate :math:`e^{-i \frac{\theta}{2} \sigma^x}`.

.. math::
    U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0 \\\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        0  &  -i \sin(\frac{\theta}{2}) \\\\
        -i \sin(\frac{\theta}{2})  & 0
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class RotateXY(Operation):
    """
Implements a rotation around an axis in the x-y plane in spherical coordinates.

.. math::
    U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & -i e^{-i \phi} \sin(\frac{\theta}{2}) \\\\
        -i e^{i \phi} \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
    phi (CalculatorFloat): The rotation axis, in spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.

"""

    def __init__(self, qubit: int, theta: Union[float, str], phi: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class RotateAroundSphericalAxis(Operation):
    """
Implements a rotation around an axis in the x-y plane in spherical coordinates.

.. math::
    U = \begin{pmatrix}
        \cos(\frac{\theta}{2}) & 0 \\\\
        0 & \cos(\frac{\theta}{2})
        \end{pmatrix}
        + \begin{pmatrix}
        -i \sin(\frac{\theta}{2}) v_z  &  \sin(\frac{\theta}{2}) \left(-i v_x - v_y \right) \\\\
        \sin(\frac{\theta}{2}) \left(-i v_x + v_y \right) & i \sin(\frac{\theta}{2}) v_z
        \end{pmatrix}

with

.. math::
 v_x = \sin(\theta_{sph}) \cos(\phi_{sph}) \ , \\
 v_y = \sin(\theta_{sph}) \sin(\phi_{sph}) \ , \\
 v_z = \cos(\theta_{sph}) \ .

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
    spherical_theta (CalculatorFloat): The rotation axis, unit-vector spherical coordinates :math:`\theta_{sph}`.
    spherical_phi (CalculatorFloat): The rotation axis, unit-vector spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.

"""

    def __init__(self, qubit: int, theta: Union[float, str], spherical_theta: Union[float, str], spherical_phi: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def spherical_theta(self): # type: ignore
        """
Returns value of attribute spherical_theta
"""

    @classmethod
    def spherical_phi(self): # type: ignore
        """
Returns value of attribute spherical_phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class PauliZ(Operation):
    """
The Pauli Z gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 \\\\
        0 & -1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PauliY(Operation):
    """
The Pauli Y gate.

.. math::
    U = \begin{pmatrix}
        0 & -i \\\\
        i & 0
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PauliX(Operation):
    """
The Pauli X gate.

.. math::
    U = \begin{pmatrix}
        0 & 1 \\\\
        1 & 0
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class SqrtPauliX(Operation):
    """
The square root of the XPower gate :math:`e^{-i \frac{\pi}{4} \sigma^x}`.

.. math::
    U = \frac{1}{\sqrt(2)} \begin{pmatrix}
        1 & -i \\\\
        -i & 1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class InvSqrtPauliX(Operation):
    """
The inverse square root XPower gate :math:`e^{i \frac{\pi}{2} \sigma^x}`.

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & i \\\\
        i & 1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class Hadamard(Operation):
    """
The Hadamard gate.

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 1 \\\\
        1 & -1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class TGate(Operation):
    """
The T gate.

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 0 \\\\
        0 & e^{i \frac{\pi}{4}}
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class SGate(Operation):
    """
The S gate.

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 0 \\\\
        0 & i
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.

"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class DefinitionUsize(Operation):
    """
DefinitionUsize is the Definition for an Integer type register.

Args:
    name (string): The name of the register that is defined.
    length (int): The length of the register that is defined, usually the number of qubits to be measured.
    is_output (bool): True/False if the variable is an output to the program.
"""

    def __init__(self, name: str, length: int, is_output: bool):
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
    def length(self): # type: ignore
        """
Get value of struct field length
"""

    @classmethod
    def is_output(self): # type: ignore
        """
Get value of struct field is_output
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class DefinitionBit(Operation):
    """
DefinitionBit is the Definition for a Bit type register.

Args:
    name (string): The name of the register that is defined.
    length (int): The length of the register that is defined, usually the number of qubits to be measured.
    is_output (bool): True/False if the variable is an output to the program.
"""

    def __init__(self, name: str, length: int, is_output: bool):
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
    def length(self): # type: ignore
        """
Get value of struct field length
"""

    @classmethod
    def is_output(self): # type: ignore
        """
Get value of struct field is_output
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class DefinitionFloat(Operation):
    """
DefinitionFloat is the Definition for a Float type register.

Args:
    name (string): The name of the register that is defined.
    length (int): The length of the register that is defined, usually the number of qubits to be measured.
    is_output (bool): True/False if the variable is an output to the program.
"""

    def __init__(self, name: str, length: int, is_output: bool):
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
    def length(self): # type: ignore
        """
Get value of struct field length
"""

    @classmethod
    def is_output(self): # type: ignore
        """
Get value of struct field is_output
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class DefinitionComplex(Operation):
    """
DefinitionComplex is the Definition for a Complex type register.

Args:
    name (string): The name of the register that is defined.
    length (int): The length of the register that is defined, usually the number of qubits to be measured.
    is_output (bool): True/False if the variable is an output to the program.
"""

    def __init__(self, name: str, length: int, is_output: bool):
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
    def length(self): # type: ignore
        """
Get value of struct field length
"""

    @classmethod
    def is_output(self): # type: ignore
        """
Get value of struct field is_output
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class InputSymbolic(Operation):
    """
InputSymbolic is the Definition for a Float which will replace a certain symbolic parameter.

Args:
    name (string): The name of the register that is defined.
    input (float): The float by which to replace the quantities marked as "name".
"""

    def __init__(self, name: str, input: float):
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
    def input(self): # type: ignore
        """
Get value of struct field input
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class MeasureQubit(Operation):
    """
Measurement gate operation.

This Operation acts on one qubit writing the result of the measurement into a readout.
The classical register for the readout needs to be defined in advance by using a Definition operation.

Args:
    qubit (int): The measured qubit.
    readout (string): The classical register for the readout.
    readout_index (int): The index in the readout the result is saved to.

"""

    def __init__(self, qubit: int, readout: str, readout_index: int):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def readout_index(self): # type: ignore
        """
Get value of struct field readout_index
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
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

class PragmaGetStateVector(Operation):
    """
This PRAGMA measurement operation returns the statevector of a quantum register.

Args:
    readout (string): The name of the classical readout register.
    circuit (Optional[Circuit]): The measurement preparation Circuit, applied on a copy of the register before measurement.

"""

    def __init__(self, readout: str, circuit: Optional[Circuit]):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaGetDensityMatrix(Operation):
    """
This PRAGMA measurement operation returns the density matrix of a quantum register.

Args:
    readout (string): The name of the classical readout register.
    circuit (Optional[Circuit]): The measurement preparation Circuit, applied on a copy of the register before measurement.

"""

    def __init__(self, readout: str, circuit: Optional[Circuit]):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaGetOccupationProbability(Operation):
    """
This PRAGMA measurement operation returns the vector of the occupation probabilities.

Occupation probabilities in the context of this PRAGMA operation are probabilities of finding the quantum
register in each :math:`\sigma_z` basis state. The quantum register remains unchanged by this PRAGMA measurement operation.

Args:
    readout (string): The name of the classical readout register.
    circuit (Optional[Circuit]): The Circuit used to rotate the qureg.

"""

    def __init__(self, readout: str, circuit: Optional[Circuit]):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaGetPauliProduct(Operation):
    """
This PRAGMA measurement operation returns a Pauli product expectation value.

This PRAGMA operation returns a Pauli product expectation value after applying
a Rotate to another basis. It performs all of the operation on a clone of the quantum register,
sothat the actual quantum register remains unchanged.

Args:
    qubit_paulis (Dict[int, int]): The dictionary of the pauli matrix to apply to each qubit in the form
                                   {qubit: pauli}. Allowed values to be provided for 'pauli' are: 0 = identity, 1 = PauliX, 2 = PauliY, 3 = PauliZ.
    readout (string): The name of the classical readout register.
    circuit (Circuit): The measurement preparation Circuit, applied on a copy of the register before measurement.

"""

    def __init__(self, qubit_paulis: Dict[int, int], readout: str, circuit: Circuit):
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
    def qubit_paulis(self): # type: ignore
        """
Get value of struct field qubit_paulis
"""

    @classmethod
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaRepeatedMeasurement(Operation):
    """
This PRAGMA measurement operation returns a measurement record for N repeated measurements.

Args:
    readout (string): The name of the classical readout register.
    qubit_mapping (Dict[int, int]): The mapping of qubits to indices in readout register.
    number_measurements (int): The number of times to repeat the measurement.

"""

    def __init__(self, readout: str, qubit_mapping: Dict[int, int], number_measurements: int):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def number_measurements(self): # type: ignore
        """
Get value of struct field number_measurements
"""

    @classmethod
    def qubit_mapping(self): # type: ignore
        """
Get value of struct field qubit_mapping
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaSetNumberOfMeasurements(Operation):
    """
Wrap function automatically generates functions in these traits.
This PRAGMA operation sets the number of measurements of the circuit.

This is used for backends that allow setting the number of tries. However, setting the number of
measurements does not allow access to the underlying wavefunction or density matrix.

Args:
    number_measurements (uint): The number of measurements.
    readout (string): The register for the readout.
"""

    def __init__(self, number_measurements: int, readout: str):
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
    def number_measurements(self): # type: ignore
        """
Get value of struct field number_measurements
"""

    @classmethod
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaSetStateVector(Operation):
    """
This PRAGMA operation sets the statevector of a quantum register.

The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
operation allows you to set the state of the qubits to a state of your choosing.
For instance, to initialize the psi-minus Bell state, we pass the following vector to
the PRAGMA:
    vector = np.array([0, 1 / np.sqrt(2), -1 / np.sqrt(2), 0])

Args:
    internal (PragmaSetStateVector): The statevector that is initialized.
"""

    def __init__(self, internal: Operation):
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
    def statevector(self) -> np.ndarray: # type: ignore
        """
Return the statevector.

Returns:
    np.ndarray: The statevector representing the qubit register.
"""

    @classmethod
    def involved_qubits(self) -> Set[int]: # type: ignore
        """
List all involved qubits (here, all).

Returns:
    Set[int]: The involved qubits of the PRAGMA operation.
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Return tags classifying the type of the operation.

Used for the type based dispatch in ffi interfaces.

Returns:
    List[str]: The tags of the operation.
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Return hqslang name of the operation.

Returns:
    str: The hqslang name of the operation.
"""

    @classmethod
    def is_parametrized(self) -> bool: # type: ignore
        """
Return true when the operation has symbolic parameters.

Returns:
    bool: True if the operation contains symbolic parameters, False if it does not.
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> PragmaSetStateVector: # type: ignore
        """
Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation operation with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaSetStateVector: # type: ignore
        """
Remap qubits in a clone of the PRAGMA operation.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def __copy__(self) -> Operation: # type: ignore
        """
Return a copy of the PRAGMA operation (copy here produces a deepcopy).

Returns:
    PragmaSetStateVector: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Operation: # type: ignore
        """
Return a deep copy of the PRAGMA operation.

Returns:
    PragmaSetStateVector: A deep copy of self.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the PRAGMA operation.

Returns:
    str: The string representation of the operation.
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

class PragmaSetDensityMatrix(Operation):
    """
This PRAGMA operation sets the density matrix of a quantum register.

The Circuit() module automatically initializes the qubits in the |0> state, so this PRAGMA
operation allows you to set the state of the qubits by setting a density matrix of your choosing.

Args:
    density_matrix (a 2d array of complex numbers): The density matrix that is initialized.

"""

    def __init__(self, density_matrix):
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
    def density_matrix(self) -> np.ndarray: # type: ignore
        """
Return the set density matrix.

Returns:
    np.ndarray: The density matrix (2d array) representing the qubit register.
"""

    @classmethod
    def involved_qubits(self) -> Set[int]: # type: ignore
        """
List all involved qubits (here, all).

Returns:
    Set[int]: The involved qubits of the PRAGMA operation.
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Return tags classifying the type of the operation.

Used for type based dispatch in ffi interfaces.

Returns:
    List[str]: The tags of the Operation.
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Return hqslang name of the operation.

Returns:
    str: The hqslang name of the operation.
"""

    @classmethod
    def is_parametrized(self) -> bool: # type: ignore
        """
Return true when the operation has symbolic parameters.

Returns:
    bool: True if the operation contains symbolic parameters, False if it does not.
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> PragmaSetDensityMatrix: # type: ignore
        """
Substitute the symbolic parameters in a clone of the PRAGMA operation according to the input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaSetDensityMatrix: # type: ignore
        """
Remap qubits in a clone of the PRAGMA operation.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def __copy__(self) -> Operation: # type: ignore
        """
Return a copy of the PRAGMA operation (copy here produces a deepcopy).

Returns:
    PragmaSetDensityMatrix: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Operation: # type: ignore
        """
Return a deep copy of the PRAGMA operation.

Returns:
    PragmaSetDensityMatrix: A deep copy of self.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the PRAGMA operation.

Returns:
    str: The string representation of the operation.
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

class PragmaRepeatGate(Operation):
    """
The repeated gate PRAGMA operation.

This PRAGMA operation repeats the next gate in the circuit the given number of times
to increase the rate for error mitigation.

Args:
    repetition_coefficient (int): The number of times the following gate is repeated.
"""

    def __init__(self, repetition_coefficient: int):
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
    def repetition_coefficient(self): # type: ignore
        """
Get value of struct field repetition_coefficient
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaOverrotation(Operation):
    """
The statistical overrotation PRAGMA operation.

This PRAGMA applies a statistical overrotation to the next rotation gate in the circuit, which
matches the hqslang name in the `gate` parameter of PragmaOverrotation and the involved qubits in `qubits`.

The applied overrotation corresponds to adding a random number to the rotation angle.
The random number is drawn from a normal distribution with mean `0`
and standard deviation `variance` and is multiplied by the `amplitude`.

Args:
    gate (str): The unique hqslang name of the gate to overrotate.
    qubits (List[int]): The qubits of the gate to overrotate.
    amplitude (float): The amplitude the random number is multiplied by.
    variance (float): The standard deviation of the normal distribution the random number is drawn from.

"""

    def __init__(self, gate: str, qubits: List[int], amplitude: float, variance: float):
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
    def gate_hqslang(self): # type: ignore
        """
Get value of struct field gate_hqslang
"""

    @classmethod
    def amplitude(self): # type: ignore
        """
Get value of struct field amplitude
"""

    @classmethod
    def variance(self): # type: ignore
        """
Get value of struct field variance
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
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

class PragmaBoostNoise(Operation):
    """
This PRAGMA operation boosts noise and overrotations in the circuit.

Args:
    noise_coefficient (CalculatorFloat): The coefficient by which the noise is boosted.
"""

    def __init__(self, noise_coefficient: Union[float, str]):
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
    def noise_coefficient(self): # type: ignore
        """
Returns value of attribute noise_coefficient
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaStopParallelBlock(Operation):
    """
This PRAGMA operation signals the STOP of a parallel execution block.

Args:
    qubits (List[int]): The qubits involved in parallel execution block.
    execution_time (CalculatorFloat): The time for the execution of the block in seconds.
"""

    def __init__(self, qubits: List[int], execution_time: Union[float, str]):
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
    def execution_time(self): # type: ignore
        """
Returns value of attribute execution_time
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
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

class PragmaGlobalPhase(Operation):
    """
The global phase PRAGMA operation.

This PRAGMA operation signals that the quantum register picks up a global phase,
i.e. it provides information that there is a global phase to be considered.

Args:
    phase (CalculatorFloat): The picked up global phase.
"""

    def __init__(self, phase: Union[float, str]):
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
    def phase(self): # type: ignore
        """
Returns value of attribute phase
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaSleep(Operation):
    """
This PRAGMA operation makes the quantum hardware wait a given amount of time.

This PRAGMA operation is used for error mitigation reasons, for instance.
It can be used to boost the noise on the qubits since it gets worse with time.

Args:
    qubits (List[int]): The qubits involved in the sleep block.
    sleep_time (CalculatorFloat): The time for the execution of the block in seconds.
"""

    def __init__(self, qubits: List[int], sleep_time: Union[float, str]):
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
    def sleep_time(self): # type: ignore
        """
Returns value of attribute sleep_time
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
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

class PragmaActiveReset(Operation):
    """
This PRAGMA operation resets the chosen qubit to the zero state.

Args:
    qubit (int): The qubit to be reset.
"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
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

class PragmaStartDecompositionBlock(Operation):
    """
This PRAGMA operation signals the START of a decomposition block.

Args:
    qubits (List[int]): The qubits involved in the decomposition block.
    reordering_dictionary (Dict[int, int]): The reordering dictionary of the block.
"""

    def __init__(self, qubits: List[int], reordering_dictionary: Dict[int, int]):
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
    def reordering_dictionary(self): # type: ignore
        """
Get value of struct field reordering_dictionary
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
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

class PragmaStopDecompositionBlock(Operation):
    """
This PRAGMA operation signals the STOP of a decomposition block.

Args:
    qubits (List[int]): The qubits involved in the decomposition block.
"""

    def __init__(self, qubits: List[int]):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
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

class PragmaDamping(Operation):
    """
The damping PRAGMA noise operation.

This PRAGMA operation applies a pure damping error corresponding to zero temperature environments.

Note

Damping means going from state `|1>` to `|0>` and corresponds to zero-temperature in a physical
device where `|0>` is the ground state.
With respect to the definition of the Pauli operator `Z`, `|0>` is the excited state and damping leads to
an increase in energy.

Args:
    qubit (int): The qubit on which to apply the damping.
    gate_time (CalculatorFloat): The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    rate (CalculatorFloat): The error rate of the damping (in 1/second).
"""

    def __init__(self, qubit: int, gate_time: Union[float, str], rate: Union[float, str]):
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
    def gate_time(self): # type: ignore
        """
Returns value of attribute gate_time
"""

    @classmethod
    def rate(self): # type: ignore
        """
Returns value of attribute rate
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def superoperator(self): # type: ignore
        """
Return the superoperator defining the evolution of the density matrix under the noise gate

Returns:
    np.ndarray

"""

    @classmethod
    def powercf(self): # type: ignore
        """
Return the power of the noise gate

Args:
    `power` (CalculatorFloat): exponent in the power operation of the noise gate

Returns:
    Self

"""

    @classmethod
    def probability(self): # type: ignore
        """
Returns the probability associated with the noise operation

Returns:
    CalculatorFloat
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

class PragmaDepolarising(Operation):
    """
The depolarising PRAGMA noise operation.

This PRAGMA operation applies a depolarising error corresponding to infinite temperature environments.

Args:
    qubit (int): The qubit on which to apply the depolarising.
    gate_time (CalculatorFloat): The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    rate (CalculatorFloat): The error rate of the depolarisation (in 1/second).
"""

    def __init__(self, qubit: int, gate_time: Union[float, str], rate: Union[float, str]):
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
    def gate_time(self): # type: ignore
        """
Returns value of attribute gate_time
"""

    @classmethod
    def rate(self): # type: ignore
        """
Returns value of attribute rate
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def superoperator(self): # type: ignore
        """
Return the superoperator defining the evolution of the density matrix under the noise gate

Returns:
    np.ndarray

"""

    @classmethod
    def powercf(self): # type: ignore
        """
Return the power of the noise gate

Args:
    `power` (CalculatorFloat): exponent in the power operation of the noise gate

Returns:
    Self

"""

    @classmethod
    def probability(self): # type: ignore
        """
Returns the probability associated with the noise operation

Returns:
    CalculatorFloat
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

class PragmaDephasing(Operation):
    """
The dephasing PRAGMA noise operation.

This PRAGMA operation applies a pure dephasing error.

Args:
    qubit (int): The qubit on which to apply the dephasing.
    gate_time (CalculatorFloat): The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    rate (CalculatorFloat): The error rate of the dephasing (in 1/second).
"""

    def __init__(self, qubit: int, gate_time: Union[float, str], rate: Union[float, str]):
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
    def gate_time(self): # type: ignore
        """
Returns value of attribute gate_time
"""

    @classmethod
    def rate(self): # type: ignore
        """
Returns value of attribute rate
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def superoperator(self): # type: ignore
        """
Return the superoperator defining the evolution of the density matrix under the noise gate

Returns:
    np.ndarray

"""

    @classmethod
    def powercf(self): # type: ignore
        """
Return the power of the noise gate

Args:
    `power` (CalculatorFloat): exponent in the power operation of the noise gate

Returns:
    Self

"""

    @classmethod
    def probability(self): # type: ignore
        """
Returns the probability associated with the noise operation

Returns:
    CalculatorFloat
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

class PragmaRandomNoise(Operation):
    """
The random noise PRAGMA operation.

This PRAGMA operation applies a pure damping error corresponding to zero temperature environments.

Args:
    qubit (int): The qubit on which to apply the damping.
    gate_time (CalculatorFloat): The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    depolarising_rate (CalculatorFloat): The error rate of the depolarisation (in 1/second).
    dephasing_rate (CalculatorFloat): The error rate of the dephasing (in 1/second).
"""

    def __init__(self, qubit: int, gate_time: Union[float, str], depolarising_rate: Union[float, str], dephasing_rate: Union[float, str]):
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
    def gate_time(self): # type: ignore
        """
Returns value of attribute gate_time
"""

    @classmethod
    def depolarising_rate(self): # type: ignore
        """
Returns value of attribute depolarising_rate
"""

    @classmethod
    def dephasing_rate(self): # type: ignore
        """
Returns value of attribute dephasing_rate
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def superoperator(self): # type: ignore
        """
Return the superoperator defining the evolution of the density matrix under the noise gate

Returns:
    np.ndarray

"""

    @classmethod
    def powercf(self): # type: ignore
        """
Return the power of the noise gate

Args:
    `power` (CalculatorFloat): exponent in the power operation of the noise gate

Returns:
    Self

"""

    @classmethod
    def probability(self): # type: ignore
        """
Returns the probability associated with the noise operation

Returns:
    CalculatorFloat
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

class PragmaGeneralNoise(Operation):
    """
The general noise PRAGMA operation.

This PRAGMA operation applies a noise term according to the given operators.

Args:
    qubit (int): The qubit the PRAGMA operation is applied to.
    gate_time (CalculatorFloat): The time (in seconds) the gate takes to be applied to the qubit on the (simulated) hardware
    Rates: The rates representing the general noise matrix M (a 3x3 matrix as 2d array).

"""

    def __init__(self, qubit: int, gate_time: Union[float, str], Rates):
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
    def qubit(self) -> int: # type: ignore
        """
Return the qubit on which the PRAGMA operation is applied.

Returns:
    int: The qubit of the PRAGMA operation.
"""

    @classmethod
    def gate_time(self) -> Union[float, str]: # type: ignore
        """
Return the `gate_time` of the PRAGMA operation.

Returns:
    CalculatorFloat: The gate time of the PRAGMA operation.
"""

    @classmethod
    def rates(self) -> np.ndarray: # type: ignore
        """
Return the rates of the PRAGMA operation.

Returns:
    np.ndarray: The rates of the PRAGMA operation.
"""

    @classmethod
    def superoperator(self) -> np.ndarray: # type: ignore
        """
Return the superoperator of the PRAGMA operation.

Returns:
    np.ndarray: The matrix form of the superoperator of the PRAGMA operation.
"""

    @classmethod
    def involved_qubits(self) -> Set[int]: # type: ignore
        """
List all involved qubits.

Returns:
    Set[int]: The involved qubits of the PRAGMA operation.
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Return tags classifying the type of the operation.

Used for the type based dispatch in ffi interfaces.

Returns:
    List[str]: The tags of the Operation.
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Return hqslang name of the operation.

Returns:
    str: The hqslang name of the operation.
"""

    @classmethod
    def is_parametrized(self) -> is_parametrized (bool): # type: ignore
        """
Return true when the operation has symbolic parameters.

Returns:
    is_parametrized (bool): True if the operation contains symbolic parameters, False if it does not.
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> PragmaGeneralNoise: # type: ignore
        """
Substitute the symbolic parameters in a clone of the PRAGMA operation according to the input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaGeneralNoise: # type: ignore
        """
Remap qubits in a clone of the PRAGMA operation.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def __copy__(self) -> Operation: # type: ignore
        """
Return a copy of the PRAGMA operation (copy here produces a deepcopy).

Returns:
    PragmaGeneralNoise: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Operation: # type: ignore
        """
Return a deep copy of the PRAGMA operation.

Returns:
    PragmaGeneralNoise: A deep copy of self.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the PRAGMA operation.

Returns:
    str: The string representation of the operation.
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

class PragmaConditional(Operation):
    """
The conditional PRAGMA operation.

This PRAGMA executes a circuit when the condition bit/bool stored in a classical bit register is true.

Args:
    condition_register (str): The name of the bit register containting the condition bool value.
    condition_index (int): - The index in the bit register containting the condition bool value.
    circuit (Circuit): - The circuit executed if the condition is met.
"""

    def __init__(self, condition_register: str, condition_index: int, circuit: Circuit):
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
    def condition_register(self): # type: ignore
        """
Get value of struct field condition_register
"""

    @classmethod
    def condition_index(self): # type: ignore
        """
Get value of struct field condition_index
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PragmaChangeDevice(Operation):
    """
A wrapper around backend specific PRAGMA operations capable of changing a device.

This PRAGMA is a thin wrapper around device specific operations that can change
device properties.
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
    def wrapped_tags(self) -> List[str]: # type: ignore
        """
Return the tags of the wrapped operations.

Returns:
    List[str]: The list of tags.
"""

    @classmethod
    def wrapped_hqslang(self) -> str: # type: ignore
        """
Return the hqslang name of the wrapped operations.

Returns:
    str: The name of the wrapped operation.
"""

    @classmethod
    def wrapped_operation(self) -> bytearray: # type: ignore
        """
Return the binary representation of the wrapped operations.

Returns:
    ByteArray: The the binary representation of the wrapped operation.
"""

    @classmethod
    def involved_qubits(self) -> Set[int]: # type: ignore
        """
List all involved qubits.

Returns:
    Set[int]: The involved qubits of the PRAGMA operation.
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Return tags classifying the type of the operation.

Used for the type based dispatch in ffi interfaces.

Returns:
    List[str]: The tags of the Operation.
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Return hqslang name of the operation.

Returns:
    str: The hqslang name of the operation.
"""

    @classmethod
    def is_parametrized(self) -> is_parametrized (bool): # type: ignore
        """
Return true when the operation has symbolic parameters.

Returns:
    is_parametrized (bool): True if the operation contains symbolic parameters, False if it does not.
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> PragmaChangeDevice: # type: ignore
        """
Substitute the symbolic parameters in a clone of the PRAGMA operation according to the input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaChangeDevice: # type: ignore
        """
Remap qubits in a clone of the PRAGMA operation.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def __copy__(self) -> Operation: # type: ignore
        """
Return a copy of the PRAGMA operation (copy here produces a deepcopy).

Returns:
    PragmaChangeDevice: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Operation: # type: ignore
        """
Return a deep copy of the PRAGMA operation.

Returns:
    PragmaChangeDevice: A deep copy of self.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the PRAGMA operation.

Returns:
    str: The string representation of the operation.
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

class CNOT(Operation):
    """
The controlled NOT quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & 1 \\\\
        0 & 0 & 1 & 0
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of NOT on the target qubit.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit NOT is applied to.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class SWAP(Operation):
    """
The controlled SWAP quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 0 & 1 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class FSwap(Operation):
    """
The controlled fermionic SWAP gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 0 & 1 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & -1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class ISwap(Operation):
    """
The controlled ISwap quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 0 & i & 0 \\\\
        0 & i & 0 & 0 \\\\
        0 & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class SqrtISwap(Operation):
    """
The controlled square root ISwap quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & \frac{1}{\sqrt{2}} & \frac{i}{\sqrt{2}} & 0 \\\\
        0 & \frac{i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
        0 & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class InvSqrtISwap(Operation):
    """
The controlled inverse square root ISwap quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & \frac{1}{\sqrt{2}} & \frac{-i}{\sqrt{2}} & 0 \\\\
        0 & \frac{-i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
        0 & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class XY(Operation):
    """
The controlled XY quantum operation

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\
        0 & \cos(\theta/2) & i \sin(\theta/2) & 0 \\\
        0 & i \sin(\theta/2) & \cos(\theta/2) & 0 \\\
        0 & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.
    theta (CalculatorFloat): The rotation angle :math:`\theta`.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class ControlledPhaseShift(Operation):
    """
The controlled-PhaseShift quantum operation.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 1 & 0 \\\\
        0 & 0 & 0 & e^{i \theta}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
    theta (CalculatorFloat): The rotation angle :math:`\theta`.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class ControlledPauliY(Operation):
    """
The controlled PauliY quantum operation

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & -i \\\\
        0 & 0 & i & 0
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of PauliY gate on the target qubit.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliY is applied to.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class ControlledPauliZ(Operation):
    """
The controlled PauliZ quantum operation

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 1 & 0 \\\\
        0 & 0 & 0 & -1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of PauliZ gate on the target qubit.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class MolmerSorensenXX(Operation):
    """
The fixed phase MolmerSorensen XX gate. <http://arxiv.org/abs/1705.02771>

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & 0 & 0 & -i \\\\
        0 & 1 & -i & 0 \\\\
        0 & -i & 1 & 0 \\\\
        -i & 0 & 0 & 1
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
    target (int): The index of the least significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.

"""

    def __init__(self, control: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class VariableMSXX(Operation):
    """
The variable-angle MolmerSorensen XX gate.

.. math::
    U = \begin{pmatrix}
        \cos(\theta/2) & 0 & 0 & -i \sin(\theta/2) \\\\
        0 & \cos(\theta/2) & -i \sin(\theta/2) & 0 \\\\
        0 & -i \sin(\theta/2) & \cos(\theta/2) & 0 \\\\
        -i \sin(\theta/2) & 0 & 0 & \cos(\theta/2)
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
    target (int): The index of the least significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
    theta (CalculatorFloat): The rotation angle :math:`\theta`.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class GivensRotation(Operation):
    """
The Givens rotation interaction gate in big endian notation: :math:`e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}`.

Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & \cos(\theta) \cdot e^{i \phi} & \sin(\theta) & 0 \\\\
        0 & -\sin(\theta) \cdot e^{i \phi} & \cos(\theta) & 0 \\\\
        0 & 0 & 0 & e^{i \phi}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.
    theta (CalculatorFloat): The rotation angle :math:`\theta`.
    phase (CalculatorFloat): The phase :math:`\phi` of the rotation.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str], phase: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class GivensRotationLittleEndian(Operation):
    """
The Givens rotation interaction gate in little endian notation: :math:`e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}`.

Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & \cos(\theta) & \sin(\theta) & 0 \\\\
        0 & -\sin(\theta) \cdot e^{i \phi} & \cos(\theta) \cdot e^{i \phi} & 0 \\\\
        0 & 0 & 0 & e^{i \phi}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int): The index of the least significant qubit in the unitary representation.
    theta (CalculatorFloat): The rotation angle :math:`\theta`.
    phase (CalculatorFloat): The phase :math:`\phi` of the rotation.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str], phase: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class Qsim(Operation):
    """
The qubit simulation (Qsim) gate.

.. math::
    U = \begin{pmatrix}
        \cos(x-y) \cdot e^{-i z} & 0 & 0 & -i\sin(x-y)\cdot e^{-i z} \\\\
        0 & -i \sin(x+y)\cdot e^{i z} & \cos(x+y)\cdot e^{i z} & 0 \\\\
        0 & \cos(x+y)\cdot e^{i z}& -i \sin(x+y)\cdot e^{i z} & 0 \\\\
        -\sin(x-y)\cdot e^{-i z} & 0 & 0 & \cos(x-y)\cdot e^{-i z}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    x (CalculatorFloat): The prefactor of the XX interaction.
    y (CalculatorFloat): The prefactor of the YY interaction.
    z (CalculatorFloat): The prefactor of the ZZ interaction.

"""

    def __init__(self, control: int, target: int, x: Union[float, str], y: Union[float, str], z: Union[float, str]):
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
    def x(self): # type: ignore
        """
Returns value of attribute x
"""

    @classmethod
    def y(self): # type: ignore
        """
Returns value of attribute y
"""

    @classmethod
    def z(self): # type: ignore
        """
Returns value of attribute z
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class Fsim(Operation):
    """
The fermionic qubit simulation (Fsim) gate.

.. math::
    U = \begin{pmatrix}
        \cos(\Delta) & 0 & 0 & i \sin(\Delta) \\\\
        0 & -i \sin(t) & \cos(t) & 0 \\\\
        0 & \cos(t) & -i \sin(t) & 0 \\\\
        -\sin(\Delta) \cdot e^{-i U} & 0 & 0 & -\cos(\Delta) \cdot e^{-i U}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    t (CalculatorFloat): The hopping strength.
    u (CalculatorFloat): The interaction strength.
    delta (CalculatorFloat): The Bogoliubov interaction strength :math:`\Delta`.

Note:
The qubits have to be adjacent, i.e., :math:`|i-j|=1` has to hold. This is the only case
in which the gate is valid as a two-qubit gate (due to the Jordan-Wigner transformation).

"""

    def __init__(self, control: int, target: int, t: Union[float, str], u: Union[float, str], delta: Union[float, str]):
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
    def t(self): # type: ignore
        """
Returns value of attribute t
"""

    @classmethod
    def u(self): # type: ignore
        """
Returns value of attribute u
"""

    @classmethod
    def delta(self): # type: ignore
        """
Returns value of attribute delta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class SpinInteraction(Operation):
    """
The generalized, anisotropic XYZ Heisenberg interaction between spins.

:math:`e^{-\mathrm{i} (x \cdot X_c X_t + y \cdot Y_c Y_t + z \cdot Z_c Z_t)}`

Where x, y, z are prefactors of the :math:`X_c X_t`, :math:`Y_c Y_t`, :math:`Z_c Z_t` Pauliproducts acting on control and target qubit,
with :math:`XX \equiv \sigma_x \sigma_x`, :math:`YY \equiv \sigma_y \sigma_y` and :math:`ZZ \equiv \sigma_z \sigma_z`.

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    x (CalculatorFloat): The prefactor of the XX interaction.
    y (CalculatorFloat): The prefactor of the YY interaction.
    z (CalculatorFloat): The prefactor of the ZZ interaction.

"""

    def __init__(self, control: int, target: int, x: Union[float, str], y: Union[float, str], z: Union[float, str]):
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
    def x(self): # type: ignore
        """
Returns value of attribute x
"""

    @classmethod
    def y(self): # type: ignore
        """
Returns value of attribute y
"""

    @classmethod
    def z(self): # type: ignore
        """
Returns value of attribute z
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class Bogoliubov(Operation):
    """
The Bogoliubov DeGennes interaction gate.

:math:`e^{-\mathrm{i} Re(\Delta) (X_c X_t - Y_c Y_t)/2 + Im(\Delta) (X_c Y_t+Y_c X_t)/2}`

Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.

The unitary matrix representation is:

.. math::
    U = \begin{pmatrix}
        \cos(|\Delta|) & 0 & 0 & \mathrm{i} \sin(|\Delta|) e^{\mathrm{i} \cdot \mathrm{angle}(\Delta)} \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & 1 & 0 \\\\
        \mathrm{i} \sin(|\Delta|) e^{-\mathrm{i} \cdot \mathrm{angle}(\Delta)} & 0 & 0 & \cos(|\Delta|)
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    delta_real (CalculatorFloat): The real part of the complex Bogoliubov interaction strength :math:`Re(\Delta)`.
    delta_imag (CalculatorFloat): The imaginary part of the complex Bogoliubov interaction strength :math:`Im(\Delta)`.

"""

    def __init__(self, control: int, target: int, delta_real: Union[float, str], delta_imag: Union[float, str]):
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
    def delta_real(self): # type: ignore
        """
Returns value of attribute delta_real
"""

    @classmethod
    def delta_imag(self): # type: ignore
        """
Returns value of attribute delta_imag
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PMInteraction(Operation):
    """
The transversal interaction gate.

:math:`e^{-\mathrm{i} \theta (X_c X_t + Y_c Y_t)} = e^{-\mathrm{i} \theta (\sigma^+_c \sigma^-_t + \sigma^-_c \sigma^+_t)}`

Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    t (CalculatorFloat): The strength of the rotation :math:`\theta`.

"""

    def __init__(self, control: int, target: int, t: Union[float, str]):
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
    def t(self): # type: ignore
        """
Returns value of attribute t
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class ComplexPMInteraction(Operation):
    """
The complex hopping gate.

:math:`e^{-\mathrm{i} \left[ Re(\theta) \cdot (X_c X_t + Y_c Y_t) - Im(\theta) \cdot (X_c Y_t - Y_c X_t) \right] }`

Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.

Args:
    control (int): The index of the most significant qubit in the unitary representation.
    target (int):: The index of the least significant qubit in the unitary representation.
    t_real (CalculatorFloat): The real part of the strength of the rotation :math:`Re(\theta)`.
    t_imag (CalculatorFloat): The imaginary part of the strength of the rotation :math:`Im(\theta)`.

"""

    def __init__(self, control: int, target: int, t_real: Union[float, str], t_imag: Union[float, str]):
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
    def t_real(self): # type: ignore
        """
Returns value of attribute t_real
"""

    @classmethod
    def t_imag(self): # type: ignore
        """
Returns value of attribute t_imag
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PhaseShiftedControlledZ(Operation):
    """
The phased-shifted controlled-Z gate.

Modified, i.e. phase-shifted ControlledPauliZ two-qubit gate. <https://arxiv.org/pdf/1908.06101.pdf eq.(1)>

The unitary matrix representation is:

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & e^{i \phi} & 0 & 0 \\\\
        0 & 0 & e^{i \phi} & 0 \\\\
        0 & 0 & 0 & e^{i (2\cdot\phi + \pi)}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
    target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
    phi (CalculatorFloat): The single qubit phase $\phi$.

"""

    def __init__(self, control: int, target: int, phi: Union[float, str]):
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
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PhaseShiftState0(Operation):
    """
The phase shift gate applied on state |0>.

Rotation around Z-axis by an arbitrary angle $\theta$ (AC Stark shift of the state |0>).

.. math::
    U = \begin{pmatrix}
        e^{i \theta} & 0 \\\\
        0 & 1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class PhaseShiftState1(Operation):
    """
The phase shift gate applied on state |1>.

Rotation around Z-axis by an arbitrary angle $\theta$ (AC Stark shift of the state |1>).

.. math::
    U = \begin{pmatrix}
        1 & 0 \\\\
        0 & e^{i \theta}
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class MultiQubitMS(Operation):
    """
The Molmer-Sorensen gate between multiple qubits.

The gate applies the rotation under the product of Pauli X operators on multiple qubits.
In mathematical terms the gate applies exp(-i * theta/2 * X_i0 * X_i1 * ... * X_in).
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Return circuit implementing MultiQubitGateOperation

Returns:
    Circuit
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class MultiQubitZZ(Operation):
    """
The multi qubit Pauli-Z-Product gate.

The gate applies the rotation under the product of Pauli Z operators on multiple qubits.
In mathematical terms the gate applies exp(-i * theta/2 * Z_i0 * Z_i1 * ... * Z_in).
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubits(self): # type: ignore
        """
Return list of qubits of the multi qubit operation in order of descending significance

Returns:
    List[int]
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Return circuit implementing MultiQubitGateOperation

Returns:
    Circuit
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class InputBit(Operation):
    """
InputBit sets a certain bit in an existing BitRegister of the circuit.

Args:
    name (string): The name of the register that is defined.
    index (int): The index in the register that is set.
    value (int): The value the bit is set to.
"""

    def __init__(self, name: str, index: int, value: int):
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
    def index(self): # type: ignore
        """
Get value of struct field index
"""

    @classmethod
    def value(self): # type: ignore
        """
Get value of struct field value
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def name(self): # type: ignore
        """
Return name of definition operation.

Returns:
    str
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

class PragmaLoop(Operation):
    """
This PRAGMA measurement operation returns the statevector of a quantum register.

Args:
    repetitions (CalculatorFloat): The number of repetitions as a symbolic float. At evaluation the floor of any float value is taken
    circuit (Circuit): The Circuit that is looped.

"""

    def __init__(self, repetitions: Union[float, str], circuit: Circuit):
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
    def repetitions(self): # type: ignore
        """
Returns value of attribute repetitions
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class PhaseShiftedControlledPhase(Operation):
    """
Implements the phase-shifted controlled PhaseShift gate.

The unitary matrix representation is:

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & e^{i \phi} & 0 & 0 \\\\
        0 & 0 & e^{i \phi} & 0 \\\\
        0 & 0 & 0 & e^{i(2\cdot\phi + \theta)}
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
    target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
    theta (CalculatorFloat): The phase rotation $\theta$.
    phi (CalculatorFloat): The single qubit phase $\phi$.

"""

    def __init__(self, control: int, target: int, theta: Union[float, str], phi: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class ControlledRotateX(Operation):
    """
Implements the controlled RotateX operation.

The unitary matrix representation is:

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & \cos(\frac{\theta}{2}) & -i \sin(\frac{\theta}{2}) \\\\
        0 & 0 & -i \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the Rotatex Operation on the target qubit.
    target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit RotateX Operation is applied to.
    theta (CalculatorFloat): The angle $\theta$ of the rotation.
"""

    def __init__(self, control: int, target: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class ControlledRotateXY(Operation):
    """
Implements the controlled RotateXY operation.

The unitary matrix representation is:

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 \\\\
        0 & 0 & \cos(\frac{\theta}{2}) & -i e^{-i \phi} \sin(\frac{\theta}{2}) \\\\
        0 & 0 & -i e^{-i \phi} \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the Rotatex Operation on the target qubit.
    target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit RotateX Operation is applied to.
    theta (CalculatorFloat): The angle $\theta$ of the rotation.
    phi (CalculatorFloat): The rotation axis, in spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.
"""

    def __init__(self, control: int, target: int, theta: Union[float, str], phi: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class ControlledControlledPauliZ(Operation):
    """
Implements the double-controlled PauliZ gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 0 & -1
        \end{pmatrix}

Args:
    control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
"""

    def __init__(self, control_0: int, control_1: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control_0(self): # type: ignore
        """
Returns control_0 qubit of the three-qubit operation
"""

    @classmethod
    def control_1(self): # type: ignore
        """
Returns control_1 qubit of the three-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the three-qubit operation
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Returns circuit implementing the ThreeQubitGateOperation

Returns:
    Circuit
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class ControlledControlledPhaseShift(Operation):
    """
Implements the double-controlled PhaseShift gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i \theta}
        \end{pmatrix}

Args:
    control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
    theta (float): The rotation angle θ.
"""

    def __init__(self, control_0: int, control_1: int, target: int, theta: float):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control_0(self): # type: ignore
        """
Returns control_0 qubit of the three-qubit operation
"""

    @classmethod
    def control_1(self): # type: ignore
        """
Returns control_1 qubit of the three-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the three-qubit operation
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Returns circuit implementing the ThreeQubitGateOperation

Returns:
    Circuit
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class Toffoli(Operation):
    """
Implements Toffoli gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\\\
        0 & 0 & 0 & 0 & 0 & 0 & 1 & 0
        \end{pmatrix}

Args:
    control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the PauliX gate is applied to.
"""

    def __init__(self, control_0: int, control_1: int, target: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control_0(self): # type: ignore
        """
Returns control_0 qubit of the three-qubit operation
"""

    @classmethod
    def control_1(self): # type: ignore
        """
Returns control_1 qubit of the three-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the three-qubit operation
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Returns circuit implementing the ThreeQubitGateOperation

Returns:
    Circuit
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class GPi(Operation):
    """
Implements a pi-rotation with an embedded phase.

.. math::
    U = \begin{pmatrix}
        0 & e^{-i \theta} \\\\
        e^{i \theta} & 0
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class GPi2(Operation):
    """
Implements a pi/2-rotation with an embedded phase.

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        1 & -i e^{-i \theta} \\\\
        -i e^{i \theta} & 1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
    theta (CalculatorFloat): The angle :math:`\theta` of the rotation.

"""

    def __init__(self, qubit: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
"""

    @classmethod
    def powercf(self) -> Self: # type: ignore
        """
Returns Rotated gate raised to power

Args:
    `power`(CalculatorFloat): exponent of the power operation.

Returns:
    Self: gate raised to the power of `power`

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

class PragmaControlledCircuit(Operation):
    """
A circuit controlled by a qubit.

The circuit is applied when the qubit is in state 1.
Note that this is a unitary operation (for example a CNOT(0,1)
is equvalent to a PragmaControlledCircuit(0, [PauliX(1)]) but it cannot be represented
by a unitary operation in qoqo for arbitraty circuits.

Args:
    controlling_qubit (int): - The qubit controlling circuit application.
    circuit (Circuit): - The circuit executed if the condition is met.
"""

    def __init__(self, controlling_qubit: int, circuit: Circuit):
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
    def controlling_qubit(self): # type: ignore
        """
Get value of struct field controlling_qubit
"""

    @classmethod
    def circuit(self): # type: ignore
        """
Get value of struct field circuit
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
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

class Squeezing(Operation):
    """
The single-mode squeezing gate with tunable squeezing.

The squeezing gate is a quantum operation that allows for precise manipulation of quantum states,
by reducing the uncertainty in one variable and therefore increasing the uncertainty of another.
https://arxiv.org/pdf/quant-ph/0106157.pdf

Args:
    mode (int): The mode the squeezing gate is applied to.
    squeezing (CalculatorFloat): The coefficient of the squeezing operation.
    phase (CalculatorFloat): The squeezing phase angle of the squeezing operation.
"""

    def __init__(self, mode: int, squeezing: Union[float, str], phase: Union[float, str]):
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
    def squeezing(self): # type: ignore
        """
Returns value of attribute squeezing
"""

    @classmethod
    def phase(self): # type: ignore
        """
Returns value of attribute phase
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class PhaseShift(Operation):
    """
The single-mode phase-shift gate with variable phase, given by R(θ) = eexp(i * θ * 𝑁̂).

https://arxiv.org/pdf/2104.03241.pdf

Args:
    mode (int): The mode the phase-shift gate is applied to.
    phase (CalculatorFloat): The phase by which to shift the mode.
"""

    def __init__(self, mode: int, phase: Union[float, str]):
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
    def phase(self): # type: ignore
        """
Returns value of attribute phase
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class BeamSplitter(Operation):
    """
The 2-mode beam splitter which splits a beam with a transmission amplitude cos(θ) and a reflection amplitude exp(i * φ) * sin(θ).

Args:
    mode_0 (int): The first mode the beam-splitter is applied to.
    mode_1 (int): The second mode the beam-splitter is applied to.
    theta (CalculatorFloat): The transmittivity angle of the beam-splitter.
    phi (CalculatorFloat): The phase angle of the beam-splitter.
"""

    def __init__(self, mode_0: int, mode_1: int, theta: Union[float, str], phi: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def phi(self): # type: ignore
        """
Returns value of attribute phi
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode_0(self): # type: ignore
        """
Return `mode_0` bosonic mode of two bosonic mode Operation.

Returns:
    int
"""

    @classmethod
    def mode_1(self): # type: ignore
        """
Return `mode_1` bosonic mode of two bosonic mode Operation.

Returns:
    int
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

class PhotonDetection(Operation):
    """
The photon number-resolving detector measurement for bosons.

This can be used as a single-shot measurement of the photon number.
https://arxiv.org/pdf/0902.4824.pdf

Args:
    mode (int): The mode the detector (measurement) is applied to.
    readout (str): The register for the readout.
    readout_index (int): The index in the readout the result is saved to.
"""

    def __init__(self, mode: int, readout: str, readout_index: int):
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
    def readout(self): # type: ignore
        """
Get value of struct field readout
"""

    @classmethod
    def readout_index(self): # type: ignore
        """
Get value of struct field readout_index
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class Identity(Operation):
    """
The Identity gate.

.. math::
    U = \begin{pmatrix}
        1 & 0 \\\\
        0 & 1
        \end{pmatrix}

Args:
    qubit (int): The qubit the unitary gate is applied to.
"""

    def __init__(self, qubit: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def global_phase(self): # type: ignore
        """
Return the global phase :math:`g` of a unitary gate acting on one qubit

Here global_phase is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_r(self): # type: ignore
        """
Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit

Here alpha_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def alpha_i(self): # type: ignore
        """
Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_r(self): # type: ignore
        """
Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit

Here beta_r is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}

Returns:
    CalculatorFloat
"""

    @classmethod
    def beta_i(self): # type: ignore
        """
Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit

Here beta_i is defined by

.. math::
    U =e^{i \cdot g}\begin{pmatrix}
    \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
    \beta_r+i \beta_i & \alpha_r-i\alpha_i
    \end{pmatrix}


Returns:
    CalculatorFloat
"""

    @classmethod
    def mul(self) -> Operation: # type: ignore
        """
Multiplies two compatible operations implementing OperateSingleQubitGate.

Does not consume the two operations being multiplied.
Only Operations

Args:
    `other` - An Operation implementing [OperateSingleQubitGate].

Returns:
    Operation: Result of the multiplication, i.e. the multiplied single qubit gate.

Example:
```
from qoqo.operations import RotateZ, RotateX

gate1 =  RotateZ(qubit=0, theta=1)
gate2 = RotateX(qubit=0, theta=1)
multiplied = gate1.mul(gate2)
print("Multiplied gate: ", multiplied)
```

"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PhaseDisplacement(Operation):
    """
The single-mode phase-displacement gate with variable magnitude and phase.

Args:
    mode (int): The mode the phase-shift gate is applied to.
    displacement (CalculatorFloat): The magnitude by which to displace the mode.
    phase (CalculatorFloat): The angle by which to displace the mode.
"""

    def __init__(self, mode: int, displacement: Union[float, str], phase: Union[float, str]):
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
    def displacement(self): # type: ignore
        """
Returns value of attribute displacement
"""

    @classmethod
    def phase(self): # type: ignore
        """
Returns value of attribute phase
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class EchoCrossResonance(Operation):
    """
Implements the controlled RotateXY operation.

The unitary matrix representation is:

.. math::
    U = \frac{1}{\sqrt{2}} \begin{pmatrix}
        0 & 1 & 0 & i \\\\
        1 & 0 & -i & 0 \\\\
        0 & i & 0 & 1 \\\\
        -i & 0 & 1 & 0
        \end{pmatrix}

Args:
    control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the Rotatex Operation on the target qubit.
    target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit RotateX Operation is applied to.
    theta (CalculatorFloat): The angle $\theta$ of the rotation.
    phi (CalculatorFloat): The rotation axis, in spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.
"""

    def __init__(self, control: int, target: int, theta: Union[float, str], phi: Union[float, str]):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def control(self): # type: ignore
        """
Returns control qubit of the two-qubit operation
"""

    @classmethod
    def target(self): # type: ignore
        """
Returns target qubit of the two-qubit operation
"""

    @classmethod
    def unitary_matrix(self): # type: ignore
        """
Return unitary matrix of gate.

Returns:
    np.ndarray

Raises:
    ValueError: Error symbolic operation cannot return float unitary matrix
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

class PragmaAnnotatedOp(Operation):
    """
An annotated Operation.

Args:
    operation (Operation): - The Operation to be annotated.
    annotation (str): - The annotation.
"""

    def __init__(self, operation: Operation, annotation: str):
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
    def operation(self) -> Operation: # type: ignore
        """
Return the internal Operation.

Returns:
    Operation: The annotated Operation.
"""

    @classmethod
    def annotation(self) -> str: # type: ignore
        """
Return the annotation.

Returns:
    str: The annotation.
"""

    @classmethod
    def involved_qubits(self) -> Set[int]: # type: ignore
        """
List all involved qubits.

Returns:
    Set[int]: The involved qubits of the PRAGMA operation.
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Return tags classifying the type of the operation.

Used for the type based dispatch in ffi interfaces.

Returns:
    List[str]: The tags of the Operation.
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Return hqslang name of the operation.

Returns:
    str: The hqslang name of the operation.
"""

    @classmethod
    def is_parametrized(self) -> is_parametrized (bool): # type: ignore
        """
Return true when the operation has symbolic parameters.

Returns:
    is_parametrized (bool): True if the operation contains symbolic parameters, False if it does not.
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> PragmaAnnotatedOp: # type: ignore
        """
Substitute the symbolic parameters in a clone of the PRAGMA operation according to the input.

Args:
    substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the parameters substituted.

Raises:
    RuntimeError: The parameter substitution failed.
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> PragmaAnnotatedOp: # type: ignore
        """
Remap qubits in a clone of the PRAGMA operation.

Args:
    mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.

Returns:
    self: The PRAGMA operation with the qubits remapped.

Raises:
    RuntimeError: The qubit remapping failed.
"""

    @classmethod
    def __copy__(self) -> Operation: # type: ignore
        """
Return a copy of the PRAGMA operation (copy here produces a deepcopy).

Returns:
    PragmaAnnotatedOp: A deep copy of self.
"""

    @classmethod
    def __deepcopy__(self) -> Operation: # type: ignore
        """
Return a deep copy of the PRAGMA operation.

Returns:
    PragmaAnnotatedOp: A deep copy of self.
"""

    @classmethod
    def __format__(self) -> str: # type: ignore
        """
Return a string containing a formatted (string) representation of the PRAGMA operation.

Returns:
    str: The string representation of the operation.
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

class QuantumRabi(Operation):
    """
The quantum Rabi interaction exp(-i * θ * X * (b^† + b))

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
    theta (CalculatorFloat): The strength of the interaction.
"""

    def __init__(self, qubit: int, mode: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class LongitudinalCoupling(Operation):
    """
Longitudinal coupling gate exp(-i * θ * Z * (b^† + b))

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
    theta (CalculatorFloat): The strength of the interaction.
"""

    def __init__(self, qubit: int, mode: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class JaynesCummings(Operation):
    """
The Jaynes-Cummings gate exp(-i * θ * (σ^- * b^† + σ^+ * b))

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
    theta (CalculatorFloat): The strength of the interaction.
"""

    def __init__(self, qubit: int, mode: int, theta: Union[float, str]):
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
    def theta(self): # type: ignore
        """
Returns value of attribute theta
"""

    @classmethod
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class SingleExcitationStore(Operation):
    """
Stores a single excitation from the involved qubit into the involved bosonic mode as follows
|0⟩_B ⨂ (a |0⟩_Q + b |1⟩_Q) -> (a|0⟩_B + b |1⟩_B ) ⨂ |0⟩_Q

Note: not defined if the bosonic mode is in a state |n⟩ with n != 0

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
"""

    def __init__(self, qubit: int, mode: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class SingleExcitationLoad(Operation):
    """
Loads a single excitation from a bosonic mode into a qubit as follows
(c1 |0⟩_B + c2 |1⟩_B) ⨂ |0⟩_Q -> |0⟩_B ⨂ (c1 |0⟩_Q + c2 |1⟩_Q)

Note: if the initial qubit state is |1⟩_Q the operation is only defined if c2 = 0

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
"""

    def __init__(self, qubit: int, mode: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

class CZQubitResonator(Operation):
    """
Controlled-Z operation between a qubit and a bosonic mode, where the two-dimensional subspace of
the bosonic mode spanned by the occupation number states |0⟩_B and |1⟩_B is considered
as the second qubit involved in the CZ operation.

Args:
    qubit (int): The qubit the gate is applied to.
    mode (int): The mode the gate is applied to.
"""

    def __init__(self, qubit: int, mode: int):
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
    def is_parametrized(self): # type: ignore
        """
Returns true if operation contains symbolic parameters

Returns:
    bool
"""

    @classmethod
    def tags(self) -> List[str]: # type: ignore
        """
Returns tags identifying the Operation

Returns:
    List[str]: The tags identifying the operation
"""

    @classmethod
    def hqslang(self) -> str: # type: ignore
        """
Returns hqslang name of Operation

Returns:
    str: The name
"""

    @classmethod
    def substitute_parameters(self, substitution_parameters: Dict[str, float]) -> Operation: # type: ignore
        """
Substitutes internal symbolic parameters with float values

Only available when all symbolic expressions can be evaluated to float with the
provided parameters.

Args:
    substitution_parameters (Dict[str, float]): The substituted free parameters

Returns:
    Operation: The operation with the parameters substituted

Raises:
    RuntimeError: Parameter Substitution failed
"""

    @classmethod
    def remap_qubits(self, mapping: Dict[int, int]) -> Operation: # type: ignore
        """
Remap qubits

Args:
    mapping (Dict[int, int]): The mapping

Returns:
    Operation: The operation with the remapped qubits

Raises:
    RuntimeError: Qubit remapping failed
"""

    @classmethod
    def involved_qubits(self) -> Union[Set[int], str]: # type: ignore
        """
List all involved Qubits

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def __copy__(self): # type: ignore
        """
Copies Operation

For qoqo operations copy is always a deep copy
"""

    @classmethod
    def __deepcopy__(self): # type: ignore
        """
Creates deep copy of Operation
"""

    @classmethod
    def qubit(self): # type: ignore
        """
Return the qubit the operation acts on

Returns:
    int
"""

    @classmethod
    def involved_modes(self) -> Union[Set[int], str]: # type: ignore
        """
List of modes the operation acts on.

Returns:
    Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
"""

    @classmethod
    def remap_modes(self, mapping: Dict[int, int]): # type: ignore
        """
Remap the bosonic modes in copy of the operation.

Args:
    mapping (Dict[int, int]): Mapping for bosonic modes in operation.

Returns:
    self

Raises:
    PyValueError: Remapping could not be performed
"""

    @classmethod
    def mode(self): # type: ignore
        """
Return `mode` the bosonic Operation acts on.

Returns:
    int
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

