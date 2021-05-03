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
"""Measurement based on basis rotation"""
from typing import (
    List,
    Optional,
    Dict,
    Any,
    cast
)
from qoqo import Circuit
from qoqo.backends import BackendBaseClass
from qoqo.registers import RegisterOutput
from qoqo.devices import DeviceBaseClass
from qoqo.measurements._measurement_base_class import MeasurementBaseClass
from qoqo.measurements.measurement_auxiliary_data_input import (
    BRMeasurementInput
)
import numpy as np
import pandas as pd
import networkx as nx
from hqsbase.qonfig import Qonfig, empty


class BasisRotationMeasurement(MeasurementBaseClass):
    r"""Measurement by basis rotation

    Simple measurement of expectation values that uses a basis rotation measurement preparation.
    The expectation values we want to measure are defined in
    qoqo.measurements.measurement_auxiliary_data_input.py, in BRMeasurementInput. These are then
    passed to BasisRotationMeasurement as the "measurement_input". These expectation values are
    measured on a certain backend and device, with additional parameters that can be defined.

    Note:
    (1) the BasisRotationMeasurement class does not perform the basis rotation itself, just the
        associated measurement. The user must perform relevant gate operations for the qubits to be
        in the Z-basis before the BasisRotationMeasurement is applied.
    (2) When setting the backend for the BasisRotationMeasurement, the user does not need to give
        the circuit(s) in circuit_list to the backend. These will be passed on automatically to
        the backend by the DoUnitary.

    """

    _qonfig_defaults_dict = {
        'measurement_input': {'doc': 'Additional input (matrices) required for measurement',
                              'default': Qonfig[BRMeasurementInput]},
        'backend': {'doc': 'qoqo backend',
                    'default': empty},
        'circuit_list': {'doc': 'The circuits that are run and measured on the backend',
                         'default': None},
        'constant_circuit': {
            'doc': 'Circuit that is always applied before each circuit in the circuit_list',
            'default': None},
        'verbose': {'doc': 'Print progress statements during measurement',
                    'default': True},
        'device': {'doc': 'Quantum computer device used in the backend',
                   'default': None},
        'resume_list': {'doc': 'List of resume information for async backends',
                        'default': None},
    }

    def __init__(self,
                 backend: BackendBaseClass = None,
                 measurement_input: Optional[BRMeasurementInput] = None,
                 circuit_list: Optional[List[Circuit]] = None,
                 constant_circuit: Optional[Circuit] = None,
                 verbose: bool = True,
                 device: Optional[DeviceBaseClass] = None,
                 resume_list: Optional[List[Dict[str, Any]]] = None,
                 **kwargs) -> None:
        """Initialize measurement

        Args:
            backend: qoqo backend (see qoqo_interfaces: each interface has an associated backend)
            measurement_input: Additional input (matrices) required for measurement
            circuit_list: The circuits that are run and measured on the backend
            constant_circuit: Circuit that is always applied before each circuit in the circuit_list
            verbose: Print progress statements during measurement
            device: Quantum computer device used in the backend
            resume_list: List of resume information for async backends
            kwargs: Additional keyword arguments

        """
        super().__init__(
            circuit_list=circuit_list,
            constant_circuit=constant_circuit,
            backend=backend,
            resume_list=resume_list,
            device=device,
            **kwargs
        )
        if measurement_input is None:
            self.measurement_input = BRMeasurementInput()
        else:
            self.measurement_input = measurement_input
        self._verbose = verbose

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['BasisRotationMeasurement']
                    ) -> 'BasisRotationMeasurement':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            BasisRotationMeasurement
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
            constant_circuit=config['constant_circuit'].to_instance(no_copy=True),
            verbose=config['verbose'],
            device=init_device,
            resume_list=config['resume_list'],
        )

    def to_qonfig(self) -> 'Qonfig[BasisRotationMeasurement]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[BasisRotationMeasurement]
        """
        if self.device is None:
            qonfig_device = None
        else:
            qonfig_device = self.device.to_qonfig()
        # Serialize pauli product qubit masks for qonfig
        config = Qonfig(self.__class__)
        if self.backend is None:
            config['backend'] = None
        else:
            self.backend.circuit = Circuit()
            config['backend'] = self.backend.to_qonfig()
        config._values['measurement_input'] = self.measurement_input.to_qonfig()
        config._values['circuit_list'] = [c.to_qonfig() for c in self.circuit_list]
        config._values['constant_circuit'] = self._constant_circuit.to_qonfig()
        config['verbose'] = self.verbose
        config['device'] = qonfig_device
        config['resume_list'] = self._resume_list

        return config

    @property
    def verbose(self) -> bool:
        r"""Print measurement progress during measurements

        Returns:
            bool
        """
        return self._verbose

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
        pauli_product_dict: Dict[str, np.ndarray] = dict()
        return_None = False
        for co, circuit in enumerate(self._circuit_list):
            self.backend.circuit = self._constant_circuit + circuit
            if self._resume_list is None:
                # Running backend normally when there is no
                tmp_output_register_dict = self.backend.run()
            else:
                tmp_output_register_dict = self.backend.run(**self._resume_list[co])
            self._resume_info.append(getattr(self.backend, 'resume_info', None))
            if tmp_output_register_dict is not None:
                output_register_dict.update(tmp_output_register_dict)
            else:
                return_None = True
        if return_None:
            return None

        if self._device is not None:
            measurement_errors = nx.get_edge_attributes(
                self._device.device_graph, 'measurement_error')
            measurement_fidelities = np.zeros(len(self._device.device_graph.nodes))
            for qubit in range(self.measurement_input._number_qubits):  # (data=True):
                edge = (qubit, qubit, 'MeasureQubit')
                measurement_fidelities[edge[0]] = (2
                                                   - measurement_errors[edge]['prob_0_as_1']
                                                   - measurement_errors[edge]['prob_1_as_0']
                                                   ) / 2
        else:
            measurement_fidelities = np.ones(self.backend.number_qubits)
        if self.measurement_input._use_flipped_measurement:
            measurement_correction_factors: Dict[str, np.ndarray] = dict()
            for (name,
                 pauli_product_mask) in self.measurement_input._pauli_product_qubit_masks.items():
                measurement_correction_factor = np.ones(
                    self.measurement_input._number_pauli_products, dtype=float)
                for index, val in pauli_product_mask.items():
                    if len(val) == 0:
                        measurement_correction_factor[index] = 1
                    else:
                        measurement_correction_factor[index] = np.prod(
                            measurement_fidelities[val])
                measurement_correction_factors[name] = measurement_correction_factor

        for register_name, mask in self.measurement_input._pauli_product_qubit_masks.items():
            register = output_register_dict[register_name]
            single_shot_pauli_products = np.zeros(
                (register.size, self.measurement_input._number_pauli_products))
            # Calculating the pauli products for each single shot measurement
            tmp_array = np.array(register.register)
            if register_name.endswith('flipped'):
                for index, val in mask.items():
                    if len(val) == 0:
                        single_shot_pauli_products[:, index] = 1
                    else:
                        single_shot_pauli_products[:, index] = np.power(
                            -1, np.sum(1 - tmp_array[:, val], axis=1))
            else:
                for index, val in mask.items():
                    if len(val) == 0:
                        single_shot_pauli_products[:, index] = 1
                    else:
                        single_shot_pauli_products[:, index] = np.power(
                            -1, np.sum(tmp_array[:, val], axis=1))
            # Averaging over the single shots to obtain pauli product expectation values
            pauli_product_dict[register_name] = np.array(np.mean(
                single_shot_pauli_products, axis=0))
        # Applying measurement correction when flipped measurement is used
        if self.measurement_input._use_flipped_measurement:
            for register_name in pauli_product_dict.keys():
                if not register_name.endswith('flipped'):
                    pauli_product_dict[register_name] = np.divide(
                        (pauli_product_dict[register_name]
                         + pauli_product_dict[register_name + '_flipped']) / 2,
                        measurement_correction_factors[register_name]
                    )
        pauli_products = np.zeros(self.measurement_input._number_pauli_products)
        for register_name, nd_array_val in pauli_product_dict.items():
            if not register_name.endswith('flipped'):
                pauli_products += nd_array_val

        expectation_values = self.measurement_input._pp_to_exp_val_matrix @ pauli_products

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
