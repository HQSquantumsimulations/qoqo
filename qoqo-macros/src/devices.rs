// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use quote::quote;
use syn::{parse_macro_input, ItemImpl};

// A macro to generate impl Device Wrapper for qoqo devices
pub fn device_wrapper_def(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let q = quote! {
        #[pymethods]
        impl #ident {
            #(#items)*

            /// Return number of qubits in device.
            ///
            /// Returns:
            ///     int: The number of qubits.
            pub fn number_qubits(&self) -> usize {
                self.internal.number_qubits()
            }

            /// Return the list of pairs of qubits linked by a native two-qubit-gate in the device.
            ///
            /// A pair of qubits is considered linked by a native two-qubit-gate if the device
            /// can implement a two-qubit-gate between the two qubits without decomposing it
            /// into a sequence of gates that involves a third qubit of the device.
            /// The two-qubit-gate also has to form a universal set together with the available
            /// single qubit gates.
            ///
            /// The returned vectors is a simple, graph-library independent, representation of
            /// the undirected connectivity graph of the device.
            /// It can be used to construct the connectivity graph in a graph library of the user's
            /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
            ///
            /// Returns:
            ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
            ///
            pub fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
                self.internal.two_qubit_edges()
            }

            /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a single qubit gate.
            ///     qubit[int]: The qubit the gate acts on
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubit)")]
            pub fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> Option<f64> {
                self.internal.single_qubit_gate_time(hqslang, &qubit)
            }

            /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a single qubit gate.
            ///     control[int]: The control qubit the gate acts on.
            ///     target[int]: The target qubit the gate acts on.
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            ///
            #[pyo3(text_signature = "(gate, control, target)")]
            pub fn two_qubit_gate_time(&self, hqslang: &str, control: usize, target: usize) -> Option<f64> {
                self.internal
                    .two_qubit_gate_time(hqslang, &control, &target)
            }

            /// Returns the gate time of a three qubit operation if the three qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a single qubit gate.
            ///     control_0[int]: The control_0 qubit the gate acts on.
            ///     control_1[int]: The control_1 qubit the gate acts on.
            ///     target[int]: The target qubit the gate acts on.
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            ///
            #[pyo3(text_signature = "(gate, control_0, control_1, target")]
            pub fn three_qubit_gate_time(&self, hqslang: &str, control_0: usize, control_1: usize, target: usize) -> Option<f64> {
                self.internal
                    .three_qubit_gate_time(hqslang, &control_0, &control_1, &target)
            }

            /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a multi qubit gate.
            ///     qubits[List[int]]: The qubits the gate acts on.
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubits)")]
            pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> Option<f64> {
                self.internal.multi_qubit_gate_time(hqslang, &qubits)
            }

            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubit (int): The qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubit, gate_time)")]
            pub fn set_single_qubit_gate_time(&mut self, gate: &str, qubit: usize, gate_time: f64) -> PyResult<()> {
                self.internal.set_single_qubit_gate_time(gate, qubit, gate_time).map_err(|err|
                PyValueError::new_err(format!("{:?}", err)))
            }

            /// Set the gate time of a two qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     control (int): The control qubit for which the gate time is set
            ///     target (int): The control qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, control, target, gate_time)")]
            pub fn set_two_qubit_gate_time(&mut self, gate: &str, control: usize, target: usize, gate_time: f64) -> PyResult<()> {
                self.internal.set_two_qubit_gate_time(gate, control, target, gate_time).map_err(|err|
                    PyValueError::new_err(format!("{:?}", err)))
            }

            /// Set the gate time of a three qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     control_0 (int): The control_0 qubit for which the gate time is set
            ///     control_1 (int): The control_1 qubit for which the gate time is set
            ///     target (int): The control qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, control_0, control_1, target, gate_time)")]
            pub fn set_three_qubit_gate_time(&mut self, gate: &str, control_0: usize, control_1: usize, target: usize, gate_time: f64) -> PyResult<()> {
                self.internal.set_three_qubit_gate_time(gate, control_0, control_1, target, gate_time).map_err(|err|
                    PyValueError::new_err(format!("{:?}", err)))
            }


            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubits (int): The qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, rates)")]
            pub fn set_qubit_decoherence_rates(&mut self, qubit: usize, rates: PyReadonlyArray2<f64>) -> PyResult<()> {
                let rates_matrix = rates.as_array().to_owned();
                self.internal
                    .set_qubit_decoherence_rates(qubit, rates_matrix)
                    .map_err(|err| {
                        PyValueError::new_err(format!("Could not set rates: {}", err))
                    })
            }

            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubits (List[int]): The qubits for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubits not in device
            #[pyo3(text_signature = "(gate, qubits, gate_time)")]
            pub fn set_multi_qubit_gate_time(&self, gate: &str, qubits: Vec<usize>, gate_time: f64) -> PyResult<()> {
                self.internal.clone().set_multi_qubit_gate_time(gate, qubits, gate_time).map_err(|err|
                    PyValueError::new_err(format!("{:?}", err)))
            }

            /// Return the matrix of the decoherence rates of the Lindblad equation.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the rate matrix M is returned
            ///
            /// Returns:
            ///     numpy.array: 3 by 3 numpy array of decoherence rates
            ///
            #[pyo3(text_signature = "(qubit)")]
            fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
                Python::with_gil(|py| -> Py<PyArray2<f64>> {
                    match self.internal.qubit_decoherence_rates(&qubit) {
                        Some(matrix) => matrix.to_pyarray(py).to_owned(),
                        None => {
                            let matrix = Array2::<f64>::zeros((3, 3));
                            matrix.to_pyarray(py).to_owned()
                        }
                    }
                })
            }

            /// Adds single qubit damping to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     damping (float): The damping rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, damping)")]
            pub fn add_damping(&mut self, qubit: usize, damping: f64) -> PyResult<()> {
                self.internal.add_damping(qubit,damping).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))
            }

             /// Adds single qubit dephasing to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     dephasing (float): The dephasing rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, dephasing)")]
            pub fn add_dephasing(&mut self, qubit: usize, dephasing: f64) -> PyResult<()> {
                self.internal.add_dephasing(qubit,dephasing).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))

            }

            /// Adds single qubit depolarising to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     depolarising (float): The depolarising rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, depolarising)")]
            pub fn add_depolarising(&mut self, qubit: usize, depolarising: f64) -> PyResult<()> {
                self.internal.add_depolarising(qubit,depolarising).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))
            }


            /// Turns Device into GenericDevice
            ///
            /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
            /// (for example when the interface needs to be serialized)
            ///
            /// Returns:
            ///     GenericDevice: The device in generic representation
            ///
            /// Note:
            ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
            ///     The memory usage will be inefficient for devices with large qubit numbers.
            fn generic_device(&self) -> GenericDeviceWrapper {
                GenericDeviceWrapper{ internal: self.internal.to_generic_device()}
            }

            /// Turns Device into GenericDevice
            ///
            /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
            /// (for example when the interface needs to be serialized)
            ///
            /// Returns:
            ///     GenericDevice: The device in generic representation
            ///
            /// Note:
            ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
            ///     The memory usage will be inefficient for devices with large qubit numbers.
            fn to_generic_device(&self) -> GenericDeviceWrapper {
                GenericDeviceWrapper{ internal: self.internal.to_generic_device()}
            }

            /// Returns the names of a single qubit operations available on the device.
            ///
            /// Returns:
            ///     List[strt]: The list of gate names.
            pub fn single_qubit_gate_names(&self) -> Vec<String>{
                self.internal.single_qubit_gate_names()
            }

            /// Returns the names of a two qubit operations available on the device.
            ///
            /// Returns:
            ///     List[strt]: The list of gate names.
            pub fn two_qubit_gate_names(&self) -> Vec<String>{
                self.internal.two_qubit_gate_names()
            }

            /// Returns the names of a mutli qubit operations available on the device.
            ///
            /// The list of names also includes the three qubit gate operations.
            ///
            /// Returns:
            ///     List[strt]: The list of gate names.
            ///
            pub fn multi_qubit_gate_names(&self) -> Vec<String>{
                self.internal.multi_qubit_gate_names()
            }

            /// Returns a copy of the device (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __copy__(&self) -> Self {
                self.clone()
            }

            /// Creates deep copy of Device.
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> Self {
                self.clone()
            }

            /// Return the bincode representation of the Device using the bincode crate.
            ///
            /// Returns:
            ///     ByteArray: The serialized Device (in bincode form).
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Device to bytes.
            ///
            pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
                let serialized = serialize(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize Device to bytes"))?;
                let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
                    PyByteArray::new(py, &serialized[..]).into()
                });
                Ok(b)
            }

            /// Return the json representation of the Device.
            ///
            /// Returns:
            ///     str: The serialized form of Device.
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Device to json.
            ///
            pub fn to_json(&self) -> PyResult<String> {
                let serialized = serde_json::to_string(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize Device to json"))?;
                Ok(serialized)
            }

            /// Convert the bincode representation of the qoqo device to a device using the bincode crate.
            ///
            /// Args:
            ///     input (ByteArray): The serialized Device (in bincode form).
            ///
            /// Returns:
            ///     The deserialized Device.
            ///
            /// Raises:
            ///     TypeError: Input cannot be converted to byte array.
            ///     ValueError: Input cannot be deserialized to selected Device.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_bincode(input: &PyAny) -> PyResult<#ident> {
                let bytes = input
                    .extract::<Vec<u8>>()
                    .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

                Ok(#ident {
                    internal: deserialize(&bytes[..]).map_err(|_| {
                        PyValueError::new_err("Input cannot be deserialized to selected Device.")
                    })?,
                })
            }

            /// Convert the json representation of a device to a qoqo device.
            ///
            /// Args:
            ///     input (str): The serialized device in json form.
            ///
            /// Returns:
            ///     The deserialized device.
            ///
            /// Raises:
            ///     ValueError: Input cannot be deserialized to selected Device.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_json(input: &str) -> PyResult<#ident> {
                Ok(#ident {
                    internal: serde_json::from_str(input).map_err(|_| {
                        PyValueError::new_err("Input cannot be deserialized to selected Device.")
                    })?,
                })
            }

            fn __repr__(&self) -> String{
                format!("{:?}", self.internal)
            }


            /// Return the __richcmp__ magic method to perform rich comparison operations on mixed system.
            ///
            /// Args:
            ///     other: The object to compare self to.
            ///     op: Whether they should be equal or not.
            ///
            /// Returns:
            ///     bool
            ///
            /// Raises:
            ///     NotImplementedError: Other comparison not implemented.
            ///
            fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                let other = #ident::from_pyany(other);
                match op {
                    pyo3::class::basic::CompareOp::Eq => match other {
                        Ok(osystem) => Ok(self.internal == osystem),
                        _ => Ok(false),
                    },
                    pyo3::class::basic::CompareOp::Ne => match other {
                        Ok(osystem) => Ok(self.internal != osystem),
                        _ => Ok(true),
                    },
                    _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                        "Other comparison not implemented",
                    )),
                }
            }
        }
    };
    q.into()
}

