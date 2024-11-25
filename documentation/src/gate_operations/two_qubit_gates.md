# Two-qubit gates

Two-qubit gates in qoqo/roqoqo represent atomic instructions of any quantum computer that act on a pair of qubits. In two-qubit gates the two qubits are referred to as `control` and `target`. When initializing two-qubit gates, the `control` is always the first argument and `target` the second argument. For two-qubit gates, the created matrix always corresponds to `control=1`, `target=0`, and is a 4x4-dimensional matrix. This convention corresponds to the little-endian encoding as described in chapter [conventions](../conventions.md). The full matrix form of the two-qubit gates implemented in qoqo/roqoqo is documented in this chapter.

The given form of the unitary matrix is consistent with the following ordering of qubit states in a two-qubit state space:

 \\[
 \left|00 \right>  =  \textrm{state} 0 \\\\
 \left|01 \right>  =  \textrm{state} 1 \\\\
 \left|10 \right>  =  \textrm{state} 2 \\\\
 \left|11 \right>  =  \textrm{state} 3 \\\\
 \\].

## Bogoliubov

The Bogoliubov DeGennes gate representing interactions of the form:
\\[ 
 e^{-\mathrm{i} \mathrm{Re}(\Delta)\cdot(X_c X_t - Y_c Y_t)/2 + \mathrm{Im}(\Delta)\cdot(X_c Y_t+Y_c X_t)/2},
 \\],

where \\(X_c\\) is the Pauli matrix \\(\sigma_x\\) acting on the control qubit, and \\(Y_t\\) is the Pauli matrix \\(\sigma_y\\) acting on the target qubit, and \\( \Delta \\) is the complex Bogoliubov interaction strength.

The full matrix form is given by:

\\[
 U = \begin{pmatrix}
 \cos(|\Delta|) & 0 & 0 & \mathrm{i} \sin(|\Delta|) e^{\mathrm{i} \cdot \varphi(\Delta)} \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 \mathrm{i} \sin(|\Delta|) e^{-\mathrm{i} \cdot \varphi(\Delta)} & 0 & 0 & \cos(|\Delta|)
 \end{pmatrix}
\\],

with the value \\(|\Delta|\\) of the complex Bogoliubov interaction strength \\( \Delta \\), and its phase angle \\(\varphi(\Delta)\\).

## CNOT

The controlled not gate can be used to entangle two qubits. The unitary matrix for the CNOT gate is defined as:

\\[
 U  = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 1 \\\\
 0 & 0 & 1 & 0
 \end{pmatrix}
 \\].

 ## ComplexPMInteraction

This gate implements the complex hopping operation of the form:

\\[
 e^{-\mathrm{i} \left[ \mathrm{Re}(\theta) \cdot (X_c X_t + Y_c Y_t) - \mathrm{Im}(\theta) \cdot (X_c Y_t - Y_c X_t) \right] } , 
\\],

where \\(X_c\\) is the Pauli matrix \\(\sigma_x\\) acting on the control qubit, and \\(Y_t\\) is the Pauli matrix \\(\sigma_y\\) acting on the target qubit. The complex parameter \\( \theta \\) gives the strength of the rotation.

## ControlledPauliY

The controlled PauliY gate applies the PauliY gate on the `target` qubit based on the value of the `control` qubit. The corresponding unitary matrix reads:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & -\mathrm{i} \\\\
 0 & 0 & \mathrm{i}  & 0
 \end{pmatrix}
 \\].

## ControlledPauliZ

The controlled PauliZ gate applies the PauliZ gate on the `target` qubit controlled by the `control` qubit. The definition for the unitary matrix is as follows:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 0 & 0 & -1
 \end{pmatrix}
 \\].

## ControlledPhaseShift

The controlled phase shift gate implements a phase shift applied on `target` qubit based on the value of the `control` qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 0 & 0 & e^{\mathrm{i} \theta}
 \end{pmatrix}
\\].

## ControlledRotateX

The controlled RotateX implements a RotateX gate applied on `target` qubit based on the value of the `control` qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & \cos(\frac{\theta}{2}) & -i \sin(\frac{\theta}{2}) \\\\
 0 & 0 & -i \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
 \end{pmatrix}
\\].

## ControlledRotateXY

The controlled RotateXY implements a RotateXY gate applied on `target` qubit based on the value of the `control` qubit. The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & \cos(\frac{\theta}{2}) & -i e^{-i \phi} \sin(\frac{\theta}{2}) \\\\
 0 & 0 & -i e^{-i \phi} \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
 \end{pmatrix}
\\].

## Fsim

