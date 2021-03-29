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
"""Auxiliary data input for measurements"""
from typing import (
    List,
    Optional,
    Dict,
)
import numpy as np
from hqsbase.qonfig import Qonfig
import scipy.sparse as sp

DEFAULT_PP_TO_EXP_VAL_MATRIX = np.zeros((0, 0))


class BRMeasurementInput(object):
    """Necessary Information to run a BasisRotationMeasurement.

    BRMeasurementInput us the input class for BasisRotationMeasurement, dictating which expectation
    values are measured by BasisRotationMeasurement. These expecation values are defined as
    expectation values of pauli products, such as 3 * < Z0 > + < Z0 Z1 > (where Z0 represents the
    Pauli sigma-Z matrix acting on the 0 qubit).
    In order to pass 3 * < Z0 > + < Z0 Z1 > to BRMeasurementInput, we define:
    1) the pauli_product_qubit_masks: these are the masks for the involved qubits in the pauli
       products. In our example, 3 * < Z0 > + < Z0 Z1 > is composed of < Z0 > and < Z0Z1 >. These
       will be measured into the classical register 'ro'. The pauli products involve qubits [0] and
       [0, 1], respectively, so pauli_product_qubit_masks = {'ro': {0: [0], 1: [0, 1]}}.
       If you have several registers you are measuring, the structure of the
       pauli_product_qubit_masks is:
       {'1st reg': {0: [qubits], 1: [qubits], ..., n: [qubits]}, '2nd reg': {n + 1: [qubits], ...}}
    2) the values by which the pauli products are multiplied. In our example, those are 3 and 1 as
       we have 3 * < Z0 > + 1 * < Z0 Z1 >. The way we pass this to BRMeasurementInput is using
       pp_to_exp_val_matrix, which here will be np.array([3, 1]). This array should have the same
       number of entries as the number of entries in the pauli_product_qubit_masks dictionaries.
    3) the number of qubits in the measurement, here 2 (qubit 0 and qubit 1)
    4) the number of pauli products, here 2 (< Z0 > and < Z0Z1 >)
    5) the name of the measured expectation value, here (for instance) ["example"].

    There are additional inputs that can be defined, should the user want to.
    """

    _qonfig_defaults_dict = {
        'pauli_product_qubit_masks': {'doc': ('Masks for involved qubits in pauli products'
                                              + 'for the readout registers'),
                                      'default': dict()},
        'pp_to_exp_val_matrix_real': {
            'doc': ('Matrix converting measured pauli products to expectation values'
                    + 'by multiplication of vector of pauli products, real part'),
            'default': list()},
        'pp_to_exp_val_matrix_imag': {
            'doc': ('Matrix converting measured pauli products to expectation values'
                    + 'by multiplication of vector of pauli products, imaginary part'),
            'default': list()},
        'number_qubits': {
            'doc': 'The number of qubits in the measurement',
            'default': 0},
        'number_pauli_products': {
            'doc': ('The number of different pauli products'
                    + 'measured during the full measurement'),
            'default': 0},
        'use_flipped_measurement': {
            'doc': 'Use flipped readout to mitigate readout errors',
            'default': False},
        'measured_exp_vals': {
            'doc': 'List of names of measured expectation values',
            'default': 0},
    }
    _qonfig_never_receives_values = True

    def __init__(self,
                 pauli_product_qubit_masks: Optional[Dict[str, Dict[int, List[int]]]] = None,
                 pp_to_exp_val_matrix: np.ndarray = DEFAULT_PP_TO_EXP_VAL_MATRIX,
                 number_qubits: int = 0,
                 number_pauli_products: int = 0,
                 measured_exp_vals: Optional[List[str]] = None,
                 use_flipped_measurement: bool = False,
                 ) -> None:
        """Initialize measurement input

        Args:
            pauli_product_qubit_masks: Masks for involved qubits in pauli products
                                       for the readout registers
            pp_to_exp_val_matrix: Matrix converting measured pauli products to expectation values
                                  by multiplication of vector of pauli products
            number_qubits: The number of qubits in the measurement
            number_pauli_products: The number of different pauli products
                                   measured during the full measurement
            measured_exp_vals: List of names of measured expectation values
            use_flipped_measurement: Use flipped readout to mitigate readout errors

        """
        if pauli_product_qubit_masks is None:
            self._pauli_product_qubit_masks: Dict[str, Dict[int, List[int]]] = dict()
        else:
            self._pauli_product_qubit_masks = pauli_product_qubit_masks
        self._pp_to_exp_val_matrix = pp_to_exp_val_matrix
        self._number_qubits = number_qubits
        self._use_flipped_measurement = use_flipped_measurement
        self._number_pauli_products = number_pauli_products
        if measured_exp_vals is None:
            self._measured_exp_vals: List[str] = list()
        else:
            self._measured_exp_vals = measured_exp_vals

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['BRMeasurementInput']
                    ) -> 'BRMeasurementInput':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            BRMeasurementInput
        """
        # Reconstructing pauli product qubit masks from qonfig serialisation
        pp_qubit_masks: Dict[str, Dict[str, List[int]]] = config['pauli_product_qubit_masks']
        pauli_product_qubit_masks: Dict[str, Dict[int, List[int]]] = dict()
        for readout_key, readout_val in pp_qubit_masks.items():
            dict_for_readout: Dict[int, List] = dict()
            for pp_key, pp_val in readout_val.items():
                dict_for_readout[int(pp_key)] = pp_val
            pauli_product_qubit_masks[readout_key] = dict_for_readout
        # Reconstructing pauli product to expectation values matrix from qonfig serialisation
        real_matrix_flattened: List[float] = config['pp_to_exp_val_matrix_real']
        imag_matrix_flattened: List[float] = config['pp_to_exp_val_matrix_imag']
        matrix_shape = (len(config['measured_exp_vals']), config['number_pauli_products'])
        pp_to_exp_val_matrix = (np.array(real_matrix_flattened).reshape(matrix_shape)
                                + 1j * np.array(imag_matrix_flattened).reshape(matrix_shape))
        return cls(
            pauli_product_qubit_masks=pauli_product_qubit_masks,
            pp_to_exp_val_matrix=pp_to_exp_val_matrix,
            number_qubits=config['number_qubits'],
            number_pauli_products=config['number_pauli_products'],
            measured_exp_vals=config['measured_exp_vals'],
            use_flipped_measurement=config['use_flipped_measurement'],
        )

    def to_qonfig(self) -> 'Qonfig[BRMeasurementInput]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[BRMeasurementInput]
        """
        # Serialize pauli product qubit masks for qonfig
        pp_qubit_masks: Dict[str, Dict[str, List[int]]] = dict()
        for readout_key, readout_val in self._pauli_product_qubit_masks.items():
            dict_for_readout: Dict[str, List[int]] = dict()
            for pp_key, pp_val in readout_val.items():
                dict_for_readout[str(pp_key)] = pp_val
            pp_qubit_masks[readout_key] = dict_for_readout
        config = Qonfig(self.__class__)
        config._values['pauli_product_qubit_masks'] = pp_qubit_masks
        config._values['pp_to_exp_val_matrix_real'] = np.real(self._pp_to_exp_val_matrix).flatten()
        config._values['pp_to_exp_val_matrix_imag'] = np.imag(self._pp_to_exp_val_matrix).flatten()
        config['number_qubits'] = self._number_qubits
        config['number_pauli_products'] = self._number_pauli_products
        config['measured_exp_vals'] = self._measured_exp_vals
        config['use_flipped_measurement'] = self._use_flipped_measurement

        return config


