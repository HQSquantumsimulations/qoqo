# Cheated Measurement

A `Cheated` measurement in qoqo/roqoqo reads out the state vector or the density matrix of a quantum computer and obtains expectation values by multiplying the matrix representation of the observable with the state vector or multiplying the operator with the density matrix and taking the trace (respectively).

`Cheated` measurements are only possible with simulator backends that can return the state vector or the density matrix of the quantum state. The expectation values are defined by a sparse matrix representation of the measured observables. Using `Cheated` measurements, expectation values can be obtained directly without decomposing operators into Pauli products and post-processing the raw output of quantum computers.

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import Cheated, CheatedInput
from qoqo import QuantumProgram
from qoqo_quest import Backend
import numpy as np

# initialize |psi> = (|0> + |1>)/ sqrt(2)
circuit = Circuit()
circuit += ops.Hadamard(0)

# Add definition for state-vector readout
circuit += ops.DefinitionComplex("state_vec", 2, is_output=True)

# The dictionary of the pauli matrix to measure for each qubit in the product in the form {qubit: pauli}.
# Allowed values to be provided for 'pauli' are: 1 = PauliX, 2 = PauliY, 3 = PauliZ.
x_matrix = np.array([[0, 1],[1, 0]])
z_matrix = np.array([[1, 0],[0, -1]])
h_matrix = 0.1 * x_matrix + 0.2 * z_matrix
operator = [(0,0, 0.2), (0,1, 0.1), (1,0, 0.1), (1,1, -0.2)]
# Directly get the state vector from the simulator backend
circuit += ops.PragmaGetStateVector(readout="state_vec", circuit=Circuit())

# Preparing the measurement input for CheatedPauliZProductInput
measurement_input = CheatedInput(number_qubits=1)
# Add the measured operator
measurement_input.add_operator_exp_val(name="<H>", operator=operator, readout="state_vec")

measurement = Cheated(
   constant_circuit=None,
   circuits=[circuit,],
   input=measurement_input,
)
backend = Backend(1)

result = backend.run_measurement(measurement)
print(result)

```

```rust
use num_complex::Complex64;
use roqoqo::measurements::{Cheated, CheatedInput};
use roqoqo::operations as ops;
use roqoqo::prelude::EvaluatingBackend;
use roqoqo::Circuit;
use roqoqo_quest::Backend;

// initialize |psi> = (|0> + |1>)/ sqrt(2)
let mut circuit = Circuit::new();
circuit += ops::Hadamard::new(0);

// Add definition for state-vector readout
circuit += ops::DefinitionComplex::new("state_vec".to_string(), 2, true);

// Defining the sparse operator
let operator: Vec<(usize, usize, Complex64)> = vec![
   (0, 0, 0.2.into()),
   (0, 1, 0.1.into()),
   (1, 0, 0.1.into()),
   (1, 1, (-0.2).into()),
];
// Directly get the state vector from the simulator backend
circuit += ops::PragmaGetStateVector::new("state_vec".to_string(), None);

// Preparing the measurement input for CheatedPauliZProductInput
let mut measurement_input = CheatedInput::new(1);
// Add the measured operator
measurement_input.add_operator_exp_val("<H>".to_string(), operator, "state_vec".to_string()).unwrap();

let measurement = Cheated {
   constant_circuit: None,
   circuits: vec![circuit],
   input: measurement_input,
};
let backend = Backend::new(1);

let result = backend.run_measurement(&measurement);
println!("{:?}", result);
```