The fermionic qubit simulation gate that can be represented as the following unitary matrix:

\\[
 U = \begin{pmatrix}
 \cos(\Delta) & 0 & 0 & \mathrm{i} \sin(\Delta) \\\\
 0 & -\mathrm{i} \sin(t) & \cos(t) & 0 \\\\
 0 & \cos(t) & -\mathrm{i} \sin(t) & 0 \\\\
 -\sin(\Delta) \cdot e^{-\mathrm{i} U} & 0 & 0 & -\cos(\Delta) \cdot e^{-\mathrm{i} U}
 \end{pmatrix}
\\],

where `t` is the hopping strength, `U` is the interaction strength and \\( \Delta \\) is the Bogoliubov interaction strength.

_Note_: The qubits have to be adjacent, *i.e.*, \\( |i-j|=1 \\) has to hold. This is the only case in which the gate is valid as a two-qubit gate (due to the Jordan-Wigner transformation).


## FSwap

The fermionic SWAP gate can be represented as the following unitary matrix:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & -1
 \end{pmatrix}
\\].

## GivensRotation

The Givens rotation interaction gate in big endian notation:
\\[
    e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}\cdot e^{-\mathrm{i} \phi Z_t/2},
\\],

where \\(X_c\\) is the Pauli matrix \\(\sigma_x\\) acting on the control qubit, \\(Y_t\\) is the Pauli matrix \\(\sigma_y\\) acting on the target qubit, and \\(Z_t\\) is the Pauli matrix \\(\sigma_z\\) acting on the target qubit.

The unitary matrix representation is:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \cos(\theta) \cdot e^{\mathrm{i} \phi} & \sin(\theta)  & 0 \\\\
 0 & -\sin(\theta) \cdot e^{\mathrm{i} \phi} & \cos(\theta)  & 0 \\\\
 0 & 0 & 0 & e^{\mathrm{i} \phi}
 \end{pmatrix}
\\].

Further information on the endianness in context of qoqo can be found in the section [conventions](../conventions.md).

## GivensRotationLittleEndian

The Givens rotation interaction gate in little-endian notation:
\\[
    e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}\cdot e^{-\mathrm{i} \phi Z_c/2},
\\],
with Pauli matrices \\( X := \sigma_x\\), \\( Y := \sigma_y\\), \\( Z := \sigma_z\\) and indices `c` for control qubit and `t` for target qubit.

The unitary matrix form is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \cos(\theta) & \sin(\theta)  & 0 \\\\
 0 & -\sin(\theta) \cdot e^{\mathrm{i} \phi} & \cos(\theta) \cdot e^{\mathrm{i} \phi}  & 0 \\\\
 0 & 0 & 0 & e^{\mathrm{i} \phi}
 \end{pmatrix}
\\].

## InvSqrtISwap

The inverse square root of the ISwap gate has the full matrix form:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \frac{1}{\sqrt{2}} & \frac{-\mathrm{i}}{\sqrt{2}} & 0 \\\\
 0 & \frac{-\mathrm{i}}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\].

## ISwap

The unitary matrix of the complex ISwap gate reads:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & \mathrm{i} & 0 \\\\
 0 & \mathrm{i} & 0 & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\].

## MolmerSorensenXX

This gate implements the fixed-phase MolmerSorensen XX operation as introduced in this [reference](http://arxiv.org/abs/1705.02771). The MolmerSorensenXX gate can be used to represent global laser-driver entangling operations, for example in the context of quantum computing hardware based on trapped ions. The unitary matrix is given by:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & 0 & 0 & -\mathrm{i} \\\\
 0 &1 & -\mathrm{i} & 0 \\\\
 0 & -\mathrm{i} & 1 & 0 \\\\
 -\mathrm{i} & 0 & 0 & 1
 \end{pmatrix}
\\].

## PhaseShiftedControlledPhase

The phase-shifted controlled PhaseShift gate of the form:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & e^{\mathrm{i} \phi} & 0 & 0 \\\\
 0 & 0 & e^{\mathrm{i} \phi} & 0 \\\\
 0 & 0 & 0 & e^{\mathrm{i} (2\cdot\phi - \pi)}
 \end{pmatrix}
\\],

with the single-qubit phase \\( \phi \\).

## PhaseShiftedControlledZ

The phase-shifted controlled-Z gate, modified, *i.e.* phase-shifted ControlledPauliZ two-qubit gate, which corresponds to the equation (1) in the original [reference](https://arxiv.org/pdf/1908.06101.pdf). The full matrix form is defined as:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & e^{\mathrm{i} \phi} & 0 & 0 \\\\
 0 & 0 & e^{\mathrm{i} \phi} & 0 \\\\
 0 & 0 & 0 & e^{\mathrm{i} (2\cdot\phi - \pi)}
 \end{pmatrix}
\\],

