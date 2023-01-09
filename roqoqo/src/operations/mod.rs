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

//! Operations are the atomic instructions in any quantum program that can be represented by roqoqo.
//!
//! Operations can be of various kinds: Definitions, GateOperations, PRAGMAs or measurement Operations.
//! * Definition operations define the classical registers and variables in the Circuit.
//! * GateOperations are single-, two- or multi-qubit gate operations that act on a set of qubits
//! and can be executed on a quantum computing device.
//! * PRAGMAs are operations that can be used when running a simulation of a quantum computing program.
//! * Measurement Operations are operations that perform a measurement either on a quantum computing device (MeasuareQubit)
//! or on a simulation of a quantum computing program (PRAGMA measurement operations).

use crate::RoqoqoError;
#[cfg(feature = "dynamic")]
use dyn_clone::DynClone;
use ndarray::Array2;
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
use roqoqo_derive::*;
use std::collections::{HashMap, HashSet};
/// Collection of roqoqo definition operations.
#[doc(hidden)]
mod define_operations;
pub use define_operations::*;
/// Collection of roqoqo measurement operations.
#[doc(hidden)]
mod measurement_operations;
pub use measurement_operations::*;
/// Collection of roqoqo multi qubit gate operations.
#[doc(hidden)]
mod multi_qubit_gate_operations;
pub use multi_qubit_gate_operations::*;
/// Collection of roqoqo PRAGMA operation structs.\
#[doc(hidden)]
mod pragma_operations;
pub use pragma_operations::*;
/// Collection of roqoqo single qubit gate operations.
#[doc(hidden)]
mod single_qubit_gate_operations;
pub use single_qubit_gate_operations::*;
/// Collection of roqoqo two qubit gate operations.
#[doc(hidden)]
mod two_qubit_gate_operations;
pub use two_qubit_gate_operations::*;

include!(concat!(env!("OUT_DIR"), "/_auto_generated_operations.rs"));

/// Represents qubits involved in a roqoqo Operation.
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum InvolvedQubits {
    /// Operation affects all qubits no matter how many there are.
    All,
    /// Operation affects no qubits (annotations etc.).
    None,
    /// Operation affects a specific set of qubits.
    Set(HashSet<usize>),
}

/// Represents classical register entries involved in a roqoqo Operation.
#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum InvolvedClassical {
    /// Operation affects all entries of a classical register.
    All(String),
    /// Operation affects all entries of a classical register up to the number of qubits in device.
    AllQubits(String),
    /// Operation affects no classical entries (annotations etc.).
    None,
    /// Operation affects a specific set of classical entries.
    Set(HashSet<(String, usize)>),
}

/// Trait for returning minimum roqoqo version for which a operation is supported
pub trait SupportedVersion {
    /// Returns the minimum roqoqo version that supports the operation.
    ///
    /// Expects a semver version string. Returns the major and minor version
    /// already converted to unsigned integers and the optionla extension of the
    /// version string.
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 0, 0)
    }
}

#[cfg(feature = "dynamic")]
/// Universal basic trait for all operations of roqoqo.
#[cfg_attr(feature = "dynamic", typetag::serde(tag = "Operate"))]
pub trait Operate:
    InvolveQubits + SubstituteDyn + DynClone + std::fmt::Debug + Send + SupportedVersion
{
    /// Returns tags classifying the type of operation.
    ///
    /// Used for type based dispatch in ffi interfaces.
    fn tags(&self) -> &'static [&'static str];
    /// Returns hqslang name of operation.
    ///
    /// As a general rule that should correspond to the roqoqo name of the operation.
    fn hqslang(&self) -> &'static str;
    /// Returns true when operation has symbolic parameters.
    fn is_parametrized(&self) -> bool;
}

