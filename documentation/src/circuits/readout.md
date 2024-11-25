# Readout

To obtain results from running a quantum circuit, the quantum computer needs to return classical information.
qoqo/roqoqo uses register-based readouts where all classical information is returned from the quantum circuit using classical registers declared at the start of the circuit. 
Classical registers can contain three types of classical data:

* Bit (or bool)
* Float (f64/double)
* Complex.

Each register is declared in a `Circuit` by a Pragma operation setting the register name and a length. The Pragma operation also declares whether the register is an output or not.
After being declared at the start of the circuit, information is written to the registers in the `Circuit` by `Measurement` or `Pragma` operations.
If the register is declared as an output register, it is returned after the execution of the circuit.

A python example:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
# A bit register that is not returned
circuit += ops.DefinitionBit("bit_register", length=2, is_output=False)
# A float register that is returned
circuit += ops.DefinitionFloat("float_register", length=2, is_output=True)
# A complex register that is not returned
circuit += ops.DefinitionComplex("complex_register", length=3, is_output=False)
```

A Rust example:

```rust
use roqoqo::Circuit;
use roqoqo::operations;

fn main(){
let mut circuit = Circuit::new();
// A bit register of length 2 that is not returned
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, false);
// A float register of length 2 that is returned as an output of the circuit
circuit += operations::DefinitionFloat::new("float_register".to_string(), 2, true);
// A complex register of length 3 that is not returned as an output
circuit += operations::DefinitionComplex::new("complex_register".to_string(), 3, false);
}
```

## Writing to registers

Information is written to registers by the `MeasureQubit` operation or Pragma operations in a quantum circuit.

* On quantum computing _hardware_ only _projective_ measurements into a bit register are available, that is a measurement in the `Z`-basis yielding `0` or `1`.
* On _simulators_ one can also read out the full state vector or density matrix into a complex register.

`MeasureQubit` corresponds directly to a _projectivce_ measurement. By definition, projective measurements are available on universal quantum computers. 
`PragmaRepeatedMeasurement` is shorthand for repeatedly running the circuit and applying a projective measurement each time. While it is not necessarily available on every [backend](backends.md) it is compatible with hardware quantum computers.

As shown in the example below, the operation `MeasureQubit` can be used to provide measurement instructions for each individual qubit. The input parameter `qubit` specifies the qubit to be measured, whereas the parameter `readout_index` defines the position in the classical register `readout` where the measurement value of the `qubit` is stored. The explicit assignment of a qubit measurement to a readout register index can be used to handle qubit remapping in a quantum circuit.

When supported by the backend, `PragmaRepeatedMeasurement` can be used instead of `MeasureQubit` command to provide the measurement instruction for all qubits in `qubit_mapping` that needs to be repeated N times (`number_measurements`). For further available Pragma measurement instructions, please refer to the section [Pragma operations](pragma.md).

Setting up readouts in Python:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
# Add a Bit register to the circuit for the qubit readout
circuit += ops.DefinitionBit("bit_register", 2, is_output = True)
# Add measurement instructions for each qubit, when using hardware
circuit += ops.MeasureQubit(qubit=0, readout="bit_register", readout_index=0)
circuit += ops.MeasureQubit(qubit=1, readout="bit_register", readout_index=1)

# Alternatively, define a Complex register to readout the state vector
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
# Measure the state vector when running the circuit on a simulator
circuit += ops.PragmaGetStateVector("complex_register", None)
```

Setting up readouts in Rust:

```rust
use roqoqo::Circuit;
use roqoqo::operations;


let mut circuit = Circuit::new();
// Add a Bit register to the circuit for the qubit readout
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
// Add measurement instructions for each qubit, when using hardware
circuit += operations::MeasureQubit::new(0, "bit_register".to_string(), 0);
circuit += operations::MeasureQubit::new(1, "bit_register".to_string(), 1);

// Alternatively, define a Complex register to readout the state vector
circuit += operations::DefinitionComplex::new(
    "complex_register".to_string(), 3, false,
);
// Measure the state vector when running the circuit on a simulator
circuit += operations::PragmaGetStateVector::new(
    "complex_register".to_string(), None,
);
```
