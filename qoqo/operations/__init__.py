# Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Operations are the atomic instructions in any quantum program that can be represented by qoqo.

Operations can be of various kinds: GateOperations, Definitions, Pragmas or Measurements. All stem
from the Operation Base Class.

.. autosummary::
    :toctree: generated/

    OperationNotInBackendError
    Operation
    GateOperation
    Pragma
    single_qubit_gate_operations
    two_qubit_gate_operations
    measurement_operations
    pragma_operations
    Definition

"""

from qoqo.operations._operations_base_classes import (
    OperationNotInBackendError,
    GateOperation,
    Operation,
    Pragma,
)

from qoqo.operations.two_qubit_gate_operations import (
    Bogoliubov,
    CNOT,
    ControlledPauliY,
    ControlledPauliZ,
    ControlledPhaseShift,
    FSwap,
    Fsim,
    Qsim,
    SpinInteraction,
    GivensRotation,
    GivensRotationLittleEndian,
    ISwap,
    InvSqrtISwap,
    MolmerSorensenXX,
    VariableMSXX,
    PMInteraction,
    ComplexPMInteraction,
    SWAP,
    SqrtISwap,
    XY,
    TwoQubitGateOperation,
)

from qoqo.operations.single_qubit_gate_operations import (
    SingleQubitGateOperation,
    SingleQubitGate,
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    TGate,
    SGate,
    SqrtPauliX,
    InvSqrtPauliX,
    RotateX,
    RotateY,
    RotateZ,
    RotateAroundSphericalAxis,
    W,
)

from qoqo.operations.measurement_operations import (
    MeasureQubit,
    PragmaRepeatedMeasurement,
    PragmaGetStateVector,
    PragmaGetDensityMatrix,
    PragmaGetOccupationProbability,
    PragmaPauliProdMeasurement,
    PragmaGetRotatedOccupationProbability,
    PragmaGetPauliProduct
)

from qoqo.operations.pragma_operations import (
    PragmaSetNumberOfMeasurements,
    PragmaSetStateVector,
    PragmaSetDensityMatrix,
    PragmaNoise,
    PragmaDamping,
    PragmaDephasing,
    PragmaDepolarise,
    PragmaRandomNoise,
    PragmaGeneralNoise,
    PragmaRepeatGate,
    PragmaBoostNoise,
    PragmaOverrotation,
    PragmaStop,
    PragmaSleep,
    PragmaParameterSubstitution,
    PragmaGlobalPhase,
    PragmaActiveReset,
    PragmaStartDecompositionBlock,
    PragmaStopDecompositionBlock
)

from qoqo.operations.define_operations import Definition
