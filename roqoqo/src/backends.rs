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

//! Traits defining the standard form of roqoqo backends.
//!
//! roqoqo can be used to implement interfaces and backends to quantum hardware, quantum simulators and other software packages.
//! While roqoqo does not require a certain design for general interfaces or backends,
//! roqoqo::backends provides a trait for implementing backends that produce measurement results which can be evaluated to
//! expectation values.
//! This trait is supposed to be implemented for backends connecting to quantum simulators or to real quantum hardware devices.
//!
//! Note: The following backends are implemented in roqoqo and supported by HQS Quantum Simulations GmbH.
//!
//! Evaluated backends:
//! * `aqt`, ( <https://github.com/HQSquantumsimulations/qoqo_aqt> ),
//! * `mock` ( <https://github.com/HQSquantumsimulations/qoqo_mock> ),
//! * `quest` ( <https://github.com/HQSquantumsimulations/qoqo-quest> ).
//!
//! Other backends:
//! * `qasm` ( <https://github.com/HQSquantumsimulations/qoqo_qasm> ).

use std::collections::HashMap;

use crate::operations::Operation;
use crate::registers::Registers;
use crate::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use crate::Circuit;
use crate::{
    measurements::{Measure, MeasureExpectationValues},
    RoqoqoBackendError,
};
#[cfg(feature = "async")]
use async_trait::async_trait;

/// Result of functions running a full circuit and producing output registers.
pub type RegisterResult = Result<Registers, RoqoqoBackendError>;

/// Trait for Backends that can evaluate measurements to expectation values.
pub trait EvaluatingBackend: Sized {
    /// Runs a circuit with the backend.
    ///
    /// A circuit is passed to the backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The circuit that is run on the backend.
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated circuits.
    fn run_circuit(&self, circuit: &Circuit) -> RegisterResult {
        self.run_circuit_iterator(circuit.iter())
    }

    /// Runs each operation obtained from an iterator over operations on the backend.
    ///
    /// An iterator over operations is passed to the backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The iterator over operations that is run on the backend (corresponds to a circuit).
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated circuits.
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult;

    /// Runs all circuits corresponding to one measurement with the backend.
    ///
    /// An expectation value measurement in general involves several circuits.
    /// Each circuit is passes to the backend and executed separately.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    /// At the end all OutputRegisters are combined in a single HashMap for each type of register.
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement that is run on the backend.
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated measurement circuits.
    fn run_measurement_registers<T>(&self, measurement: &T) -> RegisterResult
    where
        T: Measure,
    {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let mut float_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let mut complex_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();

        for circuit in measurement.circuits() {
            let (tmp_bit_reg, tmp_float_reg, tmp_complex_reg) = match measurement.constant_circuit()
            {
                Some(x) => self.run_circuit_iterator(x.iter().chain(circuit.iter()))?,
                None => self.run_circuit_iterator(circuit.iter())?,
            };

            for (key, mut val) in tmp_bit_reg.into_iter() {
                if let Some(x) = bit_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = bit_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_float_reg.into_iter() {
                if let Some(x) = float_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = float_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_complex_reg.into_iter() {
                if let Some(x) = complex_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = complex_registers.insert(key, val);
                }
            }
        }
        Ok((bit_registers, float_registers, complex_registers))
    }
    /// Evaluates expectation values of a measurement with the backend.
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement that is run on the backend.
    ///
    /// # Returns
    ///
    /// `Ok(Option<HashMap<String, f64>>)` - The HashMap of measurement results.
    /// `Err(RoqoqoBackendError)` - The measurement run failed.
    fn run_measurement<T>(
        &self,
        measurement: &T,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoBackendError>
    where
        T: MeasureExpectationValues,
    {
        let (bit_registers, float_registers, complex_registers) =
            self.run_measurement_registers(measurement)?;
        Ok(measurement.evaluate(bit_registers, float_registers, complex_registers)?)
    }
}

#[cfg(feature = "async")]
#[async_trait]
/// Trait for Backends that can evaluate measurements to expectation values as async functions
///
/// Especially useful for Backends communicating with remote devices.
pub trait AsyncEvaluatingBackend: Sized {
    /// Runs a circuit with the backend.
    ///
    /// A circuit is passed to the backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The circuit that is run on the backend.
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated circuits.
    async fn async_run_circuit(&self, circuit: &Circuit) -> RegisterResult {
        self.async_run_circuit_iterator(circuit.iter()).await
    }

    /// Runs each operation obtained from an iterator over operations on the backend.
    ///
    /// An iterator over operations is passed to the backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The iterator over operations that is run on the backend (corresponds to a circuit).
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated circuits.
    async fn async_run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation> + std::marker::Send,
    ) -> RegisterResult;

    /// Runs all circuits corresponding to one measurement with the backend.
    ///
    /// An expectation value measurement in general involves several circuits.
    /// Each circuit is passes to the backend and executed separately.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    /// At the end all OutputRegisters are combined in a single HashMap for each type of register.
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement that is run on the backend.
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated measurement circuits.
    async fn async_run_measurement_registers<T>(&self, measurement: &T) -> RegisterResult
    where
        T: Measure,
        T: std::marker::Sync,
    {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let mut float_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let mut complex_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();

        let mut circuit_futures = Vec::new();
        for circuit in measurement.circuits() {
            let circuit_future = match measurement.constant_circuit() {
                Some(x) => self.async_run_circuit_iterator(x.iter().chain(circuit.iter())),
                None => self.async_run_circuit_iterator(circuit.iter()),
            };
            circuit_futures.push(circuit_future)
        }
        let circuit_results = futures::future::try_join_all(circuit_futures).await?;

        for (tmp_bit_reg, tmp_float_reg, tmp_complex_reg) in circuit_results {
            for (key, mut val) in tmp_bit_reg.into_iter() {
                if let Some(x) = bit_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = bit_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_float_reg.into_iter() {
                if let Some(x) = float_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = float_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_complex_reg.into_iter() {
                if let Some(x) = complex_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = complex_registers.insert(key, val);
                }
            }
        }
        Ok((bit_registers, float_registers, complex_registers))
    }
    /// Evaluates expectation values of a measurement with the backend.
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement that is run on the backend.
    ///
    /// # Returns
    ///
    /// `Ok(Option<HashMap<String, f64>>)` - The HashMap of measurement results.
    /// `Err(RoqoqoBackendError)` - The measurement run failed.
    async fn async_run_measurement<T>(
        &self,
        measurement: &T,
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoBackendError>
    where
        T: MeasureExpectationValues,
        T: std::marker::Sync,
    {
        // Futures trick so that compiler recognizes that clone of measurement in send safe
        let (bit_registers, float_registers, complex_registers) =
            self.async_run_measurement_registers(measurement).await?;
        Ok(measurement.evaluate(bit_registers, float_registers, complex_registers)?)
    }
}
