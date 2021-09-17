// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

#![deny(missing_docs)]
#![warn(private_intra_doc_links)]
#![warn(missing_crate_level_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![deny(missing_debug_implementations)]

//! # roqoqo
//!
//! `Rust only Quantum Operation Quantum Operation` - the quantum computing toolkit by HQS Quantum Simulations.
//!
pub use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorError;
pub use qoqo_calculator::CalculatorFloat;
use thiserror::Error;

/// roqoqo version information, used for roqoqo import/export checks
pub const ROQOQO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Errors that can occur in roqoqo.
#[derive(Error, Debug, PartialEq)]
pub enum RoqoqoError {
    /// Error when values of alpha and beta lead to an invalid unitary matrix.
    #[error("Resulting gate matrix is not unitary. Please check values of alpha and beta: alpha_r: {alpha_r:?}, alpha_i: {alpha_i:?}, beta_r: {beta_r:?}, beta_i: {beta_i:?}, norm: {norm:?}.")]
    UnitaryMatrixErrror {
        /// Real part of diagonal element of (not) unitary matrix.
        alpha_r: f64,
        /// Imaginary part of diagonal element of (not) unitary matrix.
        alpha_i: f64,
        /// Real part of off-diagonal element of (not) unitary matrix.
        beta_r: f64,
        /// Real part of off-diagonal element of (not) unitary matrix.
        beta_i: f64,
        /// Norm of (not) unitary matrix.
        norm: f64,
    },
    /// Error when remapping qubits fails because qubit in operation is not in keys of HashMap/dict.
    #[error("Mapping of qubit {qubit:?} failed")]
    QubitMappingError {
        /// Qubit that can not be mapped.
        qubit: usize,
    },
    /// Custom error for failed conversion between enums with the TryFrom trait.
    #[error("Conversion from {start_type} to {end_type} failed")]
    ConversionError {
        /// Type from which should be converted.
        start_type: &'static str,
        /// Type into which should be converted.
        end_type: &'static str,
    },
    /// Error using try from  
    #[error("TryFrom conversion failed")]
    TryFromError,
    /// Custom error for failed multipliction of two gates acting on different qubits.
    #[error("Qubits {squbit} and {oqubit} incompatible. Gates acting on different qubits can not be multiplied.")]
    MultiplicationIncompatibleQubits {
        /// Self qubit of the operation on the left hand.
        squbit: usize,
        /// Other qubit of the operation on the right hand.
        oqubit: usize,
    },
    /// Error adding a PauliProduct involving qubits larger than number of qubit to measurement input.
    #[error("Pauli product involves qubit {pp_qubit} but number qubits is lower {number_qubits}.")]
    PauliProductExceedsQubits {
        /// Qubit involved in Pauli product.
        pp_qubit: usize,
        /// Number of qubits in measurement.
        number_qubits: usize,
    },
    /// Error when adding a new operator to expectation values.
    #[error(
        "Index of operator {index:?} exceeds Hilbert space dimension of {number_qubits} qubits."
    )]
    MismatchedOperatorDimension {
        /// Index not matching dimensions.
        index: (usize, usize),
        /// Number of qubits in measurement.
        number_qubits: usize,
    },
    /// Error when a complex register does not correspond to the expected dimension for cheated measurement.
    #[error(
        "Dimension of register {dim:?} exceeds Hilbert space dimension of {number_qubits} qubits."
    )]
    MismatchedRegisterDimension {
        /// Index not matching dimensions.
        dim: usize,
        /// Number of qubits in measurement.
        number_qubits: usize,
    },
    /// Error adding an expectation value, name of expectation value already take.
    #[error("Name {name} of expectation value already taken.")]
    ExpValUsedTwice {
        /// Name of the expecataion value missing.
        name: String,
    },
    /// Expected register is missing from the Output registers.
    #[error("OutputRegister {name} is missing.")]
    MissingRegister {
        /// Name of the missing register.
        name: String,
    },
    /// Error occured in basis rotation measurement.
    #[error("Error occured in basis rotation measurement. {msg}")]
    BasisRotationMeasurementError {
        /// Error message.
        msg: String,
    },
    // /// Rates matrix has negative eigenvalues, when they should be positive semi-definite.
    // #[error("Rates matrix has a negative eigenvalue: {value}")]
    // NegativeEigenvalue {
    //     /// Negative eigenvalue.
    //     value: f64,
    // },
    /// Transparent propagation of CalculatorError.
    #[error(transparent)]
    CalculatorError(#[from] CalculatorError),
}

/// Errors that can occur in roqoqo backends.
#[derive(Error, Debug, PartialEq)]
pub enum RoqoqoBackendError {
    /// Error operation not supported by backend
    #[error("Operation {hqslang} not supported by backend {hqslang}: ")]
    OperationNotInBackend {
        /// Name of the backend.
        backend: &'static str,
        /// hqslang name of the operation.
        hqslang: &'static str,
    },
    /// Error for backends missing authentification information.
    #[error("Backend authentification information is missing: {msg} ")]
    MissingAuthentification {
        /// Error msg
        msg: String,
    },
    /// Error when communicating with backend over the network.
    #[error("NetworkError communicating with: {msg} ")]
    NetworkError {
        /// Error msg
        msg: String,
    },
    /// Error when communicating with backend over the network.
    #[error("Backend timed out: {msg} ")]
    Timeout {
        /// Error msg
        msg: String,
    },
    /// Error when communicating with backend over the network.
    #[error("The file at this location already exists: {path} ")]
    FileAlreadyExists {
        /// Path of file to be created
        path: String,
    },
    /// Error when communicating with backend over the network.
    #[error("An error occured in the backend: {msg} ")]
    GenericError {
        /// Path of file to be created
        msg: String,
    },
    /// Transparent propagation of RoqoqoError.
    #[error(transparent)]
    RoqoqoError(#[from] RoqoqoError),
    /// Transparent propagation of CalculatorError.
    #[error(transparent)]
    CalculatorError(#[from] CalculatorError),
}

#[doc(hidden)]
mod circuit;
pub mod operations;
pub mod prelude;
pub use circuit::*;
pub mod backends;
pub mod devices;
pub mod measurements;
pub mod registers;
