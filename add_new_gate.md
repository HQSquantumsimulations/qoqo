# GUIDE: HOW TO ADD A NEW GATE TO ROQOQO/QOQO

Depending on its type, adding support for a new quantum gate to roqoqo/qoqo can be more or less straightforward. As a rule of thumb, looking at the already available implementations of similar gates should help in understanding what to do.

All supported gates are currently defined in the `roqoqo/src/operations` module (for qoqo, `qoqo/src/operations`). The specific script to put a new definition in depends on the type of gate.

The general idea is: a `pub struct` defines the name, qubit-related fields as well as parameters (if any). Then, in order to add any kind of property to the gate, `traits` are either derived or manually implemented for it.

As an examples, we'll take the roqoqo and qoqo definition of `RotateX`.

### ROQOQO
```rust
/// The XPower gate exp(-i * θ/2 * σ^x).
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateSingleQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RotateX {
    /// The qubit the unitary gate is applied to.
    qubit: usize,
    /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
    theta: CalculatorFloat,
}
#[allow(non_upper_case_globals)]
const TAGS_RotateX: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "SingleQubitGateOperation",
    "Rotation",
    "RotateX",
];

/// Trait for all operations acting with a unitary gate on a set of qubits.
impl OperateGate for RotateX {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of theta to f64 failed.
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
        let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
        Ok(array![
            [Complex64::new(c, 0.0), Complex64::new(0.0, -1.0 * s)],
            [Complex64::new(0.0, -1.0 * s), Complex64::new(c, 0.0)]
        ])
    }
}

/// Trait for unitary operations acting on exactly one qubit.
impl OperateSingleQubitGate for RotateX {
    /// Returns the alpha_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_r(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).cos()
    }
    /// Returns the alpha_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
    fn alpha_i(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_r parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_r(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
    /// Returns the beta_i parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
    fn beta_i(&self) -> CalculatorFloat {
        (self.theta.clone() / 2.0).sin() * (-1.0)
    }
    /// Returns global_phase parameter of the operation.
    ///
    /// # Returns
    ///
    /// * `global_phase` - The global phase φ of the single-qubit unitary.
    fn global_phase(&self) -> CalculatorFloat {
        CalculatorFloat::from(0.0)
    }
}
```

In order, we have:
* Generic text definition of the gate
    ```rust
    /// The XPower gate exp(-i * θ/2 * σ^x).
    ///
    ```
* List of derive instructions, meaning traits that will be automatically implemented for the gate
    ```rust
    #[derive(
        Debug,
        Clone,
        PartialEq,
        roqoqo_derive::InvolveQubits,
        roqoqo_derive::SupportedVersion,
        roqoqo_derive::Operate,
        roqoqo_derive::Substitute,
        roqoqo_derive::OperateSingleQubit,
        roqoqo_derive::Rotate,
    )]
    #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
    ```
    Traits represent "properties" one can add to a gate. The idea, after defining the gate's `struct`, is to add these already defined properties to the gate.

    Later on the guide will explain the meaning of all of these.

* Struct containing definition of the gate
    ```rust
    pub struct RotateX {
        /// The qubit the unitary gate is applied to.
        qubit: usize,
        /// The angle θ of the rotation, in the interval from 0 to 2 * 2π.
        theta: CalculatorFloat,
    }
    ```

    In general, structs define the signature of the gate: the name, the fields related to referencing qubits as well as the gate's parameter(s) (if any). In order to make the derivation of `traits` work, some of those fields are reserved, specifically:
    ```rust
    /// Array of field names that are reserved for use with specific traits
    const RESERVED_FIELDS: &[&str; 15] = &[
        "qubit",
        "control",
        "control_0",
        "control_1",
        "target",
        "qubits",
        "global_phase",
        "alpha_r",
        "alpha_i",
        "beta_r",
        "beta_i",
        "name",
        "mode",
        "mode_0",
        "mode_1",
    ];
    ```
    ##### In `roqoqo-derive/src/lib/rs`. If necessary, more can be added.
    In this example, the `qubit` one. As for another example, a new two-qubit gate that is characterized by a `target` and `control` qubits will have to have those two fields (always of `usize` type) and the derivation of traits will work as expected, without having to do it manually.

* Tags, associating the new gate to different granularities of gate types
    ```rust
    #[allow(non_upper_case_globals)]
    const TAGS_RotateX: &[&str; 5] = &[
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateX",
    ];
    ```
    These are related to the traits. In the example, "Operation" indicates that is a gate that implements the `Operate` trait; "GateOperation" the `OperateGate` trait; "SingleQubitGateOperation" the `OperateSingleQubitGate` trait and "Rotation" the `Rotate` trait. The last has to always be the gate's name. 

