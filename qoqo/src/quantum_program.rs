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

use std::collections::HashMap;

use crate::measurements::{
    CheatedPauliZProductWrapper, CheatedWrapper, ClassicalRegisterWrapper, PauliZProductWrapper,
};
use crate::{QoqoError, QOQO_VERSION};
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use roqoqo::measurements::Measure;
use roqoqo::QuantumProgram;
use roqoqo::ROQOQO_VERSION;

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
#[pyclass(name = "QuantumProgram", module = "qoqo")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuantumProgramWrapper {
    /// Internal storage of [roqoqo::QuantumProgram]
    pub internal: QuantumProgram,
}

impl QuantumProgramWrapper {
    /// Extracts a QuantumProgram from a QuantumProgramWrapper python object.
    ///
    /// When working with qoqo and other rust based python packages compiled separately
    /// a downcast will not detect that two QuantumProgramWrapper objects are compatible.
    /// Provides a custom function to convert qoqo QuantumPrograms between different Python packages.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo::QuantumProgram]
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<QuantumProgram> {
        Python::with_gil(|py| -> PyResult<QuantumProgram> {
            let input = input.as_ref(py);
            if let Ok(try_downcast) = input.extract::<QuantumProgramWrapper>() {
                Ok(try_downcast.internal)
            } else {
                let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo QuantumProgram: Cast to binary representation failed".to_string())
            })?;
                let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                PyTypeError::new_err("Python object cannot be converted to qoqo QuantumProgram: Cast to binary representation failed".to_string())
            })?;
                deserialize(&bytes[..]).map_err(|err| {
                    PyTypeError::new_err(format!(
                    "Python object cannot be converted to qoqo QuantumProgram: Deserialization failed: {}",
                    err
                ))
                })
            }
        })
    }
}

#[pymethods]
impl QuantumProgramWrapper {
    /// Create a QuantumProgram.
    ///
    /// Args:
    ///     measurement:
    ///     input_parameter_names (List[str]):
    ///
    /// Returns:
    ///     self: The new .
    #[new]
    pub fn new(measurement: &PyAny, input_parameter_names: Vec<String>) -> PyResult<Self> {
        if let Ok(try_downcast) = PauliZProductWrapper::from_pyany(measurement.into()) {
            return Ok(Self {
                internal: QuantumProgram::PauliZProduct {
                    measurement: try_downcast,
                    input_parameter_names,
                },
            });
        }
        if let Ok(try_downcast) = CheatedPauliZProductWrapper::from_pyany(measurement.into()) {
            return Ok(Self {
                internal: QuantumProgram::CheatedPauliZProduct {
                    measurement: try_downcast,
                    input_parameter_names,
                },
            });
        }
        if let Ok(try_downcast) = CheatedWrapper::from_pyany(measurement.into()) {
            return Ok(Self {
                internal: QuantumProgram::Cheated {
                    measurement: try_downcast,
                    input_parameter_names,
                },
            });
        }
        if let Ok(try_downcast) = ClassicalRegisterWrapper::from_pyany(measurement.into()) {
            return Ok(Self {
                internal: QuantumProgram::ClassicalRegister {
                    measurement: try_downcast,
                    input_parameter_names,
                },
            });
        }

        Err(PyTypeError::new_err(
            "measurement is not of type Measurement. Are you using different versions of roqoqo?",
        ))
    }

    /// Returns the measurement attribute of the QuantumProgram as Python object.
    ///
    /// Returns:
    ///     PyObject corresponding to the qoqo measurement type of the QuantumProgram,
    ///     i.e. PauliZProduct, CheatedPauliZProduct, Cheated or ClassicalRegister.
    pub fn measurement(&self) -> PyObject {
        match self.internal.clone() {
            QuantumProgram::PauliZProduct {
                measurement,
                input_parameter_names: _,
            } => Python::with_gil(|py| -> PyObject {
                let pyref: Py<PauliZProductWrapper> = Py::new(
                    py,
                    PauliZProductWrapper {
                        internal: measurement.clone(),
                    },
                )
                .unwrap();
                pyref.to_object(py)
            }),
            QuantumProgram::CheatedPauliZProduct {
                measurement,
                input_parameter_names: _,
            } => Python::with_gil(|py| -> PyObject {
                let pyref: Py<CheatedPauliZProductWrapper> = Py::new(
                    py,
                    CheatedPauliZProductWrapper {
                        internal: measurement.clone(),
                    },
                )
                .unwrap();
                pyref.to_object(py)
            }),
            QuantumProgram::Cheated {
                measurement,
                input_parameter_names: _,
            } => Python::with_gil(|py| -> PyObject {
                let pyref: Py<CheatedWrapper> = Py::new(
                    py,
                    CheatedWrapper {
                        internal: measurement.clone(),
                    },
                )
                .unwrap();
                pyref.to_object(py)
            }),
            QuantumProgram::ClassicalRegister {
                measurement,
                input_parameter_names: _,
            } => Python::with_gil(|py| -> PyObject {
                let pyref: Py<ClassicalRegisterWrapper> = Py::new(
                    py,
                    ClassicalRegisterWrapper {
                        internal: measurement.clone(),
                    },
                )
                .unwrap();
                pyref.to_object(py)
            }),
        }
    }

