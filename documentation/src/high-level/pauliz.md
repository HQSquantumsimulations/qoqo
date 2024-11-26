# PauliZProduct Measurement

The `PauliZProduct` measurement is based on measuring the product of PauliZ operators for given qubits. Combined with a basis rotation of the measured qubits, it can be used to measure arbitrary expectation values. It uses projective qubit readouts like `MeasureQubit` or `PragmaRepeatedMeasurement`. It can be run on real quantum computer hardware and simulators.

The `PauliZProduct` measurement takes as inputs:
* `constant_circuit`: circuit that is always executed first. This circuit can be used to prepare the inital state from the \\(Z\\)-basis state, \\(|0...00\rangle\\).
* `circuits`: list of circuits to perform computations.
* `input`: Post-processing of measurements. It prescribes how readout registers from `circuits` are combined to generate the required expectation value.

```python
measurement = PauliZProduct(
constant_circuit=init_circuit,
circuits=[computation_circuit1, computation_circuit2, ... ],
input=measurement_input)
```

As an example, let us consider the measurement of the following Hamiltonian
\\[
     \hat{H} = 0.1\cdot X + 0.2\cdot Z \\:,
\\] where \\(X\\) and \\(Z\\) are Pauli operators. The target is to measure \\(\hat{H} \\) with respect to a state
\\[
    |\psi\rangle = \frac{1}{\sqrt{2}}\Big(|0\rangle + |1\rangle\Big)\\:.
\\]

An `init_circuit` will be used to prepare the state \\( |\psi\rangle \\) by applying the Hadamard gate.

```python
# initialize |psi>
init_circuit = Circuit()
init_circuit += ops.Hadamard(0)
```

The given Hamiltonian includes \\(X\\) and \\(Z\\) terms that cannot be measured at the same time, since they are measured using different bases. Thus, we need to create a list of `circuits` for each type of measurement, \\( \langle Z \rangle\\) and \\( \langle X \rangle\\). The circuit for measuring \\( \langle X \rangle\\) requires an additional `Hadamard` gate that rotates the qubit basis into the \\(X\\)-basis. In this example, each measured Pauli product contains only one Pauli operator. In general, one can measure the expectation values of the products of local Pauli operators, *e.g.* \\(\langle Z_0 \rangle\\), \\(\langle Z_1 \rangle\\), \\(\langle X_0 Z_1 \rangle\\), \\(\langle X_0 X_3 \rangle\\), *etc.* 

 We define bit-registers `"ro_z"` and `"ro_x"` to store the measurements in the \\(Z\\)- and \\(X\\)-basis respectively. The `PragmaRepeatedMeasurement` operation is used to declare that we are making `1000` measurements of each circuit.

```python
# Z-basis measurement circuit with 1000 shots
z_circuit = Circuit()
z_circuit += ops.DefinitionBit("ro_z", 1, is_output=True)
z_circuit += ops.PragmaRepeatedMeasurement("ro_z", 1000, None)

# X-basis measurement circuit with 1000 shots
x_circuit = Circuit()
x_circuit += ops.DefinitionBit("ro_x", 1, is_output=True)
# Changing to the X basis with a Hadamard gate
x_circuit += ops.Hadamard(0)
x_circuit += ops.PragmaRepeatedMeasurement("ro_x", 1000, None)
 ```

For the post-processing of the measured results, the `PauliZProduct` measurement needs two more inputs provided by the object `PauliZProductInput`:

* The definition of the measured Pauli products after basis transformations (`add_pauliz_product()`),
* The weights of the Pauli product expectation values in the final expectation values (`add_linear_exp_val()`).

The `PauliZProductInput` needs to define all of the products that are measured. In the given example, we will measure two products \\(\langle Z_0 \rangle\\) - one after a rotation in the \\(X\\)-basis (corresponding to \\(\langle X_0 \rangle\\)) and \\(\langle Z_0 \rangle\\) _without_ a rotation before the measurement.

