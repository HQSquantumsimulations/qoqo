# Multi-qubit gates

Multi-qubit gates in qoqo/roqoqo represent atomic instructions in any quantum computer that act on `N` number of qubits. In multi-qubit gates the `qubits` are given as a vector of all involved qubits. The unitary matrix of a multi-qubit gate corresponds to the notation based on `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi-qubit gate.

## ControlledControlledPauliZ

Implements the double-controlled PauliZ gate, with two control qubits and one target qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 0 & -1
 \end{pmatrix}
\\].

## ControlledControlledPhaseShift

Implements the double-controlled PhaseShift gate, with two control qubits and one target qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i \theta}
 \end{pmatrix}
\\].

## MultiQubitMS

The Mølmer–Sørensen gate between multiple qubits. The gate applies the rotation under the product of PauliX operators on multiple qubits. In mathematical terms, the gate applies

\\[
    e^{-i * \theta/2 * X_{i0} * X_{i1} * ... * X_{in}},
\\],

whereas \\(\theta\\) is the angle parameter of the multi-qubit Mølmer–Sørensen gate and `i0`, `i1` *etc.* are the qubits the gate acts on.

## MultiQubitZZ

The multi-qubit PauliZ-product gate. he gate applies the rotation under the product of PauliZ operators on multiple qubits.

\\[
    e^{-i * \theta/2 * Z_{i0} * Z_{i1} * ... * Z_{in}},
\\],

whereas \\(\theta\\) is the angle parameter of the multi-qubit PauliZ-product gate and `i0`, `i1` *etc.* are the qubits the gate acts on.

## Toffoli

Implements the Toffoli, with two control qubits and one target qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\\\
 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0
 \end{pmatrix}
\\].
