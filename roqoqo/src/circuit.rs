// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::operations::{
    Define, InvolveQubits, InvolvedQubits, Operate, Operation, Substitute, SupportedVersion,
};
#[cfg(feature = "overrotate")]
use crate::operations::{Rotate, Rotation};
use crate::RoqoqoError;
use crate::RoqoqoVersion;
#[cfg(feature = "serialize")]
use crate::RoqoqoVersionSerializable;
use qoqo_calculator::Calculator;
#[cfg(feature = "overrotate")]
use std::convert::TryFrom;
use std::ops;
use std::{
    collections::{HashMap, HashSet},
    usize,
};
use std::{
    fmt::{Display, Formatter, Write},
    iter::{FromIterator, IntoIterator},
};

/// Represents a quantum circuit in roqoqo.
///
/// In roqoqo, single operations are collected in a circuit to build up a quantum program.
/// Roqoqo circuits are strictly linear sequences of operations.
/// The circuit struct behaves similar to a list and provides several standard
/// functions of a Vec<Operation>, such as len(), is_empty(), get(), iter() and into_iter().
///
/// # Example
///
/// ```
/// use roqoqo::Circuit;
/// use roqoqo::operations::{Operation, RotateX};
/// use qoqo_calculator::CalculatorFloat;
/// // creating circuit
/// let mut circuit = Circuit::new();
/// // adding operation to circuit
/// circuit.add_operation(RotateX::new(0,CalculatorFloat::from(0)));
/// assert_eq!(circuit.len(), 1);
/// // iterating over circuit I
/// let operation_vector: Vec<&Operation>= circuit.iter().collect();
/// // iterating over circuit II
/// for op in circuit{
///    println!("{:?}", op);
/// }
/// // collecting operations into circuit
/// let vector = vec![Operation::from(RotateX::new(0,CalculatorFloat::from(0))), Operation::from(RotateX::new(0,CalculatorFloat::from(0)))];
/// let new_circuit: Circuit = vector.into_iter().collect();
/// ```
///
/// Similarly to single Operations, Circuits can be translated to other frameworks via interfaces.
///
/// For Circuits the following functions are defined:
/// * `new()`: creates an empty Circuit
/// * `add_operation(operation)`: adds the specified operation to the Circuit
/// * `get(index)`: returns the operation at the specified index in the Circuit
/// * `get_mut(index)`: returns mutable reference to the operation at the specified index in the Circuit
/// * `iter()`: creates an iterator of the Circuit
/// * `len()`: returns the length of the Circuit
/// * `is_empty()`: returns a boolean of whether the Circuit contains any definitions and operations or not
/// * `involved_qubits()`: returns the qubits invovlved in the whole Circuit
/// * `definitions()`: returns the definitions in the Circuit
/// * `operations()`: returns the operations in the Circuit
/// * `substitute_parameters(calculator)`: substitutes any symbolic parameters in (a copy of) the Circuit according to the specified Calculator
/// * `remap_qubits(mapping)`: remaps the qubits in (a copy of) the Circuit according to the specified mapping
/// * `count_occurences(operations)`: returns the number of operations in the Circuit with the specified operation tags
/// * `get_operation_types()`: returns a list of all of the operations in the Circuit (in hqslang)
/// * `from_iter(iterator)`: creates a Circuit from the items in the specified iterator
/// * `extend(iterator)`: adds the operations in the specified iterator to the Circuit
/// * `default()`: creates an empty Circuit
/// * `[...]`: gets a slice of the Circuit (returned as a vector)
/// * `+` and `+=`: add two circuits or an operation to the Circuit
///
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serialize", serde(try_from = "CircuitSerializable"))]
#[cfg_attr(feature = "serialize", serde(into = "CircuitSerializable"))]
pub struct Circuit {
    /// Definitions in the quantum circuit, must be unique.
    definitions: Vec<Operation>,
    /// Operations of the quantum circuit, do not have to be unique.
    operations: Vec<Operation>,
    /// The roqoqo version.
    _roqoqo_version: RoqoqoVersion,
}

#[cfg(feature = "serialize")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename = "Circuit"))]
struct CircuitSerializable {
    /// Definitions in the quantum circuit, must be unique.
    definitions: Vec<Operation>,
    /// Operations of the quantum circuit, do not have to be unique.
    operations: Vec<Operation>,
    /// The roqoqo version.
    _roqoqo_version: RoqoqoVersionSerializable,
}

