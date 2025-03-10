# Spin-Boson operations
The spin-boson operations facilitate interactions between two-level systems (qubits) and quantized harmonic oscillators (bosonic modes). They implement core models such as the quantum Rabi, Jaynes–Cummings, and longitudinal coupling interactions, as well as operations for transferring single excitations between qubits and resonators. This framework is essential for simulating hybrid quantum systems and implementing quantum logic that leverages both spin and bosonic degrees of freedom.


## CZQubitResonator
Controlled-Z operation between a qubit and a bosonic mode.

## JaynesCummings
The Jaynes-Cummings gate exp(-i * θ * (σ^- * b^† + σ^+ * b))

## LongitudinalCoupling
Longitudinal coupling gate exp(-i * θ * Z * (b^† + b))

## QuantumRabi
The quantum Rabi interaction exp(-i * θ * X * (b^† + b))

## SingleExcitationLoad
Loads a single excitation from a bosonic mode into a qubit.

## SingleExcitationStore
Stores a single excitation from the involved qubit into the involved bosonic mode.
