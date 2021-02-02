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
"""Base Class for Measurements in qoqo"""

import abc
from typing import (
    List,
    Any,
    Dict,
    Optional
)
import pandas as pd
from qoqo import Circuit
from qoqo.backends import BackendBaseClass
from qoqo.devices import DeviceBaseClass
from hqsbase.qonfig import Qonfig


class MeasurementBaseClass(abc.ABC):
    r"""Abstract Base Class for all classes that execute a measurement in qoqo"""

    def __init__(self,
                 backend: BackendBaseClass = None,
                 circuit_list: Optional[List[Circuit]] = None,
                 constant_circuit: Optional[Circuit] = None,
                 resume_list: Optional[List[Dict[str, Any]]] = None,
                 device: Optional[DeviceBaseClass] = None,
                 **kwargs) -> None:
        """Initialize MeasurementBaseClass

        Args:
            backend: A Qonfig configuration of a qoqo backend class that either
                     simulates the quantum circuits or runs them on a real QPU
            circuit_list: List of circuits that are executed on the quantum computer to obtain the
                          projective measurements in the set of bases required to (re)construct the
                          measurement results of the measured operators
            constant_circuit: Circuit that is always applied before each circuit in the circuit_list
            resume_list: List of resume information for async backends
            device: Physical device model on which the measurement is run
            kwargs: Additional keyword arguments
        """
        if circuit_list is None:
            self._circuit_list: List[Circuit] = list()
        else:
            self._circuit_list = circuit_list
        if constant_circuit is None:
            self._constant_circuit = Circuit()
        else:
            self._constant_circuit = constant_circuit
        self._device = device
        self._resume_list = resume_list
        self._resume_info: List[Dict[str, Any]] = list()
        self._backend = backend

    @property
    def circuit_list(self) -> List[Circuit]:
        r"""Return list of circuits that are executed on the quantum computer

        Circuits are used to obtain the projective measurements in the set of bases required to
        (re)construct the measurement results of the measured operators

        Setter:
            circuit_list (List[Circuit]): New list of measured circuits

        Returns:
            List[Circuit]
        """
        return self._circuit_list

    @circuit_list.setter
    def circuit_list(self, circuit_list: List[Circuit]) -> None:
        self._circuit_list = circuit_list

    @property
    def backend(self) -> Optional[BackendBaseClass]:
        r"""Return the qoqo backend class

        Backend used to simulate measurement or run on real QPU

        Setter:
            backend (BackendBaseClass): New backend

        Returns:
            BackendBaseClass
        """
        return self._backend

    @backend.setter
    def backend(self, backend: Optional[BackendBaseClass]) -> None:
        self._backend = backend

    @property
    def device(self) -> Optional[DeviceBaseClass]:
        """Optional qoqo device for the backend

        Setter:
            device (DeviceBaseClass): New device

        Returns:
            Optional[DeviceBaseClass]
        """
        return self._device

    @device.setter
    def device(self, device: DeviceBaseClass) -> None:
        self._device = device

    @abc.abstractmethod
    def __call__(self, **kwargs) -> pd.Series:
        """Execute the measurement and return the result as a DataFrame

        Args:
            kwargs: Additional keyword arguments
        """
        pass

    @abc.abstractmethod
    def to_qonfig(self) -> Qonfig:
        """Create a Qonfig from Instance"""
        pass