    /// Returns the input_parameter_names attribute of the qoqo QuantumProgram.
    ///
    /// Returns:
    ///     List of input parameter names.
    pub fn input_parameter_names(&self) -> Vec<String> {
        match self.internal.clone() {
            QuantumProgram::PauliZProduct {
                measurement: _,
                input_parameter_names,
            } => input_parameter_names,
            QuantumProgram::CheatedPauliZProduct {
                measurement: _,
                input_parameter_names,
            } => input_parameter_names,
            QuantumProgram::Cheated {
                measurement: _,
                input_parameter_names,
            } => input_parameter_names,
            QuantumProgram::ClassicalRegister {
                measurement: _,
                input_parameter_names,
            } => input_parameter_names,
        }
    }

    /// Runs the QuantumProgram and returns expectation values.
    ///
    /// Runs the quantum programm for a given set of parameters passed in the same order as the parameters
    /// listed in `input_parameter_names` and returns expectation values.
    ///
    /// Args:
    ///     backend (Backend): The backend the program is executed on.
    ///     parameters (Optional[List[float]): List of float  parameters of the function call in order of `input_parameter_names`
    pub fn run(&self, backend: Py<PyAny>, parameters: Option<Vec<f64>>) -> PyResult<Py<PyAny>> {
        let parameters = parameters.unwrap_or_default();
        match &self.internal{
            QuantumProgram::PauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(PyValueError::new_err( format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())))};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                ).map_err(|err| PyRuntimeError::new_err(format!("Applying parameters failed {:?}", err)))?;
                Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                    backend.call_method1(py, "run_measurement", (PauliZProductWrapper{internal: substituted_measurement}, ))
                })            }
            QuantumProgram::CheatedPauliZProduct{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(PyValueError::new_err( format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())))};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                ).map_err(|err| PyRuntimeError::new_err(format!("Applying parameters failed {:?}", err)))?;
                Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                    backend.call_method1(py, "run_measurement", (CheatedPauliZProductWrapper{internal: substituted_measurement}, ))
                })
            }
            QuantumProgram::Cheated{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(PyValueError::new_err( format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())))};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                ).map_err(|err| PyRuntimeError::new_err(format!("Applying parameters failed {:?}", err)))?;
                Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                    backend.call_method1(py, "run_measurement", (CheatedWrapper{internal: substituted_measurement}, ))
                })            }
            _ => Err(PyTypeError::new_err("A quantum programm returning classical registeres cannot be executed by `run` use `run_registers` instead".to_string()))
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
    /// Args:
    ///     backend (Backend): The backend the program is executed on.
    ///     parameters (Optional[List[float]): List of float  parameters of the function call in order of `input_parameter_names`
    pub fn run_registers(
        &self,
        backend: Py<PyAny>,
        parameters: Option<Vec<f64>>,
    ) -> PyResult<Py<PyAny>> {
        let parameters = parameters.unwrap_or_default();
        match &self.internal{
            QuantumProgram::ClassicalRegister{measurement, input_parameter_names } => {
                if parameters.len() != input_parameter_names.len() { return Err(PyValueError::new_err( format!("Wrong number of parameters {} parameters expected {} parameters given", input_parameter_names.len(), parameters.len())))};
                let substituted_parameters: HashMap<String, f64> = input_parameter_names.iter().zip(parameters.iter()).map(|(key, value)| (key.clone(), *value)).collect();
                let substituted_measurement = measurement.substitute_parameters(
                    substituted_parameters
                ).map_err(|err| PyRuntimeError::new_err(format!("Applying parameters failed {:?}", err)))?;
                Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                    backend.call_method1(py, "run_measurement_registers", (ClassicalRegisterWrapper{internal: substituted_measurement}, ))
                })           },
            _ => Err(PyTypeError::new_err("A quantum programm returning expectation values cannot be executed by `run_registers` use `run` instead".to_string()))
        }
    }

    /// Return a copy of the QuantumProgram (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     QuantumProgram: A deep copy of self.
    pub fn __copy__(&self) -> QuantumProgramWrapper {
        self.clone()
    }

    /// Return a deep copy of the QuantumProgram.
    ///
    /// Returns:
    ///     QuantumProgram: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> QuantumProgramWrapper {
        self.clone()
    }

    /// Return the roqoqo and qoqo versions from when the code was compiled.
    ///
    /// Returns:
    ///     tuple[str, str]: The roqoqo and qoqo versions.
    fn _qoqo_versions(&self) -> (String, String) {
        let mut rsplit = ROQOQO_VERSION.split('.').take(2);
        let mut qsplit = QOQO_VERSION.split('.').take(2);
        let rver = format!(
            "{}.{}",
            rsplit.next().expect("ROQOQO_VERSION badly formatted"),
            rsplit.next().expect("ROQOQO_VERSION badly formatted")
        );
        let qver = format!(
            "{}.{}",
            qsplit.next().expect("QOQO_VERSION badly formatted"),
            qsplit.next().expect("QOQO_VERSION badly formatted")
        );
        (rver, qver)
    }

    /// Return the bincode representation of the QuantumProgram using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized QuantumProgram (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QuantumProgram to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize QuantumProgram to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    #[staticmethod]
    /// Convert the bincode representation of the QuantumProgram to a QuantumProgram using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized QuantumProgram (in [bincode] form).
    ///
    /// Returns:
    ///     QuantumProgram: The deserialized QuantumProgram.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to QuantumProgram.
    pub fn from_bincode(input: &PyAny) -> PyResult<Self> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(Self {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QuantumProgram")
            })?,
        })
    }

    /// Return the json representation of the QuantumProgram.
    ///
    /// Returns:
    ///     str: The serialized form of QuantumProgram.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QuantumProgram to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize QuantumProgram to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a QuantumProgram to a QuantumProgram.
    ///
    /// Args:
    ///     input (str): The serialized QuantumProgram in json form.
    ///
    /// Returns:
    ///     QuantumProgram: The deserialized QuantumProgram.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to QuantumProgram.
    #[staticmethod]
    fn from_json(input: &str) -> PyResult<Self> {
        Ok(Self {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QuantumProgram")
            })?,
        })
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on QuantumProgram.
    ///
    /// Args:
    ///     other: The object to compare self to.
    ///     op: Type of comparison.
    ///
    /// Returns:
    ///     Whether the two operations compared evaluated to True or False
    ///
    /// Raises:
    ///     NotImplementedError: Other comparison not implemented
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other = Python::with_gil(|py| -> Result<QuantumProgram, QoqoError> {
            let other_ref = other.as_ref(py);
            convert_into_quantum_program(other_ref)
        });
        match op {
            pyo3::class::basic::CompareOp::Eq => match other {
                Ok(qp) => Ok(self.internal == qp),
                _ => Ok(false),
            },
            pyo3::class::basic::CompareOp::Ne => match other {
                Ok(qp) => Ok(self.internal != qp),
                _ => Ok(true),
            },
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented",
            )),
        }
    }
}

