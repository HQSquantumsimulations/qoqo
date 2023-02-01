// Copyright © 2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, HashSet};

use crate::operations::*;
use crate::Circuit;
use crate::RoqoqoVersionSerializable;
use crate::{RoqoqoError, RoqoqoVersion};

use petgraph::adj::NodeIndex;
use petgraph::algo;
use petgraph::algo::toposort;
use petgraph::graph::{Graph, Neighbors};
use petgraph::visit::Dfs;
use petgraph::Directed;
use petgraph::Direction::{Incoming, Outgoing};

/// Represents the Direct Acyclic Graph (DAG) of a Circuit.
///
/// The order of execution of the operations contained in a quantum circuit matters.
/// A CircuitDag instance of a roqoqo's Circuit shows the dependency of all the operations
/// in the circuit.
///
/// For CircuitDag, the following functions are defined:
/// * `with_capacity(node_number, edge_number)`: creates an empty CircuitDag with estimated capacity
/// * `add_to_back(operation)`: adds an Operation to the back of the CircuitDag
/// * `add_to_front(operation)`: adds an Operation to the front of the CircuitDag
/// * `execution_blocked(already_executed_indices, index)`: returns the blocking elements of the execution of an Operation, the scope is just the whole graph
/// * `blocking_predecessors(already_executed_indices, index)`: returns the blocking elements of the execution of an Operation, the scope is just the Operation's predecessors
/// * `new_front_layer(already_executed_indices, front_layer_indices, index)`: given an Operation index, computes a new front layer when considering that Operation as executed
/// * `parallel_blocks()`: returns an iterator over the possible parallel blocks in circuit that can be executed simultaneously
/// * `successors(index)`: returns an iterator over all successors in the CircuitDag of a given node
/// * `commuting_operators()`: returns a reference to the vector of commuting operations in CircuitDag
/// * `first_parallel_block()`: returns a reference to the HashSet containing the nodes in the first parallel block
/// * `last_parallel_block()`: returns a reference to the HashSet containing the nodes in the last parallel block
/// * `first_operation_involving_qubit()`: returns a reference to the HashMap where a key represents a qubit and its value represents the first node that involves that qubit
/// * `last_operation_involving_qubit()`: returns a reference to the HashMap where a key represents a qubit and its value represents the last node that involves that qubit
/// * `first_operation_involving_classical()`: returns a reference to the HashMap where a key is composed by the name and the size of the classical register and its value represents the first node that involves that register
/// * `last_operation_involving_classical()`: returns a reference to the HashMap where a key is composed by the name and the size of the classical register and its value represents the last node that involves that register
/// * `get(index)`: returns a reference to the Operation contained in the indexed CircuitDag's node
///
/// Note: operations PragmaStartDecompositionBlock and PragmaStopDecompositionBlock are considered part of the graph.
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct CircuitDag {
    pub(crate) graph: Graph<Operation, (), Directed, usize>,
    pub(crate) commuting_operations: Vec<NodeIndex<usize>>,
    pub(crate) first_parallel_block: HashSet<NodeIndex<usize>>,
    pub(crate) last_parallel_block: HashSet<NodeIndex<usize>>,
    pub(crate) first_all: Option<NodeIndex<usize>>,
    pub(crate) last_all: Option<NodeIndex<usize>>,
    pub(crate) first_operation_involving_qubit: HashMap<usize, NodeIndex<usize>>,
    pub(crate) last_operation_involving_qubit: HashMap<usize, NodeIndex<usize>>,
    pub(crate) first_operation_involving_classical: HashMap<(String, usize), NodeIndex<usize>>,
    pub(crate) last_operation_involving_classical: HashMap<(String, usize), NodeIndex<usize>>,
    _roqoqo_version: RoqoqoVersion,
}

#[cfg(feature = "serialize")]
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "CircuitDag"))]
struct CircuitDagSerializable {
    graph: Graph<Operation, (), Directed, usize>,
    commuting_operations: Vec<NodeIndex<usize>>,
    first_parallel_block: HashSet<NodeIndex<usize>>,
    last_parallel_block: HashSet<NodeIndex<usize>>,
    first_all: Option<NodeIndex<usize>>,
    last_all: Option<NodeIndex<usize>>,
    first_operation_involving_qubit: HashMap<usize, NodeIndex<usize>>,
    last_operation_involving_qubit: HashMap<usize, NodeIndex<usize>>,
    first_operation_involving_classical: HashMap<(String, usize), NodeIndex<usize>>,
    last_operation_involving_classical: HashMap<(String, usize), NodeIndex<usize>>,
    /// The roqoqo version.
    _roqoqo_version: RoqoqoVersionSerializable,
}

