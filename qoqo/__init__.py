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
"""Qoqo

| Quantum Operation Quantum Operation
| Yes we use `reduplication <https://en.wikipedia.org/wiki/Reduplication>`_.

qoqo is the HQS python package to represent quantum circuits.

.. autosummary::
    :toctree: generated/

    Circuit
    operations
    registers
    measurements
    DoUnitary
    devices
    BackendBaseClass

"""

from qoqo.__version__ import __version__
from qoqo import operations
from qoqo import registers
from qoqo.circuit import Circuit
from qoqo import devices
from qoqo.backends import BackendBaseClass
from qoqo import measurements
from qoqo.do_unitary import DoUnitary