/// Convert generic python object to [roqoqo::QuantumProgram].
///
/// Fallible conversion of generic python object to [roqoqo::QuantumProgram].
pub fn convert_into_quantum_program(input: &PyAny) -> Result<QuantumProgram, QoqoError> {
    if let Ok(try_downcast) = input.extract::<QuantumProgramWrapper>() {
        return Ok(try_downcast.internal);
    }
    // Everything that follows tries to extract the quantum program when two separately
    // compiled python packages are involved
    // let get_version = input
    //     .call_method0("_qoqo_versions")
    //     .map_err(|_| QoqoError::CannotExtractObject)?;
    // let version = get_version
    //     .extract::<(&str, &str)>()
    //     .map_err(|_| QoqoError::CannotExtractObject)?;
    // let mut rsplit = ROQOQO_VERSION.split('.').take(2);
    // let mut qsplit = QOQO_VERSION.split('.').take(2);
    // let rver = format!(
    //     "{}.{}",
    //     rsplit.next().expect("ROQOQO_VERSION badly formatted"),
    //     rsplit.next().expect("ROQOQO_VERSION badly formatted")
    // );
    // let qver = format!(
    //     "{}.{}",
    //     qsplit.next().expect("QOQO_VERSION badly formatted"),
    //     qsplit.next().expect("QOQO_VERSION badly formatted")
    // );
    // let test_version: (&str, &str) = (rver.as_str(), qver.as_str());
    // if version == test_version {
    let get_bytes = input
        .call_method0("to_bincode")
        .map_err(|_| QoqoError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoError::CannotExtractObject)?;
    deserialize(&bytes[..]).map_err(|_| QoqoError::CannotExtractObject)
    // } else {
    //     Err(QoqoError::VersionMismatch)
    // }
}
