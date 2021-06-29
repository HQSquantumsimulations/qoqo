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

from typing import (
    Union,
    List,
    Dict,
    Optional,
    cast,
    Any
)


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
       defined in terms of pauli product expectations, such as the BasisRotation
       (in qoqo -> measurements).
    """

    def __init__(self,
                 backend: Any,
                 measurement: Any,
                 free_parameters: List[str],
                 device: Optional[Any] = None,
                 resume_call_parameters: Optional[Dict[str, float]] = None,
                 resume_file_name: Optional[str] = None) -> None:
        """Initialize DoUnitary.

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
        self._backend = backend
        self._free_parameters = free_parameters
        self._device = device
        self._backend.device = device
        self._resume_file_name: Optional[str] = resume_file_name
        self._resume_call_parameters: Optional[Dict[str, float]] = resume_call_parameters

    def __call__(self,
                 parameters: Optional[Union[List[float], Dict['str', float]]] = None,
                 ) -> Optional[Dict[str, float]]:
        """Run DoUnitary.

        This executes the unitary evolution and the measurement, with the parameters replacing the
        free variables in the circuit.

        Args:
            parameters: The parameters t_i of the unitary evolution.
                        These must be given either as a list in the order of the H_i or as
                        a dictionary of the form {i: t}, where the t_i matches the H_i by name i

        Returns:
            Optional[Dict[str, float]]: The measured expectation values plus the set parameters

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

        run_measurement = self._measurement.substitute_parameters(parameter_substitution_dict)

        expectation_values = self._backend.run_measurement(run_measurement)
        if expectation_values is None:
            # if self._resume_file_name is not None:
            #     resume_config = self.to_qonfig()
            #     resume_config['measurement']['resume_list'] = getattr(
            #         self._measurement, '_resume_info', None)
            #     resume_config.save_to_yaml(self._resume_file_name,
            #                                overwrite=False)
            return None
        self._backend_cached = self._backend
        expectation_values.update(parameter_substitution_dict)
        return expectation_values
