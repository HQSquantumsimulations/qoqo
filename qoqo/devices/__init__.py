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
"""The Devices class provides hardware devices in qoqo.

In qoqo, device classes define all relevant properties of a quantum hardware device
which is either used as a backend itself or that is simulated.

The information is contained in a networkx directed MultiGraph and includes the topology
of the device (which qubits can be coupled by which gates) as well as physical
characteristics (gate times, decoherence rates, overrotations).
For specific devices, see the qoqo interfaces.

.. autosummary::
    :toctree: generated/

    DeviceBaseClass

"""

from qoqo.devices._devices_abstract_base_class import DeviceBaseClass
