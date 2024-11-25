# Noise Operations

qoqo/roqoqo enables the user to construct finely controlled noise models. Noise acting on the quantum computer is modeled as noise operations acting on individual qubits in between each unitary gate applied on the quantum computer.

The noise operations can be directly added to a quantum circuit and can be simulated by compatible backends. Since noise cannot be actively controlled on a quantum computer normally, the noise operations are defined as [Pragma](pragma.md) operations in qoqo/roqoqo. The strength of the noise is determined by defining a `gate_time` and a `rate`. The noise Pragma operation affects the system as a Lindblad type noise acting on the system with the rate `rate` for the time `gate_time`.

_Note_: as long as gate times and rates are scaled inversely any kind of units can be used.

## Example

For example we can add dephasing noise to qubit 0, damping noise to qubit 1, and depolarising noise to qubit 2 after a `CNOT` gate has been applied.

In Python:

```python

from qoqo import Circuit
from qoqo import operations

circuit = Circuit()
circuit += operations.CNOT(0,1)
# Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations.PragmaDephasing(qubit=0, gate_time=1.0, rate=1e-3)
circuit += operations.PragmaDamping(1, 1.0, 2e-3)
circuit += operations.PragmaDepolarising(3, 1.0, 5e-3)

```

In Rust:

```rust
use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
circuit += operations::CNOT::new(0,1);
// Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations::PragmaDephasing::new(0, 1.0, 1e-3.into());
circuit += operations::PragmaDamping::new(1, 1.0, 2e-3.into());
circuit += operations::PragmaDepolarising::new(3, 1.0, 5e-3.into());
```

## Superoperator representation

All noise operations in qoqo/roqoqo provide a `superoperator()` method that returns the definition of the noise operation in superoperator matrix form.
In the superoperator formalism, the density matrix of the system is rewritten as a vector in row-major form. Applying the noise to the quantum computer then corresponds to multiplying the vector with the superoperator matrix.
The superoperator matrix ignores the qubits the noise operation acts on to fit in the smallest possible matrix dimension.

For other methods available for noise operations see the API documentation of [roqoqo](https://docs.rs/roqoqo/latest/roqoqo/operations/index.html) and [qoqo](https://hqsquantumsimulations.github.io/qoqo/generated/generated/qoqo.operations.html).

## Noise operations

The single noise operations shown in the example above are:

* `PragmaDamping`, which applies a pure damping error corresponding to _zero_ temperature environments.
* `PragmaDepolarising`, which applies a depolarising noise.
* `PragmaDephasing`, which applies a pure dephasing noise.

For a stochastically unravelled combination of dephasing and depolarising, the user can choose to use the `PragmaRandomNoise`. The error rate of the depolaristion (`depolarising_rate`) and the error rate of the dephasing (`dephasing_rate`) are provided as input parameters for this random noise operation.

### PragmaGeneralNoise

The most general noise can be modeled in qoqo by the PragmaGeneralNoise operation. This Pragma operation applies a noise term according to the given rates. The rates are represented by a 3x3 matrix:

\\[
 M = \begin{pmatrix}
 a & b & c \\\\
 d & e & f \\\\
 g & h & j \\\\
 \end{pmatrix}
\\],

where the coefficients correspond to the following summands expanded from the first term of the non-coherent part of the Lindblad equation:
\\[
 \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \}
\\],

with \\( L_0 = \sigma^{+} \\), \\( L_1 = \sigma^{-} \\) and \\( L_3 = \sigma_{z} \\).

Applying a Pragma noise operation with a given `gate_time` corresponds to applying the full time-evolution under the Lindblad equation.

## Error Pragmas that are not noise operations

qoqo/roqoqo also supports Pragma operations that lead to errors in the execution of the quantum program that do not correspond to applying physical noise to the system

### PragmaOverrotation

This operation applies a statistical overrotation to the next rotation gate in the circuit, which matches the name given in the `gate` parameter of `PragmaOverrotation` and the involved qubits provided in `qubits`. The applied overrotation corresponds to adding a random number to the rotation angle.
The random number is drawn from a normal distribution with mean `0` and standard deviation whose variance is given by the input parameter `variance`, which is then multiplied by the `amplitude` parameter.

### PragmaBoostNoise

This operation boosts noise and overrotations in the circuit. The input parameter `noise_coefficient` defines the coefficient by which the noise is boosted, *i.e.* the number by which the `gate_time` is multiplied.

### PragmaSleep

This operation makes the quantum computer hardware, that provides this functionality, wait a given amount of time (`sleep_time`). Waiting for a given time can increase the effect of continuous noise on the result of running a quantum circuit. This is sometimes used in noise-extrapolation error mitigation techniques.