#[cfg(not(feature = "dynamic"))]
/// Universal basic trait for all operations of roqoqo.
///
/// # Example
/// ```
/// use roqoqo::operations::{Operate, PauliX, RotateZ};
/// use qoqo_calculator::CalculatorFloat;
///
/// let paulix = PauliX::new(0);
/// let gate_tags: &[&str; 4] = &[
///     "Operation",
///     "GateOperation",
///     "SingleQubitGateOperation",
///     "PauliX",
/// ];
///
/// // 1) The tags of the operation tell us what kind of operation it is, and what traits it implements
/// assert_eq!(paulix.tags(), gate_tags);
/// // 2) The name of the operation is given by hqslang
/// assert_eq!(paulix.hqslang(), "PauliX");
/// // 3) Whether a gate is parametrized is determined by whether any of its inputs are symbolic (CalculatorFloat with a string).
/// // As the PauliX gate only takes an integer input (qubit), it can never be parametrized.
/// assert!(!paulix.is_parametrized());
/// // However, a RotateZ gate can be parametrized:
/// let rotatez_param = RotateZ::new(0, CalculatorFloat::from("parametrized"));
/// assert!(rotatez_param.is_parametrized());
/// // But it can also not be parametrized:
/// let rotatez_not_param = RotateZ::new(0, CalculatorFloat::from(2.0));
/// assert!(!rotatez_not_param.is_parametrized());
/// ```
///
pub trait Operate:
    InvolveQubits + Substitute + Clone + std::fmt::Debug + Send + SupportedVersion
{
    /// Returns tags classifying the type of the operation.
    ///
    /// Used for type based dispatch in ffi interfaces.
    fn tags(&self) -> &'static [&'static str];
    /// Returns hqslang name of the operation.
    ///
    /// As a general rule that should correspond to the roqoqo name of the operation.
    fn hqslang(&self) -> &'static str;
    /// Returns `true` when operation has symbolic parameters.
    fn is_parametrized(&self) -> bool;
}

#[cfg(feature = "dynamic")]
dyn_clone::clone_trait_object!(Operate);

/// Trait for the qubits involved in each Operation.
///
/// # Example
/// ```
/// use roqoqo::operations::{CNOT, DefinitionFloat, InvolveQubits, InvolvedQubits, PragmaRepeatedMeasurement};
/// use std::collections::{HashMap, HashSet};
///
/// // The involved qubits of the operation tell us which qubits are affected by the Operation.
/// // There are three possibilities:
/// // 1) The involved qubits are a set of integers (usize): these are the qubits affected by the Operation
/// let cnot = CNOT::new(0, 1);
/// let mut qubits: HashSet<usize> = HashSet::new();
/// qubits.insert(0);
/// qubits.insert(1);
/// assert_eq!(cnot.involved_qubits(), InvolvedQubits::Set(qubits));
/// // 2) The involved qubits are None: there are no qubits affected by this Operation
/// let def_float = DefinitionFloat::new("ro".to_string(), 1, true);
/// assert_eq!(def_float.involved_qubits(), InvolvedQubits::None);
/// // 3) The involved qubits are All: all of the qubits in the Circuit are affected by the Operation
/// let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
/// qubit_mapping.insert(0, 1);
/// let pragma = PragmaRepeatedMeasurement::new("ro".to_string(), 2, Some(qubit_mapping.clone()));
/// assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
/// ```
pub trait InvolveQubits {
    /// Returns all qubits involved in operation.
    fn involved_qubits(&self) -> InvolvedQubits;

    /// Returns all classical registers involved in operation.
    fn involved_classical(&self) -> InvolvedClassical {
        InvolvedClassical::None
    }
}

