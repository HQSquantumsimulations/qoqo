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

use ndarray::{array, Array2};
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
use std::convert::TryFrom;
use std::f64::consts::PI;

use crate::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperateGate, OperateSingleQubit,
    OperateSingleQubitGate, Rotate, Substitute, SupportedVersion,
};
use crate::RoqoqoError;
#[cfg(feature = "overrotate")]
use rand_distr::{Distribution, Normal};

/// The most general unitary operation acting on one qubit.
///
/// # Warning
///
/// Due to the support of parameterized values it cannot be guaranteed that the unitary matrix of the gate
/// is always normalized to one.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SingleQubitGate {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The real part Re(α) of the on-diagonal elements of the single-qubit unitary.
    alpha_r: CalculatorFloat,
    /// The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary.
    alpha_i: CalculatorFloat,
    /// The real part Re(β) of the off-diagonal elements of the single-qubit unitary.
    beta_r: CalculatorFloat,
    /// The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary.
    beta_i: CalculatorFloat,
    /// The global phase φ of the single-qubit unitary.
    global_phase: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_SingleQubitGate: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "SingleQubitGate",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for SingleQubitGate {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed or matrix normalization failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let alpha_r: f64 = f64::try_from(self.alpha_r())?;
        let alpha_i: f64 = f64::try_from(self.alpha_i())?;
        let beta_r: f64 = f64::try_from(self.beta_r())?;
        let beta_i: f64 = f64::try_from(self.beta_i())?;
        let global_phase: f64 = f64::try_from(self.global_phase())?;
        // before building the unitary matrix, check values of alpha and beta for the matrix to be normalized.
        if alpha_r == 0.0 && alpha_i == 0.0 && beta_r == 0.0 && beta_i == 0.0
            || (alpha_r.powf(2.0) + alpha_i.powf(2.0) + beta_r.powf(2.0) + beta_i.powf(2.0) - 1.0)
                .abs()
                > 1e-6
        {
            let norm: f64 =
                alpha_r.powf(2.0) + alpha_i.powf(2.0) + beta_r.powf(2.0) + beta_i.powf(2.0);
            Err(RoqoqoError::UnitaryMatrixErrror {
                alpha_r,
                alpha_i,
                beta_r,
                beta_i,
                norm,
            })
        } else {
            let pref = Complex64::new(0.0, global_phase).exp();
            Ok(array![
                [
                    pref * Complex64::new(alpha_r, alpha_i),
                    pref * Complex64::new(-1.0 * beta_r, beta_i)
                ],
                [
                    pref * Complex64::new(beta_r, beta_i),
                    pref * Complex64::new(alpha_r, -1.0 * alpha_i)
                ]
            ])
        }
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for SingleQubitGate {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        self.alpha_r.clone()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        self.alpha_i.clone()
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        self.beta_r.clone()
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        self.beta_i.clone()
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        self.global_phase.clone()
    }
}

