# CheatedPauliZProduct Measurement

The `CheatedPauliZProduct` measurement in qoqo/roqoqo calculates expectation values based on the expectation values of products of Pauli operators. It uses the `PragmaGetPauliProduct` readout and can only be used on a simulator backend.

The measurement input `CheatedPauliZProductInput` registers Pauli products and combines the expectation values of Pauli products into results just like the measurement input of a [PauliZProduct](pauliz.md) measurement. In contrast to the `PauliZProductInput`, however, the involved qubits of the Pauli product are defined in the `PragmaGetPauliProduct` and no basis rotation is necessary.

The expectation values of Pauli products are directly derived from the state of a simulator in the backend and are exact on the level of the numerical accuracy of the simulator. The `CheatedPauliZProduct` operation can be used to benchmark an algorithm, assuming that the statistical error due to a finite amount of projective measurements vanishes.

The `CheatedPauliZProduct` only requires running one `Circuit` compared to several for more complex `PauliZProduct` measurements and can be faster.

## Example

The following code measures the same observable as the [PauliZProduct](pauliz.md) example with the `CheatedPauliZProduct` measurement.

Example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import CheatedPauliZProduct, CheatedPauliZProductInput
from qoqo_quest import Backend

# initialize |psi> = (|0> + |1>)/ sqrt(2)
circuit = Circuit()
circuit += ops.Hadamard(0)

# Add definition for z-Basis readout
circuit += ops.DefinitionFloat("ro_z", 1, is_output=True)

# Add definition for x-Basis readout
circuit += ops.DefinitionFloat("ro_x", 1, is_output=True)

# The dictionary of the pauli matrix to measure for each qubit in the product in the form {qubit: pauli}.
# Allowed values to be provided for 'pauli' are: 1 = PauliX, 2 = PauliY, 3 = PauliZ.
pauliz_products = {0: 3}
paulix_products = {0: 1}
# PragmaGetPauliProduct works only on simulators and can be used several
# times since no projective measurements are applied
circuit += ops.PragmaGetPauliProduct(qubit_paulis=pauliz_products, readout="ro_z", circuit=Circuit())
circuit += ops.PragmaGetPauliProduct(qubit_paulis=paulix_products, readout="ro_x", circuit=Circuit())

# Preparing the measurement input for CheatedPauliZProductInput
measurement_input = CheatedPauliZProductInput()
# Next, pauli products are added to the CheatedPauliZProductInput
z_basis_index = measurement_input.add_pauliz_product("ro_z")
x_basis_index = measurement_input.add_pauliz_product("ro_x")

# Add a result (the expectation value of H) that is a combination of
# the PauliProduct expectation values.
measurement_input.add_linear_exp_val(
    "<H>", {z_basis_index: 0.1, x_basis_index: 0.2},
)

measurement = CheatedPauliZProduct(
   constant_circuit=None,
   circuits=[circuit,],
   input=measurement_input,
)
backend = Backend(1)

result = backend.run_measurement(measurement)
print(result)

```

The same example in Rust:

```rust

use roqoqo::backends::{EvaluatingBackend, RegisterResult};
use roqoqo::measurements::{CheatedPauliZProduct, CheatedPauliZProductInput};
use roqoqo::{operations::*, Circuit, QuantumProgram};
use std::collections::HashMap;
use roqoqo_quest::Backend;

// initialize |psi>
let mut circuit = Circuit::new();
circuit.add_operation(Hadamard::new(0));

// Add definition for z-Basis readout
circuit.add_operation(DefinitionFloat::new("ro_z".to_string(), 1, true));
// Add definition for z-Basis readout
circuit.add_operation(DefinitionFloat::new("ro_x".to_string(), 1, true));
// PragmaGetPauliProduct works only on simulators and can be used several
// times since no projective measurements are applied
circuit.add_operation(PragmaGetPauliProduct::new(
    HashMap::from([(0, 3)]),
    "ro_z".to_string(),
    Circuit::new()),
);
circuit.add_operation(PragmaGetPauliProduct::new(
    HashMap::from([(0, 1)]),
    "ro_x".to_string(),
    Circuit::new()));

// Preparing the measurement input for CheatedPauliZProductInput

let mut measurement_input = CheatedPauliZProductInput::new();
// Next, pauli products are added to the PauliZProductInput
// Read out product of Z on site 0 for register ro_z (no basis change)
let index_z = measurement_input.add_pauliz_product("ro_z".to_string());
// Read out product of X on site 0 for register ro_x
let index_x = measurement_input.add_pauliz_product("ro_x".to_string());

// Last, instructions how to combine the single expectation values
// into the total result are provided.
// Add a result (the expectation value of H) that is a combination
// of the PauliProduct expectation values.
measurement_input
    .add_linear_exp_val("<H>".to_string(), HashMap::from([(index_z, 0.1), (index_x, 0.2)]))
    .unwrap();

let measurement = CheatedPauliZProduct {
    input: measurement_input,
    circuits: vec![circuit.clone()],
    constant_circuit: None,
};

// Now, the PauliZProduct measurement is prepared to be used
// in a QuantumProgram just like:
let program = QuantumProgram::CheatedPauliZProduct {
    measurement,
    input_parameter_names: vec![],
};

let backend = Backend::new(3);

let res = program.run(backend, &[]);
println!("{:?}", res);

```
