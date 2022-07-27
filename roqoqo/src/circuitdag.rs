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

use std::collections::{HashSet, HashMap};

use crate::operations::*;

use petgraph::adj::{NodeIndex, EdgeIndex};
use petgraph::graph::{Graph};

/// Represents a Direct Acyclic Graph (DAG) 
#[derive(Debug)]
pub struct CircuitDag {
    graph: Graph<Operation, EdgeIndex>,
    commuting_operations: Vec<Operation>,
    first_parallel_block: HashSet<NodeIndex>,
    last_parallel_block: HashSet<NodeIndex>,
    /// None if no Operation with InvolvedQubits::All in circuit
    first_all: Option<usize>,
    /// None if no Operation with InvolvedQubits::All in circuit
    last_all: Option<usize>,
    first_operation_involving_qubit: HashMap<usize, NodeIndex>,
    last_operation_involving_qubit: HashMap<usize, NodeIndex>,
    first_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
    last_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
}

impl CircuitDag {
    /// Creates a new empty CircuitDag.
    pub fn new() -> Self{
        CircuitDag { 
            graph: Graph::<Operation, EdgeIndex>::new(), 
            commuting_operations: Vec::<Operation>::new(), 
            first_parallel_block: HashSet::<NodeIndex>::new(), 
            last_parallel_block: HashSet::<NodeIndex>::new(), 
            first_all: Option::<usize>::None,
            last_all: Option::<usize>:: None, 
            first_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            last_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new(), 
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new()
        }
    }

    /// Adds an operation to the back of the CircuitDag.
    /// 
    /// # Arguments
    /// 
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    pub fn add_to_back(&mut self, operation: Operation) -> Option<NodeIndex>{
        let index:Option<NodeIndex> = None;
        match operation.involved_qubits() {
            InvolvedQubits::None => self.commuting_operations.push(operation),
            InvolvedQubits::All | InvolvedQubits::Set(_) => index = self.add_to_back_involved(operation)
        };

        return Some(index)
    }

    /// Adds an operation that involves some or all qubits to the end of the CircuitDag.
    /// 
    /// # Arguments
    /// 
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    fn add_to_back_involved(&mut self, operation: Operation) -> NodeIndex{
        let node = self.graph.add_node(operation.clone());

        if operation.clone().involved_qubits() == InvolvedQubits::All {
            if self.first_all.is_none() {
                self.first_all.insert(node.index());
            }
            self.last_all.insert(node.index());

        }

        return node.index().try_into().unwrap()
    }

    /// Returns a reference to the Operation at index.
    /// 
    /// # Arguments
    /// 
    /// * 'index' - The index of the Operation to get from CircuitDag.
    pub fn get_op(&self, index: usize) -> Option<&Operation> {
        self.commuting_operations.get(index)
    }

    /// Returns a reference to the vector of commuting operations in CircuitDag.
    /// 
    pub fn commuting_operations(&self) -> &Vec<Operation> {
        &self.commuting_operations
    }

    /// Returns a reference to the graph in CircuitDag.
    /// 
    pub fn graph(&self) -> &Graph<Operation, EdgeIndex> {
        &self.graph
    }

    /// Returns a reference to the first Operation that involves all qubits in CircuitDag.
    /// 
    pub fn first_all(&self) -> &Option<usize> {
        &self.first_all
    }

    /// Returns a reference to the last Operation that involves all qubits in CircuitDag.
    pub fn last_all(&self) -> &Option<usize> {
        &self.last_all
    }

}