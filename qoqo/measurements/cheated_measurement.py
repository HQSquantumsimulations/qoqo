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
"""Cheated Measurement"""
from qoqo.measurements._measurement_base_class import MeasurementBaseClass
from qoqo.measurements.measurement_auxiliary_data_input import (
    PurePragmaMeasurementInput
)
from typing import (
    List,
    Optional,
    Union,
    Dict,
    cast
)
from qoqo import Circuit
from qoqo.backends import BackendBaseClass
from qoqo.registers import RegisterOutput
from qoqo.devices import DeviceBaseClass
import numpy as np
import pandas as pd
from hqsbase.qonfig import Qonfig, empty


class PurePragmaMeasurement(MeasurementBaseClass):
    r"""Cheated Measurement

    Simple measurement using direct access to the statevector or density matrix for
    numerical simulation backends.
    Uses simple matrix multiplication for measurement.

    """

    _qonfig_defaults_dict = {
        'measurement_input': {'doc': 'Additional input (matrices) required for measurement',
                              'default': Qonfig[PurePragmaMeasurementInput]},
        'backend': {'doc': 'qoqo backend',
                    'default': empty},
        'circuit_list': {'doc': 'The circuits that are run and measured on the backend',
                         'default': None},
        'constant_circuit': {
            'doc': 'Circuit that is always applied before each circuit in the circuit_list',
            'default': None},
        'device': {'doc': 'Quantum computer device used in the backend',
                   'default': None},
    }

    def __init__(self,
                 backend: Qonfig[BackendBaseClass] = None,
                 measurement_input: Optional[PurePragmaMeasurementInput] = None,
                 circuit_list: Optional[List[Circuit]] = None,
                 constant_circuit: Optional[Circuit] = None,
                 device: Optional[DeviceBaseClass] = None,
                 **kwargs) -> None:
        """Initialize measurement

        Args:
            backend: qoqo backend
            measurement_input: Additional input (matrices) required for measurement
            circuit_list: The circuits that are run and measured on the backend
            constant_circuit: Circuit that is always applied before each circuit in the circuit_list
            device: Quantum computer device used in the backend
            kwargs: Additional keyword arguments

        """
        super().__init__(
            circuit_list=circuit_list,
            constant_circuit=constant_circuit,
            backend=backend,
            device=device,
            **kwargs
        )
        if measurement_input is None:
            self.measurement_input = PurePragmaMeasurementInput()
        else:
            self.measurement_input = measurement_input
        self.kwargs = kwargs

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PurePragmaMeasurement']
                    ) -> 'PurePragmaMeasurement':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PurePragmaMeasurement
        """
        if isinstance(config['backend'], Qonfig):
            init_backend = config['backend'].to_instance()
        else:
            init_backend = cast(Optional[BackendBaseClass], config['backend'])
        if isinstance(config['device'], Qonfig):
            init_device = config['device'].to_instance()
        else:
            init_device = cast(Optional[DeviceBaseClass], config['device'])
        clist: List[Circuit] = list()
        for c in config['circuit_list']:
            clist.append(c.to_instance())
        return cls(
            backend=init_backend,
            measurement_input=config['measurement_input'].to_instance(no_copy=True),
            circuit_list=clist,
            device=init_device,
        )

    def to_qonfig(self) -> 'Qonfig[PurePragmaMeasurement]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PurePragmaMeasurement]
        """
        if self.device is None:
            qonfig_device = None
        else:
            qonfig_device = self.device.to_qonfig()
        config = Qonfig(self.__class__)
        if self.backend is None:
            config['backend'] = None
        else:
            config['backend'] = self.backend.to_qonfig()
        config['measurement_input'] = self.measurement_input.to_qonfig()
        config['circuit_list'] = [c.to_qonfig() for c in self.circuit_list]
        config['device'] = qonfig_device

        return config

    def __call__(self, include_metadata: bool = False, **kwargs) -> pd.Series:
        """Execute measurement

        Args:
            include_metadata: Include metadata in output pandas Dataseries
            kwargs: Additional keyword arguments

        Returns:
            pd.Series

        """
        if self.backend is None:
            return pd.Series({}, dtype=complex)
        # Dict for all read-out registers
        output_register_dict: Dict[str, RegisterOutput] = dict()
        # Dict for pauli products calculated from each read out register
        for circuit in self._circuit_list:
            self.backend.circuit = self._constant_circuit + circuit
            tmp_output_register_dict = self.backend.run()
            if tmp_output_register_dict is not None:
                output_register_dict.update(tmp_output_register_dict)

        expectation_values: Dict[str, Union[float, str]] = dict()
        for register_name, op_matrices in self.measurement_input.operator_matrices.items():
            register = output_register_dict[register_name]
            result = register.register[0]
            for name in op_matrices.keys():
                matrix = self.measurement_input.operator_matrices[register_name][name]
                if self.measurement_input.use_density_matrix:
                    exp_val = float(np.trace(matrix @ result))
                else:
                    exp_val = (result.conj().T @ matrix @ result)
                if np.isclose(np.imag(exp_val), 0):
                    expectation_values['exp_val_' + name] = np.real(exp_val)
                else:
                    expectation_values['exp_val_' + name] = exp_val

        if 'global_phase' in output_register_dict.keys():
            expectation_values['global_phase'] = output_register_dict['global_phase'].register[0][0]

        if not expectation_values:
            return_series = pd.Series({}, dtype=complex)
        else:
            return_series = pd.Series(expectation_values)

        return return_series
