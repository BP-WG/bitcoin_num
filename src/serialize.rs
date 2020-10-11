// Bitcoin Numeric Library
// Written in 2020 by
//   Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
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

use hex::{FromHex, ToHex};
use serde::{Deserialize, Deserializer, Serializer};

/// Serializes `buffer` to a lowercase hex string.
pub fn to_hex<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&buffer.as_ref().to_hex())
}

/// Deserializes a lowercase hex string to a `Vec<u8>`.
pub fn from_hex<'de, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromHex,
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| T::from_hex(&string).map_err(|err| Error::custom(err.to_string())))
}
