// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

mod define_operations;
pub use define_operations::*;

mod measurement_operations;
pub use measurement_operations::*;

mod operation_conversions;
pub use operation_conversions::*;

mod pragma_operations;
pub use pragma_operations::*;

mod multi_qubit_gate_operations;
pub use multi_qubit_gate_operations::*;

mod single_qubit_gate_operations;
pub use single_qubit_gate_operations::*;

mod two_qubit_gate_operations;
pub use two_qubit_gate_operations::*;

mod three_qubit_gate_operations;
pub use three_qubit_gate_operations::*;

use pyo3::prelude::*;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;

// helper function to convert CalculatorFloat into a python object
pub fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}
