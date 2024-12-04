# Single-qubit gates

Single-qubit gates in qoqo/roqoqo represent atomic instructions of any quantum computer that act on a single qubit. In single-qubit gates the qubit is always referred to as `qubit`. The unitary matrices of single-qubit gates are 2x2-dimensional matrices applied on single-qubit states \\( \left \|0 \right> \\) and \\( \left \|1 \right> \\), as defined in chapter [conventions](../conventions.md). 

The most general unitary operation acting on one qubit is of the form 
\\[ 
 U =e^{\mathrm{i} \phi}\begin{pmatrix}
 \alpha_r+\mathrm{i} \alpha_i & -\beta_r+\mathrm{i} \beta_i \\\\
 \beta_r+\mathrm{i} \beta_i & \alpha_r-\mathrm{i}\alpha_i
 \end{pmatrix}
 \\].

The parameters \\( \alpha_r \\), \\( \alpha_i \\) and \\( \beta_r \\), \\( \beta_i \\) can be accessed by the functions `alpha_r()`, `alpha_i()`, `beta_r()` and `beta_i()`, applied on the particular single-qubit gate. The full matrix form of the single-qubit gates available in qoqo/roqoqo is documented in this chapter.

## GPi

The unitary matrix of the GPi gate, which is often used in the context of ion traps, is defined as follows:

\\[
 U = \begin{pmatrix}
 0 & e^{-i \theta}\\\\
 e^{i \theta} & 0
 \end{pmatrix}
\\].

## GPi2

The unitary matrix of the GPi2 gate, which is often used in the context of ion traps, is defined as follows:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & -i e^{-i \theta}\\\\
 -i e^{i \theta} & 1
 \end{pmatrix}
\\].

## Hadamard

The Hadamard gate when applied creates a superposition of states, and can therefore be used to change the basis if required. The definition of the gate in matrix form is given by:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & 1 \\\\
  1 & -1
 \end{pmatrix}
\\].

## InvSqrtPauliX

The inverse square root of the PauliX gate \\( e^{\mathrm{i} \frac{\theta}{4} \sigma_x} \\) corresponds to a unitary matrix defined as:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & \mathrm{i} \\\\
  \mathrm{i} & 1
 \end{pmatrix}
\\].

 On some hardware platforms, the gate operation `InvSqrtPauliX` together with the operation `SqrtPauliX` are the only available rotation gates. This becomes relevant when it comes to the compilation of a quantum algorithm containing any arbitrary gates to the set of basic gates supported by the hardware device.

## PauliX

The Pauli X gate implements a rotation of \\( \frac{\pi}{2} \\) about the x-axis that can be used, for example, to flip the qubit state. The full matrix form is given by:

\\[
 U = \begin{pmatrix}
 0 & 1 \\\\
 1 & 0
 \end{pmatrix}
\\].

## PauliY

The Pauli Y gate implements a rotation of \\( \frac{\pi}{2} \\) about the y-axis that can be used, for example, to flip the qubit state. The unitary matrix is defined as:

\\[
 U = \begin{pmatrix}
 0 & -\mathrm{i} \\\\
 \mathrm{i} & 0
 \end{pmatrix}
\\].

## PauliZ

The Pauli Z gate implements a rotation of \\( \frac{\pi}{2} \\) about the z-axis that can be used, for example, to flip the qubit state. The full matrix form is given by:

\\[
 U = \begin{pmatrix}
 1 & 0 \\\\
 0 & -1
 \end{pmatrix}
\\].

## PhaseShiftState0

This gate operation corresponds to the phase shift gate applied on state \\( \left \|0 \right> \\) compared to RotateZ gate. It implements a rotation around Z-axis by an arbitrary angle \\(\theta\\), also known as AC Stark shift of the state \\( \left \|0 \right> \\). The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
 e^{\mathrm{i} \theta} & 0\\\\
  0 & 1
 \end{pmatrix}
\\].

## PhaseShiftState1

This gate operation corresponds to the phase shift gate applied on state \\( \left \|1 \right> \\) compared to RotateZ gate. It implements a rotation around Z-axis by an arbitrary angle \\(\theta\\), also known as AC Stark shift of the state \\( \left \|1 \right> \\). The unitary matrix is given by:

\\[
 U = \begin{pmatrix}
  1 & 0\\\\
  0 & e^{\mathrm{i} \theta}
 \end{pmatrix}
\\].

## RotateAroundSphericalAxis

Implements a rotation around an axis in the x-y plane in spherical coordinates. The definition of the gate in matrix form is given by:

\\[
 U = \begin{pmatrix}
 \cos\left(\frac{\theta}{2}\right) & 0\\\\
 0 & \cos\left(\frac{\theta}{2}\right)
 \end{pmatrix}
 \+ \begin{pmatrix}
 -\mathrm{i} \sin\left(\frac{\theta}{2}\right) v_z  &  \sin\left(\frac{\theta}{2}\right) \left(-i v_x - v_y \right)\\\\
 \sin\left(\frac{\theta}{2}\right) \left(-\mathrm{i} v_x + v_y \right) & \mathrm{i} \sin\left(\frac{\theta}{2}\right) v_z
 \end{pmatrix}
\\],


with \\[ v_x = \sin\left(\theta_{sph}\right) \cdot \cos\left(\phi_{sph}\right), \quad v_y = \sin\left(\theta_{sph}\right)\cdot\sin\left(\phi_{sph}\right), \quad v_z = \cos\left(\theta_{sph}\right). \\].

## RotateX

The rotation gate around x-axis \\( e^{-\mathrm{i} \frac{\theta}{2} \sigma_x} \\). The definition of the unitary matrix is as follows:

\\[
 U = \begin{pmatrix}
 \cos(\frac{\theta}{2}) & -\mathrm{i} \sin(\frac{\theta}{2})\\\\
 -\mathrm{i}\sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
 \end{pmatrix}
\\].

## RotateXY

Implements a rotation around an axis in the x-y plane, where the axis is defined by an angle/spherical coordinates. The unitary matrix representing the gate is given by:

\\[
 U  = \begin{pmatrix}
 \cos \left(\frac{\theta}{2} \right) & -\mathrm{i} e^{-\mathrm{i} \phi} \sin \left(\frac{\theta}{2} \right) \\\\
 -\mathrm{i} e^{\mathrm{i} \phi} \sin \left( \frac{\theta}{2} \right) & \cos\left( \frac{\theta}{2} \right)
 \end{pmatrix}
 \\].

## RotateY

The rotation gate around the y-axis \\( e^{-\mathrm{i} \frac{\theta}{2} \sigma_y} \\). The full matrix form is given by:

\\[
 U = \begin{pmatrix}
 \cos(\frac{\theta}{2}) &  - \sin(\frac{\theta}{2})\\\\
 \sin(\frac{\theta}{2})  & \cos(\frac{\theta}{2})
 \end{pmatrix}
\\].

## RotateZ

The rotation gate around the z-axis \\( e^{-\mathrm{i} \frac{\theta}{2} \sigma_z} \\). The unitary matrix reads:

\\[
 U = \begin{pmatrix}
 \cos(\frac{\theta}{2})  -\mathrm{i} \sin(\frac{\theta}{2}) & 0\\\\
 0 & \cos(\frac{\theta}{2}) + \mathrm{i} \sin(\frac{\theta}{2})
 \end{pmatrix}
\\].

## SGate

The unitary matrix of the S gate, which is often used in the theory of error correction, reads:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & 0 \\\\
  0 & \mathrm{i}
 \end{pmatrix}
\\].

## SqrtPauliX

The square root of the PauliX gate \\( e^{-\mathrm{i} \frac{\theta}{4} \sigma_x} \\). The full matrix form is given by:

\\[
 U = \frac{1}{\sqrt(2)}\begin{pmatrix}
 1 & -\mathrm{i} \\\\
 -\mathrm{i} & 1
 \end{pmatrix}
\\].

On some hardware platforms, the gate operation `SqrtPauliX` together with the operation `InvSqrtPauliX` are the only available rotation gates. This becomes relevant when it comes to the compilation of a quantum algorithm containing any arbitrary gates to the set of basic gates supported by the hardware device.

## TGate

The unitary matrix of the T gate, which is often used in the theory of error correction, is defined as follows:

\\[
 U = \frac{1}{\sqrt{2}} \begin{pmatrix}
 1 & 0 \\\\
  0 & e^{\mathrm{i} \frac{\pi}{4}}
 \end{pmatrix}
\\].