/// Substitute trait allowing to replace symbolic parameters and to perform qubit mappings.
///
/// # Example
/// ```
/// use roqoqo::operations::{RotateZ, Substitute};
/// use qoqo_calculator::{Calculator, CalculatorFloat};
/// use std::collections::HashMap;
///
/// // 1) The substitute_parameters function substitutes all symbolic parameters in the Operation and its inputs
/// let rotatez = RotateZ::new(0, CalculatorFloat::from("sub"));
/// let mut substitution_dict: Calculator = Calculator::new();
/// substitution_dict.set_variable("sub", 0.0);
/// let result = rotatez
///     .substitute_parameters(&substitution_dict)
///     .unwrap();
/// assert_eq!(result, RotateZ::new(0, CalculatorFloat::from(0.0)));
/// // 2) The remap_qubits function remaps all qubits in the Operation and its inputs
/// let rotatez = RotateZ::new(0, CalculatorFloat::from(0.0));
/// let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
/// qubit_mapping_test.insert(0, 2);
/// qubit_mapping_test.insert(2, 0);
/// let result = rotatez.remap_qubits(&qubit_mapping_test).unwrap();
/// assert_eq!(result, RotateZ::new(2, CalculatorFloat::from(0.0)));
/// ```
///
pub trait Substitute
where
    Self: Sized,
{
    /// Substitutes symbolic parameters in clone of the operation.
    fn substitute_parameters(
        &self,
        calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError>;
    /// Remaps the qubits in clone of the operation.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError>;
}

#[cfg(feature = "dynamic")]
/// Helper trait for implementing substitute for Box<dyn> operation.
pub trait SubstituteDyn {
    /// Substitute parameters in symbolic expression in clone of operation.
    fn substitute_parameters_dyn(
        &self,
        calculator: &qoqo_calculator::Calculator,
    ) -> Result<Box<dyn Operate>, RoqoqoError>;
    /// Remap qubits in operations in clone of operation.
    fn remap_qubits_dyn(
        &self,
        mapping: &HashMap<usize, usize>,
    ) -> Result<Box<dyn Operate>, RoqoqoError>;
}

#[cfg(feature = "dynamic")]
impl<T> SubstituteDyn for T
where
    T: 'static + Operate + Substitute,
{
    /// Substitute symbolic parameters in boxed clone of operation.
    fn substitute_parameters_dyn(
        &self,
        calculator: &qoqo_calculator::Calculator,
    ) -> Result<Box<dyn Operate>, RoqoqoError> {
        Ok(Box::new(Substitute::substitute_parameters(
            self, calculator,
        )?))
    }
    /// Remap qubits in operations in boxed clone of operation.
    fn remap_qubits_dyn(
        &self,
        mapping: &HashMap<usize, usize>,
    ) -> Result<Box<dyn Operate>, RoqoqoError> {
        Ok(Box::new(Substitute::remap_qubits(self, mapping)?))
    }
}

/// Trait for operations acting on exactly one qubit.
///
/// # Example
/// ```
/// use roqoqo::operations::{OperateSingleQubit, PauliX};
/// let paulix = PauliX::new(0);
/// assert_eq!(paulix.qubit(), &0_usize);
/// ```
///
pub trait OperateSingleQubit: Operate + InvolveQubits + Substitute + Clone + PartialEq {
    /// Returns `qubit` the Operation acts on.
    fn qubit(&self) -> &usize;
}

/// Trait for Operations acting on exactly two qubits.
///
/// # Example
/// ```
/// use roqoqo::operations::{CNOT, OperateTwoQubit};
/// let cnot = CNOT::new(0, 1);
/// assert_eq!(cnot.control(), &0_usize);
/// assert_eq!(cnot.target(), &1_usize);
/// ```
///
pub trait OperateTwoQubit: Operate + InvolveQubits + Substitute + Clone + PartialEq {
    /// Returns `target` qubit of two qubit Operation.
    fn target(&self) -> &usize;
    /// Returns `control` qubit of two qubit Operation.
    fn control(&self) -> &usize;
}

/// Trait for operations acting on multiple (more than two) qubits.
///
/// # Example
/// ```
/// use roqoqo::operations::{MultiQubitMS, OperateMultiQubit};
/// use qoqo_calculator::CalculatorFloat;
/// let multi_ms = MultiQubitMS::new(vec![0, 1, 3], CalculatorFloat::from(0.0));
/// assert_eq!(multi_ms.qubits(), &vec![0, 1, 3]);
/// ```
///
pub trait OperateMultiQubit:
    Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
    /// Returns vector of qubits operation is acting on in descending order of significance
    fn qubits(&self) -> &Vec<usize>;
}

/// Trait for PRAGMA Operations that are not necessary available on all universal quantum hardware.
///
/// PRAGMA Operations are unphysical in terms of quantum mechanics and are meant to be used for simulation purposes only, i.e. to run on simulation backends.
///
pub trait OperatePragma:
    Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
}

