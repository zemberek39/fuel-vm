use crate::TxId;

use fuel_types::{
    bytes::{
        SizedBytes,
        WORD_SIZE,
    },
    mem_layout,
    Bytes32,
    MemLayout,
    MemLocType,
};

use core::{
    fmt,
    str,
};

#[cfg(feature = "std")]
use fuel_types::bytes;

#[cfg(feature = "std")]
use std::io;

#[cfg(feature = "random")]
use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    Rng,
};

/// Identification of unspend transaction output.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UtxoId {
    /// transaction id
    tx_id: TxId,
    /// output index
    output_index: u8,
}

mem_layout!(UtxoIdLayout for UtxoId
    tx_id: TxId = {TxId::LEN},
    output_index: u8 = WORD_SIZE
);

impl UtxoId {
    pub const LEN: usize = <Self as MemLayout>::LEN;

    pub const fn new(tx_id: TxId, output_index: u8) -> Self {
        Self {
            tx_id,
            output_index,
        }
    }

    pub const fn tx_id(&self) -> &TxId {
        &self.tx_id
    }

    pub const fn output_index(&self) -> u8 {
        self.output_index
    }

    pub fn replace_tx_id(&mut self, tx_id: TxId) {
        self.tx_id = tx_id;
    }
}

#[cfg(feature = "random")]
impl Distribution<UtxoId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtxoId {
        let mut tx_id = Bytes32::default();
        rng.fill_bytes(tx_id.as_mut());
        UtxoId::new(tx_id, rng.gen())
    }
}

impl fmt::LowerHex for UtxoId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "{:#x}{:02x}", self.tx_id, self.output_index)
        } else {
            write!(f, "{:x}{:02x}", self.tx_id, self.output_index)
        }
    }
}

impl fmt::UpperHex for UtxoId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "{:#X}{:02X}", self.tx_id, self.output_index)
        } else {
            write!(f, "{:X}{:02X}", self.tx_id, self.output_index)
        }
    }
}

impl str::FromStr for UtxoId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "Invalid encoded byte";
        let s = s.trim_start_matches("0x");
        let utxo_id = if s.is_empty() {
            UtxoId::new(Bytes32::default(), 0)
        } else if s.len() > 2 {
            UtxoId::new(
                Bytes32::from_str(&s[..s.len() - 2])?,
                u8::from_str_radix(&s[s.len() - 2..], 16).map_err(|_| ERR)?,
            )
        } else {
            UtxoId::new(TxId::default(), u8::from_str_radix(s, 16).map_err(|_| ERR)?)
        };
        Ok(utxo_id)
    }
}

impl SizedBytes for UtxoId {
    fn serialized_size(&self) -> usize {
        Self::LEN
    }
}

#[cfg(feature = "std")]
impl io::Write for UtxoId {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        const LEN: usize = UtxoId::LEN;
        let buf: &[_; LEN] = buf
            .get(..LEN)
            .and_then(|slice| slice.try_into().ok())
            .ok_or(bytes::eof())?;

        let tx_id = bytes::restore_at(buf, Self::layout(Self::LAYOUT.tx_id));
        let output_index =
            bytes::restore_u8_at(buf, Self::layout(Self::LAYOUT.output_index));

        self.tx_id = tx_id.into();
        self.output_index = output_index;

        Ok(Self::LEN)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl io::Read for UtxoId {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        const LEN: usize = UtxoId::LEN;
        let buf: &mut [_; LEN] = buf
            .get_mut(..LEN)
            .and_then(|slice| slice.try_into().ok())
            .ok_or(bytes::eof())?;

        bytes::store_at(buf, Self::layout(Self::LAYOUT.tx_id), &self.tx_id);
        bytes::store_number_at(
            buf,
            Self::layout(Self::LAYOUT.output_index),
            self.output_index,
        );

        Ok(Self::LEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;
    use fuel_types::Bytes32;

    #[test]
    fn fmt_utxo_id() {
        let mut tx_id = Bytes32::zeroed();
        *tx_id.get_mut(0).unwrap() = 12;
        *tx_id.get_mut(31).unwrap() = 11;

        let utxo_id = UtxoId {
            tx_id,
            output_index: 26,
        };
        assert_eq!(
            format!("{utxo_id:#x}"),
            "0x0c0000000000000000000000000000000000000000000000000000000000000b1a"
        );
        assert_eq!(
            format!("{utxo_id:x}"),
            "0c0000000000000000000000000000000000000000000000000000000000000b1a"
        );
    }

    #[test]
    fn from_str_utxo_id() -> Result<(), &'static str> {
        let utxo_id = UtxoId::from_str(
            "0x0c0000000000000000000000000000000000000000000000000000000000000b1a",
        )?;

        assert_eq!(utxo_id.output_index, 26);
        assert_eq!(utxo_id.tx_id[31], 11);
        assert_eq!(utxo_id.tx_id[0], 12);
        Ok(())
    }
}
