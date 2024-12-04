# Noise Models

When working with quantum computers it is often necessary to know the physical noise present of a target quantum device, particularly for NISQ devices. In roqoqo/qoqo, we have defined the three following noise models:

* `ContinuousDecoherenceModel`
* `DecoherenceOnGateModel`
* `ImperfectReadout`

All of the noise models use builder patterns for the setting and adding functions, in order to be easier to chain for the user.

## ContinuousDecoherenceModel

The `ContinuousDecoherenceModel` is the noise model representing a continuous decoherence process on qubits. This noise model assumes that all qubits are constantly experiencing decoherence over time (e.g. due to coupling to the environment). The noise for each qubit can be different but only single qubit noise is included in the model.

Noise is given by the rates of the Lindblad equation. The Lindblad equation is a so-called master equation for the time evolution of a density matrix. For the example of a single qubit the non-coherent part of the Lindblad equation can take the following form:
\\[
 \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i \rho \},
\\],
where the indices i and j run from 0 to 2 and with \\( L_0 = \sigma^{+}\\), \\( L_1 = \sigma^{-}\\) and \\(L_3 = \sigma^{z}\\).

Here the general incoherent part of the Lindblad equation is internally represented by a [struqture::spins::PlusMinusLindbladNoiseOperator].

To create a complex decoherence model first create the Lindblad noise and then turn it into a ContinuousDecoherenceModel. For a simple decoherence model, use new to create an empty model and use the `add_damping`, `add_excitation`, `add_depolarising` and `add_dephasing` methods. For more fine control access the internal `lindblad_noise` directly and modify it.

```rust
use struqture::{spins::PlusMinusProduct, SpinIndex, OperateOnDensityMatrix};
use roqoqo::noise_models::ContinuousDecoherenceModel;
// Create the noise model
let mut continuous_model = ContinuousDecoherenceModel::new();

continuous_model = continuous_model.add_damping_rate(vec![0, 1, 2], 0.001).unwrap();
continuous_model = continuous_model.add_dephasing_rate(vec![0, 1, 2], 0.0005).unwrap();
continuous_model = continuous_model.add_depolarising_rate(vec![0, 1, 2], 0.0001).unwrap();
continuous_model = continuous_model.add_excitation_rate(vec![0, 1, 2], 0.006).unwrap();

// Access the underlying struqture operator
let mut lindblad_noise = continuous_model.lindblad_noise;
lindblad_noise.add_operator_product((PlusMinusProduct::new().plus(0), PlusMinusProduct::new().plus(0)), 0.1.into()).unwrap();

// Create a new continuous model from modified lindblad noise
let new_continuous = ContinuousDecoherenceModel::from(lindblad_noise);
```

```python
from qoqo import noise_models
import numpy as np

continuous_model = noise_models.ContinuousDecoherenceModel()
continuous_model = continuous_model.add_damping_rate([0, 1, 2], 0.001)
continuous_model = continuous_model.add_dephasing_rate([0, 1, 2], 0.0005)
continuous_model = continuous_model.add_depolarising_rate([0, 1, 2], 0.0001)
continuous_model = continuous_model.add_excitation_rate([0, 1, 2], 0.0006)

# Access the underlying struqture operator
lindblad_noise = continuous_model.get_noise_operator()
lindblad_noise.add_operator_product(("0+", "0+"), 0.1)
new_continuous_model = noise_models.ContinuousDecoherenceModel(lindblad_noise)
```