/// Trait for PRAGMA Operations that are not necessary available on all universal quantum hardware, that indicate noise.
///
/// # Example
/// ```
/// use ndarray::{array, Array2};
/// use roqoqo::operations::{OperatePragmaNoise, OperatePragmaNoiseProba, PragmaDamping};
/// use qoqo_calculator::CalculatorFloat;
///
/// let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
///
/// // 1) The superoperator representation of the noise Pragma
/// let superop_prob: f64 = *pragma.probability().float().unwrap();
/// let superop_sqrt: f64 = (1.0 - superop_prob.clone()).sqrt();
/// let superop: Array2<f64> = array![
///     [1.0, 0.0, 0.0, superop_prob.clone()],
///     [0.0, superop_sqrt, 0.0, 0.0],
///     [0.0, 0.0, superop_sqrt, 0.0],
///     [0.0, 0.0, 0.0, 1.0 - superop_prob.clone()],
/// ];
/// assert_eq!(superop, pragma.superoperator().unwrap());
/// // 2) The power function applied to the noise Pragma
/// let pragma_test = PragmaDamping::new(
///     0,
///     CalculatorFloat::from(0.005 * 1.5),
///     CalculatorFloat::from(0.02),
/// );
/// assert_eq!(pragma_test, pragma.powercf(CalculatorFloat::from(1.5)));
/// ```
///
pub trait OperatePragmaNoise:
    Operate + InvolveQubits + Substitute + Clone + PartialEq + OperatePragma + SupportedVersion
{
    /// Returns superoperator matrix of the Operation.
    fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError>;
    /// Returns the gate to the power of `power`.
    fn powercf(&self, power: qoqo_calculator::CalculatorFloat) -> Self;
}

/// Trait for PRAGMA Operations that are not necessary available on all universal quantum hardware, that indicate noise.
///
/// # Example
/// ```
/// use ndarray::{array, Array2};
/// use roqoqo::operations::{OperatePragmaNoiseProba, PragmaDamping};
/// use qoqo_calculator::CalculatorFloat;
///
/// let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
///
/// // The probability of the noise Pragma
/// let proba_pre_exp: f64 = -1.0 * 0.005 * 0.02;
/// let proba = CalculatorFloat::from(1.0 - proba_pre_exp.exp());
/// assert_eq!(proba, pragma.probability());
/// ```
///
pub trait OperatePragmaNoiseProba:
    Operate
    + InvolveQubits
    + Substitute
    + Clone
    + PartialEq
    + OperatePragma
    + OperatePragmaNoise
    + SupportedVersion
{
    /// Returns the probability of the gate, based on its gate_time and rate.
    fn probability(&self) -> CalculatorFloat;
}

/// Trait for Operations acting with a unitary gate on a set of qubits.
///
/// # Example
/// ```
/// use ndarray::array;
/// use num_complex::Complex64;
/// use roqoqo::operations::{OperateGate, PauliX};
///
/// let paulix = PauliX::new(0);
/// let matrix = array![
///     [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
///     [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]
/// ];
/// assert_eq!(paulix.unitary_matrix().unwrap(), matrix);
/// ```
///
pub trait OperateGate:
    Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
    /// Returns unitary matrix of the gate.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError>;
}

