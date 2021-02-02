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
"""Backends process circuits and (optionally) return results in qoqo.

Backends process the circuit in order to execute it. This can be either sending the circuit
to an appropriate external simulator or running it on hardware.
The backends class for backend xyz relies heavily on the xyz interface of the operations
and is usually only a thin wrapper that takes care of initializing the backend
and applying global backend instructions.

.. autosummary::
    :toctree: generated/

    BackendBaseClass

"""

from qoqo.backends._backend_base_class import (
    BackendBaseClass,
)
