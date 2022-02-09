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

//! qoqo-macros
//!
//! Attribute proc-macros for the traits of qoqo [qoqo].

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    parse2, parse_macro_input, DataStruct, DeriveInput, Fields, GenericArgument, Ident, ItemStruct,
    PathArguments, Token, Type, TypePath,
};
mod operate;
// mod operate_unitary;

/// Array of field names that are reserved for use with specific traits
const RESERVED_FIELDS: &[&str; 11] = &[
    "qubit",
    "control",
    "target",
    "theta",
    "qubits",
    "global_phase",
    "alpha_r",
    "alpha_i",
    "beta_r",
    "beta_i",
    "name",
];

// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[derive(Debug)]
struct AttributeMacroArguments(HashSet<String>);

impl AttributeMacroArguments {
    pub fn contains(&self, st: &str) -> bool {
        self.0.contains(st)
    }
    pub fn _ids(&self) -> Vec<Ident> {
        self.0
            .clone()
            .into_iter()
            .map(|s| format_ident!("Wrap{}", s))
            .collect()
    }
}

impl Parse for AttributeMacroArguments {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse arguments as comma separated list of idents
        let arguments = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self(
            arguments.into_iter().map(|id| id.to_string()).collect(),
        ))
    }
}

/// Attribute macro for constructing the pyo3 wrappers for operation structs
#[proc_macro_attribute]
pub fn wrap(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let input2: TokenStream = input.clone().into();
    let parsed_input = parse_macro_input!(input as ItemStruct);
    let ident = parsed_input.ident;
    let struct_attributes = parsed_input.attrs;
    let str_ident = ident.to_string();
    let wrapper_ident = format_ident!("{}Wrapper", ident.to_string());
    let operate_quote = if attribute_arguments.contains("Operate") {
        derive_wrap_operate(input2)
    } else {
        TokenStream::new()
    };
    let rotate_quote = if attribute_arguments.contains("Rotate") {
        quote! {
            /// Returns angle of rotation
            pub fn theta(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.theta().clone()}
            }
            /// Returns Rotated gate raised to power
            ///
            /// Args:
            ///     power(CalculatorFloat):
            pub fn powercf(&self, power: CalculatorFloatWrapper) -> Self{
                Self{internal: self.internal.powercf(power.cf_internal)}
            }
            #[cfg(feature = "overrotate")]
            /// Returns clone of the gate with one parameter statistically overrotated.
            fn overrotate(&self, amplitude: &f64, variance: &f64) -> Self {
                Self{internal: self.internal.overrotate(amplitude, variance)}

            }
        }
    } else {
        TokenStream::new()
    };
    let operate_pragma_quote = if attribute_arguments.contains("OperatePragma") {
        quote! {}
    } else {
        TokenStream::new()
    };
    let operate_pragma_noise_quote = if attribute_arguments.contains("OperatePragmaNoise") {
        quote! {
            /// Return the superoperator defining the evolution of the density matrix under the noise gate
            ///
            /// Returns:
            ///     np.ndarray
            pub fn superoperator(&self) -> PyResult<Py<PyArray2<f64>>>{
                Python::with_gil(|py| -> PyResult<Py<PyArray2<f64>>> {
                    Ok(self.internal.superoperator().unwrap().to_pyarray(py).to_owned())
                })
            }
            /// Return the power of the noise gate
            ///
            /// Args:
            /// power (CalculatorFloat): exponent in the power operation of the noise gate
            ///
            /// Returns:
            ///     Self
            pub fn powercf(&self, power: CalculatorFloatWrapper) -> Self{
                Self{internal: self.internal.powercf(power.cf_internal)}
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_pragma_noise_proba_quote =
        if attribute_arguments.contains("OperatePragmaNoiseProba") {
            quote! {
                /// Returns the probability associated with the noise operation
                ///
                /// Returns:
                ///     CalculatorFloat
                pub fn probability(&self) -> CalculatorFloatWrapper{
                    CalculatorFloatWrapper{cf_internal: self.internal.probability().clone()}
                }
            }
        } else {
            TokenStream::new()
        };
    let operate_single_qubit_quote = if attribute_arguments.contains("OperateSingleQubit") {
        quote! {
            /// Return the qubit the operation acts on
            ///
            /// Returns:
            ///     int
            pub fn qubit(&self) -> usize{
                self.internal.qubit().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_single_qubit_gate_quote = if attribute_arguments.contains("OperateSingleQubitGate")
    {
        quote! {
            /// Return the global phase :math:`g` of a unitary gate acting on one qubit
            ///
            /// Here global_phase is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn global_phase(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.global_phase().clone()}
            }
            /// Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit
            ///
            /// Here alpha_r is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn alpha_r(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.alpha_r().clone()}
            }
            /// Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn alpha_i(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.alpha_i().clone()}
            }
            /// Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit
            ///
            /// Here beta_r is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn beta_r(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.beta_r().clone()}
            }
            /// Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit
            ///
            /// Here beta_i is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn beta_i(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{cf_internal: self.internal.beta_i().clone()}
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_two_qubit_quote = if attribute_arguments.contains("OperateTwoQubit") {
        quote! {
            /// Returns contol qubit of the two-qubit operation
            pub fn control(&self) -> usize{
                self.internal.control().clone()
            }
            /// Returns target qubit of the two-qubit operation
            pub fn target(&self) -> usize{
                self.internal.target().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    // let operate_two_qubit_gate_quote = if attribute_arguments.contains("OperateTwoQubitGate") {
    //     quote! {
    //         /// Returns kak decomposition of the two-qubit-gate operation
    //         pub fn kak_decomposition(&self) -> KakDecompositionWrapper {
    //             KakDecompositionWrapper{internal: self.internal.kak_decomposition().clone()}
    //         }
    //     }
    // } else {
    //     TokenStream::new()
    // };
    let operate_gate_quote = if attribute_arguments.contains("OperateGate") {
        quote! {
            /// Return unitary matrix of gate.
            ///
            /// Returns:
            ///     np.ndarray
            ///
            /// Raises:
            ///     ValueError: Error symbolic operation cannot return float unitary matrix
            pub fn unitary_matrix(&self) -> PyResult<Py<PyArray2<Complex64>>>{
                Python::with_gil(|py| -> PyResult<Py<PyArray2<Complex64>>> {
                    Ok(self.internal.unitary_matrix().map_err(|x| PyValueError::new_err(format!("Error symbolic operation cannot return float unitary matrix {:?}",x)))?.to_pyarray(py).to_owned())
                })
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_multi_qubit_quote = if attribute_arguments.contains("OperateMultiQubit") {
        quote! {
            /// Return list of qubits of the multi qubit operation in order of descending significance
            ///
            /// Returns:
            ///     list[int]
            pub fn qubits(&self) -> Vec<usize>{
                self.internal.qubits().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_multi_qubit_gate_quote = if attribute_arguments.contains("OperateMultiQubitGate") {
        quote! {
            /// Return circuit implementing MultiQubitGateOperation
            ///
            /// Returns:
            ///     Circuit
            pub fn circuit(&self) -> CircuitWrapper{
                CircuitWrapper { internal: self.internal.circuit().clone() }
            }
        }
    } else {
        TokenStream::new()
    };
    let define_quote = if attribute_arguments.contains("Define") {
        quote! {
        /// Return name of definition operation.
        ///
        /// Returns:
        ///     str
        pub fn name(&self) -> String {
                self.internal.name().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_constant_gate_quote = if attribute_arguments.contains("OperateConstantGate") {
        quote! {
        /// Return inverse of GateOperation:
        ///
        /// Returns:
        ///     GateOperation
        pub fn inverse(&self) -> GateOperationWrapper {
                GateOperationWrapper { internal: self.internal.inverse().clone() }
            }
        }
    } else {
        TokenStream::new()
    };
    let msg = format!("Internal storage of {} object", ident);
    let q = quote! {
        #[automatically_derived]
        #[pyclass(name=#str_ident)]
        #(#struct_attributes)*
        #[derive(Debug, Clone, PartialEq)]
        pub struct #wrapper_ident{
            #[doc = #msg]
            pub internal: #ident
        }
        #[automatically_derived]
        #[pymethods]
        impl #wrapper_ident{
            #operate_quote
            #operate_single_qubit_quote
            #operate_single_qubit_gate_quote
            #operate_two_qubit_quote
            // #operate_two_qubit_gate_quote
            #operate_multi_qubit_quote
            #operate_multi_qubit_gate_quote
            #operate_gate_quote
            #rotate_quote
            #operate_pragma_quote
            #operate_pragma_noise_quote
            #operate_pragma_noise_proba_quote
            #define_quote
            #operate_constant_gate_quote
            fn __format__(&self, _format_spec: &str) -> PyResult<String> {
                Ok(format!("{:?}", self.internal))
            }
    }
        #[pyproto]
        impl PyObjectProtocol for #wrapper_ident {
            fn __repr__(&self) -> PyResult<String> {
                Ok(format!("{:?}", self.internal))
            }



            /// Returns the __richcmp__ magic method to perform rich comparison
            /// operations on Operation.
            ///
            /// # Arguments
            ///
            /// * `&self` - the OperationWrapper object
            /// * `other` - the object to compare self to
            /// * `op` - equal or not equal
            ///
            /// # Returns
            ///
            /// `PyResult<bool>` - whether the two operations compared evaluated to True or False
            ///
            fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                Python::with_gil(|py| -> PyResult<bool> {
                    let other_ref = other.as_ref(py);
                    let other: Operation = crate::operations::convert_pyany_to_operation(other_ref).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Right hand side cannot be converted to Operation {:?}",x))
                    })?;
                    match op {
                        pyo3::class::basic::CompareOp::Eq => Ok(Operation::from(self.internal.clone()) == other),
                        pyo3::class::basic::CompareOp::Ne => Ok(Operation::from(self.internal.clone()) != other),
                        _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                            "Other comparison not implemented.",
                        )),
                    }
                })
            }

        }
    };
    q.into()
}

fn derive_wrap_operate(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse2(input).unwrap();
    operate::dispatch_struct(parsed_input)
}

/// Macro for injecting code to convert PyAny to Operation
#[proc_macro]
pub fn insert_pyany_to_operation(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}

/// Macro for injecting code to convert PyAny to Operation
#[proc_macro]
pub fn insert_operation_to_pyobject(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}

/// Extrats the identifier of fields of a named struct
/// together with the optional cast of the type to string form (where the type is a simple path) and the type as a syn object
fn extract_fields_with_types(ds: DataStruct) -> Vec<(Ident, Option<String>, Type)> {
    let fields = match ds {
        DataStruct {
            fields: Fields::Named(fields),
            ..
        } => fields,
        _ => panic!("Trait can only be derived on structs with named fields"),
    };
    fields.named.into_iter().map(|f| {
        let id = f
            .ident
            .expect("Operate can only be derived on structs with named fields");
        let ty = f.ty;
        let type_path =match &ty {
            Type::Path(TypePath{path:p,..}) => p,
            _ => panic!("Trait  only supports fields with normal types of form path (e.g. CalculatorFloat, qoqo_calculator::CalculatorFloat)")
        };
        let mut type_string = match type_path.get_ident(){
            Some(ident_path) => Some(ident_path.to_string()),
            _ => type_path
            .segments
            .last().map(|segment|{segment.ident.to_string()})
        };
        if let Some(ref x) = type_string{
            if x.as_str() == "Option"{
           let inner_type = match &type_path.segments.iter().next().unwrap().arguments{
               PathArguments::AngleBracketed(angle_argumnets) =>  match angle_argumnets.args.iter().next().unwrap() {
               GenericArgument::Type(Type::Path(TypePath{path:innerty,..})) => match innerty.get_ident(){
                   Some(ident_path) => Some(ident_path.to_string()),
                   _ =>innerty
                   .segments
                   .last().map(|segment|{segment.ident.to_string()})
               },
               _ => panic!("Expected GenericArgument")
           },
           _ => panic!("Expected AngleBracketed")
       };
       if let Some(s) = inner_type { if s.as_str() == "Circuit"{ 
           type_string = Some("Option<Circuit>".to_string())
       }}}
   }
        (id, type_string, ty)
    }).collect()
}