We prepare the measurement input for one qubit. The `PauliZProductInput` starts with just the number of qubits. The option `use_flipped_measurement` is used to specify whether the endianess is flipped.
```python
measurement_input = PauliZProductInput(1, use_flipped_measurement=False)
```
Next, pauli products are added to the `PauliZProductInput`, using `add_pauliz_product` where we specify the register and list of qubits measured.
```python
# Read out product of Z on site 0 for register ro_z (no basis change)
z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
# Read out product of Z on site 0 for register ro_x
# (after basis change effectively a <X> measurement)
x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])
```
Last, instructions on how to combine the single expectation values into the total result are provided. In this example, \\(0.1\\) is the coefficient of the first product and \\(0.2\\) of the second.
```Python
# Add a result (the expectation value of H) that is a combination of
# the PauliProduct expectation values.
measurement_input.add_linear_exp_val(
    "<H>", {x_basis_index: 0.1, z_basis_index: 0.2},
)
```
We can now define a qoqo measurement that can be seamlessly supplied into a quantum program for execution on either a simulator or quantum hardware. Below is the complete code for creating this measurement.

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import PauliZProduct, PauliZProductInput

# initialize |psi>
init_circuit = Circuit()
init_circuit += ops.Hadamard(0)

# Z-basis measurement circuit with 1000 shots
z_circuit = Circuit()
z_circuit += ops.DefinitionBit("ro_z", 1, is_output=True)
z_circuit += ops.PragmaRepeatedMeasurement("ro_z", 1000, None)

# X-basis measurement circuit with 1000 shots
x_circuit = Circuit()
x_circuit += ops.DefinitionBit("ro_x", 1, is_output=True)
# Changing to the X basis with a Hadamard gate
x_circuit += ops.Hadamard(0)
x_circuit += ops.PragmaRepeatedMeasurement("ro_x", 1000, None)

# Preparing the measurement input for one qubit
# The PauliZProductInput starts with just the number of qubits
# and if to use a flipped measurements set.
measurement_input = PauliZProductInput(1, False)
# Next, pauli products are added to the PauliZProductInput
# Read out product of Z on site 0 for register ro_z (no basis change)
z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
# Read out product of Z on site 0 for register ro_x
# (after basis change effectively a <X> measurement)
x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])

# Last, instructions how to combine the single expectation values
# into the total result are provided.
# Add a result (the expectation value of H) that is a combination of
# the PauliProduct expectation values.
measurement_input.add_linear_exp_val(
    "<H>", {x_basis_index: 0.1, z_basis_index: 0.2},
)

measurement = PauliZProduct(
   constant_circuit=init_circuit,
   circuits=[z_circuit, x_circuit],
   input=measurement_input,
)
```

The same example in Rust:

```rust
use roqoqo::{Circuit, operations::*};
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use std::collections::HashMap;

// initialize |psi>
let mut init_circuit = Circuit::new();
init_circuit.add_operation(Hadamard::new(0));

// Z-basis measurement circuit with 1000 shots
let mut z_circuit = Circuit::new();
z_circuit.add_operation(DefinitionBit::new("ro_z".to_string(), 1, true));
z_circuit.add_operation(
    PragmaRepeatedMeasurement::new("ro_z".to_string(), 1000, None),
);

// X-basis measurement circuit with 1000 shots
let mut x_circuit = Circuit::new();
x_circuit.add_operation(DefinitionBit::new("ro_x".to_string(), 1, true));
// Changing to the X-basis with a Hadamard gate
x_circuit.add_operation(Hadamard::new(0));
x_circuit.add_operation(
    PragmaRepeatedMeasurement::new("ro_x".to_string(), 1000, None),
);

// Preparing the measurement input for one qubit
// The PauliZProductInput starts with just the number of qubits
// and if to use a flipped measurements set.
let mut measurement_input = PauliZProductInput::new(1, false);
// Next, pauli products are added to the PauliZProductInput
// Read out product of Z on site 0 for register ro_z (no basis change)
measurement_input
    .add_pauliz_product("ro_z".to_string(), vec![0])
    .unwrap();
// Read out product of Z on site 0 for register ro_x
// (after basis change effectively a <X> measurement)
measurement_input
    .add_pauliz_product("ro_x".to_string(), vec![0])
    .unwrap();

// Last, instructions how to combine the single expectation values
// into the total result are provided.
// Add a result (the expectation value of H) that is a combination
// of the PauliProduct expectation values.
measurement_input
    .add_linear_exp_val(
        "<H>".to_string(), HashMap::from([(0, 0.1), (1, 0.2)]),
    )
    .unwrap();

let measurement = PauliZProduct {
    input: measurement_input,
    circuits: vec![z_circuit.clone(), x_circuit.clone()],
    constant_circuit: Some(init_circuit.clone()),

println!("{:?}", measurement);
};
```
