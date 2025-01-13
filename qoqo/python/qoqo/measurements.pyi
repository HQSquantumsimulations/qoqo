# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/qoqo

"""
Measurements
    
.. autosummary::
    :toctree: generated/

    PauliZProductInput
    CheatedPauliZProductInput
    CheatedInput
    PauliZProduct
    CheatedPauliZProduct
    DecoherenceOnIdleModel
    Cheated
    ClassicalRegister
"""

from typing import Optional, List, Dict, Union
from .qoqo import Circuit

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
    def add_pauliz_product(self, readout: str, pauli_product_mask: List[int]) -> int:
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
            RuntimeError: Failed to add pauli product."""
    def add_linear_exp_val(self, name: str, linear: Dict[int, float]):
        """
        Add linear definition of expectation value to measurement input.

        Adds an expectation value that is defined by a linear combination
        of expectation values of Pauli products.

        Args:
            name (str): The name of the expectation value.
            linear (Dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.

        Raises:
            RuntimeError: Failed to add linear expectation value."""
    def add_symbolic_exp_val(self, name: str, symbolic: str):
        """
        Add symbolic definition of expectation value to measurement input.

        Adds an expectation value that is defined by a symbolic combination
        of expectation values of Pauli products.

        Args:
            name (str): The name of the expectation value.
            symbolic (str): The symbolic expression for the expectation values given by [qoqo_calculator::CalculatorFloat].

        Raises:
            RuntimeError: Failed to add symbolic expectation value.

        The i-th PauliProducts are hardcoded as variables pauli_product_i
        in the string expression of CalculatorFloat."""
    def to_json(self) -> str:
        """
        Serialize the PauliZProductInput to json form.

        Returns:
            str: The serialized PauliZProductInput.

        Raises:
            PyRuntimeError: Unexpected error serializing PauliZProductInput."""
    def from_json(self) -> PauliZProductInput:
        """
        Deserialize the PauliZProductInput from json form.

        Returns:
            PauliZProductInput: The deserialized PauliZProductInput.

        Raises:
            PyRuntimeError: Cannot deserialize string to PauliZProductInput."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PauliZProductInput using the [bincode] crate.

        Returns:
            ByteArray: The serialized PauliZProductInput (in [bincode] form).

        Raises:
            ValueError: Cannot serialize PauliZProductInput to bytes."""
    def from_bincode(self, input: bytearray) -> PauliZProductInput:
        """
        Convert the bincode representation of the PauliZProductInput to a PauliZProductInput using the [bincode] crate.

        Args:
            input (ByteArray): The serialized PauliZProductInput (in [bincode] form).

        Returns:
            PauliZProductInput: The deserialized PauliZProductInput.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PauliZProductInput."""
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

class CheatedPauliZProductInput:
    """
    Collected information for executing a cheated basis rotation measurement.

    The CheatedPauliZProductInput starts with just the number of qubtis and flipped measurements set.
    The pauli_poduct_qubit_masks and measured_exp_vals start empty
    and can be extended with [CheatedPauliZProductInput::add_linear_exp_val] and
    [CheatedPauliZProductInput::add_symbolic_exp_val].

    Returns:
        self: The new instance of CheatedPauliZProductInput with measured_exp_vals = an empty
               HashMap and pauli_product_keys = an empty HashMap."""

    def __init__(self):
        return
    def add_pauliz_product(self, readout: str) -> int:
        """
        Add measured Pauli product to CheatedPauliZProductInput and returns index of Pauli product.

        When the pauli product is already in the measurement input the function only returns
        its index.

        Args:
            readout (str): The name of the readout register containing the pauli_product expectation value.

        Returns:
            int: The index of the added Pauli product in the list of all Pauli products.
        """
    def add_linear_exp_val(self, name: str, linear: Dict[int, float]):
        """
        Add linear definition of expectation value to measurement input.

        Adds an expectation value that is defined by a linear combination
        of expectation values of Pauli products.

        Args:
            name (str): The name of the expectation value.
            linear (Dict[int, float]): The linear combination of expectation values as a map between Pauli product index and coefficient.

        Raises:
            RuntimeError: Failed to add linear expectation value."""
    def add_symbolic_exp_val(self, name: str, symbolic: str):
        """
        Add symbolic definition of expectation value to measurement input.

        Adds an expectation value that is defined by a symbolic combination
        of expectation values of Pauli products.

        Args:
            name (str): The name of the expectation value.
            symbolic (str): The symbolic expression for the expectation values given by [qoqo_calculator::CalculatorFloat].

        Raises:
            RuntimeError: Failed to add symbolic expectation value.

        The i-th PauliProducts are hardcoded as variables pauli_product_i
        in the string expression of CalculatorFloat.
        """
    def to_json(self) -> str:
        """
        Serialize the CheatedPauliZProductInput to json form.

        Returns:
            str: The serialized CheatedPauliZProductInput.

        Raises:
            PyRuntimeError: Unexpected error serializing CheatedPauliZProductInput."""
    def from_json(self) -> CheatedPauliZProductInput:
        """
        Deserialize the CheatedPauliZProductInput from json form.

        Returns:
            CheatedPauliZProductInput: The deserialized CheatedPauliZProductInput.

        Raises:
            PyRuntimeError: Cannot deserialize string to CheatedPauliZProductInput."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the CheatedPauliZProductInput using the [bincode] crate.

        Returns:
            ByteArray: The serialized CheatedPauliZProductInput (in [bincode] form).

        Raises:
            ValueError: Cannot serialize CheatedPauliZProductInput to bytes."""
    def from_bincode(self, input: bytearray) -> CheatedPauliZProductInput:
        """
        Convert the bincode representation of the CheatedPauliZProductInput to a CheatedPauliZProductInput using the [bincode] crate.

        Args:
            input (ByteArray): The serialized CheatedPauliZProductInput (in [bincode] form).

        Returns:
            CheatedPauliZProductInput: The deserialized CheatedPauliZProductInput.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to CheatedPauliZProductInput."""
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
                      and an empty dictionay of expectation values."""

    def __init__(self, number_qubits: int):
        return
    def add_operator_exp_val(
        self, name: str, operator: int, int, complex, readout: str
    ):
        """
        Add operator based expectation value to measurement input.

        Adds an expectation value that is defined by an operator on the Hilbert space.

        Args:
            name (str): The name of the expectation value.
            operator (List[(int, int, complex)]): The measured operator on the Hilbert space,
                                                  given as a list of sparse matrix entries of the form (row, col, value).
            readout (str): The mame of the readout register that contains the density matrix or satevector.

        Raises:
            RuntimeError: Failed to add operator based expectation value."""
    def to_json(self) -> str:
        """
        Serialize the CheatedInput to json form.

        Returns:
            str: The serialized CheatedInput.

        Raises:
            PyRuntimeError: Unexpected error serializing CheatedInput."""
    def from_json(self) -> CheatedInput:
        """
        Deserialize the CheatedInput from json form.

        Returns:
            CheatedInput: The deserialized CheatedInput.

        Raises:
            PyRuntimeError: Cannot deserialize string to CheatedInput."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the CheatedInput using the [bincode] crate.

        Returns:
            ByteArray: The serialized CheatedInput (in [bincode] form).

        Raises:
            ValueError: Cannot serialize CheatedInput to bytes."""
    def from_bincode(self, input: bytearray) -> CheatedInput:
        """
        Convert the bincode representation of the CheatedInput to a CheatedInput using the [bincode] crate.

        Args:
            input (ByteArray): The serialized CheatedInput (in [bincode] form).

        Returns:
            CheatedInput: The deserialized CheatedInput.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to CheatedInput."""
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

    def __init__(
        self,
        constant_circuit: Optional[Circuit],
        circuits: List[Circuit],
        input: PauliZProductInput,
    ):
        return
    def evaluate(
        self,
        input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]],
        float_registers: Dict[str, List[List[float]]],
        complex_registers: Dict[str, List[List[complex]]],
    ) -> Optional[Dict[str, float]]:
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
            RuntimeError: Error evaluating PauliZ product measurement."""
    def circuits(self) -> List[Circuit]:
        """
        Return the collection of quantum circuits for the separate basis rotations.

        Returns:
            List[Circuit]: The quantum circuits."""
    def constant_circuit(self) -> Optional[Circuit]:
        """
        Return constant circuit that is executed before any Circuit in circuits.

        Returns:
            Optional[Circuit]: The constant Circuit (None if not defined)."""
    def input(self) -> PauliZProductInput:
        """
        Returns the measurement input data defining how to construct expectation values from measurements.

        Returns:
            PauliZProductInput: The measurment input of PauliZProduct."""
    def measurement_type(self) -> str:
        """
        Returns the type of the measurement in string form.

        Returns:
           str: The type of the measurement."""
    def substitute_parameters(self, substituted_parameters: Dict[str, float]):
        """
        Return clone of Measurement with symbolic parameters replaced.

        Args:
            substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
        """
    def _internal_to_bincode(self):
        """
        Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

        Returns:
            (str, ByteArray): Name and serialized measurement (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Measurement to bytes."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the PauliZProduct using the [bincode] crate.

        Returns:
            ByteArray: The serialized PauliZProduct (in [bincode] form).

        Raises:
            ValueError: Cannot serialize PauliZProduct to bytes."""
    def from_bincode(self, input: bytearray) -> PauliZProduct:
        """
        Convert the bincode representation of the PauliZProduct to a PauliZProduct using the [bincode] crate.

        Args:
            input (ByteArray): The serialized PauliZProduct (in [bincode] form).

        Returns:
            PauliZProduct: The deserialized PauliZProduct.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to PauliZProduct."""
    def to_json(self) -> str:
        """
        Serialize the PauliZProduct to json form using the [serde_json] crate.

        Returns:
            str: The serialized PauliZProduct.

        Raises:
            RuntimeError: Unexpected error serializing PauliZProduct."""
    def from_json(self) -> PauliZProduct:
        """
        Deserialize the PauliZProduct from json form using the [serde_json] crate.

        Returns:
            PauliZProduct: The deserialized PauliZProduct.

        Raises:
            RuntimeError: Cannot deserialize string to PauliZProduct."""
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

    def __init__(
        self,
        constant_circuit: Optional[Circuit],
        circuits: List[Circuit],
        input: CheatedPauliZProductInput,
    ):
        return
    def evaluate(
        self,
        input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]],
        float_registers: Dict[str, List[List[float]]],
        complex_registers: Dict[str, List[List[complex]]],
    ) -> Optional[Dict[str, float]]:
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
            RuntimeError: Error evaluating cheated PauliZ product measurement."""
    def circuits(self) -> List[Circuit]:
        """
        Returns the collection of quantum circuits for the separate basis rotations.

        Returns:
            List[Circuit]: The quantum circuits."""
    def constant_circuit(self) -> Optional[Circuit]:
        """
        Returns constant circuit that is executed before any Circuit in circuits.

        Returns:
            Optional[Circuit]: The constant Circuit (None if not defined)."""
    def input(self) -> CheatedPauliZProductInput:
        """
        Returns the measurement input data defining how to construct expectation values from measurements.

        Returns:
            CheatedPauliZProductInput: The measurment input of CheatedPauliZProduct."""
    def measurement_type(self) -> str:
        """
        Returns the type of the measurement in string form.

        Returns:
           str: The type of the measurement."""
    def substitute_parameters(self, substituted_parameters: Dict[str, float]):
        """
        Returns clone of Measurement with symbolic parameters replaced

        Args:
            substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.
        """
    def _internal_to_bincode(self):
        """
        Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

        Returns:
            (str, ByteArray): Name and serialized measurement (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Measurement to bytes."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the CheatedPauliZProduct using the [bincode] crate.

        Returns:
            ByteArray: The serialized CheatedPauliZProduct (in [bincode] form).

        Raises:
            ValueError: Cannot serialize CheatedPauliZProduct to bytes."""
    def from_bincode(self, input: bytearray) -> CheatedPauliZProduct:
        """
        Convert the bincode representation of the CheatedPauliZProduct to a CheatedPauliZProduct using the [bincode] crate.

        Args:
            input (ByteArray): The serialized CheatedPauliZProduct (in [bincode] form).

        Returns:
            CheatedPauliZProduct: The deserialized CheatedPauliZProduct.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to CheatedPauliZProduct."""
    def to_json(self) -> str:
        """
        Serializes the CheatedPauliZProduct to json form using the [serde_json] crate.

        Returns:
            str: The serialized CheatedPauliZProduct.

        Raises:
            RuntimeError: Unexpected error serializing CheatedPauliZProduct."""
    def from_json(self) -> CheatedPauliZProduct:
        """
        Deserialize the CheatedPauliZProduct from json form using the [serde_json] crate.

        Returns:
            CheatedPauliZProduct: the deserialized CheatedPauliZProduct.

        Raises:
            RuntimeError: Cannot deserialize string to CheatedPauliZProduct."""
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

class Cheated:
    """
    Collected information for executing a cheated measurement.

    Args:
        constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
        circuits (List[Circuit]): The collection of quantum circuits executed for the measurement.
        input (CheatedInput): The additional input information required for measurement.

    Returns:
        Cheated: The new measurement."""

    def __init__(
        self,
        constant_circuit: Optional[Circuit],
        circuits: List[Circuit],
        input: CheatedInput,
    ):
        return
    def evaluate(
        self,
        input_bit_registers: Dict[str, Union[List[List[int]], List[List[bool]]]],
        float_registers: Dict[str, List[List[float]]],
        complex_registers: Dict[str, List[List[complex]]],
    ) -> Optional[Dict[str, float]]:
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
            RuntimeError: Error evaluating cheated measurement."""
    def circuits(self) -> List[Circuit]:
        """
        Return the collection of quantum circuits for the separate cheated measurements.

        Returns:
            List[Circuit]: The quantum circuits."""
    def constant_circuit(self) -> Optional[Circuit]:
        """
        Returns constant circuit that is executed before any Circuit in circuits.

        Returns:
            Optional[Circuit]: The constant Circuit (None if not defined)."""
    def input(self) -> CheatedInput:
        """
        Returns the measurement input data defining how to construct expectation values from measurements.

        Returns:
            CheatedInput: The input of Cheated measurement"""
    def measurement_type(self) -> str:
        """
        Returns the type of the measurement in string form.

        Returns:
           str: The type of the measurement."""
    def substitute_parameters(self):
        """
        Return copy of Measurement with symbolic parameters replaced.

        Arguments:
            substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

        Raises:
            RuntimeError: Error substituting symbolic parameters."""
    def _internal_to_bincode(self):
        """
        Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

        Returns:
            (str, ByteArray): Name and serialized measurement (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Measurement to bytes."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the Cheated using the [bincode] crate.

        Returns:
            ByteArray: The serialized Cheated (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Cheated to bytes."""
    def from_bincode(self, input: bytearray) -> Cheated:
        """
        Convert the bincode representation of the Cheated to a Cheated using the [bincode] crate.

        Args:
            input (ByteArray): The serialized Cheated (in [bincode] form).

        Returns:
            Cheated: The deserialized Cheated.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to Cheated."""
    def to_json(self) -> str:
        """
        Serialize the Cheated measurement to json form.

        Returns:
            str: The serialized Cheated measurement.

        Raises:
            RuntimeError: Unexpected error serializing Cheated."""
    def from_json(self) -> Cheated:
        """
        Deserialize the Cheated measurement from json form.

        Returns:
            Cheated: The deserialized Cheated Measurement.

        Raises:
            RuntimeError: Cannot deserialize string to Cheated."""
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

class ClassicalRegister:
    """
    Collected information for executing a classical register.

    Args:
        constant_circuit (Optional[Circuit]): The constant Circuit that is executed before each Circuit in circuits.
        circuits (List[Circuit]): The collection of quantum circuits executed for the measurement.

    Returns:
        ClassicalRegister: The new register."""

    def __init__(self, constant_circuit: Optional[Circuit], circuits: List[Circuit]):
        return
    def circuits(self) -> List[Circuit]:
        """
        Return the collection of quantum circuits that make up the total measurement.

        Returns:
            List[Circuit]: The quantum circuits."""
    def constant_circuit(self) -> Optional[Circuit]:
        """
        Returns constant circuit that is executed before any Circuit in circuits.

        Returns:
            Optional[Circuit]: The constant Circuit (None if not defined)."""
    def measurement_type(self) -> str:
        """
        Returns the type of the measurement in string form.

        Returns:
           str: The type of the measurement."""
    def substitute_parameters(self, substituted_parameters: Dict[str, float]):
        """
        Return copy of Measurement with symbolic parameters replaced.

        Args:
            substituted_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the Circuit.

        Raises:
            RuntimeError: Error substituting symbolic parameters."""
    def _internal_to_bincode(self):
        """
        Return the name of the measurement and the bincode representation of the Measurement using the [bincode] crate.

        Returns:
            (str, ByteArray): Name and serialized measurement (in [bincode] form).

        Raises:
            ValueError: Cannot serialize Measurement to bytes."""
    def to_bincode(self) -> bytearray:
        """
        Return the bincode representation of the ClassicalRegister using the [bincode] crate.

        Returns:
            ByteArray: The serialized ClassicalRegister (in [bincode] form).

        Raises:
            ValueError: Cannot serialize ClassicalRegister to bytes."""
    def from_bincode(self, input: bytearray) -> ClassicalRegister:
        """
        Convert the bincode representation of the ClassicalRegister to a ClassicalRegister using the [bincode] crate.

        Args:
            input (ByteArray): The serialized ClassicalRegister (in [bincode] form).

        Returns:
            ClassicalRegister: The deserialized ClassicalRegister.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized to ClassicalRegister."""
    def to_json(self) -> str:
        """
        Serialize the ClassicalRegister measurement to json form.

        Returns:
            str: The serialized ClassicalRegister measurement.

        Raises:
            PyRuntimeError: Unexpected error serializing ClassicalRegister."""
    def from_json(self) -> ClassicalRegister:
        """
        Deserialize the ClassicalRegister measurement from json form.

        Returns:
            ClassicalRegister: The deserialized ClassicalRegister Measurement.

        Raises:
            PyRuntimeError: Cannot deserialize string to ClassicalRegister."""
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
