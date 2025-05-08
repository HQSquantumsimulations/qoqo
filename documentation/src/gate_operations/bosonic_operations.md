# Bosonic operations
The bosonic operations manipulate the quantum state of harmonic oscillator modes. They include single-mode gates—such as squeezing (altering quadrature uncertainties), phase displacement and phase shift (rotating states in phase space)—and a two-mode beam splitter (mixing states between modes). Additionally, a photon detection measurement operation is provided for resolving mode occupancy, enabling state readout at the single-photon level.


## BeamSplitter
The 2-mode beam splitter which splits a beam with a transmission amplitude cos(θ) and a reflection amplitude exp(i * φ) * sin(θ).

## PhaseDisplacement
The single-mode phase-displacement gate with variable magnitude and phase.

## PhaseShift
The single-mode phase-shift gate with variable phase, given by R(θ) = exp(i * θ * 𝑁̂).

## PhotonDetection
The photon number-resolving detector measurement for bosons.

## Squeezing
The single-mode squeezing gate with tunable squeezing.