class CheatedBRMeasurementInput(object):
    """Necessary Information to run a CheatedBasisRotationMeasurement"""

    _qonfig_defaults_dict = {
        'pp_to_exp_val_matrix_real': {
            'doc': ('Matrix converting measured pauli products to expectation values'
                    + 'by multiplication of vector of pauli products, real part'),
            'default': list()},
        'pp_to_exp_val_matrix_imag': {
            'doc': ('Matrix converting measured pauli products to expectation values'
                    + 'by multiplication of vector of pauli products, imaginary part'),
            'default': list()},
        'number_pauli_products': {
            'doc': ('The number of different pauli products'
                    + 'measured during the full measurement'),
            'default': 0},
        'measured_exp_vals': {
            'doc': 'List of names of measured expectation values',
            'default': 0},
    }

    def __init__(self,
                 pp_to_exp_val_matrix: np.ndarray = DEFAULT_PP_TO_EXP_VAL_MATRIX,
                 number_pauli_products: int = 0,
                 measured_exp_vals: Optional[List[str]] = None,
                 ) -> None:
        """Initialize CheatedBRMeasurementInput

        Args:
            pp_to_exp_val_matrix: Matrix converting measured pauli products to expectation values
                                  by multiplication of vector of pauli products
            number_pauli_products: The number of different pauli products
                                   measured during the full measurement
            measured_exp_vals: List of names of measured expectation values

        """
        self._pp_to_exp_val_matrix = pp_to_exp_val_matrix
        if measured_exp_vals is None:
            self._measured_exp_vals: List[str] = list()
        else:
            self._measured_exp_vals = measured_exp_vals
        self._number_pauli_products = number_pauli_products

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['CheatedBRMeasurementInput']
                    ) -> 'CheatedBRMeasurementInput':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            CheatedBRMeasurementInput
        """
        # Reconstructing pauli product to expectation values matrix from qonfig serialisation
        real_matrix_flattened: List[float] = config['pp_to_exp_val_matrix_real']
        imag_matrix_flattened: List[float] = config['pp_to_exp_val_matrix_imag']
        matrix_shape = (len(config['measured_exp_vals']), config['number_pauli_products'])
        pp_to_exp_val_matrix = (np.array(real_matrix_flattened).reshape(matrix_shape)
                                + 1j * np.array(imag_matrix_flattened).reshape(matrix_shape))
        return cls(pp_to_exp_val_matrix=pp_to_exp_val_matrix,
                   number_pauli_products=config['number_pauli_products'],
                   measured_exp_vals=config['measured_exp_vals'],
                   )

    def to_qonfig(self) -> 'Qonfig[CheatedBRMeasurementInput]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[CheatedBRMeasurementInput]
        """
        config = Qonfig(self.__class__)
        config._values['pp_to_exp_val_matrix_real'] = np.real(self._pp_to_exp_val_matrix).flatten()
        config._values['pp_to_exp_val_matrix_imag'] = np.imag(self._pp_to_exp_val_matrix).flatten()
        config['number_pauli_products'] = self._number_pauli_products
        config['measured_exp_vals'] = self._measured_exp_vals

        return config