* Manual implementation of traits
    ```rust
    /// Trait for all operations acting with a unitary gate on a set of qubits.
    impl OperateGate for RotateX {
        /// Returns unitary matrix of the gate.
        ///
        /// # Returns
        ///
        /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
        /// * `Err(RoqoqoError)` - The conversion of theta to f64 failed.
        fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
            let c: f64 = (f64::try_from(self.theta.clone())? / 2.0).cos();
            let s: f64 = (f64::try_from(self.theta.clone())? / 2.0).sin();
            Ok(array![
                [Complex64::new(c, 0.0), Complex64::new(0.0, -1.0 * s)],
                [Complex64::new(0.0, -1.0 * s), Complex64::new(c, 0.0)]
            ])
        }
    }

    /// Trait for unitary operations acting on exactly one qubit.
    impl OperateSingleQubitGate for RotateX {
        /// Returns the alpha_r parameter of the operation.
        ///
        /// # Returns
        ///
        /// * `alpha_r` - The real part Re(α) of the on-diagonal elements of the single-qubit unitary matrix.
        fn alpha_r(&self) -> CalculatorFloat {
            (self.theta.clone() / 2.0).cos()
        }
        /// Returns the alpha_i parameter of the operation.
        ///
        /// # Returns
        ///
        /// * `alpha_i` - The imaginary part Im(α) of the on-diagonal elements of the single-qubit unitary matrix.
        fn alpha_i(&self) -> CalculatorFloat {
            CalculatorFloat::from(0.0)
        }
        /// Returns the beta_r parameter of the operation.
        ///
        /// # Returns
        ///
        /// * `beta_r` - The real part Re(β) of the off-diagonal elements of the single-qubit unitary matrix.
        fn beta_r(&self) -> CalculatorFloat {
            CalculatorFloat::from(0.0)
        }
        /// Returns the beta_i parameter of the operation.
        ///
        /// # Returns
        ///
        /// * `beta_i` - The imaginary part Im(β) of the off-diagonal elements of the single-qubit unitary matrix.
        fn beta_i(&self) -> CalculatorFloat {
            (self.theta.clone() / 2.0).sin() * (-1.0)
        }
        /// Returns global_phase parameter of the operation.
        ///
        /// # Returns
        ///
        /// * `global_phase` - The global phase φ of the single-qubit unitary.
        fn global_phase(&self) -> CalculatorFloat {
            CalculatorFloat::from(0.0)
        }
    }
    ```

    These are gate-specific properties that cannot be derived. In this case, `OperateGate` and `OperateSingleQubitGate` are related to the matrix representation of the gate, so something that has be to manually defined.

### QOQO
```rust
#[wrap(
    Operate,
    OperateSingleQubit,
    Rotate,
    OperateGate,
    OperateSingleQubitGate,
    JsonSchema
)]
/// The XPower gate :math:`e^{-i \frac{\theta}{2} \sigma^x}`.
///
/// .. math::
///     U = \begin{pmatrix}
///         \cos(\frac{\theta}{2}) & 0 \\\\
///         0 & \cos(\frac{\theta}{2})
///         \end{pmatrix}
///         + \begin{pmatrix}
///         0  &  -i \sin(\frac{\theta}{2}) \\\\
///         -i \sin(\frac{\theta}{2})  & 0
///         \end{pmatrix}
///
/// Args:
///     qubit (int): The qubit the unitary gate is applied to.
///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
///
struct RotateX {
    qubit: usize,
    theta: CalculatorFloat,
}
```

We have a similar structure:

* First, the wrap section. This works just like the previous `#[derive(...)]` one. The referenced traits are the same.
    ```rust
    #[wrap(
        Operate,
        OperateSingleQubit,
        Rotate,
        OperateGate,
        OperateSingleQubitGate,
        JsonSchema
    )]
    ```

* Again, generic text definition of the gate
    ```rust
    /// The XPower gate :math:`e^{-i \frac{\theta}{2} \sigma^x}`.
    ///
    /// .. math::
    ///     U = \begin{pmatrix}
    ///         \cos(\frac{\theta}{2}) & 0 \\\\
    ///         0 & \cos(\frac{\theta}{2})
    ///         \end{pmatrix}
    ///         + \begin{pmatrix}
    ///         0  &  -i \sin(\frac{\theta}{2}) \\\\
    ///         -i \sin(\frac{\theta}{2})  & 0
    ///         \end{pmatrix}
    ///
    /// Args:
    ///     qubit (int): The qubit the unitary gate is applied to.
    ///     theta (CalculatorFloat): The angle :math:`\theta` of the rotation.
    ///
    ```
    As a general rule, we try to be more specific when it comes to docstrings in qoqo. In this example, not it contains not only a generic definition but also the matrix of the gate followed by the arguments for the constructor.

