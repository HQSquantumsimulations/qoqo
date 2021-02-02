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
"""Abstract Base Class for all Devices used in qoqo interfaces and backends"""

import abc
import networkx as nx
from typing import List, Dict, Optional
from hqsbase.qonfig import Qonfig


class DeviceBaseClass(abc.ABC):
    r"""Base Class for all devices (quantum hardware) used in qoqo interfaces and backends.

    The class is based on the networkx (imported as nx) Python graph library
    https://networkx.github.io/

    The device itself is represented as a Directed Multi Graph G.

    The edges are the physical qubits, the decoherence rates
    (dephasing_rate, damping_rate, depolarisation_rate) are node attributes in the graph accessible
    via
    dephasing_rates = nx.get_node_attributs(G, 'dephasing_rate')

    Each PHYSICAL available gate operation is represented as a graph on the nodes.
    This graph is part of the Multi Graph with the name of the gate operation as the key.

    For single-qubit states the graph consists of self loops.
    Each edge has attributes, mainly gate_time (the time the gate needs to execute) and
    overrotation, a dictionary defining which gate variables duffer from static and
    statistic overrotation.
    Take for example a two qubit system with different dephasing times on both
    qubits and a static overrotation in the rotation angle 'theta' of the RotateZ gates.
    In this case we could build the corresponding device graph as:

    .. code-block:: python

        qubits=[0,1]
        dephasing_rates={0: 1/30e-6, 1: 1/20e-6}
        G = nx.MultiDiGraph()
        G.add_node(0, dephasing_rate = dephasing_rates[0])
        G.add_node(1, dephasing_rate = dephasing_rates[1])
        G.add_edge(0,0,key='RotateZ',
                gate_time=50e-9,
                overrotation={
                    'theta':{
                        'static':{
                            'mean': 0,
                            'var': 0.01
                        }}})
        G.has_edge(0,0,key='RotateZ')
        G.has_edge(0,1,key='RotateZ')
        G.edges(0,0,'RotateZ')

    And if we add a two-qubit controlled Z gate without overrotations

    .. code-block:: python

        G.add_edge(0,1,key='ControlledPauliZ',
                gate_time='150e-9'
                )

    MeasureQubit gates are special and have no overrotation property.
    Instead they have a measurement error attribute, giving the probability
    to measure the real state 0 as 1 and the probability to measure 1 as 0.

    .. code-block:: python

        G.add_edge(0,0,key='MeasureQubit',
                gate_time='50e-9',
                measurement_error={
                    'prob_0_as_1' : 0.01,
                    'prob_1_as_0' : 0.01
                }
            )

    To determine which qubits are connected by universal two-qubit gates (and can be swapped),
    devices provides a simpler connectivity graph (an undirected Graph) that has the qubits
    as nodes and the all universal two-qubit connections as edges.

    The following properties are available for all devices:
        - overrotations, dephasing_rates, damping_rates, depolarising_rates: dictionaries
            returning the relevant information
        - number_qubits: the number of qubits in the device, regardless of connectivity
        - available_gates: all of the gates (one- and two-qubit gates) that are available on
            the hardware (device)
        - available_one_qubit_gates, available_two_qubit_gates
        - device_graph, connectivity_graph: multi DiGraph and simple graph (respectively)
            representing the device
        - depolarisation_from_t1_t2, dephasing_from_t1_t2: depolarisation and dephasing rates
            (respectively), calculated from t1 and t2 times taken from the hardware
        - to_qonfig: devices can be serialised with the HQS Qonfig package

    """

    @abc.abstractmethod
    def __init__(self) -> None:
        """Initialize the device"""
        self._connectivity_graph: nx.Graph
        self._device_multi_di_graph: nx.MultiDiGraph
        self._list_one_qubit_gates: List[str]
        self._list_two_qubit_gates: List[str]
        self._qubit_names: Dict[float, float]
        self._qubit_names = dict()

    @property
    def overrotations(self) -> dict:
        """Return a dict of overrotations of the gates in the device

        Returns:
            Dict
        """
        return nx.get_edge_attributes(self._device_multi_di_graph, 'overrotation')

    @property
    def dephasing_rates(self) -> dict:
        """Return a dict of dephasing rates in the device

        Returns:
            Dict
        """
        return nx.get_node_attributes(self._device_multi_di_graph, 'dephasing_rate')

    @property
    def damping_rates(self) -> dict:
        """Return a dict of damping rates in the device

        Returns:
            Dict
        """
        return nx.get_node_attributes(self._device_multi_di_graph, 'damping_rate')

    @property
    def depolarisation_rates(self) -> dict:
        """Return a dict of depolarisation rates in the device

        Returns:
            Dict
        """
        return nx.get_node_attributes(self._device_multi_di_graph, 'depolarisation_rate')

    @property
    def number_qubits(self) -> float:
        """Return the number of physical qubits in the device

        Returns:
            float
        """
        return self._device_multi_di_graph.number_of_nodes()

    @property
    def available_gates(self) -> List:
        """Return a list of all available gates

        Returns:
            List
        """
        list_avail = self.available_one_qubit_gates + self.available_two_qubit_gates
        return list_avail

    @property
    def available_one_qubit_gates(self) -> List[str]:
        """Return a list of all available one-qubit gates

        Returns:
            List[str]
        """
        return self._list_one_qubit_gates

    @property
    def available_two_qubit_gates(self) -> List[str]:
        """Return a list of all available two-qubit gates

        Returns:
            List[str]
        """
        return self._list_two_qubit_gates

    @property
    def device_graph(self) -> nx.MultiDiGraph:
        """Return the MultiDiGraph containing all gates and decoherence rates of the device

        Returns:
            nx.MultiDiGraph
        """
        return self._device_multi_di_graph

    @property
    def connectivity_graph(self) -> nx.Graph:
        """Return the Graph containing all two-qubit connections of the device

        Returns:
            nx.Graph()
        """
        return self._connectivity_graph

    @abc.abstractmethod
    def to_qonfig(self,
                  ) -> Qonfig:
        """Serialize the device with Qonfig"""
        pass

    @staticmethod
    def depolarisation_from_t1_t2(t1: float,
                                  t2: Optional[float] = 0,
                                  ) -> float:
        """Return the depolarisation rate from t1 and t2 times, from specific hardware

        Args:
            t1: t1 time obtained from device
            t2: t2 time obtained from device

        Returns:
            float: depolarising rate t1*
                t1* = t1
        """
        return 1 / t1

    @staticmethod
    def dephasing_from_t1_t2(t1: float,
                             t2: float,
                             ) -> float:
        """Return the dephasing rate from t1 and t2 times, from specific hardware

        Args:
            t1: t1 time obtained from device
            t2: t2 time obtained from device

        Returns:
            float: dephasing rate t2*
                t2* = 1/(1/t2 - 1/2t1)
        """
        inv_t2_star = 1 / t2 - 1 / (2 * t1)
        return inv_t2_star