#[cfg(feature = "serialize")]
impl TryFrom<CircuitSerializable> for Circuit {
    type Error = RoqoqoError;
    fn try_from(value: CircuitSerializable) -> Result<Self, Self::Error> {
        Ok(Circuit {
            definitions: value.definitions,
            operations: value.operations,
            _roqoqo_version: RoqoqoVersion,
        })
    }
}

#[cfg(feature = "serialize")]
impl From<Circuit> for CircuitSerializable {
    fn from(value: Circuit) -> Self {
        let min_version = value.minimum_supported_roqoqo_version();
        let current_version = RoqoqoVersionSerializable {
            major_version: min_version.0,
            minor_version: min_version.1,
        };
        Self {
            definitions: value.definitions,
            operations: value.operations,
            _roqoqo_version: current_version,
        }
    }
}

impl Circuit {
    /// Creates an empty quantum Circuit.
    ///
    /// # Returns
    ///
    /// * `Self` - The empty Circuit.
    pub fn new() -> Self {
        Circuit {
            definitions: Vec::new(),
            operations: Vec::new(),
            _roqoqo_version: RoqoqoVersion,
        }
    }
    /// Adds an Operation to Circuit (self).
    ///
    /// # Arguments
    ///
    /// * `op` - The Operation to add to the Circuit.
    pub fn add_operation<T>(&mut self, op: T)
    where
        T: Into<Operation>,
    {
        let input: Operation = op.into();
        match &input {
            Operation::DefinitionBit(_) => self.definitions.push(input),
            Operation::DefinitionFloat(_) => {
                self.definitions.push(input);
            }
            Operation::DefinitionComplex(_) => {
                self.definitions.push(input);
            }
            Operation::DefinitionUsize(_) => {
                self.definitions.push(input);
            }
            Operation::InputSymbolic(_) => {
                self.definitions.push(input);
            }
            _ => self.operations.push(input),
        }
    }

    /// Returns a reference to the element at index similar to std::Vec get function.
    ///
    /// Contrary to std::Vec get function not implemented for slices  .
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Operation to get in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Option<&Operation>` - The operation at the given index (if it exists).
    pub fn get(&self, index: usize) -> Option<&Operation> {
        let def_len = self.definitions.len();
        if index >= self.definitions.len() {
            self.operations.get(index - def_len)
        } else {
            self.definitions.get(index)
        }
    }

    /// Returns a mutable reference to the element at index similar to std::Vec get function.
    ///
    /// Contrary to std::Vec get function not implemented for slices.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Operation to get in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Option<mut &Operation>` - A mutable reference to the operation at the given index (if it exists).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Operation> {
        let def_len = self.definitions.len();
        if index >= self.definitions.len() {
            self.operations.get_mut(index - def_len)
        } else {
            self.definitions.get_mut(index)
        }
    }

    /// Creates an iterator of the Circuit.
    ///
    /// # Returns
    ///
    /// `Iterator<Item = &Operation>` - The Circuit in iterator form.
    pub fn iter(&self) -> impl Iterator<Item = &Operation> {
        self.definitions.iter().chain(self.operations.iter())
    }

    /// Returns true if the Circuit contains symbolic variables.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the Circuit contains symbolic values, false if it does not.
    pub fn is_parametrized(&self) -> bool {
        self.operations.iter().any(|o| o.is_parametrized())
            || self.definitions.iter().any(|o| o.is_parametrized())
    }

    /// Returns the length of the Circuit.
    ///
    /// # Returns
    ///
    /// * `usize` - The length of the Circuit.
    pub fn len(&self) -> usize {
        self.definitions.len() + self.operations.len()
    }

