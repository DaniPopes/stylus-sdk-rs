// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

use crate::abi::{AbiType, ConstString};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use alloy_sol_types::{abi::token::PackedSeqToken, private::SolTypeValue, sol_data, SolType};
use core::ops::{Deref, DerefMut};

/// Represents a [`bytes`] in Solidity.
///
/// [`bytes`]: https://docs.soliditylang.org/en/latest/types.html#bytes-and-string-as-arrays
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);

impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(b: Vec<u8>) -> Self {
        Self(b)
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Provides a corresponding [`SolType`] for an [`abi`] [`Bytes`].
///
/// [`abi`]: crate::abi
pub struct BytesSolType;

#[doc(hidden)]
impl SolTypeValue<BytesSolType> for Bytes {
    fn stv_to_tokens(&self) -> PackedSeqToken<'_> {
        <Vec<u8> as SolTypeValue<sol_data::Bytes>>::stv_to_tokens(&self.0)
    }

    fn stv_abi_encoded_size(&self) -> usize {
        <Vec<u8> as SolTypeValue<sol_data::Bytes>>::stv_abi_encoded_size(&self.0)
    }

    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        <Vec<u8> as SolTypeValue<sol_data::Bytes>>::stv_abi_encode_packed_to(&self.0, out)
    }

    fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
        <Vec<u8> as SolTypeValue<sol_data::Bytes>>::stv_eip712_data_word(&self.0)
    }
}

impl SolType for BytesSolType {
    type RustType = Bytes;

    type TokenType<'a> = PackedSeqToken<'a>;

    fn sol_type_name() -> Cow<'static, str> {
        sol_data::Bytes::sol_type_name()
    }

    fn valid_token(token: &Self::TokenType<'_>) -> bool {
        sol_data::Bytes::valid_token(token)
    }

    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        sol_data::Bytes::detokenize(token).into()
    }
}

impl AbiType for Bytes {
    type SolType = BytesSolType;

    const ABI: ConstString = ConstString::new("bytes");

    const EXPORT_ABI_ARG: ConstString = Self::ABI.concat(ConstString::new(" calldata"));

    const EXPORT_ABI_RET: ConstString = Self::ABI.concat(ConstString::new(" memory"));
}
