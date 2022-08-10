use crate::RoqoqoBackendError;
use ndarray::Array2;
use super::Device;
/// A device assuming all-to-all connectivity between all involved qubits.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct AllToAllDevice {
    number_qubits: usize,
    single_qubit_gates: HashMap<String, HashMap<usize, f64>>,
    two_qubit_gates: HashMap<String, HashMap<(usize, usize), f64>>,
    multi_qubit_gates: HashMap<String, HashMap<Vec<usize>, f64>>,
    decoherence_rates: HashMap<usize, Array2<f64>>,
}

impl AllToAllDevice {
    /// Creates a new AllToAllDevice.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the device.
    /// * `single_qubit_gates` - A list of 'hqslang' names of single-qubit-gates supported by the device.
    /// * `two_qubit_gates` - A list of 'hqslang' names of basic two-qubit-gates supported by the device.
    /// * `default_gate_time`
    /// # Returns
    ///
    /// An initiated AllToAllDevice with single and two-qubit gates and decoherence rates set to zero.
    ///
    pub fn new(
        number_qubits: usize,
        single_qubit_gates: &[String],
        two_qubit_gates: &[String],
        default_gate_time: f64
    ) -> Self {
        // Initialization of single qubit gates with empty times
        let mut new = Self{number_qubits, single_qubit_gates:HashMap::with_capacity(single_qubit_gates.len()),
        two_qubit_gates: HashMap::with_capacity(two_qubit_gates.len()), multi_qubit_gates: HashMap::new(),
    decoherence_rates: HashMap::with_capacity(number_qubits)};
        for gate_name in single_qubit_gates{
            new = new.set_all_single_qubit_gate_times(gate_name, default_gate_time);
        }
        for gate_name in two_qubit_gates{
            new = new.set_all_two_qubit_gate_times(gate_name, default_gate_time);
        }
        new = new.set_all_qubit_decoherence_rates(Array2::zeros((3,3)));
        new
    }

    /// Function that allows to set one gate time per gate type for the single-qubit-gates.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times.
    ///
    pub fn set_all_single_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if let Some(gate_times) =  self.single_qubit_gates.get_mut(gate) {
            for (_, gatetime) in gate_times.iter_mut(){
                *gatetime = time
            }
        }
        else{
            let mut gatetimes: Vec<SingleQubitMap> = Vec::with_capacity(self.number_qubits);
            for qubit in 0..self.number_qubits{  
                gatetimes.insert(qubit, time);
            }
            self.single_qubit_gates.insert(gate.to_string, gatetimes)
        }
        self
    }

    /// Setting the gate time of a single qubit gate.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the single-qubit-gate.
    /// * `qubit` - The qubit for which the gate time is set
    /// * `gate_time` - gate time for the given gate type, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times or 
    ///
    pub fn set_single_qubit_gate_time(mut self, gate: &str, qubit: usize, gate_time: f64) -> Result<Self, RoqoqoError> {
        match  self.single_qubit_gates.get_mut(gate) {
            Some(gate_times) => {let mut gatetime = gate_times.entry(q).or_insert(gate_time)
            *gatetime = gate_time;
            Ok(self)},
            None => E
        }

    }

    /// Function that allows to set the gate time for the two-qubit-gates in AllToAllDevice.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the two-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times.
    ///
    pub fn set_all_two_qubit_gate_times(mut self, gate: &str, gate_time: f64) -> Self {
        if self.two_qubit_gates.get(&gate.to_string()).is_some() {
            let mut times: Vec<TwoQubitMap> = Vec::new();
            for qubit0 in 0..self.number_qubits {
                for qubit1 in 0..self.number_qubits {
                    if qubit0 != qubit1 {
                        let map1 = TwoQubitMap {
                            control: qubit0,
                            target: qubit1,
                            time: gate_time,
                        };
                        let map2 = TwoQubitMap {
                            control: qubit1,
                            target: qubit0,
                            time: gate_time,
                        };
                        times.push(map1);
                        times.push(map2);
                    }
                }
            }
            self.two_qubit_gates.insert(gate.to_string(), times);
        }
        self
    }

    /// Function that allows to set the gate time for the multi-qubit-gates in AllToAllDevice,
    /// when applied to any qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `gate` - hqslang name of the multi-qubit-gate.
    /// * `gate_time` - gate time for the given gate, valid for all qubits in the device.
    ///
    /// # Returns
    ///
    /// An AllToAllDevice with updated gate times.
    ///
    pub fn set_multi_qubit_gate_time(mut self, gate: &str, qubits: Vec<usize>, gate_time: f64) -> Self {
        if let Some(gate_times) =  self.multi_qubit_gates.get_mut(gate) {
            for (_, gatetime) in gate_times.get_mut(q){
                *gatetime = time
            }
        }
        else{
            let mut gatetimes: Vec<SingleQubitMap> = Vec::with_capacity(self.number_qubits);
            for qubit in 0..self.number_qubits{  
                gatetimes.insert(qubit, time);
            }
            self.single_qubit_gates.insert(gate.to_string, gatetimes)
        }
        Ok(self)
    }

    /// Function to set the decoherence rates for all qubits in the device.
    ///
    /// # Arguments
    ///
    /// * `rates` - decoherence rates for the qubits in the device, provided as a (3x3)-matrix.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` -  The device with updated decoherence rates.
    /// * `Err(RoqoqoError)` - The input parameter `rates` needs to be a (3x3)-matrix.
    ///
    pub fn set_all_qubit_decoherence_rates(
        mut self,
        rates: Array2<f64>,
    ) -> Result<Self, RoqoqoError> {
        // Check if input matrix has the dimension (3x3)
        let shape = &(*rates.shape());
        if shape == [3, 3] {
            for qubit in 0..self.number_qubits() {
                self.decoherence_rates.insert(qubit, rates.clone());
            }
            Ok(self)
        } else {
            Err(RoqoqoError::GenericError {
                msg: "The input parameter `rates` needs to be a (3x3)-matrix.".to_string(),
            })
        }
    }
}

/// Implements Device trait for AllToAllDevice.
///
/// The Device trait defines standard functions available for roqoqo devices.
///
impl Device for AllToAllDevice {
    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self.single_qubit_gates.get(hqslang) {
            Some(x) => x.get(qubit),
            None => None,
        }
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self.two_qubit_gate.get(&hqslang.to_string()) {
            Some(x) => x.get(&(*control, *target)).copied(),
            None => None,
        }
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        // variable unused in AllToAllDevice, is kept here for consistency purposes.
        
        match self.multi_qubit_gates.get(&hqslang.to_string()){
            Some(x) => {
                let qubits: Vec<qubits> = qubits.iter().copied().collect();
                x.get(qubits)
            },
            None => None
        }
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.decoherence_rates
            .get(qubit)
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                vector.push((row, column));
            }
        }
        vector
    }
}