    /// Returns true if the Circuit does not contain any operations and definitions.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the Circuit is empty, false if it is not.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty() && self.operations.is_empty()
    }

    /// Returns qubits the Circuit acts on.
    ///
    /// # Returns
    ///
    /// * `InvolvedQubits` - The qubits involved in the Circuit.
    pub fn involved_qubits(&self) -> InvolvedQubits {
        let mut temp_involved: HashSet<usize> = HashSet::new();
        for op in self.operations.iter() {
            match &op.involved_qubits() {
                InvolvedQubits::All => {
                    return InvolvedQubits::All;
                }
                InvolvedQubits::None => (),
                InvolvedQubits::Set(x) => temp_involved = temp_involved.union(x).cloned().collect(),
            }
        }
        match temp_involved.is_empty() {
            true => InvolvedQubits::None,
            false => InvolvedQubits::Set(temp_involved),
        }
    }

    /// Returns reference to the vector of definitions in Circuit.
    ///
    /// Definitions need to be unique.
    ///
    /// # Returns
    ///
    /// * `&Vec<Operation>` - A vector of the definitions in the Circuit.
    pub fn definitions(&self) -> &Vec<Operation> {
        &self.definitions
    }

    /// Returns reference to the vector of quantum operations in Circuit.
    ///
    /// Operations do not need to be unique.
    ///
    /// # Returns
    ///
    /// * `&Vec<Operation>` - A vector of the operations in the Circuit.
    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    /// Substitutes the symbolic parameters in a clone of Circuit according to the calculator input.
    ///
    /// # Arguments
    ///
    /// * ``calculator` - The Calculator containing the substitutions to use in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The Circuit with the parameters substituted.
    /// * `Err(RoqoqoError)` - The subsitution failed.
    pub fn substitute_parameters(&self, calculator: &Calculator) -> Result<Self, RoqoqoError> {
        let mut tmp_calculator = calculator.clone();
        let mut tmp_def: Vec<Operation> = Vec::new();
        for def in self.definitions.iter() {
            let tmp_op = def.substitute_parameters(&tmp_calculator)?;
            if let Operation::InputSymbolic(x) = &tmp_op {
                tmp_calculator.set_variable(x.name(), *x.input())
            }
            tmp_def.push(tmp_op);
        }
        let mut tmp_op: Vec<Operation> = Vec::new();
        for op in self.operations.iter() {
            tmp_op.push(op.substitute_parameters(&tmp_calculator)?);
        }
        Ok(Self {
            definitions: tmp_def,
            operations: tmp_op,
            _roqoqo_version: RoqoqoVersion,
        })
    }
    /// Remaps the qubits in operations in clone of Circuit.
    ///
    /// # Arguments
    ///
    /// * ``mapping` - The HashMap containing the {qubit: qubit} mapping to use in the Circuit.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The Circuit with the qubits remapped.
    /// * `Err(RoqoqoError)` - The remapping failed.
    pub fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let mut tmp_op: Vec<Operation> = Vec::new();
        for op in self.operations.iter() {
            tmp_op.push(op.remap_qubits(mapping)?);
        }
        Ok(Self {
            definitions: self.definitions.clone(),
            operations: tmp_op,
            _roqoqo_version: RoqoqoVersion,
        })
    }

    /// Counts the number of occurences of a set of operation tags in the circuit.
    ///
    /// # Arguments
    ///
    /// `operations` - The list of operation tags that should be counted.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of occurences of these operation tags.
    pub fn count_occurences(&self, operations: &[&str]) -> usize {
        let mut counter: usize = 0;
        for op in self.iter() {
            if operations.iter().any(|x| op.tags().contains(x)) {
                counter += 1
            }
        }
        counter
    }

    /// Returns a list of the hqslang names of all operations occuring in the circuit.
    ///
    /// # Returns
    ///
    /// * `HashSet<&str>` - The operation types in the Circuit.
    pub fn get_operation_types(&self) -> HashSet<&str> {
        let mut operations: HashSet<&str> = HashSet::new();
        for op in self.iter() {
            let _ = operations.insert(op.hqslang());
        }
        operations
    }

    /// Returns clone of the circuit with all Overrotation Pragmas applied.
    ///
    /// # Returns
    ///
    /// * `Ok(Circuit)` - The Circuit with overrotations applied.
    /// * `Err(RoqoqoError::OverrotationError)` - Applying overrotations failed.
    ///
    /// # Example
    ///
    /// ```
    /// use roqoqo::Circuit;
    /// use roqoqo::operations::{PragmaOverrotation, RotateX, RotateY};
    /// let mut circuit = Circuit::new();
    /// // Adding Overrotation of next RotateY operation acting on qubit 1
    /// // overrotating parameter theta with a statistical value
    /// // value is drawn from normal distribution with standard deviation 30.0
    /// // and multiplied by amplitude 20.0
    /// circuit += PragmaOverrotation::new("RotateY".to_string(), vec![1], 20.0, 30.0);
    /// circuit += RotateX::new(0, 0.0.into());
    /// circuit += RotateY::new(0, 1.0.into());
    /// circuit += RotateY::new(1, 2.0.into());
    /// circuit += RotateY::new(1, 3.0.into());
    ///
    /// let circuit_overrotated = circuit.overrotate().unwrap();
    ///
    /// println!("{}", circuit);
    /// println!("{}", circuit_overrotated);
    /// ```
    ///
    #[cfg(feature = "overrotate")]
    pub fn overrotate(&self) -> Result<Self, RoqoqoError> {
        let mut tmp_vec = self.operations.clone();
        let mut return_circuit = Circuit {
            definitions: self.definitions.clone(),
            operations: Vec::new(),
            _roqoqo_version: RoqoqoVersion,
        };
        let mut length = tmp_vec.len();
        while length > 0 {
            match tmp_vec
                .iter()
                .enumerate()
                .find(|(_, op)| op.hqslang() == "PragmaOverrotation")
                .map(|(i, op)| (i, op.clone()))
            {
                Some((index, Operation::PragmaOverrotation(overrotation))) => {
                    // for op in tmp_vec[..index].iter() {
                    //     return_circuit.operations.push(op.clone())
                    // }
                    let hqslang = overrotation.gate_hqslang();
                    match tmp_vec[index..].iter().enumerate().find(|(_, op)| {
                        hqslang == op.hqslang()
                            && overrotation.involved_qubits() == op.involved_qubits()
                    }) {
                        Some((ind, _)) => {
                            let mut tmp_tmp_vec: Vec<Operation> = Vec::new();
                            for (mov_ind, op) in tmp_vec.into_iter().enumerate() {
                                if mov_ind == index + ind {
                                    tmp_tmp_vec.push(
                                        Rotation::try_from(op)?
                                            .overrotate(
                                                overrotation.amplitude(),
                                                overrotation.variance(),
                                            )
                                            .into(),
                                    )
                                } else if index != mov_ind {
                                    tmp_tmp_vec.push(op)
                                }
                            }
                            tmp_vec = tmp_tmp_vec
                        }
                        None => {
                            let mut tmp_tmp_vec: Vec<Operation> = Vec::new();
                            for (mov_ind, op) in tmp_vec.into_iter().enumerate() {
                                if index != mov_ind {
                                    tmp_tmp_vec.push(op)
                                }
                            }
                            tmp_vec = tmp_tmp_vec
                        }
                    }
                }
                _ => {
                    for op in tmp_vec {
                        return_circuit.operations.push(op)
                    }
                    tmp_vec = Vec::new();
                }
            }
            length = tmp_vec.len();
        }
        Ok(return_circuit)
    }
}