* Struct definition (non-pub)
    ```rust
    struct RotateX {
        qubit: usize,
        theta: CalculatorFloat,
    }
    ```
## TRAITS

Traits represent "properties" one can add to a gate. The idea, after defining the gate's `struct`, is to add these already defined properties to the gate.

This is a list of the most important available traits that can be implemented for a new gate. They can be derived (meaning using the `#[derive(..)]` line) depending on whether the gate has the correct field/has already implemented the necessary traits in order to support them. If they can't directly derived, they can be manually implemented via `impl TraitName for MyNewGate { ... }`.

- `Debug`: Always necessary.
- `Clone`: Always necessary.
- `PartialEq`: Always necessary.
- `InvolveQubits`: Implements the `involved_qubits()` method.

    The method returns the enum `InvolvedQubits`.

    It has to be implemented manually if the gate does not involve any qubit (so `InvolvedQubits::None`) or if it involves all of them (`InvolvedQubits::All`).

    If instead it involves a specific amount (`InvolvedQubits::Set(..)`), it can be derived so long as the new gate references the qubit(s) using one (or more) of the RESERVED_FIELDS strings. In the `RotateX` example, the only involved qubit is referenced by the field `qubit`, so it can be derived. In a `CNOT` gate, the involved qubits are `control` and `target`, so it can be derived as well. One can always implement it manually.

- `Operate`: Implements `new()`, `is_parametrized()`, `tags()` and `hqslang()`. Always necessary.

    It can almost always be derived. Once simply need to define the `const TAGS_...` of the new gatefirst, then this trait can be derived.

- `Substitute`: Implements `substitute_parameters()` and `remap_qubits()` methods.

    This trait allows replacing symbolic parameters and performing qubit remapping.

    When deriving it, the `remap_qubits()` will look for the following fields in order to perform the correct remapping: "qubit", "control", "control_0", "control_1", "target", "qubits". `substitute_parameters()` will instead look for any `CalculatorFloat` field in the struct of the new gate.

- `SupportedVersion`: Implements `minimum_supported_roqoqo_version()`. Always necessary and always to be manually implemented for new gates.

    In the example, since the `RotateX` operation was present in qoqo since the beginning it is derived. But for new gates in roqoqo this has to be manually implemented. This depends on the current version of the package and looks like this

    ```rust
    impl SupportedVersion for InvSqrtPauliY {
        fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
            (1, 15, 0)
        }
    }
    ```
    The values in the tuple should be the major, minor and patch version of the current roqoqo's main branch. In case changes are in progress, a maintainer may ask you to modify it.

- `OperateSingleQubit`: Implements the `.qubit()` method (getter). Can be derived if the struct includes a field with that name.

- `OperateTwoQubit`: Implements the `.target()` and `.control()` methods (getters). Can be derived if the struct includes two fields with those names.

- `OperateThreeQubit`: Implements the `control_0()`, `control_1()` and `target()` methods (getters).  Can be derived if the struct includes three fields with those names.

- `OperateMultiQubit`: Implements the `.qubits()` method (getter). Can be derived if the struct includes a field with that name.

- `OperateGate`: Implements the `.unitary_matrix()` method.

    Has to be manually implemented. As the examples shows, the matrix has to be in the form of `Array2<Complex64>`.

- `Rotate`: Implements the `.theta()` and `.powercf()` methods. Can be derived if the struct includes a field `theta`.

    While the first method is a simple getter, the second returns another instance the new gate with `theta` multiplied by a `CalculatorFloat`.

