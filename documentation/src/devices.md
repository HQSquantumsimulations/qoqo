# Devices

When working with quantum circuits it is often necessary to know the topology of a target quantum device. Device properties can also be used by backends, for example to accurately simulate a given quantum device.
qoqo/roqoqo defines an interface for obtaining the device topology. The interface is defined by roqoqo's `Device` trait. Additionally qoqo/roqoqo provides some simple devices that can be used to quickly define simple device topologies.

Devices based on the roqoqo `Device` trait can be abstract devices or backend devices.

**Abstract devices** contain abstract information about the device topology and the available gates.

**Backend devices** are devices that are implemented by a roqoqo [backend](backends.md). They can specify additional information for accessing the device on the backend and can contain additional information. The devices also contain all the information of the abstract devices.

A typical example for abstract devices are linear chains of square lattices in which two-qubit gate operations are only available between neighboring qubits.

It is defined by the decoherence rates `M` and the (pseudo-)time needed to execute a quantum operation.

The matrix representation of the decoherence rates of the Lindblad equation can be obtained by calling the method `qubit_decoherence_rates()` of a device.

The time required for a gate operation can be obtained from the methods `single_qubit_gate_time()`, `two_qubit_gate_time()`, and `multi_qubit_gate_time()` for a specific type of gate (defined by its name) and the qubits the gate should act on.
The gate time method can also be used to query the topology and available gate operations on a device. If a specific type of gate operation is not available on the given qubits, the gate time method will return `None`.

For further details of the `Device` trait please refer to the API documentation of [roqoqo::devices](https://docs.rs/roqoqo/latest/roqoqo/devices/index.html) (Rust core)

## Simple Devices

qoqo/roqoqo provide three simple devices

* `GenericDevice`
* `AllToAllDevice`
* `SquareLatticeDevice`

The `GenericDevice` is the most basic device. It simply contains all available gate operations, the corresponding gate times and the decoherence rate for each qubit in internal HashMaps. It can be used to create custom devices and as a device interchange format. As part of the `Device` interface, each device can be exported as a `GenericDevice` with the `to_generic_device` function. The `GenericDevice` is also used to exchange device data via JSON files or REST API calls.

```rust
use roqoqo::devices::Device;
use roqoqo::devices::{GenericDevice, AllToAllDevice};
use ndarray::array;
// Create a two-qubit device
let mut generic_device = GenericDevice::new(2);
// Create a comparison two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
let all_to_all = AllToAllDevice::new(2, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);

generic_device.set_single_qubit_gate_time("RotateZ", 0, 1.0).unwrap();
generic_device.set_single_qubit_gate_time("RotateZ", 1, 1.0).unwrap();
generic_device.set_two_qubit_gate_time("CNOT", 0, 1, 1.0).unwrap();
generic_device.set_two_qubit_gate_time("CNOT", 1, 0, 1.0).unwrap();
assert_eq!(generic_device, all_to_all.to_generic_device());
```

```python
from qoqo import devices
import numpy as np

# Create a two-qubit device
generic_device = devices.GenericDevice(2)
# Create a comparison two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
all_to_all = devices.AllToAllDevice(2, ["RotateZ"], ["CNOT"], 1.0)

generic_device.set_single_qubit_gate_time("RotateZ", 0, 1.0)
generic_device.set_single_qubit_gate_time("RotateZ", 1, 1.0)
generic_device.set_two_qubit_gate_time("CNOT", 0, 1, 1.0)
generic_device.set_two_qubit_gate_time("CNOT", 1, 0, 1.0)
assert generic_device == all_to_all.generic_device()
```

The `AllToAllDevice` can be used to quickly create a device with all-to-all connectivity. Additionally to the `set_single_qubit_time` type functions which are identical to the `GenericDevice`, it provides functions to set the gate time on *all* gates of a certain type and set the decoherence rates of *all* qubits. When setting these attributes for *all* of the qubits on the device, the `AllToAllDevice` uses a builder pattern, in order for the user to be able to chain such calls. This is demonstrated below.

```rust
use roqoqo::devices::Device;
use roqoqo::devices::{GenericDevice, AllToAllDevice};
use ndarray::array;

// Create a two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
let mut all_to_all = AllToAllDevice::new(2, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);
// Set a new time for all RotateZ gates
let mut all_to_all = all_to_all.set_all_single_qubit_gate_times("RotateZ", 2.0);
// Set a new time for all CNOT gates
let mut all_to_all = all_to_all.set_all_two_qubit_gate_times("CNOT", 0.1);
```

```python
from qoqo import devices
import numpy as np

# Create a two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
all_to_all = devices.AllToAllDevice(2, ["RotateZ"], ["CNOT"], 1.0)

# Set a new time for all RotateZ gates and CNOT gates
all_to_all = all_to_all.set_all_single_qubit_gate_times("RotateZ", 2.0).set_all_two_qubit_gate_times("CNOT", 0.1)
```

The `SquareLatticeDevice` can be used to quickly initialize a device with two-qubit operations available between next-neighbours on a square lattice. The same methods as `AllToAllDevice` are available.

```rust
use roqoqo::devices::Device;
use roqoqo::devices::{SquareLatticeDevice};
let rows = 1;
let columns = 2;
// Create a two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
let square_lattice = SquareLatticeDevice::new(rows, columns, &["RotateZ".to_string()], &["CNOT".to_string()], 1.0);
```

```python
from qoqo import devices

rows = 1
columns = 2

# Create a two-qubit device with `RotateZ` and `CNOT` as the only gates and 1.0 as the default gate time
square_lattice = devices.SquareLatticeDevice(rows, columns, ["RotateZ"], ["CNOT"], 1.0)
```