/// Implements Index Access for Circuit.
///
/// # Panics
///
/// Panics when index is out of range of operations in circuit.
/// This is consistent with standard Vec behaviour
/// and returning Option or Result enums instead would conflict with definition of Output type.
impl ops::Index<usize> for Circuit {
    type Output = Operation;

    /// Returns reference to Operation at index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the operation.
    ///
    /// # Panics
    ///
    /// Panics when index is out of range of operations in circuit.
    fn index(&self, index: usize) -> &Self::Output {
        let def_len = self.definitions.len();
        if index >= def_len {
            &self.operations[index - def_len]
        } else {
            &self.definitions[index]
        }
    }
}

impl ops::IndexMut<usize> for Circuit {
    /// Returns reference to Operation at index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the operation.
    ///
    /// # Panics
    ///
    /// Panics when index is out of range of operations in circuit.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let def_len = self.definitions.len();
        if index >= def_len {
            &mut self.operations[index - def_len]
        } else {
            &mut self.definitions[index]
        }
    }
}

impl IntoIterator for Circuit {
    type Item = Operation;
    type IntoIter = OperationIterator;
    /// Returns the Circuit in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The Circuit in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            definition_iter: self.definitions.into_iter(),
            operation_iter: self.operations.into_iter(),
        }
    }
}

impl<T> FromIterator<T> for Circuit
where
    T: Into<Operation>,
{
    /// Returns the circuit in Circuit form, from an Iterator form of the circuit.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The Circuit in Circuit form.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut circuit = Circuit::new();
        for op in iter {
            circuit.add_operation(op.into());
        }
        circuit
    }
}

