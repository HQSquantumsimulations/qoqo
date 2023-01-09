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

//! Represents a quantum program evaluating measurements based on a one or more free float parameters.

use std::collections::HashMap;

#[cfg(feature = "async")]
use crate::backends::AsyncEvaluatingBackend;
use crate::backends::{EvaluatingBackend, RegisterResult};
use crate::measurements;
use crate::measurements::Measure;
use crate::RoqoqoBackendError;
use std::fmt::{Display, Formatter};

/// Represents a quantum program evaluating measurements based on a one or more free float parameters.
///
/// The main use of QuantumProgram is to contain a Measurements implementing [crate::measurements::Measure]
/// that measures expectation values or output registers of [crate::Circuit] quantum circuits that contain
/// symbolic parameters. Circuit with symbolic parameters can not be simulated or executed on real hardware.
/// The symbolic parameters need to be replaced with real floating point numbers first.
/// A QuantumProgram contains a list of the free parameters (`input_parameter_names`) and can automatically
/// replace the parameters with its `run` methods and return the result.
///
/// The QuantumProgram should correspond as closely as possible to a normal mulit-parameter function
/// in classical computing that can be called with a set of parameters and returns a result.
/// It is the intended way to interface between normal program code and roqoqo based quantum programs.
///
#[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum QuantumProgram {
    /// Variant for basis rotation measurement based quantum programs
    PauliZProduct {
        /// The measurement that is performed
        measurement: measurements::PauliZProduct,
        /// List of free input parameters that can be set when the QuantumProgram is executed
        input_parameter_names: Vec<String>,
    },
    /// Variant for cheated basis rotation measurement based quantum programs
    CheatedPauliZProduct {
        /// The measurement that is performed
        measurement: measurements::CheatedPauliZProduct,
        /// List of free input parameters that can be set when the QuantumProgram is executed
        input_parameter_names: Vec<String>,
    },
    /// Variant for statevector/density matrix based measurements
    Cheated {
        /// The measurement that is performed
        measurement: measurements::Cheated,
        /// List of free input parameters that can be set when the QuantumProgram is executed
        input_parameter_names: Vec<String>,
    },
    /// Variant quantum programs returning full classical registers
    ClassicalRegister {
        /// The measurement that is performed
        measurement: measurements::ClassicalRegister,
        /// List of free input parameters that can be set when the QuantumProgram is executed
        input_parameter_names: Vec<String>,
    },
}

