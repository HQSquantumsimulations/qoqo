// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Noise models for qoqo/roqoqo
//!
//! A collection of noise models that describe the physical noise present of quantum computers.
//!

mod continuous_decoherence;
pub use continuous_decoherence::ContinuousDecoherenceModel;
mod imperfect_readout;
pub use imperfect_readout::ImperfectReadoutModel;
mod decoherence_on_gate;
use super::operations::SupportedVersion;
pub use decoherence_on_gate::DecoherenceOnGateModel;

/// Collection of all available noise models in this version of qoqo/roqoqo
///
/// Intended as common interface to exchange noise models.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum NoiseModel {
    /// Continuous decoherence model
    ContinuousDecoherenceModel(ContinuousDecoherenceModel),
    /// Readout error model (probabilities to measure 0 instead of 1 and vice-versa).
    ImperfectReadoutModel(ImperfectReadoutModel),
    /// additional error only when applying gate
    DecoherenceOnGateModel(DecoherenceOnGateModel),
}

impl From<ContinuousDecoherenceModel> for NoiseModel {
    fn from(value: ContinuousDecoherenceModel) -> Self {
        Self::ContinuousDecoherenceModel(value)
    }
}

impl From<ImperfectReadoutModel> for NoiseModel {
    fn from(value: ImperfectReadoutModel) -> Self {
        Self::ImperfectReadoutModel(value)
    }
}

impl From<DecoherenceOnGateModel> for NoiseModel {
    fn from(value: DecoherenceOnGateModel) -> Self {
        Self::DecoherenceOnGateModel(value)
    }
}

impl SupportedVersion for NoiseModel {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        match self {
            NoiseModel::ContinuousDecoherenceModel(internal) => {
                internal.minimum_supported_roqoqo_version()
            }
            NoiseModel::ImperfectReadoutModel(internal) => {
                internal.minimum_supported_roqoqo_version()
            }
            NoiseModel::DecoherenceOnGateModel(internal) => {
                internal.minimum_supported_roqoqo_version()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_supported_roqoqo_version_continuous() {
        let continuous_decoherence = ContinuousDecoherenceModel::new();
        let noise_model: NoiseModel = continuous_decoherence.into();
        assert_eq!(noise_model.minimum_supported_roqoqo_version(), (1, 6, 0));
    }
    #[test]
    fn minimum_supported_roqoqo_version_on_gate() {
        let noise = DecoherenceOnGateModel::new();
        let noise_model: NoiseModel = noise.into();
        assert_eq!(noise_model.minimum_supported_roqoqo_version(), (1, 6, 0));
    }
    #[test]
    fn minimum_supported_roqoqo_version_readout() {
        let noise = ImperfectReadoutModel::new();
        let noise_model: NoiseModel = noise.into();
        assert_eq!(noise_model.minimum_supported_roqoqo_version(), (1, 6, 0));
    }
}