- `OperateSingleQubitGate`: Implements `.alpha_r()`, `alpha_i()`, `.beta_r()`, `beta_i()` and `.global_phase()` methods. Always to be manually implemented for new single-qubit gates.

    These parameters have to be defined following the matrix explained [here](https://hqsquantumsimulations.github.io/qoqo_examples/gate_operations/single_qubit_gates.html).

- `OperateTwoQubitGate`: Implements `.kak_decomposition()`. Always to be manually implemented for new two-qubit gates.

    The following is the struct that defines what a KAK Decomposition is (includes the link to the original paper)
    ```rust
    /// The KAK decomposition of a two-qubit gate.
    ///
    /// Each two-qubit gate can be described by a KAK decomposition (<http://arxiv.org/abs/quant-ph/0507171>).
    ///
    /// A two qubit gate is decomposed into four single qubit gates, one for each qubit acting before and after applying the
    /// entangling operation based on the k_vector:  
    ///
    /// U(k_vector) = exp(i (k_vector(0) XX + k_vector(1) YY + k_vector(2) ZZ))
    ///
    /// This struct contains all information on the KAK decomposition of a two qubit gate.
    #[derive(Debug, Clone, PartialEq)]
    pub struct KakDecomposition {
        /// Global phase of KAK decomposition
        pub global_phase: CalculatorFloat,
        /// Three component vector of the KAK decomposition
        pub k_vector: [CalculatorFloat; 3],
        /// Circuit including operations acting on control and target qubits before two-qubit entangling
        pub circuit_before: Option<Circuit>,
        /// Circuit including operations acting on control and target qubits after two-qubit entangling
        pub circuit_after: Option<Circuit>,
    }
    ```

    As an example, this is CNOT's implementation
    ```rust
    impl OperateTwoQubitGate for CNOT {
        /// Returns [KakDecomposition] of the  gate.
        ///
        /// # Returns
        ///
        /// * struct `KakDecomposition { global_phase, k_vector, circuit_before, circuit_after }`
        fn kak_decomposition(&self) -> KakDecomposition {
            let mut circuit_b = Circuit::new();
            circuit_b += RotateZ::new(self.control, CalculatorFloat::FRAC_PI_2);
            circuit_b += RotateY::new(self.control, CalculatorFloat::FRAC_PI_2);
            circuit_b += RotateX::new(self.target, CalculatorFloat::FRAC_PI_2);

            let mut circuit_a = Circuit::new();
            circuit_a += RotateY::new(self.control, CalculatorFloat::FRAC_PI_2 * (-1.0));

            KakDecomposition {
                global_phase: CalculatorFloat::FRAC_PI_4,
                k_vector: [
                    CalculatorFloat::FRAC_PI_4,
                    CalculatorFloat::ZERO,
                    CalculatorFloat::ZERO,
                ],
                circuit_before: Some(circuit_b),
                circuit_after: Some(circuit_a),
            }
        }
    }
    ```

- `OperateThreeQubitGate`: Implements `.circuit()` method. Always to be manually implemented for new three-qubit gates.

    This method should return a `roqoqo::Circuit` sequence of operations equivalent to the new three-qubit gate. As as example, this is Toffoli's:
    ```rust
    impl OperateThreeQubitGate for Toffoli {
        fn circuit(&self) -> Circuit {
            let mut circuit = Circuit::new();
            circuit += Hadamard::new(self.target);
            circuit += CNOT::new(self.control_1, self.target);
            circuit += RotateZ::new(self.target, -CalculatorFloat::FRAC_PI_4);
            circuit += CNOT::new(self.control_0, self.target);
            circuit += TGate::new(self.target);
            circuit += CNOT::new(self.control_1, self.target);
            circuit += RotateZ::new(self.target, -CalculatorFloat::FRAC_PI_4);
            circuit += CNOT::new(self.control_0, self.target);
            circuit += TGate::new(self.control_1);
            circuit += TGate::new(self.target);
            circuit += Hadamard::new(self.target);
            circuit += CNOT::new(self.control_0, self.control_1);
            circuit += TGate::new(self.control_0);
            circuit += RotateZ::new(self.control_1, -CalculatorFloat::FRAC_PI_4);
            circuit += CNOT::new(self.control_0, self.control_1);
            circuit
        }
    }
    ```

- `OperateMultiQubitGate`: Implements `.circuit()` method. Always to be manually implemented for new multi-qubit gates.

    This method should return a `roqoqo::Circuit` sequence of operations equivalent to the new multi-qubit gate. As as example, this is MultiQubitZZ's:
    ```rust
    impl OperateMultiQubitGate for MultiQubitZZ {
        fn circuit(&self) -> Circuit {
            let dim = self.qubits.len();
            let mut circuit = Circuit::new();
            for q in self.qubits[1..].iter() {
                circuit += operations::CNOT::new(*q - 1, *q);
            }
            circuit += operations::RotateZ::new(dim - 1, self.theta.clone());
            for q in self.qubits[1..].iter() {
                circuit += operations::CNOT::new(dim - *q - 1, dim - *q);
            }
            circuit
        }
    }
    ```

- `serde::Serialize`, `serde::Deserialize`, `schemars::JsonSchema`: Always to be derived.

    These traits automatically handle serialization, deserialization and creating a Json schema for the newly added gate. Specifically, they must be derived using a feature gate. Meaning:
    ```rust
    #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
    ```