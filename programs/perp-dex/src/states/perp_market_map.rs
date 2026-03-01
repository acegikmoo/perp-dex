use crate::error::ErrorCode;
use crate::states::PerpMarket;
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

pub struct PerpMarketMap(pub BTreeMap<u16, PerpMarket>);

impl PerpMarketMap {
    pub fn get_ref(&self, idx: u16) -> Option<&PerpMarket> {
        self.0.get(&idx)
    }

    pub fn get_mut(&mut self, idx: u16) -> Option<&mut PerpMarket> {
        self.0.get_mut(&idx)
    }

    pub fn try_from_slice(data: &[u8]) -> Result<Self> {
        let mut cursor = std::io::Cursor::new(data);
        Self::deserialize_reader(&mut cursor).map_err(|_| error!(ErrorCode::InvalidMarketIndex))
    }
}

impl AnchorSerialize for PerpMarketMap {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        (self.0.len() as u32).serialize(writer)?;
        for (k, v) in &self.0 {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }
        Ok(())
    }
}

impl AnchorDeserialize for PerpMarketMap {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len = u32::deserialize_reader(reader)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            let k = u16::deserialize_reader(reader)?;
            let v = PerpMarket::deserialize_reader(reader)?;
            map.insert(k, v);
        }
        Ok(PerpMarketMap(map))
    }
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let len = u32::deserialize(buf)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            map.insert(u16::deserialize(buf)?, PerpMarket::deserialize(buf)?);
        }
        Ok(PerpMarketMap(map))
    }
}
