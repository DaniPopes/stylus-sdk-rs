// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

use super::{AbiType, ConstString};
use alloy_primitives::FixedBytes;
use alloy_sol_types::sol_data::{ByteCount, SupportedFixedBytes};

pub use alloy_sol_types::sol_data::FixedBytes as FixedBytesSolType;

impl<const N: usize> AbiType for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    type SolType = FixedBytesSolType<N>;

    const ABI: ConstString = ConstString::new("bytes").concat(ConstString::from_decimal_number(N));
}
