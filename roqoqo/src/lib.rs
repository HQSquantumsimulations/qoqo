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

#![deny(missing_docs)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::private_doc_tests)]
#![deny(missing_debug_implementations)]

//! # roqoqo
//!
//! `Rust only Quantum Operation Quantum Operation` - the quantum computing toolkit by HQS Quantum Simulations.
//!
use qoqo_calculator::CalculatorError;
use qoqo_calculator::CalculatorFloat;
// #[cfg(feature = "json_schema")]
// use schemars::{schema::Schema, JsonSchema};
use std::str::FromStr;
use thiserror::Error;

/// roqoqo version information, used for roqoqo import/export checks
pub const ROQOQO_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(try_from = "RoqoqoVersionSerializable"))]
#[cfg_attr(feature = "serialize", serde(into = "RoqoqoVersionSerializable"))]

struct RoqoqoVersion;

// #[cfg(feature = "json_schema")]
// impl JsonSchema for RoqoqoVersion {
//     fn schema_name() -> String {
//         "RoqoqoVersion".to_string()
//     }

//     fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
//         RoqoqoVersionSerializable::json_schema(gen)
//     }
// }
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
struct RoqoqoVersionSerializable {
    /// The semver major version of roqoqo
    major_version: u32,
    /// The semver minor version of roqoqo
    minor_version: u32,
}

impl TryFrom<RoqoqoVersionSerializable> for RoqoqoVersion {
    type Error = RoqoqoError;

    fn try_from(value: RoqoqoVersionSerializable) -> Result<Self, Self::Error> {
        let mut rsplit = ROQOQO_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");
        if major_version != value.major_version {
            return Err(RoqoqoError::VersionMissmatch {
                library_major_version: major_version,
                library_minor_version: minor_version,
                data_major_version: value.major_version,
                data_minor_version: value.minor_version,
            });
        }
        if major_version == 0 {
            if minor_version != value.minor_version {
                return Err(RoqoqoError::VersionMissmatch {
                    library_major_version: major_version,
                    library_minor_version: minor_version,
                    data_major_version: value.major_version,
                    data_minor_version: value.minor_version,
                });
            }
        } else if minor_version < value.minor_version {
            return Err(RoqoqoError::VersionMissmatch {
                library_major_version: major_version,
                library_minor_version: minor_version,
                data_major_version: value.major_version,
                data_minor_version: value.minor_version,
            });
        }
        Ok(RoqoqoVersion)
    }
}

impl From<RoqoqoVersion> for RoqoqoVersionSerializable {
    fn from(_: RoqoqoVersion) -> Self {
        let mut rsplit = ROQOQO_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");
        RoqoqoVersionSerializable {
            major_version,
            minor_version,
        }
    }
}

#[inline]
fn update_roqoqo_version(
    current_minimum_version: &mut (u32, u32, u32),
    comparison_version: (u32, u32, u32),
) {
    if current_minimum_version.0 < comparison_version.0
        || current_minimum_version.1 < comparison_version.1
        || current_minimum_version.2 < comparison_version.2
    {
        *current_minimum_version = comparison_version;
    }
}

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
    #[error("Mapping failed. Qubit map maps to qubit {qubit:?} but not from {qubit:?}")]
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
    PauliZProductMeasurementError {
        /// Error message.
        msg: String,
    },
    /// Error serializing an internal roqoqo object
    #[error("An error occured serializing a roqoqo object: {msg} ")]
    SerializationError {
        /// Error message
        msg: String,
    },
    /// Generic error that does not fit in other error categories.
    #[error("An error occured in roqoqo: {msg} ")]
    GenericError {
        /// Generic error message
        msg: String,
    },
    /// Error when trying to deserialize roqoqo data created with an incompatible version of roqoqo
    #[error("Trying to deserialize data created with incompatible version of roqoqo Library version: {library_major_version}.{library_minor_version} Data version: {data_major_version}.{data_minor_version}. Try to convert data with roqoqo data conversion tool.")]
    VersionMissmatch {
        /// Major version of the library
        library_major_version: u32,
        /// Minor version of the library
        library_minor_version: u32,
        /// Major version of the data
        data_major_version: u32,
        /// Minor version of the data
        data_minor_version: u32,
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
        /// Generic error message
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
pub use circuit::Circuit;
#[doc(hidden)]
pub use circuit::*;
#[cfg(feature = "circuitdag")]
mod circuitdag;
#[cfg(feature = "circuitdag")]
pub use circuitdag::CircuitDag;
pub mod backends;
pub mod devices;
pub mod measurements;
pub mod operations;
pub mod prelude;
#[doc(hidden)]
mod quantum_program;
pub mod registers;
pub use quantum_program::QuantumProgram;