/// Trait for unitary operations corresponding to rotations that can be characteriszed by a single rotation parameter theta.
///
/// # Example
/// ```
/// use qoqo_calculator::CalculatorFloat;
/// use roqoqo::operations::{Rotate, RotateX};
/// let rotatex = RotateX::new(0, 2.0.into());
///
/// // 1) The angle of rotation of the Rotate Operation
/// assert_eq!(rotatex.theta(), &CalculatorFloat::from(2.0));
/// // 2) The power function applied to the Rotate Operation
/// assert_eq!(rotatex.powercf(CalculatorFloat::from(1.5)), RotateX::new(0, 3.0.into()));
/// ```
///
pub trait Rotate:
    OperateGate + Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
    /// Returns rotation parameter theta.
    fn theta(&self) -> &CalculatorFloat;
    /// Returns the gate to the power of `power`.`
    fn powercf(&self, power: CalculatorFloat) -> Self;

    #[cfg(feature = "overrotate")]
    /// Returns clone of the gate with one parameter statistically overrotated.
    ///
    /// A random number drawn from a normal distribution N(0, variance^2)
    /// and multiplied by the amplitue  is added to the overrotated parameter.  
    /// gate_overrotated.parameter() = gate.parameter + amplitude * rand(N(0, variance^2))
    ///
    /// This functionc is specifically designed for statistical overrotations that change the angle
    /// of an applied rotation gate randomly during the execution of a quantum program.  
    /// For static overrotations that represent a drift in the callibration of gates and are constant
    /// during the execution of a quantum programm use symbolic parameters and the substitute_parameters
    /// function.
    ///
    /// # Arguments
    ///
    /// *`amplitude` - The amplitude the random number is multiplied with.
    /// *`variance` - The standard deviation of the normal distribution the random number is drawn from.
    ///
    /// # Example
    /// ```
    /// use roqoqo::prelude::*;
    /// use roqoqo::operations::RotateZ;
    ///
    /// let gate = RotateZ::new(0, 1.0.into());
    /// let overrotated_gate = gate.overrotate(&1.0, &0.5);
    /// println!("{:?}", gate);
    /// println!("{:?}", overrotated_gate);
    /// let gate_symbolic = RotateZ::new(0, "theta_var".into());
    /// let overrotated_symbolic = gate_symbolic.overrotate(&1.0, &0.5);
    /// println!("{:?}", gate_symbolic);
    /// println!("{:?}", overrotated_symbolic);
    /// ```
    fn overrotate(&self, amplitude: &f64, variance: &f64) -> Self;
}

/// Trait for definition operations.
///
/// # Example
/// ```
/// use roqoqo::operations::{Define, DefinitionFloat};
/// let definition = DefinitionFloat::new("ro".to_string(), 1, false);
/// assert_eq!(definition.name(), &"ro".to_string());
/// ```
///
pub trait Define:
    Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
    /// Returns name of definition operation.
    fn name(&self) -> &String;
}

/// Trait for unitary operations without any free parameters.
///
/// # Example
/// ```
/// use roqoqo::operations::{OperateConstantGate, PauliX};
/// let paulix = PauliX::new(0);
/// ```
///
pub trait OperateConstantGate:
    OperateGate + Operate + InvolveQubits + Substitute + Clone + PartialEq + SupportedVersion
{
    /// Returns true when unitary operation U is self inverse U*U = I.
    fn inverse(&self) -> GateOperation;
}

