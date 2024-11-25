# List of Gate Operations

Operations are the atomic instructions in any quantum program that can be represented by qoqo/roqoqo. Gate operations are single-, two- or multi-qubit unitary operations that apply a unitary transformation and can be executed on any universal quantum computer. Mathematically, a gate can be represented by a unitary matrix.

A list of the gate operations available in qoqo and roqoqo with their mathematical description is provided in this section. We differentiate between [single-qubit gates](single_qubit_gates.md) acting on a single qubit, [two-qubit gates](two_qubit_gates.md) applied on a pair of qubits and [multi-qubit gates](multi_qubit_gates.md) affecting a series of qubits.

### Notation

* A rotation angle is usually annotated with \\( \theta \\) and its corresponding argument is `theta`.
* For the phase angle, the symbol \\( \varphi \\) is used.
* The rotation angle  \\( \phi \\)  in the x-y plane is addressed by the argument name `phi`.
* \\( \sigma_x \\), \\( \sigma_y \\), \\( \sigma_z \\) are the Pauli matrices X, Y, Z
\\[
    \sigma_x = \begin{pmatrix} 0 & 1 \\\\ 1 & 0 \end{pmatrix} := X, \quad \sigma_y = \begin{pmatrix} 0 & -i \\\\ i & 0 \end{pmatrix} := Y,  \quad \sigma_z = \begin{pmatrix} 1 & 0 \\\\ 0 & -1 \end{pmatrix} := Z
\\].

## [Single-qubit gates](single_qubit_gates.md)

| Gate | Short Description |
|---------|---------|
| Hadamard     | The Hadamard gate, to create a superposition of states, and so to change the basis.  |
| InvSqrtPauliX     | The inverse square root of the PauliX gate \\( e^{i \frac{\theta}{4} \sigma_x} \\).  |
| PauliX     | The Pauli X gate, a rotation with a fixed angle of \\( \frac{\pi}{2} \\), corresponds to a "flip" on x-axis.  |
| PauliY     | The Pauli Y gate, a rotation with a fixed angle of \\( \frac{\pi}{2} \\), corresponds to a "flip" on y-axis.  |
| PauliZ     | The Pauli Z gate, a rotation with a fixed angle of \\( \frac{\pi}{2} \\), corresponds to a "flip" on z-axis.  |
| PhaseShiftState0     | Rotation around z-axis by angle \\(\theta\\) applied on state \\( \left \|0 \right> \\) results in a phase shift compared to RotateZ gate. |
| PhaseShiftState1     | Rotation around z-axis by angle \\(\theta\\) applied on state \\( \left\|1 \right> \\) results in phase shift compared to RotateZ gate. |
| RotateAroundSphericalAxis     | Implements a rotation around an axis in spherical coordinates.  |
| RotateX     | The rotation gate around x-axis \\( e^{-i \frac{\theta}{2} \sigma_x} \\).  |
| RotateXY     | Implements a rotation around an axis in the x-y plane, where the axis is defined by an angle/spherical coordinates.  |
| RotateY     | The rotation gate around y-axis \\( e^{-i \frac{\theta}{2} \sigma_y} \\).  |
| RotateZ     | The rotation gate around z-axis \\( e^{-i \frac{\theta}{2} \sigma_z} \\).  |
| SGate     | The S gate.  |
| SqrtPauliX     | The square root of the PauliX gate \\( e^{-i \frac{\theta}{4} \sigma_x} \\).  |
| TGate     | The T gate.  |

## [Two-qubit gates](two_qubit_gates.md)

| Gate | Short Description |
|---------|---------|
| Bogoliubov    |  The Bogoliubov DeGennes interaction gate. |
| CNOT    |  The controlled not gate, *e.g.* to entangle two qubits. |
| ComplexPMInteraction    |  The complex hopping gate. |
| ControlledPauliY    |  The controlled PauliY gate. |
| ControlledPauliZ    |  The controlled PauliZ gate. |
| ControlledPhaseShift    |  The controlled phase shift gate. |
| Fsim   |  The fermionic qubit simulation gate. |
| FSwap   |  The fermionic SWAP gate. |
| GivensRotation   |  The Givens rotation interaction gate in big endian notation: \\(e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}\cdot e^{-i \phi Z_t/2} \\). |
| GivensRotationLittleEndian   |  The Givens rotation interaction gate in little-endian notation: \\(e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}\cdot e^{-i \phi Z_c/2} \\). |
| InvSqrtISwap    |  The inverse square root of the ISwap gate. |
| ISwap    |  The complex swap gate. |
| MolmerSorensenXX    |  The fixed-phase Mølmer–Sørensen XX gate. |
| PhaseShiftedControlledZ    |  The phased-shifted controlled-Z gate. |
| PMInteraction    |  The transversal interaction gate. |
| Qsim    |  The qubit simulation gate. |
| SpinInteraction    |  The generalized, anisotropic XYZ Heisenberg interaction between spins. |
| SqrtISwap    |  The square root of the ISwap gate. |
| SWAP    |  The swap gate, to switch the positions of two qubits. |
| VariablesMSXX    |  The variable-angle Mølmer–Sørensen XX gate. |
| XY    |  The XY gate. |

## [Multi-qubit gates](multi_qubit_gates.md)

| Gate | Short Description |
|---------|---------|
| MultiQubitMS    |  The Mølmer–Sørensen gate between multiple qubits. |
| MultiQubitZZ    |  The multi-qubit PauliZ-product gate. |
