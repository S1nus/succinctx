use plonky2::hash::hash_types::RichField;
use serde::{Deserialize, Serialize};

/// A serializable struct containing the function input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInput {
    pub bytes: Option<String>,
    pub elements: Option<Vec<u64>>,
}

impl FunctionInput {
    /// If the input is using evm io, this returns the input bytes.
    pub fn bytes(&self) -> Vec<u8> {
        let bytes = self.bytes.as_ref().unwrap();
        hex::decode(bytes).unwrap()
    }

    /// If the input is using field io, this returns the input elements.
    pub fn elements<F: RichField>(&self) -> Vec<F> {
        let elements = self.elements.as_ref().unwrap();
        elements.iter().map(|e| F::from_canonical_u64(*e)).collect()
    }
}

/// A serializable struct containing the function output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionOutput {
    pub bytes: Option<String>,
    pub elements: Option<Vec<u64>>,
    pub proof: String,
}