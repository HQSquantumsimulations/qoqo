# Four-qubit gates

Four-qubit gates in qoqo/roqoqo represent atomic instructions of any quantum computer that act on gour qubits. In three-qubit gates the qubits are referred to as `control_0`, `control_1`, `control_2` and `target`. For four-qubit gates, the created matrix is always a 16x16-dimensional matrix. This convention corresponds to the little-endian encoding as described in chapter [conventions](../conventions.md). The full matrix form of the four-qubit gates implemented in qoqo/roqoqo is documented in this chapter.

The given form of the unitary matrix is consistent with the following ordering of qubit states in a four-qubit state space:

 \\[
 \left|0000 \right>  =  \textrm{state} 0 \\\\
 \left|0001 \right>  =  \textrm{state} 1 \\\\
 \left|0010 \right>  =  \textrm{state} 2 \\\\
 \left|0011 \right>  =  \textrm{state} 3 \\\\
 \left|0100 \right>  =  \textrm{state} 4 \\\\
 \left|0101 \right>  =  \textrm{state} 5 \\\\
 \left|0110 \right>  =  \textrm{state} 6 \\\\
 \left|0111 \right>  =  \textrm{state} 7 \\\\
 \left|1000 \right>  =  \textrm{state} 8 \\\\
 \left|1001 \right>  =  \textrm{state} 9 \\\\
 \left|1010 \right>  =  \textrm{state} 10 \\\\
 \left|1011 \right>  =  \textrm{state} 11 \\\\
 \left|1100 \right>  =  \textrm{state} 12 \\\\
 \left|1101 \right>  =  \textrm{state} 13 \\\\
 \left|1110 \right>  =  \textrm{state} 14 \\\\
 \left|1111 \right>  =  \textrm{state} 15 \\\\
 \\]

## TripleControlledPauliX
The triple-controlled PauliX gate.

## TripleControlledPauliZ
The triple-controlled PauliZ gate.

## TripleControlledPhaseShift
The triple-controlled PhaseShift gate.
