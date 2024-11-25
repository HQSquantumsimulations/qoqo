# Pragma Operations

Pragma operations in qoqo/roqoqo are operations that are _not_ part of the set of operations that can run on all universal quantum computers.

Pragma operations can be used to:

* Annotate a quantum circuit with additional information that is not necessary for execution (e.g. `PragmaGlobalPhase`, `PragmaStartDecompositionBlock`)
* Apply operations that lead to a repeated execution of a circuit (`PragmaRepeatedMeasurement`, `PragmaSetNumberOfMeasurements`)
* Apply operations that are only available on specific hardware (e.g. `PragmaChangeDevice`, `PragmaSleep`)
* Apply operations that are only available on a simulator (e.g. `PragmaSetStateVector`, `PragmaGetStateVector`)
* Model noise ([see also](noise.md))
* Model error sources (`PragmaOverrotation`)

For a full list of available Pragma operations see the API documentation of [roqoqo](https://docs.rs/roqoqo/latest/roqoqo/operations/index.html) 
 and [qoqo](https://hqsquantumsimulations.github.io/qoqo/generated/generated/qoqo.operations.html).
