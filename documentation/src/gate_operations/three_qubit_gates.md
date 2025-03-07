# Three-qubit gates

Three-qubit gates in qoqo/roqoqo represent atomic instructions of any quantum computer that act on a trio of qubits. In three-qubit gates, depending on the operation, the three qubits may either be split into two controlling qubits and a target one (`control_0`, `control_1`, `target`) or in one controlling qubit and two targeting ones (`control`, `target_0`, `target_1`). For three-qubit gates, the created matrix is always a 8x8-dimensional matrix. This convention corresponds to the little-endian encoding as described in chapter [conventions](../conventions.md). The full matrix form of the three-qubit gates implemented in qoqo/roqoqo is documented in this chapter.

The given form of the unitary matrix is consistent with the following ordering of qubit states in a three-qubit state space:

 \\[
 \left|000 \right>  =  \textrm{state} 0 \\\\
 \left|001 \right>  =  \textrm{state} 1 \\\\
 \left|010 \right>  =  \textrm{state} 2 \\\\
 \left|011 \right>  =  \textrm{state} 3 \\\\
 \left|100 \right>  =  \textrm{state} 4 \\\\
 \left|101 \right>  =  \textrm{state} 5 \\\\
 \left|110 \right>  =  \textrm{state} 6 \\\\
 \left|111 \right>  =  \textrm{state} 7 \\\\
 \\]

## ControlledControlledPauliZ
Implements the double-controlled PauliZ gate.

## ControlledControlledPhaseShift
Implements the double-controlled PhaseShift gate.

## ControlledSWAP
Implements the controlled SWAP gate.

## PhaseShiftedControlledControlledPhase
Implements the double-controlled phase-shifted PhaseShift gate.

## PhaseShiftedControlledControlledZ	
Implements the double-controlled phase-shifted PauliZ gate.

## Toffoli
Implements the Toffoli gate.