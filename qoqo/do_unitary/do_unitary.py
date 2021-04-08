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
"""Base class for DoUnitary"""

from qoqo.circuit.circuit import Circuit
from qoqo.devices import (
    DeviceBaseClass,
)
from qoqo.backends import (
    BackendBaseClass,
)
from qoqo.operations import PragmaParameterSubstitution
from qoqo.measurements import (
    MeasurementBaseClass,
)
from copy import copy
from typing import (
    Union,
    List,
    Dict,
    Optional,
    cast,
    Any
)
import pandas as pd
from hqsbase.qonfig import Qonfig, empty


class DoUnitary(object):
    r"""DoUnitary class.

    DoUnitary takes a prepared list of circuits (initialization, unitary evolution and measurement)
    and executes it. When DoUnitary is called, the parameters replace the free variables in the
    prepared circuit.

    1) The initialization phase can be anything from just starting in a certain
       state to starting in the basic |0> state and evolving to the steady-state.
    2) The unitary evolution phase is a circuit with gates usually representing a certain
       Hamiltonian, repeated a certain number of times or Trotter steps.
    3) The measurement phase is the measurement of certain expectation values. These are
       defined in terms of pauli product expectations, such as in
       qoqo.measurements.basis_rotation_measurements.py.
    """

    _qonfig_defaults_dict: Dict[str, Dict[str, Any]] = {
        'device': {
            'doc': ('A qoqo device specification giving the available gates, '
                    + 'topology and error properties'),
            'default': None},
        'backend': {
            'doc': ('A qoqo backend either simulating a circuit '
                    + 'or running it on a real QPU'),
            'default': empty},
        'measurement': {
            'doc': ('An qoqo measurement class transforming the result of the simulation'
                    + ' or QPU run into an operator measurement'),
            'default': empty},
        'free_parameters': {
            'doc': ('List of free parameters that need to be set when calling DoUnitary'
                    + 'to return expectation values'),
            'default': list()},
        'resume_call_parameters': {
            'doc': ('Set of parameters for __call__ when doUnitary is resuming old run. '
                    + 'DO NOT SET BY HAND'),
            'default': None},
        'resume_file_name': {
            'doc': ('Optional file name for resuming doUnitary run. Fails when old file '
                    + 'would be overwritten.'),
            'default': None},
    }
    _qonfig_never_receives_values = False

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['DoUnitary']
                    ) -> 'DoUnitary':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            DoUnitary
        """
        if isinstance(config['device'], Qonfig):
            init_device = config['device'].to_instance()
        else:
            init_device = cast(Optional[DeviceBaseClass], config['device'])
        return cls(backend=config['backend'].to_instance(),
                   measurement=config['measurement'].to_instance(),
                   free_parameters=config['free_parameters'],
                   device=init_device,
                   resume_call_parameters=config['resume_call_parameters'],
                   resume_file_name=config['resume_file_name'],
                   )

    def to_qonfig(self) -> 'Qonfig[DoUnitary]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[DoUnitary]
        """
        config = Qonfig(self.__class__)
        self._backend.circuit = Circuit()
        config['backend'] = self._backend.to_qonfig()
        self._measurement.backend = None
        config['measurement'] = self._measurement.to_qonfig()
        self._measurement.backend = self._backend
        config['free_parameters'] = self._free_parameters
        if self._device is None:
            config['device'] = None
        else:
            config['device'] = self._device.to_qonfig()
        config['resume_call_parameters'] = self._resume_call_parameters
        config['resume_file_name'] = self._resume_file_name
        return config

    def __init__(self,
                 backend: BackendBaseClass,
                 measurement: MeasurementBaseClass,
                 free_parameters: List[str],
                 device: Optional[DeviceBaseClass] = None,
                 resume_call_parameters: Optional[Dict[str, float]] = None,
                 resume_file_name: Optional[str] = None) -> None:
        """Initialize DoUnitary

        Args:
            backend: A qoqo backend either simulating a circuit or running it on a real QPU
            measurement: A qoqo measurement class transforming the result of the simulation
                         or QPU run into an operator measurement
            device: A qoqo device specification giving the available gates,
                    topology and error properties
            free_parameters: List of free parameters that need to be set when calling DoUnitary
                             to return expectation values
            resume_call_parameters: Set of parameters for __call__ when DoUnitary is resuming
                                    an old run. DO NOT SET BY HAND
            resume_file_name: Optional file name for resuming DoUnitary run. Fails when
                              old file would be overwritten.
        """
        self._measurement = measurement
        self._circuit_list = self._measurement.circuit_list
        self._backend = backend
        self._free_parameters = free_parameters
        self._device = device
        self._backend.device = device
        self._measurement.backend = self._backend
        self._measurement.device = self._device
        self._run_measurement = copy(self._measurement)
        self._resume_file_name: Optional[str] = resume_file_name
        self._resume_call_parameters: Optional[Dict[str, float]] = resume_call_parameters

    def __call__(self,
                 parameters: Optional[Union[List[float], Dict['str', float]]] = None,
                 ) -> pd.Series:
        """Run DoUnitary

        This executes the unitary evolution and the measurement, with the parameters replacing the
        free variables in the circuit.

        Args:
            parameters: The parameters t_i of the unitary evolution.
                        These must be given either as a list in the order of the H_i or as
                        a dictionary of the form {i: t}, where the t_i matches the H_i by name i

        Returns:
            pd.Series: Result of the measurement as a pandas Series object

        Raises:
            ValueError: All parameters of the unitary time evolution must be set
        """
        self.call_parameters = parameters

        if parameters is None:
            parameters = cast(Dict[str, float], dict())
        parameter_substitution_dict: Dict[str, float] = {}

        if isinstance(parameters, dict):
            parameter_substitution_dict = parameters
            if any([name not in parameter_substitution_dict.keys()
                    for name in self._free_parameters]):
                raise ValueError("All parameters of the unitary time evolution must be set")
        else:
            for name, p in zip(self._free_parameters, parameters):
                parameter_substitution_dict[name] = p

        if self._resume_file_name is not None:
            self._resume_call_parameters = parameter_substitution_dict

        self._run_measurement._constant_circuit += PragmaParameterSubstitution(
            substitution_dict=parameter_substitution_dict)
        expectation_values = self._run_measurement()
        del self._run_measurement._constant_circuit[len(
            self._run_measurement._constant_circuit) - 1]
        if expectation_values is None:
            if self._resume_file_name is not None:
                resume_config = self.to_qonfig()
                resume_config['measurement']['resume_list'] = getattr(
                    self._measurement, '_resume_info', None)
                resume_config.save_to_yaml(self._resume_file_name,
                                           overwrite=False)
            return None
        self._backend_cached = self._measurement._backend
        if not parameter_substitution_dict:
            parameter_series = pd.Series({}, dtype=complex)
        else:
            parameter_series = pd.Series(parameter_substitution_dict)
        parameter_series = parameter_series.add_prefix('unitary_parameter_')
        expectation_values = expectation_values.append(parameter_series)
        return expectation_values