impl<T> Extend<T> for Circuit
where
    T: Into<Operation>,
{
    /// Extends the Circuit by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the Circuit.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for op in iter {
            self.add_operation(op.into());
        }
    }
}

impl Default for Circuit {
    /// Creates a default implementation of the Circuit, which is an empty Circuit.
    ///
    /// # Returns
    ///
    /// * `Self` - The default Circuit (empty).
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for returning Vectors based on Range structs usually used for Index<> trait and slice access.
///
/// Required because Circuit does not have a continuous internal vector representation of the values.
/// Returns a Vec instead of slices.
///
/// # Example
///
/// ```
/// use roqoqo::{Circuit, AsVec};
/// use roqoqo::operations::{DefinitionFloat, Operation, RotateZ};
/// use qoqo_calculator::CalculatorFloat;
///
/// let mut circuit = Circuit::new();
/// let definition = DefinitionFloat::new(String::from("ro"), 1, false);
/// let rotatez0 = RotateZ::new(0, CalculatorFloat::from(0.0));
/// circuit.add_operation(definition.clone());
/// circuit.add_operation(rotatez0.clone());
///
/// let vec_ops = vec![
///     Operation::from(definition.clone()),
///     Operation::from(rotatez0.clone()),
/// ];
///
/// assert_eq!(circuit.as_vec(0..1).clone(), Some(vec![vec_ops[0].clone()])); // Range
/// assert_eq!(circuit.as_vec(0..).clone(), Some(vec_ops.clone())); // RangeTo
/// assert_eq!(circuit.as_vec(..1).clone(), Some(vec![vec_ops[0].clone()])); // RangeFrom
/// ```
///
pub trait AsVec<T> {
    /// Returns slice of Circuit as Vec<Operations>.
    ///
    /// # Arguments
    ///
    /// * `range` - The indices of the slice of the Circuit to be returned.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Operation>>` - A vector of the operations in the Circuit with the specified indices.
    fn as_vec(&self, range: T) -> Option<Vec<Operation>>;
}

impl AsVec<std::ops::Range<usize>> for Circuit {
    /// Returns slice of Circuit as Vec<Operations>.
    ///
    /// # Arguments
    ///
    /// * `range` - The indices of the slice of the Circuit to be returned.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Operation>>` - A vector of the operations in the Circuit with the specified indices.
    fn as_vec(&self, range: std::ops::Range<usize>) -> Option<Vec<Operation>> {
        let mut return_vec: Vec<Operation>;
        let def_len = self.definitions.len();
        if range.end - def_len >= self.operations.len() {
            return None;
        }
        if range.start < def_len {
            if range.end < def_len {
                return_vec = self.definitions[range].to_vec();
            } else {
                return_vec = self.definitions[range.start..].to_vec();
                let mut tmp_vec = self.operations[..range.end - def_len].to_vec();
                return_vec.append(&mut tmp_vec);
            }
        } else {
            return_vec = self.operations[range.start - def_len..range.end - def_len].to_vec();
        }
        Some(return_vec)
    }
}

impl AsVec<std::ops::RangeTo<usize>> for Circuit {
    /// Returns slice of Circuit as Vec<Operations>.
    ///
    /// # Arguments
    ///
    /// * `range` - The indices of the slice of the Circuit to be returned.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Operation>>` - A vector of the operations in the Circuit with the specified indices.
    fn as_vec(&self, range: std::ops::RangeTo<usize>) -> Option<Vec<Operation>> {
        let mut return_vec: Vec<Operation>;
        let def_len = self.definitions.len();
        if range.end - def_len >= self.operations.len() {
            return None;
        }
        if range.end < def_len {
            return_vec = self.definitions[range].to_vec();
        } else {
            return_vec = self.definitions.clone();
            let mut tmp_vec = self.operations[..range.end - def_len].to_vec();
            return_vec.append(&mut tmp_vec);
        }
        Some(return_vec)
    }
}

impl AsVec<std::ops::RangeFrom<usize>> for Circuit {
    /// Returns slice of Circuit as Vec<Operations>.
    ///
    /// # Arguments
    ///
    /// * `range` - The indices of the slice of the Circuit to be returned.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Operation>>` - A vector of the operations in the Circuit with the specified indices.
    fn as_vec(&self, range: std::ops::RangeFrom<usize>) -> Option<Vec<Operation>> {
        let mut return_vec: Vec<Operation>;
        let def_len = self.definitions.len();
        if range.start < def_len {
            return_vec = self.definitions[range.start..].to_vec();
            let mut tmp_vec = self.operations.clone();
            return_vec.append(&mut tmp_vec);
        } else {
            return_vec = self.operations[range.start - def_len..].to_vec();
        }
        Some(return_vec)
    }
}

/// Implements `+` (add) for Circuit and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T that implements Into<Operation> trait.
impl<T> ops::Add<T> for Circuit
where
    T: Into<Operation>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let mut return_circuit = self;
        return_circuit.add_operation(other);
        return_circuit
    }
}

