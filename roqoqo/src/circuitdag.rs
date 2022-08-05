// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use petgraph::adj::NodeIndex;
use petgraph::graph::Graph;
use petgraph::Directed;

static DEFAULT_NODE_NUMBER: usize = 100;
static DEFAULT_EDGE_NUMBER: usize = 300;

/// Represents the Direct Acyclic Graph (DAG) of a Circuit.
/// 
#[derive(Debug, Clone)]
pub struct CircuitDag {
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
}

impl Default for CircuitDag {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: temporary solution, use is_isomorphic
impl PartialEq for CircuitDag {
    fn eq(&self, other: &Self) -> bool {
        for ind in self.graph.node_indices().into_iter() {
            if self.graph.node_weight(ind) != other.graph.node_weight(ind) {
                return false;
            }
        }
        for ind in other.graph.node_indices().into_iter() {
            if self.graph.node_weight(ind) != other.graph.node_weight(ind) {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other)
    }
}

impl CircuitDag {
    /// Creates a new empty CircuitDag.
    ///
    pub fn new() -> Self {
        CircuitDag {
            graph: Graph::<Operation, (), Directed, usize>::with_capacity(
                DEFAULT_NODE_NUMBER,
                DEFAULT_EDGE_NUMBER,
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
    pub fn add_to_back(&mut self, operation: Operation) -> Option<NodeIndex<usize>> {
        // Create node
        let node = self.graph.add_node(operation.clone());

        // InvolvedQubits: push to commuting_operations or start the add to back process
        if let (InvolvedQubits::None, InvolvedClassical::None) =
            (operation.involved_qubits(), operation.involved_classical())
        {
            self.commuting_operations.push(node.index());
        } else {
            self.add_to_back_involved(node.index());
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
            self.graph.add_edge(i.into(), node.into(), ());
            self.last_parallel_block.remove(&i);
        }
        let qubit_presence = self.last_operation_involving_qubit.insert(qubit, node);
        self.last_parallel_block.insert(node);

        // Update the first layer in case the qubit has never been seen before
        if qubit_presence.is_none() {
            // Update first_operation_involving_qubit and first_parallel_block
            //  depending on last_all
            if self.last_all.is_none() {
                self.first_operation_involving_qubit.insert(qubit, node);
                self.first_parallel_block.insert(node);
            } else {
                self.first_operation_involving_qubit
                    .insert(qubit, self.last_all.unwrap());
                self.first_parallel_block.insert(self.last_all.unwrap());
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

        // All the latest nodes in the graph must now point to the new node and
        //  last_operation_involving_qubit is updated
        let mut temp_map: HashMap<usize, NodeIndex<usize>> =
            HashMap::with_capacity(self.last_operation_involving_qubit.capacity());
        for (&qubit, &old_node) in &self.last_operation_involving_qubit {
            self.graph.update_edge(old_node.into(), node.into(), ());
            temp_map.insert(qubit, node);
        }
        self.last_operation_involving_qubit.clear();
        self.last_operation_involving_qubit = temp_map.clone();
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
    pub fn add_to_front(&mut self, operation: Operation) -> Option<NodeIndex<usize>> {
        // Create node
        let node = self.graph.add_node(operation.clone());

        // InvolvedQubits: push to commuting_operations or start the add to front process
        if let (InvolvedQubits::None, InvolvedClassical::None) =
            (operation.involved_qubits(), operation.involved_classical())
        {
            self.commuting_operations.push(node.index());
        } else {
            self.add_to_front_involved(node.index());
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
            self.graph.add_edge(node.into(), i.into(), ());
            self.first_parallel_block.remove(&i);
        }
        let qubit_presence = self.first_operation_involving_qubit.insert(qubit, node);
        self.first_parallel_block.insert(node);

        // Update the last layer in case the qubit has never been seen before
        if qubit_presence.is_none() {
            // Update last_operation_involving_qubit and last_parallel_block
            //  depending on first_all
            if self.first_all.is_none() {
                self.last_operation_involving_qubit.insert(qubit, node);
                self.last_parallel_block.insert(node);
            } else {
                self.last_operation_involving_qubit
                    .insert(qubit, self.first_all.unwrap());
                self.last_parallel_block.insert(self.first_all.unwrap());
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

        // All the latest nodes in the graph must now point to the new node and
        //  last_operation_involving_qubit is updated
        let mut temp_map: HashMap<usize, NodeIndex<usize>> =
            HashMap::with_capacity(self.first_operation_involving_qubit.capacity());
        for (&qubit, &old_node) in &self.first_operation_involving_qubit {
            self.graph.update_edge(node.into(), old_node.into(), ());
            temp_map.insert(qubit, node);
        }
        self.first_operation_involving_qubit.clear();
        self.first_operation_involving_qubit = temp_map.clone();
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
                        .insert((String::clone(name), *readout), node).is_none()
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
                        .insert((String::clone(name), *readout), node).is_none()
                    {
                        self.last_operation_involving_classical
                            .insert((String::clone(name), *readout), node);
                    }
                }
            }
            InvolvedClassical::All(x) | InvolvedClassical::AllQubits(x) => {
                // Cycle first_operation_involving_classical
                let mut temp_map: HashMap<(String, usize), NodeIndex<usize>> =
                    HashMap::with_capacity(self.first_operation_involving_classical.capacity());
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

    /// Returns a reference to the graph in CircuitDag.
    ///
    pub fn graph(&self) -> &Graph<Operation, (), Directed, usize> {
        &self.graph
    }

    /// Returns a reference to the vector of commuting operations in CircuitDag.
    ///
    pub fn commuting_operations(&self) -> &Vec<NodeIndex<usize>> {
        &self.commuting_operations
    }

    /// Returns a reference to the HasSet containing the nodes in the first parallel block.
    ///
    pub fn first_parallel_block(&self) -> &HashSet<NodeIndex<usize>> {
        &self.first_parallel_block
    }

    /// Returns a reference to the HashSet containing the nodes in the last parallel block.
    ///
    pub fn last_parallel_block(&self) -> &HashSet<NodeIndex<usize>> {
        &self.last_parallel_block
    }

    /// Returns a reference to the first Operation that involves all qubits in CircuitDag.
    ///
    pub fn first_all(&self) -> &Option<NodeIndex<usize>> {
        &self.first_all
    }

    /// Returns a reference to the last Operation that involves all qubits in CircuitDag.
    ///
    pub fn last_all(&self) -> &Option<NodeIndex<usize>> {
        &self.last_all
    }

    /// Returns a reference to the HashMap where a key represents a qubit and its value represents
    /// the first node that involves that qubit.
    ///
    pub fn first_operation_involving_qubit(&self) -> &HashMap<usize, NodeIndex<usize>> {
        &self.first_operation_involving_qubit
    }

    /// Returns a reference to the HashMap where a key represents a qubit and its value represents
    /// the last node that involves that qubit.
    ///
    pub fn last_operation_involving_qubit(&self) -> &HashMap<usize, NodeIndex<usize>> {
        &self.last_operation_involving_qubit
    }

    /// Returns a reference to the HashMap where a key is composed by the name and the size
    /// of the classical register and its value represents the first node that involves that
    /// register.
    ///
    pub fn first_operation_involving_classical(
        &self,
    ) -> &HashMap<(String, usize), NodeIndex<usize>> {
        &self.first_operation_involving_classical
    }

    /// Returns a reference to the HashMap where a key is composed by the name and the size
    /// of the classical register and its value represents the last node that involves that
    /// register.
    ///
    pub fn last_operation_involving_classical(
        &self,
    ) -> &HashMap<(String, usize), NodeIndex<usize>> {
        &self.last_operation_involving_classical
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
        };

        for operation in circuit.operations() {
            new_dag.add_to_back(operation.clone());
        }

        new_dag
    }
}