#[cfg(feature = "serialize")]
impl TryFrom<CircuitDagSerializable> for CircuitDag {
    type Error = RoqoqoError;
    fn try_from(value: CircuitDagSerializable) -> Result<Self, Self::Error> {
        Ok(CircuitDag {
            _roqoqo_version: RoqoqoVersion,
            graph: value.graph,
            commuting_operations: value.commuting_operations,
            first_parallel_block: value.first_parallel_block,
            last_parallel_block: value.last_parallel_block,
            first_all: value.first_all,
            last_all: value.last_all,
            first_operation_involving_qubit: value.first_operation_involving_qubit,
            last_operation_involving_qubit: value.last_operation_involving_qubit,
            first_operation_involving_classical: value.first_operation_involving_classical,
            last_operation_involving_classical: value.last_operation_involving_classical,
        })
    }
}

#[cfg(feature = "serialize")]
impl From<CircuitDag> for CircuitDagSerializable {
    fn from(value: CircuitDag) -> Self {
        let min_version = value.minimum_supported_roqoqo_version();
        let current_version = RoqoqoVersionSerializable {
            major_version: min_version.0,
            minor_version: min_version.1,
        };
        Self {
            _roqoqo_version: current_version,
            graph: value.graph,
            commuting_operations: value.commuting_operations,
            first_parallel_block: value.first_parallel_block,
            last_parallel_block: value.last_parallel_block,
            first_all: value.first_all,
            last_all: value.last_all,
            first_operation_involving_qubit: value.first_operation_involving_qubit,
            last_operation_involving_qubit: value.last_operation_involving_qubit,
            first_operation_involving_classical: value.first_operation_involving_classical,
            last_operation_involving_classical: value.last_operation_involving_classical,
        }
    }
}

/// Iterator over all possible parallel executable blocks of a Circuit.
///
#[derive(Debug)]
pub struct ParallelBlocks<'a> {
    dag: &'a CircuitDag,
    parallel_block: Vec<NodeIndex<usize>>,
    already_executed: Vec<NodeIndex<usize>>,
}

impl PartialEq for CircuitDag {
    fn eq(&self, other: &Self) -> bool {
        let nodes = |a: &Operation, b: &Operation| a.eq(b);
        let edges = |_: &(), _: &()| true;
        algo::is_isomorphic_matching(&self.graph, &other.graph, nodes, edges)
    }
}

impl CircuitDag {
    /// Creates a new empty CircuitDag.
    ///
    pub fn with_capacity(node_number: usize, edge_number: usize) -> Self {
        CircuitDag {
            graph: Graph::<Operation, (), Directed, usize>::with_capacity(node_number, edge_number),
            commuting_operations: Vec::<NodeIndex<usize>>::new(),
            first_parallel_block: HashSet::<NodeIndex<usize>>::new(),
            last_parallel_block: HashSet::<NodeIndex<usize>>::new(),
            first_all: Option::<NodeIndex<usize>>::None,
            last_all: Option::<NodeIndex<usize>>::None,
            first_operation_involving_qubit: HashMap::<usize, NodeIndex<usize>>::new(),
            last_operation_involving_qubit: HashMap::<usize, NodeIndex<usize>>::new(),
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex<usize>>::new(
            ),
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex<usize>>::new(),
            _roqoqo_version: RoqoqoVersion,
        }
    }

