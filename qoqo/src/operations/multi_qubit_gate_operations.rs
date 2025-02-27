// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::CircuitWrapper;
use num_complex::Complex64;
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::convert_into_calculator_float;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use qoqo_macros::*;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::HashMap;

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    Rotate,
    OperateMultiQubit,
    OperateGate,
    OperateMultiQubitGate,
    JsonSchema
)]
/// The Molmer-Sorensen gate between multiple qubits.
///
/// The gate applies the rotation under the product of Pauli X operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * theta/2 * X_i0 * X_i1 * ... * X_in).
pub struct MultiQubitMS {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    Rotate,
    OperateMultiQubit,
    OperateGate,
    OperateMultiQubitGate,
    JsonSchema
)]
/// The multi qubit Pauli-Z-Product gate.
///
/// The gate applies the rotation under the product of Pauli Z operators on multiple qubits.
/// In mathematical terms the gate applies exp(-i * theta/2 * Z_i0 * Z_i1 * ... * Z_in).
pub struct MultiQubitZZ {
    /// The qubits involved in the multi qubit Molmer-Sorensen gate.
    qubits: Vec<usize>,
    /// The angle of the multi qubit Molmer-Sorensen gate.
    theta: CalculatorFloat,
}

/// The gate to be replaced by a gate defined with GateDefinition gate.
/// The gate applies a gate previously defined by GateDefinition with the name gate_name.
///
/// Args:
///     gate_name (str) : The name of the called defined operations.
///     qubits (List[int]) : The qubits that for this call replace the qubits in the internal definition of the called gate
///                           (get replaced in order of apppearance in gate defintion).
///     free_parameters (List[CalculatorFloat]) : List of float values that replace the free parameters in the internal definition of the called gate
///                                             (get replaced in order of apppearance in gate defintion).
#[cfg(feature = "unstable_operation_definition")]
#[pyclass(name = "CallDefinedGate", module = "qoqo")]
#[derive(Debug, Clone, PartialEq)]
pub struct CallDefinedGateWrapper {
    /// Internal storage of [roqoqo::CallDefinedGate]
    pub internal: CallDefinedGate,
}

#[cfg(feature = "unstable_operation_definition")]
insert_pyany_to_operation!(
    "CallDefinedGate" =>{
        let gatenm = op.call_method0("gate_name")
                      .map_err(|_| QoqoError::ConversionError)?;
        let gate_name: String = gatenm.extract().map_err(|_| QoqoError::ConversionError)?;
        let qbts = op.call_method0("qubits")
                    .map_err(|_| QoqoError::ConversionError)?;
        let qubits: Vec<usize> = qbts.extract()
                .map_err(|_| QoqoError::ConversionError)?;

        let params = op.call_method0("free_parameters")
                        .map_err(|_| QoqoError::ConversionError)?;
        let param_vec: Bound<pyo3::types::PyList> = params.extract().map_err(|_| QoqoError::ConversionError)?;
        let mut free_parameters: Vec<CalculatorFloat> = vec![];
            for param in pyo3::types::PyListMethods::iter(&param_vec) {
                free_parameters.push(convert_into_calculator_float(&param.as_borrowed()).map_err(|_| QoqoError::ConversionError)?);
            }
        Ok(CallDefinedGate::new(gate_name, qubits, free_parameters).into())
    }
);
#[cfg(feature = "unstable_operation_definition")]
insert_operation_to_pyobject!(
    Operation::CallDefinedGate(internal) => {
        {
            let pyref: Py<CallDefinedGateWrapper> =
                Py::new(py, CallDefinedGateWrapper { internal }).unwrap();
            let pyobject: PyObject = pyref.to_object(py);
            Ok(pyobject)
        }
    }
);

#[cfg(feature = "unstable_operation_definition")]
#[pymethods]
impl CallDefinedGateWrapper {
    /// Create a new CallDefinedGate.
    ///
    /// Args:
    ///     gate_name (str) : The name of the called defined operations.
    ///     qubits (List[int]) : The qubits that for this call replace the qubits in the internal definition of the called gate
    ///                           (get replaced in order of apppearance in gate defintion).
    ///     free_parameters (List[CalculatorFloat]) : List of float values that replace the free parameters in the internal defintion of the called gate
    ///                                             (get replaced in order of apppearance in gate defintion).
    #[new]
    fn new(
        gate_name: String,
        qubits: Vec<usize>,
        free_parameters: Vec<Py<PyAny>>,
    ) -> PyResult<Self> {
        let free_parameters_cf: Vec<CalculatorFloat> =
            Python::with_gil(|py| -> PyResult<Vec<CalculatorFloat>> {
                let mut a = vec![];
                for param in free_parameters {
                    a.push(convert_into_calculator_float(param.bind(py)).map_err(|_| {
                        pyo3::exceptions::PyTypeError::new_err(
                            "Argument gate time cannot be converted to CalculatorFloat",
                        )
                    })?)
                }
                Ok(a)
            })?;

        Ok(Self {
            internal: CallDefinedGate::new(gate_name, qubits, free_parameters_cf),
        })
    }

    /// Return the name of the gate to apply.
    ///
    /// Returns:
    ///     str: The name of the gate.
    fn gate_name(&self) -> String {
        self.internal.gate_name().clone()
    }

    /// Return the qubits on which the Gate operation is applied.
    ///
    /// Returns:
    ///     List[int]: The qubits of the operation.
    fn qubits(&self) -> Vec<usize> {
        self.internal.qubits().clone()
    }

