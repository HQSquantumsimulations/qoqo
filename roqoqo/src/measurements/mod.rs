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

//! Provides the functionality for **post-processing** the measurement output of the quantum computing programs.
//!
//! Measurement classes take the result of the circuit running (or being simulated)
//! on a backend and post-process a measurement record of sigma-z measurements
//! or a statevector/density matrix to expectation values of observables.
//! The measurement classes require additional information in the form of measurement input
//! to reconstruct observables.
//!
//! # Note
//! The functionality to **perform** the actual measurement is provided by the measurement operations [crate::operations].

use std::collections::HashMap;

#[doc(hidden)]
mod measurement_auxiliary_data_input;
pub use measurement_auxiliary_data_input::*;
#[doc(hidden)]
mod cheated_basis_rotation_measurement;
pub use cheated_basis_rotation_measurement::*;
#[doc(hidden)]
mod basis_rotation_measurement;
pub use basis_rotation_measurement::*;
#[doc(hidden)]
mod cheated_measurement;
pub use cheated_measurement::*;
#[doc(hidden)]
mod classical_register_measurement;
pub use classical_register_measurement::*;

use crate::registers::BitOutputRegister;
use crate::{
    registers::{ComplexOutputRegister, FloatOutputRegister},
    Circuit, RoqoqoError,
};

#[cfg(feature = "async")]
use crate::registers::Registers;
#[cfg(feature = "async")]
use crate::RoqoqoBackendError;
#[cfg(feature = "async")]
use async_trait::async_trait;
#[cfg(feature = "async")]
use futures::future::FutureExt;
#[cfg(feature = "async")]
use std::pin::Pin;

/// Allows generic interfacing with roqoqo measurements.
///
/// # Example
/// ```
/// // We want to run a measurement for the following expectation value: 3 + 4.0 * < Z0 >.
/// use roqoqo::{measurements::{PauliZProduct, PauliZProductInput, Measure}, registers::BitOutputRegister, Circuit};
/// use roqoqo::operations::RotateX;
/// use std::collections::HashMap;
///
/// // 1) Initialize our measurement input PauliZProductInput for the PauliZProduct measurement
/// let mut bri = PauliZProductInput::new(3, false);
///
/// // 2) Add the pauli products to the input
/// let _a = bri.add_pauliz_product("ro".to_string(), vec![]);
/// let _b = bri.add_pauliz_product("ro".to_string(), vec![0]);
///
/// // 3) Add corresponding linear definition of expectation values
/// let mut linear_map_0: HashMap<usize, f64> = HashMap::new();
/// linear_map_0.insert(0, 3.0);
/// bri.add_linear_exp_val("constant".to_string(), linear_map_0).unwrap();
/// let mut linear_map_1: HashMap<usize, f64> = HashMap::new();
/// linear_map_1.insert(1, 4.0);
/// bri.add_linear_exp_val("single_qubit_exp_val".to_string(), linear_map_1).unwrap();
///
/// // 4) Construct circuits for the PauliZProduct measurement
/// let mut circs: Vec<Circuit> = Vec::new();
/// circs.push(Circuit::new());
/// let mut circ1 = Circuit::new();
/// circ1 += RotateX::new(0, 0.0.into());
/// circs.push(circ1);
///
/// // 5) Initialize the PauliZProduct with the circuits and input defined above
/// let br = PauliZProduct {
///     constant_circuit: Some(Circuit::new()),
///     circuits: circs.clone(),
///     input: bri,
/// };
///
/// // 6) Check that all values are correct
/// for (index, b) in br.circuits.iter().enumerate() {
///     assert_eq!(b, circs.get(index).unwrap());
/// }
/// assert_eq!(&Circuit::new(), br.constant_circuit.as_ref().unwrap());
/// ```
///
pub trait Measure: PartialEq + Clone {
    /// Returns iterator over circuits for measurement.
    ///
    /// The returned circuits have to be executed by the backend
    /// and their measurement written into classical registers to evaluate
    /// the measurement.
    ///
    /// # Returns
    ///
    /// * `Box<dyn Iterator<Item = &'a Circuit> + 'a>` - The quantum circuits.
    fn circuits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Circuit> + 'a>;

    /// Returns the constant Circuit that is executed before each Circuit in circuits.
    ///
    /// # Returns
    ///
    /// * `&Option<Circuit` - The constant Circuit (None if not defined).
    fn constant_circuit(&self) -> &Option<Circuit>;

    /// Returns clone of Measurement with symbolic parameters replaced.
    ///
    /// # Arguments
    ///
    /// * `substituted_parameters` - The HashMap containing the substitutions to use in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The Circuits with the parameters substituted.
    /// * `Err(RoqoqoError)` - The substitution failed.
    fn substitute_parameters(
        &self,
        substituted_parameters: HashMap<String, f64>,
    ) -> Result<Self, RoqoqoError>;
}