// A macro to generate impl Device Wrapper for qoqo devices
#[cfg(feature = "unstable_chain_with_environment")]
pub fn device_chain_env_wrapper_def(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let q = quote! {
        #[pymethods]
        impl #ident {

            #(#items)*
            /// Return a list of linear chains with an environment through the device.
            ///
            /// Returns at least one chain of qubits with linear connectivity and an environment in the device.
            /// An environment is defined as at least one qubit that is connected to at least one qubit of the chain
            /// but not part of the chain.
            /// For each ratio of environment qubits to chain qubits, the list contains at least one of the longest chains
            /// in the devive with that ratio. (Unless that chain and environment is simply a subset
            /// of a chain with equal or longer length and equal or higher ratio).
            ///
            /// For example, take a device with the connectivity graph:
            /// ```
            /// 0 - 3 - 6
            /// |   |   |
            /// 1 - 4 - 7
            /// |   |
            /// 2 - 5
            /// ```
            /// It would have one chain of length 1 and environment with ratio 4 to 1:
            ///
            /// ```
            /// ([4], {4: [1,4,7,5]})
            /// ```
            ///
            /// One with length 2 and ratio 5 to 2:
            /// ```
            /// ([3,4], {3:[0,6], 4: [1,7,5]})
            /// ```
            ///
            /// The chain and environment with length 2 and ratio 2 to 1 is a subset of the one above
            /// and does not need to be listed separately.
            ///
            /// The longest chain with ratio 1 to 1 is:
            /// ```
            /// ([0,1,4,3], {1:[2], 4: [5,7], 3: [6]})
            /// ```
            /// One of the longest chains with ratio 2 to 6 is
            /// ```
            /// ([0,1,2,5,4,3], {4: [7], 3: [6]})
            /// ```
            /// And one of the possible chains with just one environment qubit is:
            /// ```
            /// ([0,1,2,5,4,3,6], {6: [7], 4: [7]})
            /// ```
            ///
            /// Returns:
            ///     List[List[int], Dict[int, List[int]]]: A list of the chains and environments.
            ///
            pub fn __environment_chains(&self) -> Vec<(Vec<usize>, HashMap<usize, Vec<usize>>)>
            {
                self.internal.environment_chains()
            }

            /// Helper function signifying support for chain_with_environment.
            pub fn __implements_environment_chains(&self) -> bool
            {
                true
            }
        }
    };
    q.into()
}
