// Bitcoin Numeric Library
// Written in 2018 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Functions for endianess conversions; required to support old Rust
//! compiler versions

macro_rules! define_slice_to_be {
    ($name: ident, $type: ty, $type_string: meta) => {
        #[doc = "Converts `&[u8]` slice in big endian encoding (most significant byte first) into `"]
        #[$type_string]
        #[doc = "`. Panics if the slice length is not equal to the size of the type"]
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << (::core::mem::size_of::<$type>() - i - 1) * 8;
            }
            res
        }
    };
}
macro_rules! define_slice_to_le {
    ($name: ident, $type: ty, $type_string: meta) => {
        #[doc = "Converts `&[u8]` slice in little endian encoding (least significant byte first) into `"]
        #[$type_string]
        #[doc = "`. Panics if the slice length is not equal to the size of the type"]
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << i * 8;
            }
            res
        }
    };
}
macro_rules! define_be_to_array {
    ($name: ident, $type: ty, $byte_len: expr, $type_string: meta) => {
        #[doc = "Converts `"]
        #[$type_string]
        #[doc = "` into a fixed-size array using big endian encoding (most significant byte first)."]
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> ($byte_len - i - 1) * 8) & 0xff) as u8;
            }
            res
        }
    };
}
macro_rules! define_le_to_array {
    ($name: ident, $type: ty, $byte_len: expr, $type_string: meta) => {
        #[doc = "Converts `"]
        #[$type_string]
        #[doc = "` into a fixed-size array using little endian encoding (least significant byte first)."]
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> i * 8) & 0xff) as u8;
            }
            res
        }
    };
}

define_slice_to_be!(slice_to_u16_be, u16, doc = "u16");
define_slice_to_be!(slice_to_u32_be, u32, doc = "u32");
define_slice_to_be!(slice_to_u64_be, u64, doc = "u64");
define_be_to_array!(u16_to_array_be, u16, 2, doc = "u16");
define_be_to_array!(u32_to_array_be, u32, 4, doc = "u32");
define_be_to_array!(u64_to_array_be, u64, 8, doc = "u64");
define_slice_to_le!(slice_to_u16_le, u16, doc = "u16");
define_slice_to_le!(slice_to_u32_le, u32, doc = "u32");
define_slice_to_le!(slice_to_u64_le, u64, doc = "u64");
define_le_to_array!(u16_to_array_le, u16, 2, doc = "u16");
define_le_to_array!(u32_to_array_le, u32, 4, doc = "u32");
define_le_to_array!(u64_to_array_le, u64, 8, doc = "u64");

define_slice_to_be!(slice_to_i16_be, i16, doc = "i16");
define_slice_to_be!(slice_to_i32_be, i32, doc = "i32");
define_slice_to_be!(slice_to_i64_be, i64, doc = "i64");
define_be_to_array!(i16_to_array_be, i16, 2, doc = "i16");
define_be_to_array!(i32_to_array_be, i32, 4, doc = "i32");
define_be_to_array!(i64_to_array_be, i64, 8, doc = "i64");
define_slice_to_le!(slice_to_i16_le, i16, doc = "i16");
define_slice_to_le!(slice_to_i32_le, i32, doc = "i32");
define_slice_to_le!(slice_to_i64_le, i64, doc = "i64");
define_le_to_array!(i16_to_array_le, i16, 2, doc = "i16");
define_le_to_array!(i32_to_array_le, i32, 4, doc = "i32");
define_le_to_array!(i64_to_array_le, i64, 8, doc = "i64");

macro_rules! define_chunk_slice_to_int {
    ($name: ident, $type: ty, $converter: ident, $type_string: meta, $converter_string: meta) => {
        #[doc = "Converts byte string into an array of `"]
        #[$type_string]
        #[doc = "` using `"]
        #[$converter_string]
        #[doc = "` encoding function."]
        #[inline]
        pub fn $name(inp: &[u8], outp: &mut [$type]) {
            assert_eq!(inp.len(), outp.len() * ::core::mem::size_of::<$type>());
            for (outp_val, data_bytes) in outp
                .iter_mut()
                .zip(inp.chunks(::core::mem::size_of::<$type>()))
            {
                *outp_val = $converter(data_bytes);
            }
        }
    };
}

define_chunk_slice_to_int!(
    bytes_to_u64_slice_le,
    u64,
    slice_to_u64_le,
    doc = "u64",
    doc = "bytes_to_u64_slice_le"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endianness_test() {
        assert_eq!(slice_to_u16_be(&[0xde, 0xad]), 0xdead);
        assert_eq!(slice_to_u32_be(&[0xde, 0xad, 0xbe, 0xef]), 0xdeadbeef);
        assert_eq!(
            slice_to_u64_be(&[0xde, 0xad, 0xbe, 0xef, 0x1b, 0xad, 0xca, 0xfe]),
            0xdeadbeef1badcafe
        );
        assert_eq!(u16_to_array_be(0xbeef), [0xbe, 0xef]);
        assert_eq!(u32_to_array_be(0xdeadbeef), [0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(
            u64_to_array_be(0x1badcafedeadbeef),
            [0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef]
        );

        assert_eq!(slice_to_u16_le(&[0xad, 0xde]), 0xdead);
        assert_eq!(slice_to_u32_le(&[0xef, 0xbe, 0xad, 0xde]), 0xdeadbeef);
        assert_eq!(
            slice_to_u64_le(&[0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]),
            0x1badcafedeadbeef
        );
        assert_eq!(u16_to_array_le(0xdead), [0xad, 0xde]);
        assert_eq!(u32_to_array_le(0xdeadbeef), [0xef, 0xbe, 0xad, 0xde]);
        assert_eq!(
            u64_to_array_le(0x1badcafedeadbeef),
            [0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]
        );
    }

    #[test]
    fn endian_chunk_test() {
        let inp = [
            0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b, 0xfe, 0xca, 0xad, 0x1b, 0xce, 0xfa,
            0x01, 0x02,
        ];
        let mut out = [0; 2];
        bytes_to_u64_slice_le(&inp, &mut out);
        assert_eq!(out, [0x1badcafedeadbeef, 0x0201face1badcafe]);
    }
}
