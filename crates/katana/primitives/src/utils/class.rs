use anyhow::Result;
use cairo_lang_starknet_classes::casm_contract_class::CasmContractClass;
use cairo_lang_starknet_classes::contract_class::ContractClass;

use crate::contract::{
    CompiledContractClass, CompiledContractClassV0, CompiledContractClassV1, SierraClass,
};

const MAX_BYTECODE_SIZE: usize = 180000;

/// Parse a [`str`] into a [`CompiledContractClass`].
pub fn parse_compiled_class(class: &str) -> Result<CompiledContractClass> {
    if let Ok(class) = parse_compiled_class_v1(class) {
        Ok(CompiledContractClass::V1(class))
    } else {
        Ok(CompiledContractClass::V0(parse_compiled_class_v0(class)?))
    }
}

/// Parse a [`str`] into a [`CompiledContractClassV1`].
pub fn parse_compiled_class_v1(class: &str) -> Result<CompiledContractClassV1> {
    let class: ContractClass = serde_json::from_str(class)?;
    let class = CasmContractClass::from_contract_class(class, true, MAX_BYTECODE_SIZE)?;
    Ok(CompiledContractClassV1::try_from(class)?)
}

/// Parse a [`str`] into a [`CompiledContractClassV0`].
pub fn parse_compiled_class_v0(class: &str) -> Result<CompiledContractClassV0, serde_json::Error> {
    serde_json::from_str(class)
}

/// Parse a [`str`] into a [`SierraClass`].
pub fn parse_sierra_class(class: &str) -> Result<SierraClass, serde_json::Error> {
    serde_json::from_str(class)
}