/// Trait for unitary operations acting on exactly one qubit.
///
/// Implements the general single qubit unitary gates  that can be brought into the form:
///
/// U =exp(i * φ) * [[Re(α)+i * Im(α), -Re(β) + i * Im(β)], [Re(β) + i * Im(β) , Re(α) - i * Im(α) ]].
///
/// These gates can be parametrized by five real parameters:
///
/// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary.
/// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary.
/// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary.
/// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary.
/// * `global_phase` - The global phase φ of the single-qubit unitary.
///
/// These are the single qubit gates that are performed in the Circuit(), and are then translated
/// to quantum hardware through the relevant backend. Two-qubit gates are also available
/// (see roqoqo/src/operations/two_qubit_gate_operations.rs).
///
/// # Example
/// ```
/// use qoqo_calculator::CalculatorFloat;
/// use roqoqo::operations::{OperateSingleQubitGate, PauliX};
/// use std::f64::consts::PI;
///
/// let paulix = PauliX::new(0);
///
/// assert_eq!(paulix.alpha_r(), 0.0.into());
/// assert_eq!(paulix.alpha_i(), 0.0.into());
/// assert_eq!(paulix.beta_r(), 0.0.into());
/// assert_eq!(paulix.beta_i(), CalculatorFloat::from(-1.0));
/// assert_eq!(paulix.global_phase(), ((PI) / 2.0).into());
/// ```
///
pub trait OperateSingleQubitGate:
    Operate
    + OperateGate
    + InvolveQubits
    + Substitute
    + OperateSingleQubit
    + Clone
    + PartialEq
    + OperateSingleQubit
    + SupportedVersion
    + std::fmt::Debug
{
    /// Returns alpha_r parameter of operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat;

    /// Returns alpha_i parameter of operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat;

    /// Returns beta_r parameter of operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat;

    /// Returns beta_i parameter of operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` -  imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat;

    /// Returns global_phase parameter of operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase phi φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat;

    /// Multiplies two compatible operations implementing OperateSingleQubitGate.
    ///
    /// Does not consume the two operations being multiplied.
    /// Only Operations
    ///
    /// # Arguments:
    ///
    /// * `other` - An Operation implementing [OperateSingleQubitGate].
    ///
    /// # Example
    /// ```
    /// use roqoqo::operations::{RotateZ, RotateX};
    /// use roqoqo::prelude::*;
    /// use qoqo_calculator::CalculatorFloat;
    ///
    /// let gate1 =  RotateZ::new(0, CalculatorFloat::from(1));
    /// let gate2 =  RotateX::new(0, CalculatorFloat::from(1));
    /// let multiplied = gate1.mul(&gate2).unwrap();
    /// ```
    fn mul<T>(&self, other: &T) -> Result<SingleQubitGate, RoqoqoError>
    where
        T: OperateSingleQubitGate,
    {
        if self.qubit() != other.qubit() {
            return Err(RoqoqoError::MultiplicationIncompatibleQubits {
                squbit: *self.qubit(),
                oqubit: *other.qubit(),
            });
        }
        let alpha = qoqo_calculator::CalculatorComplex::new(self.alpha_r(), self.alpha_i());
        let beta = qoqo_calculator::CalculatorComplex::new(self.beta_r(), self.beta_i());
        let oalpha = qoqo_calculator::CalculatorComplex::new(other.alpha_r(), other.alpha_i());
        let obeta = qoqo_calculator::CalculatorComplex::new(other.beta_r(), other.beta_i());
        let new_alpha = alpha.clone() * &oalpha - beta.conj() * &obeta;
        let new_beta = beta * oalpha + obeta * alpha.conj();

        if new_alpha.re.is_float()
            && new_alpha.im.is_float()
            && new_beta.re.is_float()
            && new_beta.im.is_float()
        {
            let norm = (new_alpha.re.float().unwrap().powf(2.0)
                + new_alpha.im.float().unwrap().powf(2.0)
                + new_beta.re.float().unwrap().powf(2.0)
                + new_beta.im.float().unwrap().powf(2.0))
            .sqrt();

            if (norm - 1.0).abs() > f64::EPSILON {
                Ok(SingleQubitGate::new(
                    *other.qubit(),
                    new_alpha.re / norm,
                    new_alpha.im / norm,
                    new_beta.re / norm,
                    new_beta.im / norm,
                    self.global_phase() + other.global_phase(),
                ))
            } else {
                Ok(SingleQubitGate::new(
                    *other.qubit(),
                    new_alpha.re,
                    new_alpha.im,
                    new_beta.re,
                    new_beta.im,
                    self.global_phase() + other.global_phase(),
                ))
            }
        } else {
            Ok(SingleQubitGate::new(
                *other.qubit(),
                new_alpha.re,
                new_alpha.im,
                new_beta.re,
                new_beta.im,
                self.global_phase() + other.global_phase(),
            ))
        }
    }
    /// Returns equivalent SingleQubitGate.
    ///
    /// Converts Operation implementing OperateSingleQubitGate Trait into SingleQubitGate.
    fn to_single_qubit_gate(&self) -> SingleQubitGate {
        SingleQubitGate::new(
            *self.qubit(),
            self.alpha_r(),
            self.alpha_i(),
            self.beta_r(),
            self.beta_i(),
            self.global_phase(),
        )
    }
}

/// Trait for all Operations operating on or affecting exactly two qubits.
///
/// # Example
/// ```
/// use roqoqo::operations::{ISwap, KakDecomposition, OperateTwoQubitGate};
/// use qoqo_calculator::CalculatorFloat;
/// let iswap = ISwap::new(0, 1);
///
/// assert_eq!(iswap.kak_decomposition().circuit_before, None);
/// assert_eq!(iswap.kak_decomposition().circuit_after, None);
/// assert_eq!(iswap.kak_decomposition().global_phase, CalculatorFloat::ZERO);
/// assert_eq!(iswap.kak_decomposition().k_vector, [CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_4, CalculatorFloat::ZERO]);
/// ```
///
pub trait OperateTwoQubitGate:
    Operate
    + OperateGate
    + OperateTwoQubit
    + InvolveQubits
    + Substitute
    + Clone
    + PartialEq
    + SupportedVersion
{
    /// Returns [KakDecomposition] of two qubit gate.
    fn kak_decomposition(&self) -> KakDecomposition;
}

