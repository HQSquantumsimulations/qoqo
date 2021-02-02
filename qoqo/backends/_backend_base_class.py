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
"""Base Class for all Backends used in qoqo interfaces and backends"""

import abc
from qoqo import Circuit
from typing import Optional, Union, Dict, cast
from qoqo import operations as ops
from qoqo.devices import DeviceBaseClass
from qoqo.registers import RegisterOutput
from hqsbase.qonfig import Qonfig


class BackendBaseClass(abc.ABC):
    r"""Abstract Base Class for all backends that execute a circuit in qoqo.

    All of the backends found in qoqo_interfaces are built on this base class, and
    therefore all have the following properties/functions available:
    - circuit: retrieves the circuit being processed on the backend
    - circuit setter: sets the circuit processed on the backend
    - run: run the backend, once all of the initialisation is complete

    """

    def __init__(self,
                 circuit: Optional[Circuit],
                 substitution_dict: Optional[Dict[str, float]] = None,
                 device: Optional[DeviceBaseClass] = None,
                 number_qubits: int = 0,
                 **kwargs) -> None:
        """Initialize backend

        Args:
            circuit: The circuit executed by the backend
            substitution_dict: Substitution dictionary used to replace symbolic parameters
            device: The device specification
            number_qubits: The number of qubits in the backend
            kwargs: Additional keyword arguments
        """
        if getattr(self, 'name', None) is None:
            self.name = 'BackendBaseClass'
        self.substitution_dict = substitution_dict
        self.device = device
        self.number_qubits = number_qubits
        if circuit is None:
            self._circuit = Circuit()
        else:
            for _, op in enumerate(circuit):
                op = cast(ops.Operation, op)
                if 'Pragma' in op._operation_tags:
                    op = cast(ops.Pragma, op)
                    if op.backend_instruction(backend=self.name) is not None:
                        instruction = op.backend_instruction(backend=self.name)
                        if instruction is not None:
                            for key, val in instruction.items():
                                setattr(self, key, val)
            self._circuit = circuit
        self._kwargs = kwargs

    @property
    def circuit(self) -> Circuit:
        """Return the circuit processed by the backend

        Setter:
            circuit (Optional[Circuit]): New circuit. The backend instructions in the circuit
                                         will be applied to the backend when using this setter

        Returns:
            Optional[Circuit]
        """
        return self._circuit

    @circuit.setter
    def circuit(self, circuit: Circuit) -> None:
        if circuit is None:
            self._circuit = Circuit()
        else:
            for _, op in enumerate(circuit):
                op = cast(ops.Operation, op)
                if 'Pragma' in op._operation_tags:
                    op = cast(ops.Pragma, op)
                    if op.backend_instruction(backend=self.name) is not None:
                        instruction = op.backend_instruction(backend=self.name)
                        if instruction is not None:
                            for key, val in instruction.items():
                                setattr(self, key, val)
            self._circuit = circuit

    @abc.abstractmethod
    def run(self,
            **kwargs
            ) -> Union[None, Dict[str, RegisterOutput]]:
        """Run the Backend

        Args:
            kwargs: Additional keyword arguments

        Returns:
            Union[None, Dict[str, RegisterOutput]]
        """
        return None

    @abc.abstractmethod
    def to_qonfig(self,
                  ) -> Qonfig:
        """Serialize the backend with Qonfig"""
        pass
