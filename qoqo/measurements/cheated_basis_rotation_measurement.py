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
"""Cheated basis rotation measurement"""
from qoqo.measurements._measurement_base_class import MeasurementBaseClass
from qoqo.measurements.measurement_auxiliary_data_input import (
    CheatedBRMeasurementInput
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


class CheatedBasisRotationMeasurement(MeasurementBaseClass):
    r"""Cheated basis rotation measurement

    Cheated measurement of expectation values that uses a basis rotation measurement preparation.

    """

    _qonfig_defaults_dict = {

        'backend': {'doc': 'qoqo backend',
                    'default': empty},
        'circuit_list': {'doc': 'The circuits that are run and measured on the backend',
                         'default': None},
        'constant_circuit': {
            'doc': 'Circuit that is always applied before each circuit in the circuit_list',
            'default': None},
        'measurement_input': {'doc': 'Additional input (matrices) required for measurement',
                              'default': Qonfig[CheatedBRMeasurementInput]},
        'device': {'doc': 'Quantum computer device used in the backend',
                   'default': None},
    }

    def __init__(self,
                 backend: Optional[BackendBaseClass] = None,
                 measurement_input: Optional[CheatedBRMeasurementInput] = None,
                 circuit_list: Optional[List[Circuit]] = None,
                 constant_circuit: Optional[Circuit] = None,
                 device: Optional[Union[
                     Qonfig,
                     DeviceBaseClass]] = None,
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
            self.measurement_input = CheatedBRMeasurementInput()
        else:
            self.measurement_input = measurement_input
        self.kwargs = kwargs

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['CheatedBasisRotationMeasurement']
                    ) -> 'CheatedBasisRotationMeasurement':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            CheatedBasisRotationMeasurement
        """
        if isinstance(config['backend'], Qonfig):
            init_backend = config['backend'].to_instance(no_copy=True)
        else:
            init_backend = cast(Optional[BackendBaseClass], config['backend'])
        if isinstance(config['device'], Qonfig):
            init_device = config['device'].to_instance(no_copy=True)
        else:
            init_device = cast(Optional[DeviceBaseClass], config['device'])
        clist: List[Circuit] = list()
        for c in config['circuit_list']:
            clist.append(c.to_instance())
        return cls(backend=init_backend,
                   measurement_input=config['measurement_input'].to_instance(no_copy=True),
                   circuit_list=clist,
                   constant_circuit=config['constant_circuit'].to_instance(no_copy=True),
                   device=init_device,
                   )

    def to_qonfig(self) -> 'Qonfig[CheatedBasisRotationMeasurement]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[CheatedBasisRotationMeasurement]
        """
        config = Qonfig(self.__class__)
        if self.backend is None:
            config['backend'] = None
        else:
            config['backend'] = self.backend.to_qonfig()
        config._values['measurement_input'] = self.measurement_input.to_qonfig()
        config._values['circuit_list'] = [c.to_qonfig() for c in self.circuit_list]
        config._values['constant_circuit'] = self._constant_circuit.to_qonfig()
        if self.device is None:
            config['device'] = None
        else:
            config['device'] = self.device.to_qonfig()

        return config

    def __call__(self, **kwargs) -> pd.Series:
        """Execute measurement

        Args:
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

        pp_to_exp_val_matrix = self.measurement_input._pp_to_exp_val_matrix

        pauli_products = np.zeros(self.measurement_input._number_pauli_products)
        for register_name, register in output_register_dict.items():
            if '_pauli_product_' in register_name:
                x = register_name.split('_pauli_product_')
                index = int(x[-1])
                pauli_products[index] = register.register[0][0]

        expectation_values = pp_to_exp_val_matrix @ pauli_products

        names = self.measurement_input._measured_exp_vals
        tmp_dict = {('exp_val_' + name): exp_val for name,
                    exp_val in zip(names, expectation_values)}

        if 'global_phase' in output_register_dict.keys():
            tmp_dict['global_phase'] = output_register_dict['global_phase'].register[0][0]

        if not tmp_dict:
            return_series = pd.Series({}, dtype=complex)
        else:
            return_series = pd.Series(tmp_dict)

        return return_series
