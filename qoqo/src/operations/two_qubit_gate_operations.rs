// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use num_complex::Complex64;
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};
use qoqo_macros::*;
use roqoqo::operations::*;
use std::collections::HashMap;

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled NOT quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 \\\\
///         0 & 0 & 1 & 0
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of NOT on the target qubit.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit NOT is applied to.
///
pub struct CNOT {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled SWAP quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///
pub struct SWAP {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled ISwap quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 0 & i & 0 \\\\
///         0 & i & 0 & 0 \\\\
///         0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///
pub struct ISwap {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled square root ISwap quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & \frac{1}{\sqrt{2}} & \frac{i}{\sqrt{2}} & 0 \\\\
///         0 & \frac{i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
///         0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///
pub struct SqrtISwap {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled inverse square root ISwap quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & \frac{1}{\sqrt{2}} & \frac{-i}{\sqrt{2}} & 0 \\\\
///         0 & \frac{-i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
///         0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///
pub struct InvSqrtISwap {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled fermionic SWAP gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & -1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///
pub struct FSwap {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The fixed phase MolmerSorensen XX gate. <http://arxiv.org/abs/1705.02771>
///
/// .. math::
///     U = \frac{1}{\sqrt{2}} \begin{pmatrix}
///         1 & 0 & 0 & -i \\\\
///         0 & 1 & -i & 0 \\\\
///         0 & -i & 1 & 0 \\\\
///         -i & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
///     target (int): The index of the least significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
///
pub struct MolmerSorensenXX {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
/// The variable-angle MolmerSorensen XX gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\theta/2) & 0 & 0 & -i \sin(\theta/2) \\\\
///         0 & \cos(\theta/2) & -i \sin(\theta/2) & 0 \\\\
///         0 & -i \sin(\theta/2) & \cos(\theta/2) & 0 \\\\
///         -i \sin(\theta/2) & 0 & 0 & \cos(\theta/2)
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
///     target (int): The index of the least significant qubit in the unitary representation. The gate is symmetric under the exchange of qubits.
///     theta (CalculatorFloat): The rotation angle :math:`\theta`.
///
pub struct VariableMSXX {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
/// The Givens rotation interaction gate in big endian notation: :math:`e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}`.
///
/// Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
/// and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & \cos(\theta) \cdot e^{i \phi} & \sin(\theta) & 0 \\\\
///         0 & -\sin(\theta) \cdot e^{i \phi} & \cos(\theta) & 0 \\\\
///         0 & 0 & 0 & e^{i \phi}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///     theta (CalculatorFloat): The rotation angle :math:`\theta`.
///     phase (CalculatorFloat): The phase :math:`\phi` of the rotation.
///
pub struct GivensRotation {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
/// The Givens rotation interaction gate in little endian notation: :math:`e^{-\mathrm{i} \theta (X_c Y_t - Y_c X_t)}`.
///
/// Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
/// and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & \cos(\theta) & \sin(\theta) & 0 \\\\
///         0 & -\sin(\theta) \cdot e^{i \phi} & \cos(\theta) \cdot e^{i \phi} & 0 \\\\
///         0 & 0 & 0 & e^{i \phi}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///     theta (CalculatorFloat): The rotation angle :math:`\theta`.
///     phase (CalculatorFloat): The phase :math:`\phi` of the rotation.
///
pub struct GivensRotationLittleEndian {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
/// The controlled XY quantum operation
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\
///         0 & cos(\theta/2) & i sin(\theta/2) & 0 \\\
///         0 & i sin(\theta/2) & cos(\theta/2) & 0 \\\
///         0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int): The index of the least significant qubit in the unitary representation.
///     theta (CalculatorFloat): The rotation angle :math:`\theta`.
///
pub struct XY {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
/// The controlled-PhaseShift quantum operation.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & e^{i \theta}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     theta (CalculatorFloat): The rotation angle :math:`\theta`.
///
pub struct ControlledPhaseShift {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled PauliY quantum operation
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & -i \\\\
///         0 & 0 & i & 0
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of PauliY gate on the target qubit.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliY is applied to.
///
pub struct ControlledPauliY {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
#[derive(Eq)]
/// The controlled PauliZ quantum operation
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & -1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of PauliZ gate on the target qubit.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
///
pub struct ControlledPauliZ {
    control: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The qubit simulation (Qsim) gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(x-y) \cdot e^{-i z} & 0 & 0 & -i\sin(x-y)\cdot e^{-i z}\\\\
///         0 & -i \sin(x+y)\cdot e^{i z} & \cos(x+y)\cdot e^{i z} & 0 \\\\
///         0 & \cos(x+y)\cdot e^{i z}& -i \sin(x+y)\cdot e^{i z} & 0 \\\\
///         -\sin(x-y)\cdot e^{-i z} & 0 & 0 & \cos(x-y)\cdot e^{-i z}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     x (CalculatorFloat): The prefactor of the XX interaction.
///     y (CalculatorFloat): The prefactor of the YY interaction.
///     z (CalculatorFloat): The prefactor of the ZZ interaction.
///
pub struct Qsim {
    control: usize,
    target: usize,
    x: CalculatorFloat,
    y: CalculatorFloat,
    z: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The fermionic qubit simulation (Fsim) gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\Delta) & 0 & 0 & i \sin(\Delta) \\\\
///         0 & -i \sin(t) & \cos(t) & 0 \\\\
///         0 & \cos(t) & -i \sin(t) & 0 \\\\
///         -\sin(\Delta) \cdot e^{-i U} & 0 & 0 & -\cos(\Delta) \cdot e^{-i U}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     t (CalculatorFloat): The hopping strength.
///     u (CalculatorFloat): The interaction strength.
///     delta (CalculatorFloat): The Bogoliubov interaction strength :math:`\Delta`.
///
/// Note:
/// The qubits have to be adjacent, i.e., :math:`|i-j|=1` has to hold. This is the only case
/// in which the gate is valid as a two-qubit gate (due to the Jordan-Wigner transformation).
///
pub struct Fsim {
    control: usize,
    target: usize,
    t: CalculatorFloat,
    u: CalculatorFloat,
    delta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The generalized, anisotropic XYZ Heisenberg interaction between spins.
///
/// :math:`e^{-\mathrm{i} (x \cdot X_c X_t + y \cdot Y_c Y_t + z \cdot Z_c Z_t)}`
///
/// Where x, y, z are prefactors of the :math:`X_c X_t`, :math:`Y_c Y_t`, :math:`Z_c Z_t` Pauliproducts acting on control and target qubit,
/// with :math:`XX \equiv \sigma_x \sigma_x`, :math:`YY \equiv \sigma_y \sigma_y` and :math:`ZZ \equiv \sigma_z \sigma_z`.
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     x (CalculatorFloat): The prefactor of the XX interaction.
///     y (CalculatorFloat): The prefactor of the YY interaction.
///     z (CalculatorFloat): The prefactor of the ZZ interaction.
///
pub struct SpinInteraction {
    control: usize,
    target: usize,
    x: CalculatorFloat,
    y: CalculatorFloat,
    z: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The Bogoliubov DeGennes interaction gate.
///
/// :math:`e^{-\mathrm{i} Re(\Delta) (X_c X_t - Y_c Y_t)/2 + Im(\Delta) (X_c Y_t+Y_c X_t)/2}`
///
/// Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
/// and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(|\Delta|) & 0 & 0 & \mathrm{i} \sin(|\Delta|) e^{\mathrm{i} \cdot \mathrm{angle}(\Delta)} \\\\
///         0 & 1 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 \\\\
///         \mathrm{i} \sin(|\Delta|) e^{-\mathrm{i} \cdot \mathrm{angle}(\Delta)} & 0 & 0 & \cos(|\Delta|)
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     delta_real (CalculatorFloat): The real part of the complex Bogoliubov interaction strength :math:`Re(\Delta)`.
///     delta_imag (CalculatorFloat): The imaginary part of the complex Bogoliubov interaction strength :math:`Im(\Delta)`.
///
pub struct Bogoliubov {
    control: usize,
    target: usize,
    delta_real: CalculatorFloat,
    delta_imag: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The transversal interaction gate.
///
/// :math:`e^{-\mathrm{i} \theta (X_c X_t + Y_c Y_t)} = e^{-\mathrm{i} \theta (\sigma^+_c \sigma^-_t + \sigma^-_c \sigma^+_t)}`
///
/// Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
/// and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     t (CalculatorFloat): The strength of the rotation :math:`\theta`.
///
pub struct PMInteraction {
    control: usize,
    target: usize,
    t: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The complex hopping gate.
///
/// :math:`e^{-\mathrm{i} \left[ Re(\theta) \cdot (X_c X_t + Y_c Y_t) - Im(\theta) \cdot (X_c Y_t - Y_c X_t) \right] }`
///
/// Where :math:`X_c` is the Pauli matrix :math:`\sigma^x` acting on the control qubit
/// and :math:`Y_t` is the Pauli matrix :math:`\sigma^y` acting on the target qubit.
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation.
///     target (int):: The index of the least significant qubit in the unitary representation.
///     t_real (CalculatorFloat): The real part of the strength of the rotation :math:`Re(\theta)`.
///     t_imag (CalculatorFloat): The imaginary part of the strength of the rotation :math:`Im(\theta)`.
///
pub struct ComplexPMInteraction {
    control: usize,
    target: usize,
    t_real: CalculatorFloat,
    t_imag: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, OperateGate, OperateTwoQubitGate)]
/// The phased-shifted controlled-Z gate.
///
/// Modified, i.e. phase-shifted ControlledPauliZ two-qubit gate (`<https://arxiv.org/pdf/1908.06101.pdf eq.(1)>`).
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & e^{i \phi} & 0 & 0 \\\\
///         0 & 0 & e^{i \phi} & 0 \\\\
///         0 & 0 & 0 & e^{i (2\cdot\phi + \pi)}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///
pub struct PhaseShiftedControlledZ {
    control: usize,
    target: usize,
    phi: CalculatorFloat,
}

/// Implements the phase-shifted controlled PhaseShift gate.
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 \\\\
///         0 & e^{i \phi} & 0 & 0 \\\\
///         0 & 0 & e^{i \phi} & 0 \\\\
///         0 & 0 & 0 & e^{i(2\cdot\phi + \theta)}
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     theta (CalculatorFloat): The phase rotation $\theta$.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///
#[allow(clippy::upper_case_acronyms)]
#[wrap(Operate, OperateTwoQubit, Rotate, OperateGate, OperateTwoQubitGate)]
pub struct PhaseShiftedControlledPhase {
    control: usize,
    target: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}