    /// Return the qubits on which the Gate operation is applied.
    ///
    /// Returns:
    ///     List[CalculatorFloat]: The qubits of the operation.
    fn free_parameters(&self) -> Vec<CalculatorFloatWrapper> {
        self.internal
            .free_parameters()
            .iter()
            .map(|param| CalculatorFloatWrapper {
                internal: param.clone(),
            })
            .collect::<Vec<CalculatorFloatWrapper>>()
    }

    /// List all involved qubits.
    ///
    /// Returns:
    ///     Set[int]: The involved qubits of the operation.
    fn involved_qubits(&self) -> PyObject {
        let pyobject: PyObject = Python::with_gil(|py| -> PyObject {
            PySet::new(py, &[self.internal.qubits().clone()])
                .unwrap()
                .to_object(py)
        });
        pyobject
    }

    /// Return tags classifying the type of the operation.
    ///
    /// Used for the type based dispatch in ffi interfaces.
    ///
    /// Returns:
    ///     List[str]: The tags of the Operation.
    fn tags(&self) -> Vec<String> {
        self.internal.tags().iter().map(|s| s.to_string()).collect()
    }

    /// Return hqslang name of the operation.
    ///
    /// Returns:
    ///     str: The hqslang name of the operation.
    fn hqslang(&self) -> &'static str {
        self.internal.hqslang()
    }

    /// Return true when the operation has symbolic parameters.
    ///
    /// Returns:
    ///     bool: True if the operation contains symbolic parameters, False if it does not.
    fn is_parametrized(&self) -> bool {
        self.internal.is_parametrized()
    }

    /// Substitute the symbolic parameters in a clone of the operation according to the input.
    ///
    /// Args:
    ///     substitution_parameters (Dict[str, float]): The dictionary containing the substitutions to use in the operation.
    ///
    /// Returns:
    ///     self: The operation with the parameters substituted.
    ///
    /// Raises:
    ///     RuntimeError: The parameter substitution failed.
    fn substitute_parameters(
        &self,
        substitution_parameters: std::collections::HashMap<String, f64>,
    ) -> PyResult<Self> {
        let mut calculator = qoqo_calculator::Calculator::new();
        for (key, val) in substitution_parameters.iter() {
            calculator.set_variable(key, *val);
        }
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(&calculator)
                .map_err(|x| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Parameter Substitution failed: {:?}",
                        x
                    ))
                })?,
        })
    }

    /// Remap qubits in a clone of the CallDefinedGate operation.
    ///
    /// Args:
    ///     mapping (Dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the operation.
    ///
    /// Returns:
    ///     self: The operation with the qubits remapped.
    ///
    /// Raises:
    ///     RuntimeError: The qubit remapping failed.
    fn remap_qubits(&self, mapping: std::collections::HashMap<usize, usize>) -> PyResult<Self> {
        let new_internal = self
            .internal
            .remap_qubits(&mapping)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Qubit remapping failed: "))?;
        Ok(Self {
            internal: new_internal,
        })
    }

    /// Return a copy of the operation (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     CallDefinedGate: A deep copy of self.
    fn __copy__(&self) -> CallDefinedGateWrapper {
        self.clone()
    }

    /// Return a deep copy of the operation.
    ///
    /// Returns:
    ///     CallDefinedGate: A deep copy of self.
    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> CallDefinedGateWrapper {
        self.clone()
    }

    /// Return a string containing a formatted (string) representation of the operation.
    ///
    /// Returns:
    ///     str: The string representation of the operation.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return a string containing a printable representation of the operation.
    ///
    /// Returns:
    ///     str: The printable string representation of the operation.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on CallDefinedGate.
    ///
    /// Args:
    ///     self: The CallDefinedGate object.
    ///     other: The object to compare self to.
    ///     op: Type of comparison.
    ///
    /// Returns:
    ///     bool: Whether the two operations compared evaluated to True or False.
    fn __richcmp__(
        &self,
        other: &Bound<PyAny>,
        op: pyo3::class::basic::CompareOp,
    ) -> PyResult<bool> {
        let other: Operation =
            crate::operations::convert_pyany_to_operation(other).map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err(
                    "Right hand side cannot be converted to Operation",
                )
            })?;
        match op {
            pyo3::class::basic::CompareOp::Eq => {
                Ok(Operation::from(self.internal.clone()) == other)
            }
            pyo3::class::basic::CompareOp::Ne => {
                Ok(Operation::from(self.internal.clone()) != other)
            }
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }

    #[cfg(feature = "json_schema")]
    /// Return the JsonSchema for the json serialisation of the class.
    ///
    /// Returns:
    ///     str: The json schema serialized to json
    #[staticmethod]
    pub fn json_schema() -> String {
        let schema = schemars::schema_for!(CallDefinedGate);
        serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
    }

    #[cfg(feature = "json_schema")]
    /// Returns the current version of the qoqo library .
    ///
    /// Returns:
    ///     str: The current version of the library.
    #[staticmethod]
    pub fn current_version() -> String {
        ROQOQO_VERSION.to_string()
    }

    #[cfg(feature = "json_schema")]
    /// Return the minimum version of qoqo that supports this object.
    ///
    /// Returns:
    ///     str: The minimum version of the qoqo library to deserialize this object.
    pub fn min_supported_version(&self) -> String {
        let min_version: (u32, u32, u32) =
            CallDefinedGate::minimum_supported_roqoqo_version(&self.internal);
        format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
    }
}
