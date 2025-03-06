// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

pub fn noise_model_wrapper_def(
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

            /// Returns a copy of the device (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __copy__(&self) -> Self {
                self.clone()
            }

            /// Creates deep copy of Noise-Model.
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __deepcopy__(&self, _memodict: &Bound<PyAny>) -> Self {
                self.clone()
            }

            /// Return the bincode representation of the Noise-Model using the bincode crate.
            ///
            /// Returns:
            ///     ByteArray: The serialized Noise-Model (in bincode form).
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Noise-Model to bytes.
            ///
            pub fn to_bincode(&self) -> PyResult<Py<pyo3::types::PyByteArray>> {
                let noise_model = NoiseModel::from(self.internal.clone());
                let serialized = bincode::serialize(&noise_model)
                    .map_err(|_| pyo3::exceptions::PyValueError::new_err("Cannot serialize Noise-Model to bytes"))?;
                let b: Py<pyo3::types::PyByteArray> = Python::with_gil(|py| -> Py<pyo3::types::PyByteArray> {
                    pyo3::types::PyByteArray::new(py, &serialized[..]).into()
                });
                Ok(b)
            }

            /// Return the json representation of the Noise-Model.
            ///
            /// Returns:
            ///     str: The serialized form of Noise-Model.
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Noise-Model to json.
            ///
            pub fn to_json(&self) -> PyResult<String> {
                let noise_model = NoiseModel::from(self.internal.clone());
                let serialized = serde_json::to_string(&noise_model)
                    .map_err(|_| pyo3::exceptions::PyValueError::new_err("Cannot serialize Noise-Model to json"))?;
                Ok(serialized)
            }

            #[cfg(feature = "json_schema")]
            /// Returns the current version of the qoqo library .
            ///
            /// Returns:
            ///     str: The current version of the library.
            #[staticmethod]
            pub fn current_version() -> String {
                ROQOQO_VERSION.to_string()
            }

            #[cfg(feature = "json_schema")]
            /// Return the minimum version of qoqo that supports this object.
            ///
            /// Returns:
            ///     str: The minimum version of the qoqo library to deserialize this object.
            pub fn min_supported_version(&self) -> String {
                let min_version: (u32, u32, u32) =
                    NoiseModel::minimum_supported_roqoqo_version(&NoiseModel::from(self.internal.clone()));
                format!("{}.{}.{}", min_version.0, min_version.1, min_version.2)
            }

            /// Return the __richcmp__ magic method to perform rich comparison operations on mixed system.
            ///
            /// Args:
            ///     other: The object to compare self to.
            ///     op: Whether they should be equal or not.
            ///
            /// Returns:
            ///     bool: Whether they are equal or not.
            ///
            /// Raises:
            ///     NotImplementedError: Other comparison not implemented.
            ///
            fn __richcmp__(&self, other: &Bound<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                let other = #ident::from_pyany(other);

                match op {
                    pyo3::class::basic::CompareOp::Eq => match other {
                        Ok(osystem) => Ok(NoiseModel::from(self.internal.clone()) == osystem),
                        _ => Ok(false),
                    },
                    pyo3::class::basic::CompareOp::Ne => match other {
                        Ok(osystem) => Ok(NoiseModel::from(self.internal.clone()) != osystem),
                        _ => Ok(true),
                    },
                    _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                        "Other comparison not implemented",
                    )),
                }
            }
        }

        impl #ident {
            /// Fallible conversion of generic python object..
            pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<NoiseModel> {
                    if let Ok(try_downcast) = input.extract::<#ident>() {
                        Ok(try_downcast.internal.into())
                    } else {
                        let get_bytes = input.call_method0("to_bincode")?;
                        let bytes = get_bytes.extract::<Vec<u8>>()?;
                        bincode::deserialize(&bytes[..]).map_err(|err| {
                            pyo3::exceptions::PyValueError::new_err(format!(
                                "Cannot treat input as NoiseModel: {}",
                                err
                            ))
                        })
                    }
            }
        }
    };
    q.into()
}
