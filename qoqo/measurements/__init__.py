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
"""Provides measurement functionality

Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.

Measurement classes take the result of the circuit running (or being simulated)
on a backend and post-process a measurement record of sigma-z measurements
or a statevector/density matrix to expectation values of observables.
The measurement classes require additional information in the form of measurement input
to reconstruct observables.

.. autosummary::
    :toctree: generated/

    MeasurementBaseClass
    BasisRotationMeasurement
    CheatedBasisRotationMeasurement
    PurePragmaMeasurement
    BRMeasurementInput
    CheatedBRMeasurementInput
    PurePragmaMeasurementInput

"""

from qoqo.measurements._measurement_base_class import MeasurementBaseClass
from qoqo.measurements.measurement_auxiliary_data_input import (
    BRMeasurementInput,
    CheatedBRMeasurementInput,
    PurePragmaMeasurementInput
)
from qoqo.measurements.basis_rotation_measurement import (
    BasisRotationMeasurement,
)
from qoqo.measurements.cheated_measurement import (
    PurePragmaMeasurement,
)
from qoqo.measurements.cheated_basis_rotation_measurement import (
    CheatedBasisRotationMeasurement,

)