For further details of the `ContinuousDecoherenceModel` trait please refer to the API documentation of [roqoqo::noise_models](https://docs.rs/roqoqo/latest/roqoqo/noise_models/index.html) (Rust core)

## DecoherenceOnGateModel

The `DecoherenceOnGateModel` is the error model for noise that is only present on gate executions.Adds additional noise when specific gates (identified by hqslang name and qubits acted on) are executed. The noise is given in the form of a [struqture::spins::PlusMinusLindbladNoiseOperator] the same way it is for the ContinuousDecoherence model.

To create a complex decoherence model, first create the Lindblad noise and then turn it into a DecoherenceOnGateModel. For a simple decoherence model, use new to create an empty model and use the `set_single_qubit_gate_error`, `set_two_qubit_gate_error`, `set_three_qubit_gate_error` and `set_multi_qubit_gate_error` methods. For more fine control access the internal `lindblad_noise` directly and modify it.

```rust
use struqture::{spins::{PlusMinusProduct, PlusMinusLindbladNoiseOperator}, SpinIndex, OperateOnDensityMatrix};
use roqoqo::noise_models::DecoherenceOnGateModel;

let mut noise_model = DecoherenceOnGateModel::new();
let mut lindblad_noise = PlusMinusLindbladNoiseOperator::new();
lindblad_noise.add_operator_product(
   (PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0)),
   0.9.into(),).unwrap();
lindblad_noise.add_operator_product(
   (PlusMinusProduct::new().z(1), PlusMinusProduct::new().z(1)),
   0.9.into(),).unwrap();

noise_model = noise_model.set_two_qubit_gate_error(
    "CNOT", 0,1,
    lindblad_noise
);
```

```python
from qoqo import noise_models
from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct
import numpy as np

noise_model = noise_models.DecoherenceOnGateModel()
lindblad_noise = PlusMinusLindbladNoiseOperator()
lindblad_noise.add_operator_product(
   (PlusMinusProduct().z(0), PlusMinusProduct().z(0)),
   0.9)
lindblad_noise.add_operator_product(
   (PlusMinusProduct().z(1), PlusMinusProduct().z(1)),
   0.9)

noise_model = noise_model.set_two_qubit_gate_error(
    "CNOT", 0,1,
    lindblad_noise
)
```

For further details of the `DecoherenceOnGateModel` trait please refer to the API documentation of [roqoqo::noise_models](https://docs.rs/roqoqo/latest/roqoqo/noise_models/index.html) (Rust core)

## ImperfectReadoutModel

The `ImperfectReadoutModel` is the noise model representing readout errors. This noise model assumes that all qubits are constantly experiencing decoherence over time (e.g. due to coupling to the environment).

Readout errors are modeled by two probabilities in this simple model. One probability to detect a 1 instead of a 0 when the quantum measurement gives 0 and one probability to detect a 0 instead of a 1 when the quantum measurement gives 1.

To create an ImperfectReadoutModel, use new to create an empty model and use the `set_error_probabilites` method, or use the `new_with_uniform_error` method.

```rust
use roqoqo::noise_models::ImperfectReadoutModel;
// Create the noise model
let model = ImperfectReadoutModel::new_with_uniform_error(3, 0.5, 0.5).unwrap();
let model = model.set_error_probabilites(2, 0.3, 0.7).unwrap();
let uniform_prob = model.prob_detect_0_as_1(&0);
assert_eq!(uniform_prob, 0.5);
let lower_prob = model.prob_detect_0_as_1(&2);
assert_eq!(lower_prob, 0.3);
let higher_prob = model.prob_detect_1_as_0(&2);
assert_eq!(higher_prob, 0.7);
```

```python
from qoqo import noise_models
import numpy as np

model = noise_models.ImperfectReadoutModel.new_with_uniform_error(3, 0.5, 0.5)
model = model.set_error_probabilites(2, 0.3, 0.7)
uniform_prob = model.prob_detect_0_as_1(0)
assert uniform_prob == 0.5
lower_prob = model.prob_detect_0_as_1(2)
assert lower_prob == 0.3
higher_prob = model.prob_detect_1_as_0(2)
assert higher_prob == 0.7
```

For further details of the `ImperfectReadoutModel` trait please refer to the API documentation of [roqoqo::noise_models](https://docs.rs/roqoqo/latest/roqoqo/noise_models/index.html) (Rust core)
