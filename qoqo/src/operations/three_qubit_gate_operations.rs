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

use num_complex::Complex64;
use numpy::{PyArray2, ToPyArray};

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PySet;

use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::{convert_into_calculator_float, CalculatorFloatWrapper};

use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

use std::collections::HashMap;

use crate::CircuitWrapper;

use qoqo_macros::*;

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
#[derive(Eq)]
/// Implements the double-controlled PauliZ gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & -1
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
pub struct ControlledControlledPauliZ {
    control_0: usize,
    control_1: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    Rotate,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// Implements the double-controlled PhaseShift gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i \theta}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
///     theta (float): The rotation angle θ.
pub struct ControlledControlledPhaseShift {
    control_0: usize,
    control_1: usize,
    target: usize,
    theta: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// Implements Toffoli gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
///     target (int): The index of the least significant qubit in the unitary representation. Here, the qubit the PauliX gate is applied to.
pub struct Toffoli {
    control_0: usize,
    control_1: usize,
    target: usize,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(OperateGate, OperateThreeQubitGate, JsonSchema)]
/// Implements ControlledSWAP gate.
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & 1
///         \end{pmatrix}
///
/// Args:
///     control (int): The index of the most significant qubit in the unitary representation. Here, the controlling qubit of the operation.
///     target_0 (int): The index of the second most significant qubit in the unitary representation. Here, the first targeting qubit of the operation.
///     target_1 (int): The index of the least significant qubit in the unitary representation. Here, the second targeting qubit of the operation.
pub struct ControlledSWAP {
    control: usize,
    target_0: usize,
    target_1: usize,
}

#[pymethods]
impl ControlledSWAPWrapper {
    #[new]
    /// Creates new instance of Operation ControlledSWAP
    fn new(control: usize, target_0: usize, target_1: usize) -> PyResult<Self> {
        Ok(Self {
            internal: ControlledSWAP::new(control, target_0, target_1),
        })
    }
    /// Returns true if operation contains symbolic parameters
    ///
    /// Returns:
    ///     bool: Whether or not the operation contains symbolic parameters.
    fn is_parametrized(&self) -> bool {
        self.internal.is_parametrized()
    }
    /// Returns tags identifying the Operation
    ///
    /// Returns:
    ///     List[str]: The tags identifying the operation
    fn tags(&self) -> Vec<String> {
        self.internal.tags().iter().map(|s| s.to_string()).collect()
    }
    /// Returns hqslang name of Operation
    ///
    /// Returns:
    ///     str: The name
    fn hqslang(&self) -> &'static str {
        self.internal.hqslang()
    }
    /// Substitutes internal symbolic parameters with float values
    ///
    /// Only available when all symbolic expressions can be evaluated to float with the
    /// provided parameters.
    ///
    /// Args:
    ///     substitution_parameters (Dict[str, float]): The substituted free parameters
    ///
    /// Returns:
    ///     Operation: The operation with the parameters substituted
    ///
    /// Raises:
    ///     RuntimeError: Parameter Substitution failed
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
    /// Remap qubits in the ControlledSWAP operation
    ///
    /// Args:
    ///     mapping (Dict[int, int]): The mapping to be used in the remapping.
    ///
    /// Returns:
    ///     Operation: The operation with the remapped qubits
    ///
    /// Raises:
    ///     RuntimeError: Qubit remapping failed
    fn remap_qubits(&self, mapping: HashMap<usize, usize>) -> PyResult<Self> {
        let new_internal = self
            .internal
            .remap_qubits(&mapping)
            .map_err(|x| PyRuntimeError::new_err(format!("Qubit remapping failed: {:?}", x)))?;
        Ok(Self {
            internal: new_internal,
        })
    }
    /// List all involved qubits in the ControlledSWAP operation.
    ///
    /// Returns:
    ///     Union[Set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
    fn involved_qubits(&self) -> PyObject {
        Python::with_gil(|py| -> PyObject {
            let involved = self.internal.involved_qubits();
            match involved {
                InvolvedQubits::All => {
                    let pyref: &Bound<PySet> = &PySet::new(py, ["All"]).unwrap();
                    let pyobject: PyObject = pyref.to_object(py);
                    pyobject
                }
                InvolvedQubits::None => {
                    let pyref: &Bound<PySet> = &PySet::empty(py).unwrap();
                    let pyobject: PyObject = pyref.to_object(py);
                    pyobject
                }
                InvolvedQubits::Set(x) => {
                    let mut vector: Vec<usize> = Vec::new();
                    for qubit in x {
                        vector.push(qubit)
                    }
                    let pyref: &Bound<PySet> = &PySet::new(py, &vector[..]).unwrap();
                    let pyobject: PyObject = pyref.to_object(py);
                    pyobject
                }
            }
        })
    }
    /// Copies Operation
    ///
    /// For qoqo operations copy is always a deep copy
    fn __copy__(&self) -> Self {
        self.clone()
    }
    /// Creates deep copy of Operation
    fn __deepcopy__(&self, _memodict: &Bound<PyAny>) -> Self {
        self.clone()
    }
    /// Returns control qubit of the three-qubit operation
    pub fn control(&self) -> usize {
        *self.internal.control_0()
    }
    /// Returns target_0 qubit of the three-qubit operation
    pub fn target_0(&self) -> usize {
        *self.internal.control_1()
    }
    /// Returns target_1 qubit of the three-qubit operation
    pub fn target_1(&self) -> usize {
        *self.internal.target()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// The phased-shifted double-controlled-Z gate.
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & e^{i \phi} & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & e^{i \phi} & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & e^{i \phi} & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i (3\cdot\phi + \pi)}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first qubit that controls the application of the phase-shift on the target qubit.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///
pub struct PhaseShiftedControlledControlledZ {
    control_0: usize,
    control_1: usize,
    target: usize,
    phi: CalculatorFloat,
}

#[allow(clippy::upper_case_acronyms)]
#[wrap(
    Operate,
    OperateThreeQubit,
    OperateGate,
    Rotate,
    OperateThreeQubitGate,
    JsonSchema
)]
/// The phased-shifted double-controlled-Z gate.
///
/// The unitary matrix representation is:
///
/// .. math::
///     U = \begin{pmatrix}
///         1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & e^{i \phi} & 0 & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & e^{i \phi} & 0 & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & e^{i \phi} & 0 & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & e^{i (2\cdot\phi)} & 0 \\\\
///         0 & 0 & 0 & 0 & 0 & 0 & 0 & e^{i (3\cdot\phi + \theta)}
///         \end{pmatrix}
///
/// Args:
///     control_0 (int): The index of the most significant qubit in the unitary representation. Here, the first qubit that controls the application of the phase-shift on the target qubit.
///     control_1 (int): The index of the second most significant qubit in the unitary representation. Here, the second qubit that controls the application of the phase-shift on the target qubit.
///     target (int):: The index of the least significant qubit in the unitary representation. Here, the qubit phase-shift is applied to.
///     phi (CalculatorFloat): The single qubit phase $\phi$.
///     theta (CalculatorFloat): The phase rotation $\theta$.
///
pub struct PhaseShiftedControlledControlledPhase {
    control_0: usize,
    control_1: usize,
    target: usize,
    theta: CalculatorFloat,
    phi: CalculatorFloat,
}
