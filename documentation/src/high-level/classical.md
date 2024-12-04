# ClassicalRegister Measurement

A `ClassicalRegister` measurement returns the unprocessed register readouts of `Circuits`. 
A `ClassicalRegister` measurement can be used when end-users want to perform their own post-processing, for instance when working with external tools that expect full measurement records. 
 
The registers are returned as a tuple of three dictionaries/HashMaps: 

* The collection of bit registers
* The collection of float registers
* The collection of complex registers

A `ClassicalRegister` measurement does _not_ need a separate `measurement_input` since no post-processing takes place.

To distinguish between a method returning expectation values and a method returning registers, the method `run_registers()` is used when executing a `ClassicalRegister` measurement.

An example for running a `ClassicalRegister` measurement:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import ClassicalRegister
from qoqo import QuantumProgram
from qoqo_quest import Backend

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

measurement = ClassicalRegister(
    constant_circuit=init_circuit,
    circuits=[z_circuit, x_circuit],
)

# A quantum program is created from the measurement
program = QuantumProgram(measurement=measurement, input_parameter_names=[])

# Create a backend simulating one qubit.
backend = Backend(1)

(bit_registers, float_registers, complex_registers) = program.run_registers(backend, [])
```

The same example in Rust:

```Rust
use roqoqo::{Circuit, operations::*, registers::*, QuantumProgram};
use roqoqo::measurements::ClassicalRegister;
use roqoqo::backends::{EvaluatingBackend, RegisterResult};
use roqoqo_quest::Backend;
use std::collections::{HashMap, HashSet};

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

let measurement = ClassicalRegister {
    circuits: vec![z_circuit.clone(), x_circuit.clone()],
    constant_circuit: Some(init_circuit.clone()),
};

// A quantum program is created from the measurement
let program = QuantumProgram::ClassicalRegister {
    measurement,
    input_parameter_names: vec![],
};

// Create a backend simulating one qubit
let backend = Backend::new(1);

let result: RegisterResult = program.run_registers(backend.clone(), &[]);
let result_registers: (
    HashMap<String, BitOutputRegister>,
    HashMap<String, FloatOutputRegister>,
    HashMap<String, ComplexOutputRegister>,
) = result.unwrap();
```
