# Conventions

This section gives a quick overview of some of the conventions used in qoqo/roqoqo.

## Definitions

* `operation`: An atomic instruction applied to a quantum computer (or simulated quantum computer).
* `gate`: An `operation` that corresponds to a unitary transformation of the state of the quantum computer and can be implemented on all universal quantum computers.
* `qubit`: A quantum bit. Can be in a superposition of two basis states.
* `Circuit`: A linear sequence of `operations`.

## Qubit states

For the two basis states of a single qubit we define

\\[
 \left\| 0 \right>  =  \left|\textrm{false} \right> =  \left| \uparrow \right> = \begin{pmatrix}
 1 \\\\
 0
 \end{pmatrix} \\\\
 \\].

 \\[
 \left \|1 \right>  =  \left|\textrm{true} \right> =  \left| \downarrow \right> = \begin{pmatrix}
 0 \\\\
 1
 \end{pmatrix} \\\\
 \\].

Before  any operations are applied in a circuit a quantum computer is always assumed to be in the zero state (all qubits in state `|0>`).

### Note

This convention implies that `|0>` is the **excited** state with respect to the `Z` Pauli operator and `|1>` is the **ground** state. This is in contract with the physical implementation of qubits, where `|0>` typically corresponds to the state with lower energy and damping will lead to the system relaxing from `|1>` to `|0>`.

This means that in this convention, when identifying the qubits with spins with respect to the `Z` operator, the system starts out in the highest energy case and damping leads to a heating effect where the system population shifts to higher energy spin states.

## Endianness

qoqo and roqoqo use little-endian encoding, where the least significant qubit is at the smallest memory address (or at the lowest index in a vector and at the rightmost entry when writing the qubit state as a sequence of `0` and `1` like a binary number).

For a two-qubit state space we write the states of the qubits in the following order:
 \\[
 \left|00 \right>  =  \textrm{state} 0 \\\\
 \left|01 \right>  =  \textrm{state} 1 \\\\
 \left|10 \right>  =  \textrm{state} 2 \\\\
 \left|11 \right>  =  \textrm{state} 3 \\\\
 \\].

## Operation order

 When adding qoqo/roqoqo operations to circuits, the first operation added will be executed first. When writing qoqo/roqoqo operations, they are read left to right. This leads to an inversion of the order when transcribing to matrix form, where the matrix to the right acts first.

 \\[
 \textrm{PauliX}(0) \cdot  \textrm{PauliZ}(0)  \\\\ =
   \textrm{PauliZ(0).unitary_matrix()} \cdot  \textrm{PauliX(0).unitary_matrix()}  \\\\
   = \begin{pmatrix}
 1 & 0 \\\\
 0 & -1
 \end{pmatrix}  \begin{pmatrix}
 0 & 1 \\\\
 1 & 0
 \end{pmatrix}
 \\].

## Qubit naming

qoqo uses a unified naming scheme for qubits in operations

* In single-qubit operations, the qubit is always referred to as `qubit`,
* In two-qubit gates, the two qubits are referred to as `control` and `target`,
* In multi-qubit gates, the ordered list/vector of qubits the gates acts on is referred to as `qubits`.

When initializing two-qubit gates, the `control` is always the first argumemt and `target` the second argument.

## Unitary Matrix

Unitary operations in qoqo/roqoqo provide a `unitary_matrix()` method that returns the definition of the gate in matrix form. This definition ignores the qubits of the gate to fit in the smallest possible matrix dimension.

* For single-qubit gates, the created matrix always corresponds to `qubit=0` and has a 2x2-dimension.
* For two-qubit gates, the created matrix always corresponds to `control=1`, `target=0` and is a 4x4-dimensional matrix. This convention corresponds to the little-endian encoding described above.
* For multi-qubit gates, the created matrix always corresponds to `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi-qubit gate.