/// Trait for all Operations operating on or affecting more than two qubits.
///
/// # Example
/// ```
/// use roqoqo::operations::{CNOT, Hadamard, MultiQubitMS, OperateMultiQubitGate, RotateZ};
/// use roqoqo::Circuit;
/// use qoqo_calculator::CalculatorFloat;
///
/// let multi_ms = MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.0));
/// let mut circuit = Circuit::new();
/// circuit += Hadamard::new(0);
/// circuit += Hadamard::new(1);
/// circuit += Hadamard::new(2);
/// circuit += CNOT::new(0, 1);
/// circuit += CNOT::new(1, 2);
/// circuit += RotateZ::new(2, CalculatorFloat::from(0.5));
/// circuit += CNOT::new(1, 2);
/// circuit += CNOT::new(0, 1);
/// circuit += Hadamard::new(0);
/// circuit += Hadamard::new(1);
/// circuit += Hadamard::new(2);
///
/// assert_eq!(multi_ms.circuit(), circuit);
/// ```
///
pub trait OperateMultiQubitGate:
    Operate
    + OperateGate
    + OperateMultiQubit
    + InvolveQubits
    + Substitute
    + Clone
    + PartialEq
    + SupportedVersion
{
    /// Returns a decomposition of the multi-qubit operation using a circuit with two-qubit-operations.
    fn circuit(&self) -> crate::Circuit;
}

// Implementing DynOperation for storing dynamic operations from extern crates in trait object

/// Marker trait to show that some operation has been implemented in roqoqo 1.1.0
pub(crate) trait ImplementedIn1point1: Operate {}

/// Marker trait to show that some operation has been implemented in roqoqo 1.2.0
pub(crate) trait ImplementedIn1point2: Operate {}

#[cfg(feature = "dynamic")]
/// A wrapper for Operate trait objects.
///
/// This wrapper struct can be used to insert Operate trait objects in a circuit.
/// The intended use case is to store structs from an external crate that implement Operate,
/// in a circuit.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DynOperation(Box<dyn Operate>);

#[cfg(feature = "dynamic")]
#[cfg_attr(feature = "dynamic", typetag::serde)]
impl Operate for DynOperation {
    fn tags(&self) -> &'static [&'static str] {
        self.0.tags()
    }
    fn hqslang(&self) -> &'static str {
        self.0.hqslang()
    }
    fn is_parametrized(&self) -> bool {
        self.0.is_parametrized()
    }
}

#[cfg(feature = "dynamic")]
impl SupportedVersion for DynOperation {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        self.0.minimum_supported_roqoqo_version()
    }
}

#[cfg(feature = "dynamic")]
impl InvolveQubits for DynOperation {
    fn involved_qubits(&self) -> InvolvedQubits {
        self.0.involved_qubits()
    }
}
#[cfg(feature = "dynamic")]
/// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
impl Substitute for DynOperation {
    fn substitute_parameters(
        &self,
        calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError> {
        Ok(DynOperation(self.0.substitute_parameters_dyn(calculator)?))
    }
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        Ok(DynOperation(self.0.remap_qubits_dyn(mapping)?))
    }
}
#[cfg(feature = "dynamic")]
impl PartialEq for DynOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0.hqslang() == other.0.hqslang()
    }
}

/// Check if a HashMap is a valid mapping for remapping_qubits
#[inline]
pub(crate) fn check_valid_mapping(mapping: &HashMap<usize, usize>) -> Result<(), RoqoqoError> {
    for q in mapping.values() {
        if !mapping.contains_key(q) {
            return Err(RoqoqoError::QubitMappingError { qubit: *q });
        }
    }
    Ok(())
}
