use sp_core::U256;

pub trait GetCompact {
    fn get_compact(self) -> Option<u32>;
}

// https://github.com/bitcoin/bitcoin/blob/7fcf53f7b4524572d1d0c9a5fdc388e87eb02416/src/arith_uint256.cpp#L223
impl GetCompact for U256 {
    fn get_compact(self) -> Option<u32> {
        let mut size = (self.bits() + 7) / 8;
        let mut compact = if size <= 3 {
            (self.low_u64() << (8 * (3 - size))) as u32
        } else {
            let bn = self >> (8 * (size - 3));
            bn.low_u32()
        };

        if (compact & 0x00800000) != 0 {
            compact >>= 8;
            size += 1;
        }

        if !(compact & !0x007fffff == 0) {
            None
        } else if !(size < 256) {
            None
        } else {
            Some(compact | (size << 24) as u32)
        }
    }
}

pub trait SetCompact {
    fn set_compact(value: u32) -> Option<Self>
    where
        Self: Sized;
}

// https://github.com/bitcoin/bitcoin/blob/7fcf53f7b4524572d1d0c9a5fdc388e87eb02416/src/arith_uint256.cpp#L203
impl SetCompact for U256 {
    fn set_compact(compact: u32) -> Option<Self>
    where
        Self: Sized,
    {
        let size = compact >> 24;
        let word = compact & 0x007fffff;

        if word != 0 && (compact & 0x00800000) != 0 {
            // negative
            None
        } else if word != 0 && ((size > 34) || (word > 0xff && size > 33) || (word > 0xffff && size > 32)) {
            // overflow
            None
        } else {
            let word = U256::from(word);
            Some(if size <= 3 {
                word >> 8 * (3 - size)
            } else {
                word << 8 * (size - 3)
            })
        }
    }
}