/// The ZPower gate exp(-i * θ/2 * σ^z).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateZ {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_RotateZ: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateZ",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateZ {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of theta to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        Ok(array![
            [Complex64::new(c, -1.0 * s), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(c, s)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateZ {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }

    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).sin() * (-1.0)
    }

    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }

    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }

    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// The XPower gate exp(-i * θ/2 * σ^x).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateX {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
}
#[allow(non_upper_case_globals)]
const TAGS_RotateX: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateX",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateX {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of theta to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        Ok(array![
            [Complex64::new(c, 0.0), Complex64::new(0.0, -1.0 * s)],
            [Complex64::new(0.0, -1.0 * s), Complex64::new(c, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateX {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).sin() * (-1.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// The YPower gate exp(-i * θ/2 * σ^y).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateY {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
}
#[allow(non_upper_case_globals)]
const TAGS_RotateY: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateY",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateY {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of theta to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        Ok(array![
            [Complex64::new(c, 0.0), Complex64::new(-1.0 * s, 0.0)],
            [Complex64::new(s, 0.0), Complex64::new(c, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateY {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).sin()
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// The Pauli X gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PauliX {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PauliX: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "PauliX",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for PauliX {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for PauliX {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(-1.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI) / 2.0)
    }
}

/// The Pauli Y gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PauliY {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PauliY: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "PauliY",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for PauliY {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
            [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for PauliY {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(1.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(PI / 2.0)
    }
}

/// The Pauli Z gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PauliZ {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_PauliZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "PauliZ",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for PauliZ {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for PauliZ {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(-1.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(PI / 2.0)
    }
}

/// The square root of the XPower gate exp(-i * π/4 * σ^x).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SqrtPauliX {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_SqrtPauliX: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "SqrtPauliX",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for SqrtPauliX {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let theta: f64 = PI / 2.0;
        let c: f64 = (theta / 2.0).cos();
        let s: f64 = (theta / 2.0).sin();
        Ok(array![
            [Complex64::new(c, 0.0), Complex64::new(0.0, -1.0 * s)],
            [Complex64::new(0.0, -1.0 * s), Complex64::new(c, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for SqrtPauliX {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI / 4.0).cos())
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI / 4.0).sin() * (-1.0))
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// The inverse square root of the XPower gate: exp(i * π/4 * σ^x).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct InvSqrtPauliX {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_InvSqrtPauliX: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "InvSqrtPauliX",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for InvSqrtPauliX {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let theta: f64 = PI / 2.0;
        let c: f64 = (theta / 2.0).cos();
        let s: f64 = (theta / 2.0).sin();
        Ok(array![
            [Complex64::new(c, 0.0), Complex64::new(0.0, 1.0 * s)],
            [Complex64::new(0.0, 1.0 * s), Complex64::new(c, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for InvSqrtPauliX {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI / 4.0).cos())
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI / 4.0).sin())
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// The Hadamard gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Hadamard {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_Hadamard: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Hadamard",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for Hadamard {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let f: f64 = 1.0 / ((2.0_f64).sqrt());
        Ok(array![
            [Complex64::new(f, 0.0), Complex64::new(f, 0.0)],
            [Complex64::new(f, 0.0), Complex64::new(-1.0 * f, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for Hadamard {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(-1.0 / ((2.0_f64).sqrt()))
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(-1.0 / ((2.0_f64).sqrt()))
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(PI / 2.0)
    }
}

/// The S gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SGate {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_SGate: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "SGate",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for SGate {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for SGate {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(1.0 / ((2.0_f64).sqrt()))
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(-1.0 / ((2.0_f64).sqrt()))
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(PI / 4.0)
    }
}

/// The T gate.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TGate {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_TGate: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "TGate",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for TGate {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new((PI / 4.0).cos(), (PI / 4.0).sin())
            ] //exp(i*pi/4) = cos(pi/4) + i*sin(pi/4)
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for TGate {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        CalculatorFloat::from((PI / 8.0).cos())
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from((-1.0) * (PI / 8.0).sin())
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(PI / 8.0)
    }
}

/// The phase shift gate applied on state |1>.
///
/// Rotation around Z-axis by an arbitrary angle θ (AC Stark shift of the state |1>).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseShiftState1 {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2π.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseShiftState1: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "PhaseShiftState1",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for PhaseShiftState1 {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let theta: f64 = f64::try_from(self.theta().clone())?;
        Ok(array![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(theta.cos(), theta.sin())
            ]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for PhaseShiftState1 {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta().clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        (self.theta().clone() / 2.0).sin() * (-1.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        self.theta().clone() / 2.0
    }
}

/// The phase shift gate applied on state |0>.
///
/// Rotation around Z-axis by an arbitrary angle θ (AC Stark shift of the state |0>).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct PhaseShiftState0 {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2π.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_PhaseShiftState0: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "PhaseShiftState0",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for PhaseShiftState0 {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The parameter conversion to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let theta: f64 = f64::try_from(self.theta().clone())?;
        Ok(array![
            [
                Complex64::new(theta.cos(), theta.sin()),
                Complex64::new(0.0, 0.0)
            ],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for PhaseShiftState0 {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta().clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        (self.theta().clone() / 2.0).sin()
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        self.theta().clone() / 2.0
    }
}

/// Implements a rotation around an axis in the x-y plane in spherical coordinates.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateAroundSphericalAxis {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
    /// The rotation axis, unit-vector spherical coordinates θ_{sph}.
    spherical_theta: CalculatorFloat,
    /// The rotation axis, unit-vector spherical coordinates φ_{sph}  gives the angle in the x-y plane.
    spherical_phi: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_RotateAroundSphericalAxis: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateAroundSphericalAxis",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateAroundSphericalAxis {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        let vx: f64 = ((f64::try_from(self.spherical_theta.clone())?).sin())
            * ((f64::try_from(self.spherical_phi.clone())?).cos());
        let vy: f64 = ((f64::try_from(self.spherical_theta.clone())?).sin())
            * ((f64::try_from(self.spherical_phi.clone())?).sin());
        let vz: f64 = (f64::try_from(self.spherical_theta.clone())?).cos();
        Ok(array![
            [
                Complex64::new(c, -1.0 * s * vz),
                Complex64::new(-1.0 * s * vy, -1.0 * s * vx)
            ],
            [
                Complex64::new(s * vy, -1.0 * s * vx),
                Complex64::new(c, s * vz)
            ]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateAroundSphericalAxis {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        let s = (self.theta.clone() / 2.0).sin();
        let vz = (self.spherical_theta.clone()).cos();
        s * vz * (-1.0) // CHECK sign (?)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        let s = (self.theta.clone() / 2.0).sin();
        let vy = (self.spherical_phi.clone()).sin();
        let st = (self.spherical_theta.clone()).sin();
        s * vy * st
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        let s = (self.theta.clone() / 2.0).sin();
        let vx = (self.spherical_phi.clone()).cos();
        let st = (self.spherical_theta.clone()).sin();
        s * vx * st * (-1.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}

/// Implements a rotation around an x- and y-axis in spherical coordinates.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateXY {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
    /// The rotation axis, in spherical coordinates φ gives the angle in the x-y plane.
    phi: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_RotateXY: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateXY",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateXY {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        let vx: f64 = (f64::try_from(self.phi.clone())?).cos();
        let vy: f64 = (f64::try_from(self.phi.clone())?).sin();
        Ok(array![
            [
                Complex64::new(c, 0.0),
                Complex64::new(-1.0 * s * vy, -1.0 * s * vx)
            ],
            [
                Complex64::new(s * vy, -1.0 * s * vx),
                Complex64::new(c, 0.0)
            ]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateXY {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        let s = (self.theta.clone() / 2.0).sin();
        let vy = (self.phi.clone()).sin();
        s * vy
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        let s = (self.theta.clone() / 2.0).sin();
        let vx = (self.phi.clone()).cos();
        s * vx * (-1.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}
