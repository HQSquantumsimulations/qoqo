# Backends

Backends in qoqo are used for two things:

* Running quantum programs and obtaining results from simulators or real quantum computing hardware,
* Translating qoqo objects to other quantum computing frameworks.

## Running quantum programs

To obtain results based on a quantum program (or quantum circuit) defined in qoqo, the program must run on a simulator or real quantum computing hardware.

Any backend - whether simulator or hardware - implemented as a so called `EvaluatingBackend` allows users to directly evaluate the returned measurement results, for example to obtain expectation values instead of pure probabilities. Backends that purely translate qoqo `Circuits` to other frameworks or representations do _not_ implement the EvaluatingBackend, like e.g. qoqo_qasm.

All implemented backends are provided in separate modules. Currently the following packages provide the implemented backends for qoqo:


| Backend | Description |
| -------- | ------- |
| [qoqo_qiskit](https://github.com/HQSquantumsimulations/qoqo-qiskit) | Simulation/QPU backend that gives access to IBM's machines. |
| [qoqo_for_braket](https://github.com/HQSquantumsimulations/qoqo-for-braket) | Simulation/QPU backend that gives access to Amazon Braket's machines. |
| [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest) | Simulation backend based on QuEST. |
| [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm) | Translating Backend to translate qoqo circuits to qasm. |
| [qoqo_myqml](https://github.com/HQSquantumsimulations/qoqo_myqlm) | Simulation/QPU backend that gives access to the machines built as part of the QSolid project. |
| [qoqo_qryd](https://github.com/HQSquantumsimulations/qoqo_qryd)    | Simulation backend that gives access to QRyd's machines. |
|[qoqo_strawberry_fields](https://github.com/HQSquantumsimulations/qoqo-strawberry-fields)| Backend to translate qoqo circuits to strawberry-fields toolkit for photonic quantum computing.


An `EvaluatingBackend` provides the functionality to run:

* A _single_ circuit. The backend will execute just the circuit and return the measurement results of all registers in a tuple (bit-registers, float-registers, complex-registers). More details on registers can be found in section [readout](circuits/readout.md). All the postprocessing of the bare results needs to be done manually.
* A measurement. _All_ circuits collected in the measurement are executed and the post-processed expectation values are returned.
* A quantum program. A qoqo QuantumProgram also handles replacement of symbolic variables. It provides its own `run()` method and calls the given backend internally.

All evaluating backends provide the same methods: `run_circuit()`, `run_measurement()` or `run_measurement_registers()`, and `run()`.

### Example

A [QuantumProgram](hight-level/program.md) is created to be executed on the [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest) simulator backend. In the below example, all three options supported by an `EvaluatingBackend` are presented for demonstrative purposes:

* to run a single circuit,
* to run a measurement, and
* to run a quantum program.

When building the final application, users can select the method that best fits their application's architecture.

In python:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import PauliZProduct, PauliZProductInput
from qoqo import QuantumProgram
from qoqo_quest import Backend
# initialize |psi>
init_circuit = Circuit()
# Apply a RotateY gate with a symbolic angle
# To execute the circuit this symbolic parameter must replaced
# with a real number with the help of a QuantumProgram
init_circuit += ops.RotateX(0, "angle")
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
measurement_input = PauliZProductInput(1, False)
# Read out product of Z on site 0 for register ro_z (no basis change)
z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
# Read out product of Z on site 0 for register ro_x
# (after basis change effectively a <X> measurement)
x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])

# Add a result (the expectation value of H) that is a combination of the PauliProduct
# expectation values
measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

measurement = PauliZProduct(
   constant_circuit=init_circuit,
   circuits=[z_circuit, x_circuit],
   input=measurement_input,
)

# Here we show three alternative options that can be ran:
# a single circuit, a measurement, and a quantum program.

# Create a backend simulating one qubit
backend = Backend(1)

# a) Run a single circuit
(bit_registers, float_registers, complex_registers) = backend.run_circuit(z_circuit)

# b) To run a measurement we need to replace the free parameter by hand
executable_measurement = measurement.substitute_parameters({"angle": 0.2})
expectation_values = backend.run_measurement(executable_measurement)
print(expectation_values)

# c) Run a quantum program
# The QuantumProgram now has one free parameter that must be set when executing it.
# The symbolic value "angle" in the circuits will be replaced by that free parameter
# during execution.
program = QuantumProgram(measurement=measurement, input_parameter_names=["angle"])
# Run the program with 0.1 substituting `angle`
expectation_values = program.run(backend, [0.1])
```

In Rust:

```rust
use std::collections::HashMap;

use roqoqo::Circuit;
use roqoqo::operations as ops;
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use roqoqo::QuantumProgram;
use roqoqo::prelude::EvaluatingBackend;
use roqoqo::prelude::Measure;
use roqoqo_quest::Backend;

// initialize |psi>
let mut init_circuit = Circuit::new();
// Apply a RotateY gate with a symbolic angle
// To execute the circuit this symbolic parameter needs to be replaced
// with a real number with the help of a QuantumProgram
init_circuit += ops::RotateX::new(0, "angle".into());
// Z-basis measurement circuit with 1000 shots
let mut z_circuit = Circuit::new();
z_circuit += ops::DefinitionBit::new("ro_z".to_string(), 1, true);
z_circuit += ops::PragmaRepeatedMeasurement::new("ro_z".to_string(), 1000, None);
// X-basis measurement circuit with 1000 shots
let mut x_circuit = Circuit::new();
x_circuit += ops::DefinitionBit::new("ro_x".to_string(), 1, true);
// Changing to the X basis with a Hadamard gate
x_circuit += ops::Hadamard::new(0);
x_circuit += ops::PragmaRepeatedMeasurement::new("ro_x".to_string(), 1000, None);

// Preparing the measurement input for one qubit
let mut measurement_input = PauliZProductInput::new(1, false);
// Read out product of Z on site 0 for register ro_z (no basis change)
let z_basis_index = measurement_input.add_pauliz_product("ro_z".to_string(), vec![0,]).unwrap();
// Read out product of Z on site 0 for register ro_x
// (after basis change effectively a <X> measurement)
let x_basis_index = measurement_input.add_pauliz_product("ro_x".to_string(), vec![0,]).unwrap();

//Add a result (the expectation value of H) that is a combination of the PauliProduct
// expectation values
let mut linear: HashMap<usize, f64> = HashMap::new();
linear.insert(x_basis_index, 0.1);
linear.insert(z_basis_index, 0.2);
measurement_input.add_linear_exp_val("<H>".to_string(), linear).unwrap();

let measurement = PauliZProduct{
   constant_circuit: Some(init_circuit),
   circuits: vec![z_circuit.clone(), x_circuit],
   input: measurement_input,
};

// Here we show three alternative options that can be ran:
// a single circuit, a measurement, and a quantum program.

// Create a backend simulating one qubit
let backend = Backend::new(1);

// a) Run a single circuit
let (_bit_registers, _float_registers, _complex_registers) = backend.run_circuit(&z_circuit).unwrap();

// b) To run a measurement we need to replace the free parameter by hand
let executable_measurement = measurement.substitute_parameters(HashMap::from([("angle".to_string(), 0.2)])).unwrap();
let expectation_values = backend.run_measurement(&executable_measurement).unwrap();
println!("{expectation_values:?}");

// c) Run a quantum program
// The QuantumProgram now has one free parameter that must be set when executing it.
// The symbolic value "angle" in the circuits will be replaced by that free parameter
// during execution.
let program = QuantumProgram::PauliZProduct{ measurement, input_parameter_names: vec!["angle".to_string()]};
// Run the program with 0.1 substituting `angle`
let expectation_values = program.run(backend, &[0.1]).unwrap();
println!("{expectation_values:?}");
```

## Translating to other quantum computing frameworks

There are many open- and closed-source quantum frameworks. For some use cases, it may be advantageous to interface between qoqo and another quantum computing framework. Depending on the target framework, circuits containing an available subset of qoqo operations can be translated to other frameworks by backends. 

Note: Backends that translate qoqo objects (for example Circuits) to other frameworks or representations do _not_ implement the `EvaluatingBackend`.

Currently, there is one translating backend, from qoqo `Circuits` to qasm: 
| Backend | Description |
| -------- | ------- |
|[qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm)|Translating Backend to translate qoqo circuits to qasm.|
