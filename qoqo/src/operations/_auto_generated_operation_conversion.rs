use crate::convert_into_circuit;
use crate::operations::*;
use crate::QoqoError;
use ndarray::{Array, Array1};
use num_complex::Complex64;
use pyo3::conversion::ToPyObject;
use pyo3::prelude::*;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::convert_into_calculator_float;
use roqoqo::operations::*;
use std::collections::HashMap;
#[doc = r" Tries to convert a [roqoqo::operations::Operation] to a PyObject"]
pub fn convert_operation_to_pyobject(operation: Operation) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        match operation {
            Operation::SingleQubitGate(internal) => {
                let pyref: Py<SingleQubitGateWrapper> =
                    Py::new(py, SingleQubitGateWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::RotateX(internal) => {
                let pyref: Py<RotateXWrapper> = Py::new(py, RotateXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::RotateY(internal) => {
                let pyref: Py<RotateYWrapper> = Py::new(py, RotateYWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::RotateZ(internal) => {
                let pyref: Py<RotateZWrapper> = Py::new(py, RotateZWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PauliX(internal) => {
                let pyref: Py<PauliXWrapper> = Py::new(py, PauliXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PauliY(internal) => {
                let pyref: Py<PauliYWrapper> = Py::new(py, PauliYWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PauliZ(internal) => {
                let pyref: Py<PauliZWrapper> = Py::new(py, PauliZWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::SqrtPauliX(internal) => {
                let pyref: Py<SqrtPauliXWrapper> =
                    Py::new(py, SqrtPauliXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::InvSqrtPauliX(internal) => {
                let pyref: Py<InvSqrtPauliXWrapper> =
                    Py::new(py, InvSqrtPauliXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::Hadamard(internal) => {
                let pyref: Py<HadamardWrapper> = Py::new(py, HadamardWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::SGate(internal) => {
                let pyref: Py<SGateWrapper> = Py::new(py, SGateWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::TGate(internal) => {
                let pyref: Py<TGateWrapper> = Py::new(py, TGateWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::RotateAroundSphericalAxis(internal) => {
                let pyref: Py<RotateAroundSphericalAxisWrapper> =
                    Py::new(py, RotateAroundSphericalAxisWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaSetNumberOfMeasurements(internal) => {
                let pyref: Py<PragmaSetNumberOfMeasurementsWrapper> =
                    Py::new(py, PragmaSetNumberOfMeasurementsWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaRepeatGate(internal) => {
                let pyref: Py<PragmaRepeatGateWrapper> =
                    Py::new(py, PragmaRepeatGateWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaOverrotation(internal) => {
                let pyref: Py<PragmaOverrotationWrapper> =
                    Py::new(py, PragmaOverrotationWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaBoostNoise(internal) => {
                let pyref: Py<PragmaBoostNoiseWrapper> =
                    Py::new(py, PragmaBoostNoiseWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaStopParallelBlock(internal) => {
                let pyref: Py<PragmaStopParallelBlockWrapper> =
                    Py::new(py, PragmaStopParallelBlockWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGlobalPhase(internal) => {
                let pyref: Py<PragmaGlobalPhaseWrapper> =
                    Py::new(py, PragmaGlobalPhaseWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaSleep(internal) => {
                let pyref: Py<PragmaSleepWrapper> =
                    Py::new(py, PragmaSleepWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaActiveReset(internal) => {
                let pyref: Py<PragmaActiveResetWrapper> =
                    Py::new(py, PragmaActiveResetWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaStartDecompositionBlock(internal) => {
                let pyref: Py<PragmaStartDecompositionBlockWrapper> =
                    Py::new(py, PragmaStartDecompositionBlockWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaStopDecompositionBlock(internal) => {
                let pyref: Py<PragmaStopDecompositionBlockWrapper> =
                    Py::new(py, PragmaStopDecompositionBlockWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaDamping(internal) => {
                let pyref: Py<PragmaDampingWrapper> =
                    Py::new(py, PragmaDampingWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaDepolarising(internal) => {
                let pyref: Py<PragmaDepolarisingWrapper> =
                    Py::new(py, PragmaDepolarisingWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaDephasing(internal) => {
                let pyref: Py<PragmaDephasingWrapper> =
                    Py::new(py, PragmaDephasingWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaRandomNoise(internal) => {
                let pyref: Py<PragmaRandomNoiseWrapper> =
                    Py::new(py, PragmaRandomNoiseWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaConditional(internal) => {
                let pyref: Py<PragmaConditionalWrapper> =
                    Py::new(py, PragmaConditionalWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::CNOT(internal) => {
                let pyref: Py<CNOTWrapper> = Py::new(py, CNOTWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::SWAP(internal) => {
                let pyref: Py<SWAPWrapper> = Py::new(py, SWAPWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::ISwap(internal) => {
                let pyref: Py<ISwapWrapper> = Py::new(py, ISwapWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::SqrtISwap(internal) => {
                let pyref: Py<SqrtISwapWrapper> =
                    Py::new(py, SqrtISwapWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::InvSqrtISwap(internal) => {
                let pyref: Py<InvSqrtISwapWrapper> =
                    Py::new(py, InvSqrtISwapWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::FSwap(internal) => {
                let pyref: Py<FSwapWrapper> = Py::new(py, FSwapWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::MolmerSorensenXX(internal) => {
                let pyref: Py<MolmerSorensenXXWrapper> =
                    Py::new(py, MolmerSorensenXXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::VariableMSXX(internal) => {
                let pyref: Py<VariableMSXXWrapper> =
                    Py::new(py, VariableMSXXWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::GivensRotation(internal) => {
                let pyref: Py<GivensRotationWrapper> =
                    Py::new(py, GivensRotationWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::GivensRotationLittleEndian(internal) => {
                let pyref: Py<GivensRotationLittleEndianWrapper> =
                    Py::new(py, GivensRotationLittleEndianWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::XY(internal) => {
                let pyref: Py<XYWrapper> = Py::new(py, XYWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::ControlledPhaseShift(internal) => {
                let pyref: Py<ControlledPhaseShiftWrapper> =
                    Py::new(py, ControlledPhaseShiftWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::ControlledPauliY(internal) => {
                let pyref: Py<ControlledPauliYWrapper> =
                    Py::new(py, ControlledPauliYWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::ControlledPauliZ(internal) => {
                let pyref: Py<ControlledPauliZWrapper> =
                    Py::new(py, ControlledPauliZWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::Qsim(internal) => {
                let pyref: Py<QsimWrapper> = Py::new(py, QsimWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::Fsim(internal) => {
                let pyref: Py<FsimWrapper> = Py::new(py, FsimWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::SpinInteraction(internal) => {
                let pyref: Py<SpinInteractionWrapper> =
                    Py::new(py, SpinInteractionWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::Bogoliubov(internal) => {
                let pyref: Py<BogoliubovWrapper> =
                    Py::new(py, BogoliubovWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PMInteraction(internal) => {
                let pyref: Py<PMInteractionWrapper> =
                    Py::new(py, PMInteractionWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::ComplexPMInteraction(internal) => {
                let pyref: Py<ComplexPMInteractionWrapper> =
                    Py::new(py, ComplexPMInteractionWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PhaseShiftedControlledZ(internal) => {
                let pyref: Py<PhaseShiftedControlledZWrapper> =
                    Py::new(py, PhaseShiftedControlledZWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::MeasureQubit(internal) => {
                let pyref: Py<MeasureQubitWrapper> =
                    Py::new(py, MeasureQubitWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGetStateVector(internal) => {
                let pyref: Py<PragmaGetStateVectorWrapper> =
                    Py::new(py, PragmaGetStateVectorWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGetDensityMatrix(internal) => {
                let pyref: Py<PragmaGetDensityMatrixWrapper> =
                    Py::new(py, PragmaGetDensityMatrixWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGetOccupationProbability(internal) => {
                let pyref: Py<PragmaGetOccupationProbabilityWrapper> =
                    Py::new(py, PragmaGetOccupationProbabilityWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGetPauliProduct(internal) => {
                let pyref: Py<PragmaGetPauliProductWrapper> =
                    Py::new(py, PragmaGetPauliProductWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaRepeatedMeasurement(internal) => {
                let pyref: Py<PragmaRepeatedMeasurementWrapper> =
                    Py::new(py, PragmaRepeatedMeasurementWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::DefinitionFloat(internal) => {
                let pyref: Py<DefinitionFloatWrapper> =
                    Py::new(py, DefinitionFloatWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::DefinitionComplex(internal) => {
                let pyref: Py<DefinitionComplexWrapper> =
                    Py::new(py, DefinitionComplexWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::DefinitionUsize(internal) => {
                let pyref: Py<DefinitionUsizeWrapper> =
                    Py::new(py, DefinitionUsizeWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::DefinitionBit(internal) => {
                let pyref: Py<DefinitionBitWrapper> =
                    Py::new(py, DefinitionBitWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::InputSymbolic(internal) => {
                let pyref: Py<InputSymbolicWrapper> =
                    Py::new(py, InputSymbolicWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaSetStateVector(internal) => {
                let pyref: Py<PragmaSetStateVectorWrapper> =
                    Py::new(py, PragmaSetStateVectorWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaSetDensityMatrix(internal) => {
                let pyref: Py<PragmaSetDensityMatrixWrapper> =
                    Py::new(py, PragmaSetDensityMatrixWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
            Operation::PragmaGeneralNoise(internal) => {
                let pyref: Py<PragmaGeneralNoiseWrapper> =
                    Py::new(py, PragmaGeneralNoiseWrapper { internal }).unwrap();
                let pyobject: PyObject = pyref.to_object(py);
                Ok(pyobject)
            }
        }
    })
}
#[doc = r" Tries to convert any python object to a [roqoqo::operations::Operation]"]
pub fn convert_pyany_to_operation(op: &PyAny) -> Result<Operation, QoqoError> {
    let hqslang_pyobject = op
        .call_method0("hqslang")
        .map_err(|_| QoqoError::ConversionError)?;
    let hqslang: String =
        String::extract(hqslang_pyobject).map_err(|_| QoqoError::ConversionError)?;
    match hqslang.as_str() {
        "SingleQubitGate" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let alpha_r_pyobject = op
                .call_method0("alpha_r")
                .map_err(|_| QoqoError::ConversionError)?;
            let alpha_r = convert_into_calculator_float(alpha_r_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let alpha_i_pyobject = op
                .call_method0("alpha_i")
                .map_err(|_| QoqoError::ConversionError)?;
            let alpha_i = convert_into_calculator_float(alpha_i_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let beta_r_pyobject = op
                .call_method0("beta_r")
                .map_err(|_| QoqoError::ConversionError)?;
            let beta_r = convert_into_calculator_float(beta_r_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let beta_i_pyobject = op
                .call_method0("beta_i")
                .map_err(|_| QoqoError::ConversionError)?;
            let beta_i = convert_into_calculator_float(beta_i_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let global_phase_pyobject = op
                .call_method0("global_phase")
                .map_err(|_| QoqoError::ConversionError)?;
            let global_phase = convert_into_calculator_float(global_phase_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SingleQubitGate::new(qubit, alpha_r, alpha_i, beta_r, beta_i, global_phase).into())
        }
        "RotateX" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(RotateX::new(qubit, theta).into())
        }
        "RotateY" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(RotateY::new(qubit, theta).into())
        }
        "RotateZ" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(RotateZ::new(qubit, theta).into())
        }
        "PauliX" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PauliX::new(qubit).into())
        }
        "PauliY" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PauliY::new(qubit).into())
        }
        "PauliZ" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PauliZ::new(qubit).into())
        }
        "SqrtPauliX" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SqrtPauliX::new(qubit).into())
        }
        "InvSqrtPauliX" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(InvSqrtPauliX::new(qubit).into())
        }
        "Hadamard" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(Hadamard::new(qubit).into())
        }
        "SGate" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SGate::new(qubit).into())
        }
        "TGate" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(TGate::new(qubit).into())
        }
        "RotateAroundSphericalAxis" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let spherical_theta_pyobject = op
                .call_method0("spherical_theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let spherical_theta = convert_into_calculator_float(spherical_theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let spherical_phi_pyobject = op
                .call_method0("spherical_phi")
                .map_err(|_| QoqoError::ConversionError)?;
            let spherical_phi = convert_into_calculator_float(spherical_phi_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(RotateAroundSphericalAxis::new(qubit, theta, spherical_theta, spherical_phi).into())
        }
        "PragmaSetNumberOfMeasurements" => {
            let number_measurements_pyobject = op
                .call_method0("number_measurements")
                .map_err(|_| QoqoError::ConversionError)?;
            let number_measurements: usize = number_measurements_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaSetNumberOfMeasurements::new(number_measurements, readout).into())
        }
        "PragmaRepeatGate" => {
            let repetition_coefficient_pyobject = op
                .call_method0("repetition_coefficient")
                .map_err(|_| QoqoError::ConversionError)?;
            let repetition_coefficient: usize = repetition_coefficient_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaRepeatGate::new(repetition_coefficient).into())
        }
        "PragmaOverrotation" => {
            let gate_hqslang_pyobject = op
                .call_method0("gate_hqslang")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_hqslang: String = gate_hqslang_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits_pyobject = op
                .call_method0("qubits")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits: Vec<usize> = qubits_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let amplitude_pyobject = op
                .call_method0("amplitude")
                .map_err(|_| QoqoError::ConversionError)?;
            let amplitude: f64 = amplitude_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let variance_pyobject = op
                .call_method0("variance")
                .map_err(|_| QoqoError::ConversionError)?;
            let variance: f64 = variance_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaOverrotation::new(gate_hqslang, qubits, amplitude, variance).into())
        }
        "PragmaBoostNoise" => {
            let noise_coefficient_pyobject = op
                .call_method0("noise_coefficient")
                .map_err(|_| QoqoError::ConversionError)?;
            let noise_coefficient = convert_into_calculator_float(noise_coefficient_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaBoostNoise::new(noise_coefficient).into())
        }
        "PragmaStopParallelBlock" => {
            let qubits_pyobject = op
                .call_method0("qubits")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits: Vec<usize> = qubits_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let execution_time_pyobject = op
                .call_method0("execution_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let execution_time = convert_into_calculator_float(execution_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaStopParallelBlock::new(qubits, execution_time).into())
        }
        "PragmaGlobalPhase" => {
            let phase_pyobject = op
                .call_method0("phase")
                .map_err(|_| QoqoError::ConversionError)?;
            let phase = convert_into_calculator_float(phase_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaGlobalPhase::new(phase).into())
        }
        "PragmaSleep" => {
            let qubits_pyobject = op
                .call_method0("qubits")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits: Vec<usize> = qubits_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let sleep_time_pyobject = op
                .call_method0("sleep_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let sleep_time = convert_into_calculator_float(sleep_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaSleep::new(qubits, sleep_time).into())
        }
        "PragmaActiveReset" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaActiveReset::new(qubit).into())
        }
        "PragmaStartDecompositionBlock" => {
            let qubits_pyobject = op
                .call_method0("qubits")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits: Vec<usize> = qubits_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let reordering_dictionary_pyobject = op
                .call_method0("reordering_dictionary")
                .map_err(|_| QoqoError::ConversionError)?;
            let reordering_dictionary: HashMap<usize, usize> = reordering_dictionary_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaStartDecompositionBlock::new(qubits, reordering_dictionary).into())
        }
        "PragmaStopDecompositionBlock" => {
            let qubits_pyobject = op
                .call_method0("qubits")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubits: Vec<usize> = qubits_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaStopDecompositionBlock::new(qubits).into())
        }
        "PragmaDamping" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time_pyobject = op
                .call_method0("gate_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time = convert_into_calculator_float(gate_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let rate_pyobject = op
                .call_method0("rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let rate = convert_into_calculator_float(rate_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaDamping::new(qubit, gate_time, rate).into())
        }
        "PragmaDepolarising" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time_pyobject = op
                .call_method0("gate_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time = convert_into_calculator_float(gate_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let rate_pyobject = op
                .call_method0("rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let rate = convert_into_calculator_float(rate_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaDepolarising::new(qubit, gate_time, rate).into())
        }
        "PragmaDephasing" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time_pyobject = op
                .call_method0("gate_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time = convert_into_calculator_float(gate_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let rate_pyobject = op
                .call_method0("rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let rate = convert_into_calculator_float(rate_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaDephasing::new(qubit, gate_time, rate).into())
        }
        "PragmaRandomNoise" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time_pyobject = op
                .call_method0("gate_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time = convert_into_calculator_float(gate_time_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let depolarising_rate_pyobject = op
                .call_method0("depolarising_rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let depolarising_rate = convert_into_calculator_float(depolarising_rate_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let dephasing_rate_pyobject = op
                .call_method0("dephasing_rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let dephasing_rate = convert_into_calculator_float(dephasing_rate_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaRandomNoise::new(qubit, gate_time, depolarising_rate, dephasing_rate).into())
        }
        "PragmaConditional" => {
            let condition_register_pyobject = op
                .call_method0("condition_register")
                .map_err(|_| QoqoError::ConversionError)?;
            let condition_register: String = condition_register_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let condition_index_pyobject = op
                .call_method0("condition_index")
                .map_err(|_| QoqoError::ConversionError)?;
            let condition_index: usize = condition_index_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit_pyobject = op
                .call_method0("circuit")
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit =
                convert_into_circuit(circuit_pyobject).map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaConditional::new(condition_register, condition_index, circuit).into())
        }
        "CNOT" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(CNOT::new(control, target).into())
        }
        "SWAP" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SWAP::new(control, target).into())
        }
        "ISwap" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(ISwap::new(control, target).into())
        }
        "SqrtISwap" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SqrtISwap::new(control, target).into())
        }
        "InvSqrtISwap" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(InvSqrtISwap::new(control, target).into())
        }
        "FSwap" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(FSwap::new(control, target).into())
        }
        "MolmerSorensenXX" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(MolmerSorensenXX::new(control, target).into())
        }
        "VariableMSXX" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(VariableMSXX::new(control, target, theta).into())
        }
        "GivensRotation" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let phi_pyobject = op
                .call_method0("phi")
                .map_err(|_| QoqoError::ConversionError)?;
            let phi = convert_into_calculator_float(phi_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(GivensRotation::new(control, target, theta, phi).into())
        }
        "GivensRotationLittleEndian" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let phi_pyobject = op
                .call_method0("phi")
                .map_err(|_| QoqoError::ConversionError)?;
            let phi = convert_into_calculator_float(phi_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(GivensRotationLittleEndian::new(control, target, theta, phi).into())
        }
        "XY" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(XY::new(control, target, theta).into())
        }
        "ControlledPhaseShift" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let theta_pyobject = op
                .call_method0("theta")
                .map_err(|_| QoqoError::ConversionError)?;
            let theta = convert_into_calculator_float(theta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(ControlledPhaseShift::new(control, target, theta).into())
        }
        "ControlledPauliY" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(ControlledPauliY::new(control, target).into())
        }
        "ControlledPauliZ" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(ControlledPauliZ::new(control, target).into())
        }
        "Qsim" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let x_pyobject = op
                .call_method0("x")
                .map_err(|_| QoqoError::ConversionError)?;
            let x = convert_into_calculator_float(x_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let y_pyobject = op
                .call_method0("y")
                .map_err(|_| QoqoError::ConversionError)?;
            let y = convert_into_calculator_float(y_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let z_pyobject = op
                .call_method0("z")
                .map_err(|_| QoqoError::ConversionError)?;
            let z = convert_into_calculator_float(z_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(Qsim::new(control, target, x, y, z).into())
        }
        "Fsim" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let t_pyobject = op
                .call_method0("t")
                .map_err(|_| QoqoError::ConversionError)?;
            let t = convert_into_calculator_float(t_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let u_pyobject = op
                .call_method0("u")
                .map_err(|_| QoqoError::ConversionError)?;
            let u = convert_into_calculator_float(u_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let delta_pyobject = op
                .call_method0("delta")
                .map_err(|_| QoqoError::ConversionError)?;
            let delta = convert_into_calculator_float(delta_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(Fsim::new(control, target, t, u, delta).into())
        }
        "SpinInteraction" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let x_pyobject = op
                .call_method0("x")
                .map_err(|_| QoqoError::ConversionError)?;
            let x = convert_into_calculator_float(x_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let y_pyobject = op
                .call_method0("y")
                .map_err(|_| QoqoError::ConversionError)?;
            let y = convert_into_calculator_float(y_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let z_pyobject = op
                .call_method0("z")
                .map_err(|_| QoqoError::ConversionError)?;
            let z = convert_into_calculator_float(z_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(SpinInteraction::new(control, target, x, y, z).into())
        }
        "Bogoliubov" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let delta_real_pyobject = op
                .call_method0("delta_real")
                .map_err(|_| QoqoError::ConversionError)?;
            let delta_real = convert_into_calculator_float(delta_real_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let delta_imag_pyobject = op
                .call_method0("delta_imag")
                .map_err(|_| QoqoError::ConversionError)?;
            let delta_imag = convert_into_calculator_float(delta_imag_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(Bogoliubov::new(control, target, delta_real, delta_imag).into())
        }
        "PMInteraction" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let t_pyobject = op
                .call_method0("t")
                .map_err(|_| QoqoError::ConversionError)?;
            let t = convert_into_calculator_float(t_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PMInteraction::new(control, target, t).into())
        }
        "ComplexPMInteraction" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let t_real_pyobject = op
                .call_method0("t_real")
                .map_err(|_| QoqoError::ConversionError)?;
            let t_real = convert_into_calculator_float(t_real_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            let t_imag_pyobject = op
                .call_method0("t_imag")
                .map_err(|_| QoqoError::ConversionError)?;
            let t_imag = convert_into_calculator_float(t_imag_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(ComplexPMInteraction::new(control, target, t_real, t_imag).into())
        }
        "PhaseShiftedControlledZ" => {
            let control_pyobject = op
                .call_method0("control")
                .map_err(|_| QoqoError::ConversionError)?;
            let control: usize = control_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let target_pyobject = op
                .call_method0("target")
                .map_err(|_| QoqoError::ConversionError)?;
            let target: usize = target_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let phi_pyobject = op
                .call_method0("phi")
                .map_err(|_| QoqoError::ConversionError)?;
            let phi = convert_into_calculator_float(phi_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PhaseShiftedControlledZ::new(control, target, phi).into())
        }
        "MeasureQubit" => {
            let qubit_pyobject = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qubit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let readout_index_pyobject = op
                .call_method0("readout_index")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout_index: usize = readout_index_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(MeasureQubit::new(qubit, readout, readout_index).into())
        }
        "PragmaGetStateVector" => {
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit_pyobject = op
                .call_method0("circuit")
                .map_err(|_| QoqoError::ConversionError)?;
            let tmp: Option<&PyAny> = circuit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit = match tmp {
                Some(cw) => Some(convert_into_circuit(cw).map_err(|_| QoqoError::ConversionError)?),
                _ => None,
            };
            Ok(PragmaGetStateVector::new(readout, circuit).into())
        }
        "PragmaGetDensityMatrix" => {
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit_pyobject = op
                .call_method0("circuit")
                .map_err(|_| QoqoError::ConversionError)?;
            let tmp: Option<&PyAny> = circuit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit = match tmp {
                Some(cw) => Some(convert_into_circuit(cw).map_err(|_| QoqoError::ConversionError)?),
                _ => None,
            };
            Ok(PragmaGetDensityMatrix::new(readout, circuit).into())
        }
        "PragmaGetOccupationProbability" => {
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit_pyobject = op
                .call_method0("circuit")
                .map_err(|_| QoqoError::ConversionError)?;
            let tmp: Option<&PyAny> = circuit_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit = match tmp {
                Some(cw) => Some(convert_into_circuit(cw).map_err(|_| QoqoError::ConversionError)?),
                _ => None,
            };
            Ok(PragmaGetOccupationProbability::new(readout, circuit).into())
        }
        "PragmaGetPauliProduct" => {
            let qubit_paulis_pyobject = op
                .call_method0("qubit_paulis")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit_paulis: std::collections::HashMap<usize, usize> = qubit_paulis_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit_pyobject = op
                .call_method0("circuit")
                .map_err(|_| QoqoError::ConversionError)?;
            let circuit =
                convert_into_circuit(circuit_pyobject).map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaGetPauliProduct::new(qubit_paulis, readout, circuit).into())
        }
        "PragmaRepeatedMeasurement" => {
            let readout_pyobject = op
                .call_method0("readout")
                .map_err(|_| QoqoError::ConversionError)?;
            let readout: String = readout_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit_mapping_pyobject = op
                .call_method0("qubit_mapping")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit_mapping: Option<std::collections::HashMap<usize, usize>> =
                qubit_mapping_pyobject
                    .extract()
                    .map_err(|_| QoqoError::ConversionError)?;
            let number_measurements_pyobject = op
                .call_method0("number_measurements")
                .map_err(|_| QoqoError::ConversionError)?;
            let number_measurements: usize = number_measurements_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(PragmaRepeatedMeasurement::new(readout, qubit_mapping, number_measurements).into())
        }
        "DefinitionFloat" => {
            let name_pyobject = op
                .call_method0("name")
                .map_err(|_| QoqoError::ConversionError)?;
            let name: String = name_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let length_pyobject = op
                .call_method0("length")
                .map_err(|_| QoqoError::ConversionError)?;
            let length: usize = length_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output_pyobject = op
                .call_method0("is_output")
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output: bool = is_output_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(DefinitionFloat::new(name, length, is_output).into())
        }
        "DefinitionComplex" => {
            let name_pyobject = op
                .call_method0("name")
                .map_err(|_| QoqoError::ConversionError)?;
            let name: String = name_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let length_pyobject = op
                .call_method0("length")
                .map_err(|_| QoqoError::ConversionError)?;
            let length: usize = length_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output_pyobject = op
                .call_method0("is_output")
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output: bool = is_output_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(DefinitionComplex::new(name, length, is_output).into())
        }
        "DefinitionUsize" => {
            let name_pyobject = op
                .call_method0("name")
                .map_err(|_| QoqoError::ConversionError)?;
            let name: String = name_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let length_pyobject = op
                .call_method0("length")
                .map_err(|_| QoqoError::ConversionError)?;
            let length: usize = length_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output_pyobject = op
                .call_method0("is_output")
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output: bool = is_output_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(DefinitionUsize::new(name, length, is_output).into())
        }
        "DefinitionBit" => {
            let name_pyobject = op
                .call_method0("name")
                .map_err(|_| QoqoError::ConversionError)?;
            let name: String = name_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let length_pyobject = op
                .call_method0("length")
                .map_err(|_| QoqoError::ConversionError)?;
            let length: usize = length_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output_pyobject = op
                .call_method0("is_output")
                .map_err(|_| QoqoError::ConversionError)?;
            let is_output: bool = is_output_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(DefinitionBit::new(name, length, is_output).into())
        }
        "InputSymbolic" => {
            let name_pyobject = op
                .call_method0("name")
                .map_err(|_| QoqoError::ConversionError)?;
            let name: String = name_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            let input_pyobject = op
                .call_method0("input")
                .map_err(|_| QoqoError::ConversionError)?;
            let input: f64 = input_pyobject
                .extract()
                .map_err(|_| QoqoError::ConversionError)?;
            Ok(InputSymbolic::new(name, input).into())
        }
        "PragmaSetStateVector" => {
            let array = op.call_method0("statevector").expect("error extracting");
            let statevec_casted: Vec<Complex64> = array.extract().unwrap();
            let statevec_array: Array1<Complex64> = Array1::from(statevec_casted);
            Ok(PragmaSetStateVector::new(statevec_array).into())
        }
        "PragmaSetDensityMatrix" => {
            let array = op
                .call_method0("density_matrix")
                .map_err(|_| QoqoError::ConversionError)?;
            let densmat_casted: Vec<Complex64> = Vec::extract(array).unwrap();
            let length: usize = densmat_casted.len();
            let dim: usize = (length as f64).sqrt() as usize;
            let densmat_array = Array::from_shape_vec((dim, dim), densmat_casted).unwrap();
            Ok(PragmaSetDensityMatrix::new(densmat_array).into())
        }
        "PragmaGeneralNoise" => {
            let qbt = op
                .call_method0("qubit")
                .map_err(|_| QoqoError::ConversionError)?;
            let qubit: usize = qbt.extract().map_err(|_| QoqoError::ConversionError)?;
            let gatetm = op
                .call_method0("gate_time")
                .map_err(|_| QoqoError::ConversionError)?;
            let gate_time: CalculatorFloat =
                convert_into_calculator_float(gatetm).map_err(|_| QoqoError::ConversionError)?;
            let rt = op
                .call_method0("rate")
                .map_err(|_| QoqoError::ConversionError)?;
            let rate: CalculatorFloat =
                convert_into_calculator_float(rt).map_err(|_| QoqoError::ConversionError)?;
            let array = op
                .call_method0("operators")
                .map_err(|_| QoqoError::ConversionError)?;
            let densmat_casted: Vec<Complex64> = Vec::extract(array).unwrap();
            let length: usize = densmat_casted.len();
            let dim: usize = (length as f64).sqrt() as usize;
            let operators = Array::from_shape_vec((dim, dim), densmat_casted).unwrap();
            Ok(PragmaGeneralNoise::new(qubit, gate_time, rate, operators).into())
        }
        _ => Err(QoqoError::ConversionError),
    }
}