/// Allows generic interfacing with roqoqo measurements that evaluate expectation values.
///
/// # Example
/// ```
/// // We want to run a measurement for the following expectation value: 3 + 4.0 * < Z0 >.
/// use roqoqo::{measurements::{PauliZProduct, PauliZProductInput, MeasureExpectationValues}, registers::BitOutputRegister, Circuit};
/// use std::collections::HashMap;
///
/// // 1) Create and fill PauliZProductInput for the PauliZProduct measurement
/// let mut bri = PauliZProductInput::new(3, false);
///
/// let _a = bri.add_pauliz_product("ro".to_string(), vec![]);
/// let _b = bri.add_pauliz_product("ro".to_string(), vec![0]);
///
/// let mut linear_map_0: HashMap<usize, f64> = HashMap::new();
/// linear_map_0.insert(0, 3.0);
/// bri.add_linear_exp_val("constant".to_string(), linear_map_0).unwrap();
/// let mut linear_map_1: HashMap<usize, f64> = HashMap::new();
/// linear_map_1.insert(1, 4.0);
/// bri.add_linear_exp_val("single_qubit_exp_val".to_string(), linear_map_1).unwrap();
///
/// // 2) Create and fill PauliZProduct measurement
/// let mut circs: Vec<Circuit> = Vec::new();
/// circs.push(Circuit::new());
///
/// let br = PauliZProduct {
///     constant_circuit: None,
///     circuits: circs.clone(),
///     input: bri,
/// };
///
/// // 3) Construct measured registers
/// let register = vec![
///    vec![true, true, false],
///    vec![true, true, false],
///    vec![false, false, true],
///    vec![false, false, true],
/// ];
/// let mut measured_registers: HashMap<String, BitOutputRegister> = HashMap::new();
/// let new_output_register: BitOutputRegister = register;
/// let _ = measured_registers.insert("ro".to_string(), new_output_register);
///
/// // 4) Evaluate PauliZProduct measurement
/// let result = br
///     .evaluate(measured_registers, HashMap::new(), HashMap::new())
///     .unwrap()
///     .unwrap();
///     assert_eq!(result.get("constant").unwrap(), &3.0);
///     assert_eq!(
///     result.get("single_qubit_exp_val").unwrap(),
///     &0.0
/// );
///
/// // 5) Check that all values are correct
/// assert_eq!(result.get("constant").unwrap(), &3.0);
/// assert_eq!(result.get("single_qubit_exp_val").unwrap(), &0.0);
/// ```
///
#[cfg_attr(feature = "async", async_trait)]
pub trait MeasureExpectationValues: PartialEq + Clone + Measure {
    /// Evaluates measurement results based on classical registers.
    ///
    /// Arguments:
    ///
    /// * `bit_registers` - The classical bit registers as a HashMap with the register name as key.
    /// * `float_registers` - The classical float registers as a HashMap with the register name as key.
    /// * `complex_registers` - The classical complex registers as a HashMap with the register name as key.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(HashMap<String, f64>))` - The measurement has been evaluated successfully. The HashMap contains the measured expectation values.
    /// * `Ok(None)` - The measurement did not fail but is incomplete. A new round of measurements is needed.
    /// * `Err(RoqoqoError)` - The measurement evaluation failed.
    fn evaluate(
        &self,
        bit_registers: HashMap<String, BitOutputRegister>,
        float_registers: HashMap<String, FloatOutputRegister>,
        complex_registers: HashMap<String, ComplexOutputRegister>,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoError>;

    /// Evaluates measurement results based on a [futures::future::Future] of classical registers.
    ///
    /// Arguments:
    ///
    /// * `registers` - Future .
    ///
    /// # Returns
    ///
    /// * `Ok(Some(HashMap<String, f64>))` - The measurement has been evaluated successfully. The HashMap contains the measured expectation values.
    /// * `Ok(None)` - The measurement did not fail but is incomplete. A new round of measurements is needed.
    /// * `Err(RoqoqoError)` - The measurement evaluation failed.
    #[cfg(feature = "async")]
    async fn async_evaluate(
        &self,
        registers: Pin<
            Box<dyn FutureExt<Output = Result<Registers, RoqoqoBackendError>> + std::marker::Send>,
        >,
    ) -> Result<HashMap<String, f64>, RoqoqoBackendError> {
        let (bit_registers, float_registers, complex_registers) = registers.await?;
        Ok(self
            .evaluate(bit_registers, float_registers, complex_registers)?
            .unwrap())
    }
}
