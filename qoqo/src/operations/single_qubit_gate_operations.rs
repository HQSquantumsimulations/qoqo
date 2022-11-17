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

#[wrap(Operate, OperateSingleQubit, OperateSingleQubitGate, OperateGate)]
/// The general single qubit unitary gate.
///
/// .. math::
///     U =\begin{pmatrix}
///         \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
///         \beta_r+i \beta_i & \alpha_r-i\alpha_i
///         \end{pmatrix}
///
/// Args:
///     qubit: The qubit that the unitary gate is applied to.
///     alpha_r: The real part of the on-diagonal elements of the single-qubit unitary.
///     alpha_i: The imaginary part of the on-diagonal elements of the single-qubit unitary.
///     beta_r: The real part of the off-diagonal elements of the single-qubit unitary.
///     beta_i: The imaginary part of the off-diagonal elements of the single-qubit unitary.
///     global_phase: The global phase of the single-qubit unitary.
///
struct SingleQubitGate {
    qubit: usize,
    alpha_r: CalculatorFloat,
    alpha_i: CalculatorFloat,
    beta_r: CalculatorFloat,
    beta_i: CalculatorFloat,
    global_phase: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// The XPower gate :math:`e^{-i \frac{\theta}{2} \sigma^x}`.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\frac{\theta}{2}) & 0 \\\\
///         0 & \cos(\frac{\theta}{2})
///         \end{pmatrix}
///         + \begin{pmatrix}
///         0  &  -i \sin(\frac{\theta}{2}) \\\\
///         -i \sin(\frac{\theta}{2})  & 0
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct RotateX {
    qubit: usize,
    theta: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// The YPower gate :math:`e^{-i \frac{\theta}{2} \sigma^y}`.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\frac{\theta}{2}) & 0 \\\\
///         0 & \cos(\frac{\theta}{2})
///         \end{pmatrix}
///         + \begin{pmatrix}
///         0 & - \sin(\frac{\theta}{2}) \\\\
///         \sin(\frac{\theta}{2}) & 0
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct RotateY {
    qubit: usize,
    theta: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// The ZPower gate :math:`e^{-i \frac{\theta}{2} \sigma^z}`.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\frac{\theta}{2}) & 0 \\\\
///         0 & \cos(\frac{\theta}{2})
///         \end{pmatrix}
///         + \begin{pmatrix}
///         - i \sin(\frac{\theta}{2}) & 0 \\\\
///         0 & i \sin(\frac{\theta}{2})
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct RotateZ {
    qubit: usize,
    theta: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// The phase shift gate applied on state |1>.
///
/// Rotation around Z-axis by an arbitrary angle $\theta$ (AC Stark shift of the state |1>).
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0\\\\
///         0 & e^{i \theta}
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct PhaseShiftState1 {
    qubit: usize,
    theta: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// The phase shift gate applied on state |0>.
///
/// Rotation around Z-axis by an arbitrary angle $\theta$ (AC Stark shift of the state |0>).
///
/// .. math::
///     U = \begin{pmatrix}
///         e^{i \theta} & 0\\\\
///         0 & 1
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct PhaseShiftState0 {
    qubit: usize,
    theta: CalculatorFloat,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The Pauli X gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         0 & 1 \\\\
///         1 & 0
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct PauliX {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The Pauli Y gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         0 & -i \\\\
///         i & 0
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct PauliY {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The Pauli Z gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 \\\\
///         0 & -1
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct PauliZ {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The square root of the XPower gate :math:`e^{-i \frac{\pi}{4} \sigma^x}`.
///
/// .. math::
///     U = \frac{1}{\sqrt(2)}\begin{pmatrix}
///         1 & -i \\\\
///         -i & 1
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct SqrtPauliX {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The inverse square root XPower gate :math:`e^{i \frac{\pi}{2} \sigma^x}`.
///
/// .. math::
///     U = \frac{1}{\sqrt{2}} \begin{pmatrix}
///         1 & i \\\\
///         i & 1
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct InvSqrtPauliX {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The Hadamard gate.
///
/// .. math::
///     U = \frac{1}{\sqrt{2}} \begin{pmatrix}
///         1 & 1\\\\
///         1 & -1
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct Hadamard {
    qubit: usize,
}

#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The S gate.
///
/// .. math::
///     U = \frac{1}{\sqrt{2}} \begin{pmatrix}
///         1 & 0 \\\\
///         0 & i
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct SGate {
    qubit: usize,
}
#[wrap(Operate, OperateSingleQubit, OperateGate, OperateSingleQubitGate)]
#[derive(Eq)]
/// The T gate.
///
/// .. math::
///     U = \frac{1}{\sqrt{2}} \begin{pmatrix}
///         1 & 0 \\\\
///         0 & e^{i \frac{\pi}{4}}
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///
struct TGate {
    qubit: usize,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// Implements a rotation around an axis in the x-y plane in spherical coordinates.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\frac{\theta}{2}) & 0 \\\\
///         0 & \cos(\frac{\theta}{2})
///         \end{pmatrix}
///         + \begin{pmatrix}
///         -i \sin(\frac{\theta}{2}) v_z  &  \sin(\frac{\theta}{2}) \left(-i v_x - v_y \right) \\\\
///         \sin(\frac{\theta}{2}) \left(-i v_x + v_y \right) & i \sin(\frac{\theta}{2}) v_z
///         \end{pmatrix}
///
/// with
///
/// .. math::
///  v_x = \sin(\theta_{sph}) \cos(\phi_{sph}) \ , \\
///  v_y = \sin(\theta_{sph}) \sin(\phi_{sph}) \ , \\
///  v_z = \cos(\theta_{sph}) \ .
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///     spherical_theta (CalculatorFloat): The rotation axis, unit-vector spherical coordinates :math:`\theta_{sph}`.
///     spherical_phi (CalculatorFloat): The rotation axis, unit-vector spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.
///
struct RotateAroundSphericalAxis {
    qubit: usize,
    theta: CalculatorFloat,
    spherical_theta: CalculatorFloat,
    spherical_phi: CalculatorFloat,
}

#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate
)]
/// Implements a rotation around an axis in the x-y plane in spherical coordinates.
///
/// .. math::
/// U = \begin{pmatrix}
/// \cos(\frac{\theta}{2}) & -i e^{-i \phi} \sin(\frac{\theta}{2})\\\\
/// -i e^{i \phi} \sin(\frac{\theta}{2}) & \cos(\frac{\theta}{2})
/// \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///     phi (CalculatorFloat): The rotation axis, in spherical coordinates :math:`\phi_{sph}`  gives the angle in the x-y plane.
///
struct RotateXY {
    qubit: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}
