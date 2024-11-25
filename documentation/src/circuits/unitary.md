# Unitary Operations

Unitary operations or gate operations are operations that can be executed on all universal quantum computers. The operations correspond to unitary transformations of the state of a quantum computer.

## Unitary Matrix

Gate operations in qoqo/roqoqo provide a `unitary_matrix()` method that returns the definition of the gate in matrix form. This definition ignores the qubits the gates acts on to fit in the smallest possible matrix dimension.

* For single-qubit gates, the created matrix always corresponds to `qubit=0` and has dimension 2x2.
* For two-qubit gates, the created matrix always corresponds to `control=1`, `target=0` and has dimension a 4x4. See also the state ordering [conventions](../conventions.md).
* For multi-qubit gates, the created matrix always corresponds to `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi-qubit gate.

For a list of unitary operations see [gate operations](../gate_operations/intro.md).