class PurePragmaMeasurementInput(object):
    """Necessary Information to run a PurePragmaMeasurement"""

    _qonfig_defaults_dict = {
        'operator_matrices_real_data': {'doc': 'Dict of measured operators in matrix form real',
                                        'default': dict()},
        'operator_matrices_imag_data': {
            'doc': 'Dict of measured operators in matrix form imaginary',
            'default': dict()},
        'operator_matrices_indices': {'doc': 'Dict of measured operators in matrix form indices',
                                      'default': dict()},
        'operator_matrices_indptr': {'doc': 'Dict of measured operators in matrix form indptr',
                                     'default': dict()},
        'operator_matrices_dim': {'doc': 'Dict of measured operators in matrix form dimension',
                                  'default': 0},
        'use_density_matrix': {'doc': 'Use density matrix in simulator backend',
                               'default': False},
    }
    _qonfig_never_receives_values = True

    def __init__(self,
                 operator_matrices: Optional[Dict[str, Dict[str, sp.spmatrix]]] = None,
                 use_density_matrix: bool = False,
                 ) -> None:
        """Initialize PurePragmaMeasurementInput

        Args:
            operator_matrices: Dict of measured operators in matrix form
            use_density_matrix: Use density matrix in simulator backend
        """
        if operator_matrices is None:
            self.operator_matrices: Dict[str, Dict[str, sp.spmatrix]] = dict()
        else:
            self.operator_matrices = operator_matrices
        self.use_density_matrix = use_density_matrix

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['PurePragmaMeasurementInput']
                    ) -> 'PurePragmaMeasurementInput':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            PurePragmaMeasurementInput
        """
        # Reconstructing operator matrices
        operator_matrices: Dict[str, sp.spmatrix] = dict()
        dim: int = config['operator_matrices_dim']
        real_dict: Dict[str, Dict[str, List[float]]] = config['operator_matrices_real_data']
        imag_dict: Dict[str, Dict[str, List[float]]] = config['operator_matrices_imag_data']
        indices_dict: Dict[str, Dict[str, List[float]]] = config['operator_matrices_indices']
        indptr_dict: Dict[str, Dict[str, List[float]]] = config['operator_matrices_indptr']
        for readout_key, readout_dict_real in real_dict.items():
            readout_dict_imag = imag_dict[readout_key]
            readout_dict_indices = indices_dict[readout_key]
            readout_dict_indptr = indptr_dict[readout_key]
            operator_matrices[readout_key] = dict()
            for key, val_real in readout_dict_real.items():
                val_imag = readout_dict_imag[key]
                val_indices = readout_dict_indices[key]
                val_indptr = readout_dict_indptr[key]
                operator_matrices[readout_key][key] = (
                    sp.csr_matrix((val_real, val_indices, val_indptr), (dim, dim))
                    + 1j * sp.csr_matrix((val_imag, val_indices, val_indptr), (dim, dim)))
        return cls(
            operator_matrices=operator_matrices,
            use_density_matrix=config['use_density_matrix']
        )

    def to_qonfig(self) -> 'Qonfig[PurePragmaMeasurementInput]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[PurePragmaMeasurementInput]
        """
        # Serialise operator matrices
        operator_matrices_real_data: Dict[str, Dict[str, List[float]]] = dict()
        operator_matrices_imag_data: Dict[str, Dict[str, List[float]]] = dict()
        operator_matrices_indices: Dict[str, Dict[str, List[int]]] = dict()
        operator_matrices_indptr: Dict[str, Dict[str, List[int]]] = dict()

        if self.operator_matrices != dict():
            # extracting dimension from first values
            operator_matrices_dim = list(
                list(
                    self.operator_matrices.values())[0].values())[0].shape[0]
        else:
            operator_matrices_dim = 0
        for readout_key, readout_val in self.operator_matrices.items():
            operator_matrices_real_data[readout_key] = dict()
            operator_matrices_imag_data[readout_key] = dict()
            operator_matrices_indices[readout_key] = dict()
            operator_matrices_indptr[readout_key] = dict()
            for key, val in readout_val.items():
                operator_matrices_real_data[readout_key][key] = np.real(val.data)
                operator_matrices_imag_data[readout_key][key] = np.imag(val.data)
                operator_matrices_indices[readout_key][key] = val.indices
                operator_matrices_indptr[readout_key][key] = val.indptr
        config = Qonfig(self.__class__)
        config._values['operator_matrices_real_data'] = operator_matrices_real_data
        config._values['operator_matrices_imag_data'] = operator_matrices_imag_data
        config._values['operator_matrices_indices'] = operator_matrices_indices
        config._values['operator_matrices_indptr'] = operator_matrices_indptr
        config['operator_matrices_dim'] = operator_matrices_dim
        config['use_density_matrix'] = self.use_density_matrix
        return config