/// Implements `+` (add) for two Circuits.
///
/// # Arguments
///
/// * `other` - The Circuit to be added.
impl ops::Add<Circuit> for Circuit {
    type Output = Self;
    fn add(self, other: Circuit) -> Self {
        Self {
            definitions: self
                .definitions
                .into_iter()
                .chain(other.definitions.into_iter())
                .collect(),
            operations: self
                .operations
                .into_iter()
                .chain(other.operations.into_iter())
                .collect(),
            _roqoqo_version: RoqoqoVersion,
        }
    }
}

/// Implements `+` (add) for Circuit and Circuit reference.
///
/// # Arguments
///
/// * `other` - The Circuit reference to be added.
impl ops::Add<&Circuit> for Circuit {
    type Output = Self;
    fn add(self, other: &Circuit) -> Self {
        Self {
            definitions: self
                .definitions
                .into_iter()
                .chain(other.definitions.iter().cloned())
                .collect(),
            operations: self
                .operations
                .into_iter()
                .chain(other.operations.iter().cloned())
                .collect(),
            _roqoqo_version: RoqoqoVersion,
        }
    }
}

/// Implements `+=` (add) for Circuit and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T that implements Into<Operation> trait.
impl<T> ops::AddAssign<T> for Circuit
where
    T: Into<Operation>,
{
    fn add_assign(&mut self, other: T) {
        self.add_operation(other);
    }
}

/// Implements `+=` (add) for two Circuits.
///
/// # Arguments
///
/// * `other` - The Circuit to be appended.
impl ops::AddAssign<Circuit> for Circuit {
    fn add_assign(&mut self, other: Circuit) {
        self.definitions.extend(other.definitions.into_iter());
        self.operations.extend(other.operations.into_iter())
    }
}

/// Implements `+=` (add) for Circuits and Circuit reference.
///
/// # Arguments
///
/// * `other` - The Circuit to be appended.
impl ops::AddAssign<&Circuit> for Circuit {
    fn add_assign(&mut self, other: &Circuit) {
        self.definitions.extend(other.definitions.iter().cloned());
        self.operations.extend(other.operations.iter().cloned())
    }
}

/// Implements the Display trait for Circuit.
impl Display for Circuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();
        for op in self.iter() {
            _ = writeln!(s, "{:?}", op)
        }
        write!(f, "{}", s)
    }
}

/// Iterator over roqoqo operations.
#[derive(Debug, Clone)]
pub struct OperationIterator {
    /// Definitions in the quantum circuit in Iterator form, must be unique.
    definition_iter: std::vec::IntoIter<Operation>,
    /// Operations in the quantum circuit in Iterator form, must not be unique.
    operation_iter: std::vec::IntoIter<Operation>,
}

impl Iterator for OperationIterator {
    type Item = Operation;
    /// Advances the iterator and returns the next value.
    ///
    /// Returns None when iteration is finished. Individual iterator implementations may choose to resume iteration,
    /// and so calling next() again may or may not eventually start returning Some(Operation) again at some point.
    ///
    /// # Returns
    ///
    /// * `Option<Self::Item>` - The Operation that is next in the Iterator.
    fn next(&mut self) -> Option<Self::Item> {
        match self.definition_iter.next() {
            Some(x) => Some(x),
            None => self.operation_iter.next(),
        }
    }
}

impl SupportedVersion for Circuit {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        let mut current_minimum_version = (1, 0, 0);
        for op in self.iter() {
            let comparison_version = op.minimum_supported_roqoqo_version();
            crate::update_roqoqo_version(&mut current_minimum_version, comparison_version);
        }
        current_minimum_version
    }
}