with the single-qubit phase \\( \phi \\).

## PMInteraction

The transversal interaction gate of the form:

\\[
 e^{-\mathrm{i} \theta (X_c X_t + Y_c Y_t)} = e^{-\mathrm{i} \theta (\sigma^+_c \sigma^-_t + \sigma^-_c \sigma^+_t)} , 
\\],

where \\(X_c\\) is the Pauli matrix \\(\sigma_x\\) acting on the control qubit, and \\(Y_t\\) is the Pauli matrix \\(\sigma_y\\) acting on the target qubit. The parameter \\( \theta \\) gives the strength of the rotation.

## Qsim

The Qsim gate implements a `SWAP` between two spins and a `SpinInteraction` simultaneously. In this context, spins are meant to be localized and therefore have distinguishable degrees of freedom. The definition of the Qsim gate in matrix form is given by:


\\[
 U = \begin{pmatrix}
 \cos(x-y) \cdot e^{-\mathrm{i} z} & 0 & 0 & -\mathrm{i}\sin(x-y)\cdot e^{-\mathrm{i} z}\\\\
 0 & -\mathrm{i} \sin(x+y)\cdot e^{\mathrm{i} z} & \cos(x+y)\cdot e^{\mathrm{i} z} & 0 \\\\
 0 & \cos(x+y)\cdot e^{\mathrm{i} z}& -\mathrm{i} \sin(x+y)\cdot e^{\mathrm{i} z} & 0 \\\\
 -\sin(x-y)\cdot e^{-\mathrm{i} z} & 0 & 0 & \cos(x-y)\cdot e^{-\mathrm{i} z}
 \end{pmatrix}
\\],

where x, y, z are the prefactors of the products of Pauli matrices \\(X_c X_t\\), \\(Y_c Y_t\\), \\(Z_c Z_t\\) acting on the control (`c`) and target (`t`) qubits that are part of the spin interaction.

## SpinInteraction

The gate represents the generalized, anisotropic XYZ Heisenberg interaction between spins of the form:

\\[
 e^{-\mathrm{i} (x \cdot X_c X_t + y \cdot Y_c Y_t + z \cdot Z_c Z_t)} , 
\\],

where x, y, z are the prefactors of the products of Pauli matrices \\(X_c X_t\\), \\(Y_c Y_t\\), \\(Z_c Z_t\\) acting on control (`c`) and target (`t`) qubit,
with the underlying definition \\(XX \equiv \sigma_x \sigma_x\\), \\(YY \equiv \sigma_y \sigma_y\\) and \\(ZZ \equiv \sigma_z \sigma_z\\). 

In this context, spins are meant to be localized and therefore have distinguishable degrees of freedom.

## SqrtISwap

The square root of the ISwap gate is represented by the matrix:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \frac{1}{\sqrt{2}} & \frac{\mathrm{i}}{\sqrt{2}} & 0 \\\\
 0 & \frac{\mathrm{i}}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\].

## SWAP

The SWAP gate is used to change the positions between two qubits. For example, the SWAP gate is used by many compilation routines if the given connectivity on the quantum computing device is limited and the qubits need to be remapped in order to run a quantum program successfully on the quantum computing hardware. The full matrix form is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\].

## VariablesMSXX

The variable-angle MolmerSorensen XX gate as defined by the unitary matrix of the form:

\\[
 U = \begin{pmatrix}
 \cos(\theta/2) & 0 & 0 & -\mathrm{i} \sin(\theta/2) \\\\
 0 & \cos(\theta/2) & -\mathrm{i} \sin(\theta/2) & 0 \\\\
 0 & -\mathrm{i} \sin(\theta/2) & \cos(\theta/2) & 0 \\\\
 -\mathrm{i}\sin(\theta/2) & 0 & 0 & \cos(\theta/2)
 \end{pmatrix}
\\].

In general, the MolmerSorensenXX gate can be used to represent global laser-driver entangling operations, for example in the context of quantum computing hardware based on trapped ions, as introduced in this [reference](http://arxiv.org/abs/1705.02771).

## XY

The definition of the XY gate in matrix form is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \cos(\theta/2) & \mathrm{i} \sin(\theta/2) & 0 \\\\
 0 & \mathrm{i} \sin(\theta/2) & \cos(\theta/2) & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\].

It implements the same interaction as described by the `PMInteraction` gate but with a different prefactor.