    /// Adds an operation to the back of the CircuitDag, if necessary.
    ///
    /// # Arguments
    ///
    /// * 'operation' - The Operation to add to the back of the CircuitDag.
    ///
    /// # Returns
    ///
    /// * 'Option<NodeIndex<usize>>' - The NodeIndex relative to the Operation, if added to CircuitGraph.
    pub fn add_to_back(&mut self, operation: Operation) -> Option<usize> {
        // Create node
        let node = self.graph.add_node(operation.clone());

        // InvolvedQubits: push to commuting_operations or start the add to back process
        match operation {
            Operation::DefinitionBit(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionFloat(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionUsize(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionComplex(_) => self.commuting_operations.push(node.index()),
            _ => {
                if let (InvolvedQubits::None, InvolvedClassical::None) =
                    (operation.involved_qubits(), operation.involved_classical())
                {
                    self.commuting_operations.push(node.index());
                } else {
                    self.add_to_back_involved(node.index());
                }
            }
        }

        // InvolvedClassical: populate for the first time the classical register data
        // structure or start the update process
        if !self.is_definition_classical_populate(node.index(), operation.clone()) {
            self.update_classical_back(node.index(), operation);
        }

        Some(node.index())
    }

    /// Adds an operation that involves some or all qubits to the end of the CircuitDag.
    ///
    /// # Arguments
    ///
    /// * 'node' - The NodeIndex<usize> of the node to add to the end of the CircuitDag.
    fn add_to_back_involved(&mut self, node: NodeIndex<usize>) {
        let node_involved_qubits: InvolvedQubits = self
            .graph
            .node_weight(node.into())
            .unwrap()
            .involved_qubits();
        // Calls the proper subfunction depending on the qubits involved
        //  in the operation
        if let InvolvedQubits::Set(x) = node_involved_qubits {
            for qubit in x {
                self.update_from_qubit_back(node, qubit);
            }
            if self
                .graph
                .neighbors_directed(node.into(), Incoming)
                .next()
                .is_none()
            {
                self.first_parallel_block.insert(node);
            }
        } else if let InvolvedQubits::All = node_involved_qubits {
            self.update_from_all_operation_back(node);
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag from a single qubit involved
    /// in an Operation added to the back of the CircuitDag.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves the qubit.
    /// * 'qubit' - The qubit involved in the Operation.
    fn update_from_qubit_back(&mut self, node: NodeIndex<usize>, qubit: usize) {
        // Update last_operation_involving qubit and last_parallel_block
        //  depending on current structure
        if let Some(&i) = self.last_operation_involving_qubit.get(&qubit) {
            self.graph.update_edge(i.into(), node.into(), ());
            self.last_parallel_block.remove(&i);
        } else if let Some(la) = self.last_all {
            self.graph
                .update_edge(self.last_all.unwrap().into(), node.into(), ());
            self.last_parallel_block.remove(&la);
        }
        let qubit_presence = self.last_operation_involving_qubit.insert(qubit, node);
        self.last_parallel_block.insert(node);

        // Update the first layer in case the qubit has never been seen before
        if qubit_presence.is_none() {
            // Update first_operation_involving_qubit depending on last_all
            if self.last_all.is_none() {
                self.first_operation_involving_qubit.insert(qubit, node);
            } else {
                self.first_operation_involving_qubit
                    .insert(qubit, self.last_all.unwrap());
            }
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag when an Operation that involves
    /// all qubits is added to the back.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves all qubits.
    fn update_from_all_operation_back(&mut self, node: NodeIndex<usize>) {
        // Update first_all and last_all
        if self.first_all.is_none() {
            self.first_all = Some(node);
        }
        self.last_all = Some(node);

        // Set the node as the only one in last_parallel_block and updates
        //  first_parallel_block if necessary
        self.last_parallel_block.clear();
        self.last_parallel_block.insert(node);
        if self.first_parallel_block.is_empty() {
            self.first_parallel_block.insert(node);
        }

        // Handling InvolvedQubits::All as first Operation
        if self.first_operation_involving_qubit.is_empty()
            && self.last_operation_involving_qubit.is_empty()
        {
            self.first_operation_involving_qubit.insert(0, node);
            self.last_operation_involving_qubit.insert(0, node);
        } else {
            // All the latest nodes in the graph must now point to the new node and
            //  last_operation_involving_qubit is updated
            let mut temp_map: HashMap<usize, NodeIndex<usize>> =
                HashMap::with_capacity(self.last_operation_involving_qubit.capacity());
            for (&qubit, &old_node) in &self.last_operation_involving_qubit {
                self.graph.update_edge(old_node.into(), node.into(), ());
                temp_map.insert(qubit, node);
            }
            self.last_operation_involving_qubit = temp_map;
        }
    }

    /// Adds an operation to the front of the CircuitDag is necessary.
    ///
    /// # Arguments
    ///
    /// * 'operation' - The Operation to add to the front of the CircuitDag.
    ///
    /// # Returns
    ///
    /// * 'Option<NodeIndex<usize>>' - The NodeIndex<usize> relative to the Operation, if added to CircuitGraph.
    pub fn add_to_front(&mut self, operation: Operation) -> Option<usize> {
        // Create node
        let node = self.graph.add_node(operation.clone());

        // InvolvedQubits: push to commuting_operations or start the add to front process
        match operation {
            Operation::DefinitionBit(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionFloat(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionUsize(_) => self.commuting_operations.push(node.index()),
            Operation::DefinitionComplex(_) => self.commuting_operations.push(node.index()),
            _ => {
                if let (InvolvedQubits::None, InvolvedClassical::None) =
                    (operation.involved_qubits(), operation.involved_classical())
                {
                    self.commuting_operations.push(node.index());
                } else {
                    self.add_to_front_involved(node.index());
                }
            }
        }

        // InvolvedClassical: populate for the first time the classical register data
        // structure or start the update process
        if !self.is_definition_classical_populate(node.index(), operation.clone()) {
            self.update_classical_front(node.index(), operation);
        }

        Some(node.index())
    }

    /// Adds an operation that involves some or all qubits to the front of the CircuitDag.
    ///
    /// # Arguments
    ///
    /// * 'node' - The NodeIndex<usize> of the node to add to the end of the CircuitDag.
    fn add_to_front_involved(&mut self, node: NodeIndex<usize>) {
        let node_involved_qubits: InvolvedQubits = self
            .graph
            .node_weight(node.into())
            .unwrap()
            .involved_qubits();
        // Calls the proper subfunction depending on the qubits involved
        //  in the operation
        if let InvolvedQubits::Set(x) = node_involved_qubits {
            for qubit in x {
                self.update_from_qubit_front(node, qubit);
            }
            if self
                .graph
                .neighbors_directed(node.into(), Outgoing)
                .next()
                .is_none()
            {
                self.last_parallel_block.insert(node);
            }
        } else if let InvolvedQubits::All = node_involved_qubits {
            self.update_from_all_operation_front(node);
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag from a single qubit involved
    /// in an Operation added to the front of the CircuitDag.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves the qubit.
    /// * 'qubit' - The qubit involved in the Operation.
    fn update_from_qubit_front(&mut self, node: NodeIndex<usize>, qubit: usize) {
        // Update first_operation_involving qubit and first_parallel_block
        //  depending on current structure
        if let Some(&i) = self.first_operation_involving_qubit.get(&qubit) {
            self.graph.update_edge(node.into(), i.into(), ());
            self.first_parallel_block.remove(&i);
        } else if let Some(fa) = self.first_all {
            self.graph
                .update_edge(node.into(), self.first_all.unwrap().into(), ());
            self.first_parallel_block.remove(&fa);
        }
        let qubit_presence = self.first_operation_involving_qubit.insert(qubit, node);
        self.first_parallel_block.insert(node);

        // Update the last layer in case the qubit has never been seen before
        if qubit_presence.is_none() {
            // Update last_operation_involving_qubit depending on first_all
            if self.first_all.is_none() {
                self.last_operation_involving_qubit.insert(qubit, node);
            } else {
                self.last_operation_involving_qubit
                    .insert(qubit, self.first_all.unwrap());
            }
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag when an Operation that involves
    /// all qubits is added to the front.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves all qubits.
    fn update_from_all_operation_front(&mut self, node: NodeIndex<usize>) {
        // Update last_all and first_all
        if self.last_all.is_none() {
            self.last_all = Some(node);
        }
        self.first_all = Some(node);

        // Set the node as the only one in first_parallel_block and updates
        //  last_parallel_block if necessary
        self.first_parallel_block.clear();
        self.first_parallel_block.insert(node);
        if self.last_parallel_block.is_empty() {
            self.last_parallel_block.insert(node);
        }

        // Handling InvolvedQubits::All as first Operation
        if self.first_operation_involving_qubit.is_empty()
            && self.last_operation_involving_qubit.is_empty()
        {
            self.first_operation_involving_qubit.insert(0, node);
            self.last_operation_involving_qubit.insert(0, node);
        } else {
            // The new node in the graph must point to the last first layer nodes and
            //  first_operation_involving_qubit is updated
            let mut temp_map: HashMap<usize, NodeIndex<usize>> =
                HashMap::with_capacity(self.first_operation_involving_qubit.capacity());
            for (&qubit, &old_node) in &self.first_operation_involving_qubit {
                self.graph.update_edge(node.into(), old_node.into(), ());
                temp_map.insert(qubit, node);
            }
            self.first_operation_involving_qubit = temp_map;
        }
    }

    /// Given an Operation and its node, checks that it is a Definition and populates the
    /// classical layer accordingly.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node of the Operation.
    /// * 'operation' - The Operation itself.
    fn is_definition_classical_populate(
        &mut self,
        node: NodeIndex<usize>,
        operation: Operation,
    ) -> bool {
        match &operation {
            Operation::DefinitionBit(_) => {
                let new_op: DefinitionBit = operation.clone().try_into().unwrap();
                for i in 0..*new_op.length() {
                    self.first_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                    self.last_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                }
                true
            }
            Operation::DefinitionComplex(_) => {
                let new_op: DefinitionComplex = operation.clone().try_into().unwrap();
                for i in 0..*new_op.length() {
                    self.first_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                    self.last_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                }
                true
            }
            Operation::DefinitionFloat(_) => {
                let new_op: DefinitionFloat = operation.clone().try_into().unwrap();
                for i in 0..*new_op.length() {
                    self.first_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                    self.last_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                }
                true
            }
            Operation::DefinitionUsize(_) => {
                let new_op: DefinitionUsize = operation.clone().try_into().unwrap();
                for i in 0..*new_op.length() {
                    self.first_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                    self.last_operation_involving_classical
                        .insert((String::from(new_op.name()), i), node);
                }
                true
            }
            _ => false,
        }
    }

    /// Checks and updates the relevant classical registers attributes from a given Operation
    /// that was added to the back of the graph.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node of the Operation that was added to the back of the graph.
    /// * 'operation' - The Operation that was added to the back of the graph.
    fn update_classical_back(&mut self, node: NodeIndex<usize>, operation: Operation) {
        // Depending on InvolvedClassical, update both last_ and first_operation_involving_classical
        match operation.involved_classical() {
            InvolvedClassical::Set(x) => {
                // Cycle InvolvedClassical::Set, insert node everywhere in last_operation_involving_classical
                for (name, readout) in &x {
                    // If the classical register has never been seen before, insert it in
                    //  first_operation_involving_classical as well
                    if self
                        .last_operation_involving_classical
                        .insert((String::clone(name), *readout), node)
                        .is_none()
                    {
                        self.first_operation_involving_classical
                            .insert((String::clone(name), *readout), node);
                    }
                }
            }
            InvolvedClassical::All(x) | InvolvedClassical::AllQubits(x) => {
                let mut temp_map: HashMap<(String, usize), NodeIndex<usize>> =
                    HashMap::with_capacity(self.last_operation_involving_classical.capacity());
                // Cycle last_operation_involving_classical
                for (name, readout) in self.last_operation_involving_classical.keys() {
                    // If the classical register's name in InvolvedClassical::All or ::AllQubits
                    //  is present in last_operation_involving_classical, insert the node
                    //  in the temporary HashMap
                    if *name == x {
                        temp_map.insert((String::clone(name), *readout), node);
                    }
                }
                // Update last_operation_involving_classical with the temporary HashMap
                self.last_operation_involving_classical = temp_map.clone();
            }
            InvolvedClassical::None => (),
        }
    }

    /// Checks and updates the relevant classical registers attributes from a given Operation
    /// that was added to the front of the graph.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node of the Operation that was added to the front of the graph.
    /// * 'operation' - The Operation that was added to the front of the graph.
    fn update_classical_front(&mut self, node: NodeIndex<usize>, operation: Operation) {
        // Depending on InvolvedClassical, update both last_ and first_operation_involving_classical
        match operation.involved_classical() {
            InvolvedClassical::Set(x) => {
                // Cycle InvolvedClassical::Set, insert node everywhere in first_operation_involving_classical
                for (name, readout) in &x {
                    // If the classical register has never been seen before, insert it in
                    //  last_operation_involving_classical as well
                    if self
                        .first_operation_involving_classical
                        .insert((String::clone(name), *readout), node)
                        .is_none()
                    {
                        self.last_operation_involving_classical
                            .insert((String::clone(name), *readout), node);
                    }
                }
            }
            InvolvedClassical::All(x) | InvolvedClassical::AllQubits(x) => {
                let mut temp_map: HashMap<(String, usize), NodeIndex<usize>> =
                    HashMap::with_capacity(self.first_operation_involving_classical.capacity());
                // Cycle first_operation_involving_classical
                for (name, readout) in self.first_operation_involving_classical.keys() {
                    // If the classical register's name in InvolvedClassical::All or ::AllQubits
                    //  is present in first_operation_involving_classical, insert the node
                    //  in the temporary HashMap
                    if *name == x {
                        temp_map.insert((String::clone(name), *readout), node);
                    }
                }
                // Update first_operation_involving_classical with the temporary HashMap
                self.first_operation_involving_classical = temp_map.clone();
            }
            InvolvedClassical::None => (),
        }
    }

    /// Checks if executing an Operation is blocked by any previous not-yet executed Operation.
    /// The scope of the check is the whole graph.
    ///
    /// # Arguments:
    ///
    /// * `already_executed` - Slice of NodeIndices of Nodes that have already been executed in the Circuit.
    /// * `to_be_executed` - NodeIndex of the Operation that should be executed next.
    ///
    /// # Returns:
    ///
    /// * `Vec<NodeIndex<usize>>` - Vector containing the sorted blocking elements.
    pub fn execution_blocked(
        &self,
        already_executed: &[NodeIndex<usize>],
        to_be_executed: &NodeIndex<usize>,
    ) -> Vec<NodeIndex<usize>> {
        let mut blocking_elements: Vec<NodeIndex<usize>> = vec![];
        let mut rev_graph: Graph<Operation, (), Directed, usize> = self.graph.clone();
        rev_graph.reverse();
        let mut dfs = Dfs::new(&rev_graph, (*to_be_executed).into());
        dfs.next(&rev_graph);

        // Perform a DFS on the reversed graph starting from to_be_executed,
        //  pushing on blocking_elements the nodes not contained in already_executed
        while let Some(nxt) = dfs.next(&rev_graph) {
            if !already_executed.contains(&nxt.index()) {
                blocking_elements.push(nxt.index());
            }
        }

        blocking_elements.sort_unstable();
        blocking_elements
    }

    /// Checks which of the direct predecessors of an Operation in the CircuitDag blocks the execution.
    ///
    /// # Warning:
    ///
    /// This method can only be used to determine if an operation can be executed when `already_executed` is consistent.
    /// When the list `already_executed` is inconsistent (an operation is reported as executed that could not have been executed yet)
    /// this method returning an empty vector does not imply that the `to_be_executed` operation can be executed.
    ///
    /// # Arguments:
    ///
    /// * `already_executed` - Slice of NodeIndices of Nodes that have already been executed in the Circuit.
    /// * `to_be_executed` - NodeIndex of the Operation that should be executed next.
    pub fn blocking_predecessors(
        &self,
        already_executed: &[NodeIndex<usize>],
        to_be_executed: &NodeIndex<usize>,
    ) -> Vec<NodeIndex<usize>> {
        let mut blocking_elements: Vec<NodeIndex<usize>> = vec![];
        let neighbor_iter = self
            .graph
            .neighbors_directed((*to_be_executed).into(), Incoming);

        for nxt in neighbor_iter {
            if !already_executed.contains(&nxt.index()) {
                blocking_elements.push(nxt.index());
            }
        }

        blocking_elements.sort_unstable();
        blocking_elements
    }

    /// Returns a new front layer after executing an operation from the current front layer.
    /// A front layer is a set of Operations that are ready to be executed, because all required
    /// predecessors in the graph have already been executed.
    ///
    /// Returns an error if operation to be executed is not in the current front layer.
    ///
    /// # Arguments:
    ///
    /// * `already_executed` - Slice of NodeIndices of Nodes that have already been executed in the Circuit.
    /// * `current_front_layer` - Slice of NodeIndices in the current front layer ready to be executed if physically possible.
    /// * `to_be_executed` - NodeIndex of the operation that should be executed next.
    pub fn new_front_layer(
        &self,
        already_executed: &[NodeIndex<usize>],
        current_front_layer: &[NodeIndex<usize>],
        to_be_executed: &NodeIndex<usize>,
    ) -> Result<Vec<NodeIndex<usize>>, RoqoqoError> {
        // Raise an error if the node to be executed is not in the current
        //  front layer
        if !current_front_layer.contains(to_be_executed) {
            Err(RoqoqoError::GenericError {
                msg: "The Operation to be executed is not in the current front layer.".to_string(),
            })
        } else {
            let mut current_front_layer = current_front_layer.to_vec();
            let mut added: bool = false;
            let neighbor_iter = self
                .graph
                .neighbors_directed((*to_be_executed).into(), Outgoing);

            // Boolean needed for end of graph case
            let empty: bool = neighbor_iter.clone().next().is_none();

            // Already_executed vector extension for execution_blocked() compatibility
            let mut extended_a_e: Vec<NodeIndex<usize>> = Vec::from(already_executed);
            extended_a_e.push(*to_be_executed);
            // Cycle through each neighbor of to_be_executed
            for nxt in neighbor_iter {
                // Push the neighbor into the current front layer if it is not execution blocked
                //  by any other node
                if self
                    .execution_blocked(&extended_a_e, &nxt.index())
                    .is_empty()
                {
                    current_front_layer.push(nxt.index());
                    added = true;
                }
            }
            // Remove to_be_executed from the current front layer
            if added || empty {
                current_front_layer.remove(
                    current_front_layer
                        .iter()
                        .position(|&x| x == *to_be_executed)
                        .unwrap(),
                );
            }
            Ok(current_front_layer)
        }
    }

    /// Returns an iterator over the possible parallel blocks in circuit that can be executed simultaneously
    ///
    /// Returns an Iterator over Vectors of references to the NodeIndices in the parallel block as well
    /// as references to the Operation in the blocks
    pub fn parallel_blocks(&self) -> ParallelBlocks {
        ParallelBlocks {
            dag: self,
            parallel_block: Vec::<NodeIndex<usize>>::new(),
            already_executed: Vec::<NodeIndex<usize>>::new(),
        }
    }

    /// Returns an iterator over all successors in the CircuitDag of a given node.
    ///
    pub fn successors(&self, node: NodeIndex<usize>) -> Neighbors<(), usize> {
        self.graph.neighbors_directed(node.into(), Outgoing)
    }

    /// Returns a reference to the vector of commuting operations in CircuitDag.
    ///
    pub fn commuting_operations(&self) -> &Vec<usize> {
        &self.commuting_operations
    }

    /// Returns a reference to the HashSet containing the nodes in the first parallel block.
    ///
    pub fn first_parallel_block(&self) -> &HashSet<usize> {
        &self.first_parallel_block
    }

    /// Returns a reference to the HashSet containing the nodes in the last parallel block.
    ///
    pub fn last_parallel_block(&self) -> &HashSet<usize> {
        &self.last_parallel_block
    }

    /// Returns a reference to the HashMap where a key represents a qubit and its value represents
    /// the first node that involves that qubit.
    ///
    pub fn first_operation_involving_qubit(&self) -> &HashMap<usize, usize> {
        &self.first_operation_involving_qubit
    }

    /// Returns a reference to the HashMap where a key represents a qubit and its value represents
    /// the last node that involves that qubit.
    ///
    pub fn last_operation_involving_qubit(&self) -> &HashMap<usize, usize> {
        &self.last_operation_involving_qubit
    }

    /// Returns a reference to the HashMap where a key is composed by the name and the size
    /// of the classical register and its value represents the first node that involves that
    /// register.
    ///
    pub fn first_operation_involving_classical(&self) -> &HashMap<(String, usize), usize> {
        &self.first_operation_involving_classical
    }

    /// Returns a reference to the HashMap where a key is composed by the name and the size
    /// of the classical register and its value represents the last node that involves that
    /// register.
    ///
    pub fn last_operation_involving_classical(&self) -> &HashMap<(String, usize), usize> {
        &self.last_operation_involving_classical
    }

    /// Given a NodeIndex, returns a reference to the Operation contained in the node of
    /// the CircuitDag.
    ///
    pub fn get(&self, node: NodeIndex<usize>) -> Option<&Operation> {
        self.graph.node_weight(node.into())
    }
}

/// Creates a new CircuitDag from a given Circuit.
///
impl From<Circuit> for CircuitDag {
    fn from(circuit: Circuit) -> Self {
        let mut new_dag = CircuitDag {
            graph: Graph::<Operation, (), Directed, usize>::with_capacity(
                circuit.len(),
                circuit.operations().len(),
            ),
            commuting_operations: Vec::<NodeIndex<usize>>::new(),
            first_parallel_block: HashSet::<NodeIndex<usize>>::new(),
            last_parallel_block: HashSet::<NodeIndex<usize>>::new(),
            first_all: Option::<NodeIndex<usize>>::None,
            last_all: Option::<NodeIndex<usize>>::None,
            first_operation_involving_qubit: HashMap::<usize, NodeIndex<usize>>::new(),
            last_operation_involving_qubit: HashMap::<usize, NodeIndex<usize>>::new(),
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex<usize>>::new(
            ),
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex<usize>>::new(),
            _roqoqo_version: RoqoqoVersion,
        };

        for operation in circuit.iter() {
            new_dag.add_to_back(operation.clone());
        }

        new_dag
    }
}

/// Creates a new Circuit from a given CircuitDag.
///
impl From<CircuitDag> for Circuit {
    fn from(dag: CircuitDag) -> Circuit {
        let mut circuit: Circuit = Circuit::new();

        match toposort(&dag.graph, None) {
            Ok(order) => {
                for i in order {
                    circuit.add_operation(dag.graph.node_weight(i).unwrap().clone());
                }
            }
            Err(_) => {
                println!("Error: graph not acyclic");
            }
        }

        circuit
    }
}

impl<'a> Iterator for ParallelBlocks<'a> {
    type Item = Vec<NodeIndex<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        // First case
        if self.parallel_block.is_empty() && self.already_executed.is_empty() {
            for node in &self.dag.first_parallel_block {
                self.parallel_block.push(*node);
            }
            return Some(self.parallel_block.clone());
        }

        // Populate already_executed with current parallel_block
        for node in &self.parallel_block {
            self.already_executed.push(*node);
        }

        let mut new_parallel_block: Vec<NodeIndex<usize>> = Vec::new();
        // Cycle through the current parallel block
        for node in &self.parallel_block {
            let neighbor_iter = self.dag.graph.neighbors_directed((*node).into(), Outgoing);
            // Cycle through its neighbors
            for nxt in neighbor_iter {
                // Add the neighbor to the new parallel block if ready to be executed and has not been pushed before
                if self
                    .dag
                    .execution_blocked(self.already_executed.as_slice(), &nxt.index())
                    .is_empty()
                    && !new_parallel_block.iter().any(|id| *id == nxt.index())
                {
                    new_parallel_block.push(nxt.index());
                }
            }
        }

        // Update parallel_block and return it
        self.parallel_block = new_parallel_block.clone();
        if new_parallel_block.is_empty() {
            return None;
        }
        Some(new_parallel_block)
    }
}

impl crate::operations::SupportedVersion for CircuitDag {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        let mut current_minimum_version = (1, 0, 0);
        for index in self.graph.node_indices() {
            let node_op = self.get(index.index()).unwrap();
            let comparison_version = node_op.minimum_supported_roqoqo_version();
            crate::update_roqoqo_version(&mut current_minimum_version, comparison_version);
        }
        current_minimum_version
    }
}

/// The following module contains tests that need access to internal data structures.
///
#[cfg(test)]
mod tests {
    use crate::operations::*;
    use crate::{Circuit, CircuitDag};
    use test_case::test_case;

    static DEFAULT_NODE_NUMBER: usize = 10;
    static DEFAULT_EDGE_NUMBER: usize = 30;

    /// Test graph node existance after adding an operation that involves qubits.
    ///
    #[test_case(Operation::from(PauliX::new(0)))]
    #[test_case(Operation::from(PauliY::new(1)))]
    #[test_case(Operation::from(ControlledPauliZ::new(0, 1)))]
    fn check_node_existance(operation: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        dag.add_to_back(operation.clone());

        assert!(dag.graph.node_count() == 1);

        dag.add_to_front(operation);
        dag.add_to_back(Operation::from(CNOT::new(0, 1)));

        assert!(dag.graph.node_count() == 3);
    }

    #[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(0)))]
    #[test_case(Operation::from(PauliZ::new(0)), Operation::from(CNOT::new(0, 1)))]
    fn check_node_count(operation1: Operation, operation2: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        dag.add_to_back(operation1.clone());
        dag.add_to_front(operation2);

        assert!(dag.graph.node_count() == 2);

        dag.add_to_back(operation1);

        assert!(dag.graph.node_count() == 3);
    }

    #[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(0)))]
    #[test_case(
        Operation::from(PauliX::new(0)),
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None,))
    )]
    #[test_case(
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None,)),
        Operation::from(PauliX::new(0))
    )]
    #[test_case(
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None,)),
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None,))
    )]
    fn check_edge(operation1: Operation, operation2: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);
        dag.add_to_back(Operation::from(DefinitionBit::new(
            "ro".to_string(),
            4,
            false,
        )));

        let ind1 = dag.add_to_back(operation1);
        let ind2 = dag.add_to_back(operation2);

        assert!(dag
            .graph
            .contains_edge(ind1.unwrap().into(), ind2.unwrap().into()));
    }

    #[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)))]
    #[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ri"), 2, None)))]
    fn check_first_last_all_existence(operation: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        assert!(dag.first_all.is_none());
        assert!(dag.last_all.is_none());

        let ind_back = dag.add_to_back(operation.clone());
        let ind_front = dag.add_to_front(operation);

        assert!(dag.first_all.is_some());
        assert!(dag.last_all.is_some());

        assert!(dag.first_all.unwrap() == ind_front.unwrap());
        assert!(dag.last_all.unwrap() == ind_back.unwrap());
    }

    #[test_case(
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)),
        Operation::from(PragmaRepeatedMeasurement::new(String::from("ri"), 2, None))
    )]
    fn check_first_last_all_order(operation1: Operation, operation2: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        dag.add_to_back(operation1);
        dag.add_to_front(operation2);

        assert!(dag.first_all.is_some());
        assert!(dag.last_all.is_some());

        assert_ne!(
            dag.graph.node_weight(dag.first_all.unwrap().into()),
            dag.graph.node_weight(dag.last_all.unwrap().into())
        );
    }

    #[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None,)))]
    #[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ri"), 2, None,)))]
    fn check_operation_involving_qubits_all(operation: Operation) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        assert!(dag.first_operation_involving_qubit().is_empty());
        assert!(dag.last_operation_involving_qubit().is_empty());

        dag.add_to_front(operation.clone());

        let back = dag.add_to_back(Operation::from(PauliX::new(0)));

        assert_eq!(dag.last_operation_involving_qubit().get(&0), back.as_ref());

        let front = dag.add_to_front(Operation::from(CNOT::new(0, 1)));

        assert_eq!(dag.last_operation_involving_qubit().get(&0), back.as_ref());
        assert_eq!(
            dag.first_operation_involving_qubit().get(&0),
            front.as_ref()
        );

        assert_ne!(dag.last_operation_involving_qubit().get(&0), front.as_ref());
        assert_ne!(dag.first_operation_involving_qubit().get(&0), back.as_ref());

        let new_front_all = dag.add_to_front(operation.clone());
        let new_back_all = dag.add_to_back(operation);

        assert!(dag
            .graph
            .contains_edge(new_front_all.unwrap().into(), front.unwrap().into()));
        assert!(dag
            .graph
            .contains_edge(back.unwrap().into(), new_back_all.unwrap().into()));
    }

    #[test_case(vec![Operation::from(CNOT::new(0,1)), Operation::from(PauliX::new(0)), Operation::from(PauliY::new(1))])]
    #[test_case(vec![Operation::from(PauliZ::new(0)), Operation::from(ControlledPauliZ::new(1,2))])]
    fn test_new_from_circuit(op_vec: Vec<Operation>) {
        let mut circuit: Circuit = Circuit::new();
        for op in &op_vec {
            circuit.add_operation((*op).clone());
        }

        let dag: CircuitDag = CircuitDag::from(circuit);

        assert!(!dag.first_operation_involving_qubit().is_empty());
        assert!(!dag.last_operation_involving_qubit().is_empty());

        assert_eq!(dag.graph.node_count(), op_vec.len());
    }

    #[test_case(vec![Operation::from(CNOT::new(0,1)), Operation::from(PauliX::new(0)), Operation::from(PauliY::new(0)), Operation::from(PauliZ::new(0))])]
    #[test_case(vec![Operation::from(PauliZ::new(0)), Operation::from(ControlledPauliZ::new(0,1))])]
    fn test_from_circuitdag(op_vec: Vec<Operation>) {
        let mut dag: CircuitDag =
            CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

        for op in &op_vec {
            dag.add_to_back(op.clone());
        }

        let circuit = Circuit::from(dag.clone());

        assert_eq!(circuit.len(), dag.graph.node_count());

        circuit
            .iter()
            .enumerate()
            .into_iter()
            .for_each(|(ind, oper)| {
                assert_eq!(*oper, *dag.graph.node_weight(ind.into()).unwrap());
            });
    }
}