impl QuantumProgram {
    /// Runs the QuantumProgram and returns expectation values.
    ///
    /// Runs the quantum programm for a given set of parameters passed in the same order as the parameters
    /// listed in `input_parameter_names` and returns expectation values.
    ///
    /// Arguments:
    ///
    /// * `backend` - The backend the program is executed on.
    /// * `parameters` - List of float ([f64]) parameters of the function call in order of `input_parameter_names`
    pub fn run<T>(
        &self,
        backend: T,
        parameters: &[f64],
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoBackendError>
    where
        T: EvaluatingBackend,
    {
        match self{
            QuantumProgram::PauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.run_measurement(&substituted_measurement)
            }
            QuantumProgram::CheatedPauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.run_measurement(&substituted_measurement)
            }
            QuantumProgram::Cheated{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.run_measurement(&substituted_measurement)
            }
            _ => Err(RoqoqoBackendError::GenericError{msg: "A quantum programm returning classical registeres cannot be executed by `run` use `run_registers` instead".to_string()})
        }
    }

    /// Runs the QuantumProgram and returns the classical registers of the quantum program.
    ///
    /// Runs the quantum programm for a given set of parameters passed in the same order as the parameters
    /// listed in `input_parameter_names` and returns the classical register output.  
    /// The classical registers usually contain a record of measurement values for the repeated execution
    /// of a [crate::Circuit] quantum circuit for real quantum hardware
    /// or the readout of the statevector or the density matrix for simulators.
    ///
    /// Arguments:
    ///
    /// * `backend` - The backend the program is executed on.
    /// * `parameters` - List of float ([f64]) parameters of the function call in order of `input_parameter_names`
    pub fn run_registers<T>(&self, backend: T, parameters: &[f64]) -> RegisterResult
    where
        T: EvaluatingBackend,
    {
        match self{
            QuantumProgram::ClassicalRegister{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.run_measurement_registers(&substituted_measurement)
            }
            _ => Err(RoqoqoBackendError::GenericError{msg: "A quantum programm returning expectation values cannot be executed by `run_registers` use `run` instead".to_string()})
        }
    }

    /// Runs the QuantumProgram and returns expectation values.
    ///
    /// Runs the quantum programm for a given set of parameters passed in the same order as the parameters
    /// listed in `input_parameter_names` and returns expectation values.
    ///
    /// Arguments:
    ///
    /// * `backend` - The backend the program is executed on.
    /// * `parameters` - List of float ([f64]) parameters of the function call in order of `input_parameter_names`
    #[cfg(feature = "async")]
    pub async fn async_run<T>(
        &self,
        backend: T,
        parameters: &[f64],
    ) -> Result<Option<HashMap<String, f64>>, RoqoqoBackendError>
    where
        T: AsyncEvaluatingBackend,
        T: Sync,
    {
        match self{
            QuantumProgram::PauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.async_run_measurement(&substituted_measurement).await
            }
            QuantumProgram::CheatedPauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.async_run_measurement(&substituted_measurement).await
            }
            QuantumProgram::Cheated{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.async_run_measurement(&substituted_measurement).await
            }
            _ => Err(RoqoqoBackendError::GenericError{msg: "A quantum programm returning classical registeres cannot be executed by `run` use `run_registers` instead".to_string()})
        }
    }

    /// Runs the QuantumProgram and returns the classical registers of the quantum program.
    ///
    /// Runs the quantum programm for a given set of parameters passed in the same order as the parameters
    /// listed in `input_parameter_names` and returns the classical register output.  
    /// The classical registers usually contain a record of measurement values for the repeated execution
    /// of a [crate::Circuit] quantum circuit for real quantum hardware
    /// or the readout of the statevector or the density matrix for simulators.
    ///
    /// Arguments:
    ///
    /// * `backend` - The backend the program is executed on.
    /// * `parameters` - List of float ([f64]) parameters of the function call in order of `input_parameter_names`
    #[cfg(feature = "async")]
    pub async fn async_run_registers<T>(&self, backend: T, parameters: &[f64]) -> RegisterResult
    where
        T: AsyncEvaluatingBackend,
        T: Sync,
    {
        match self{
            QuantumProgram::ClassicalRegister{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(RoqoqoBackendError::GenericError{msg: format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())})};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                )?;
                backend.async_run_measurement_registers(&substituted_measurement).await
            }
            _ => Err(RoqoqoBackendError::GenericError{msg: "A quantum programm returning expectation values cannot be executed by `run_registers` use `run` instead".to_string()})
        }
    }
}

/// Implements the Display trait for QuantumProgram.
impl Display for QuantumProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();

        match self {
            QuantumProgram::PauliZProduct { .. } => {
                s.push_str("QuantumProgram::PauliZProduct");
            }
            QuantumProgram::CheatedPauliZProduct { .. } => {
                s.push_str("QuantumProgram::CheatedPauliZProduct");
            }
            QuantumProgram::Cheated { .. } => {
                s.push_str("QuantumProgram::Cheated");
            }
            QuantumProgram::ClassicalRegister { .. } => {
                s.push_str("QuantumProgram::ClassicalRegister");
            }
        }

        write!(f, "{}", s)
    }
}

impl crate::operations::SupportedVersion for QuantumProgram {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        match self {
            QuantumProgram::PauliZProduct { measurement, .. } => {
                measurement.minimum_supported_roqoqo_version()
            }
            QuantumProgram::CheatedPauliZProduct { measurement, .. } => {
                measurement.minimum_supported_roqoqo_version()
            }
            QuantumProgram::Cheated { measurement, .. } => {
                measurement.minimum_supported_roqoqo_version()
            }
            QuantumProgram::ClassicalRegister { measurement, .. } => {
                measurement.minimum_supported_roqoqo_version()
            }
        }
    }
}
