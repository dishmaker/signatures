//! Hexadecimal encoding support
use crate::{Error, Signature};
use core::{fmt, str};

impl fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for component in [&self.R, &self.s] {
            for byte in component {
                write!(f, "{byte:02x}")?;
            }
        }
        Ok(())
    }
}

impl fmt::UpperHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for component in [&self.R, &self.s] {
            for byte in component {
                write!(f, "{byte:02X}")?;
            }
        }
        Ok(())
    }
}

/// Decode a signature from hexadecimal.
///
/// Upper and lower case hexadecimal are both accepted, however mixed case is
/// rejected.
impl str::FromStr for Signature {
    type Err = Error;

    fn from_str(hex: &str) -> signature::Result<Self> {
        if hex.len() != Signature::BYTE_SIZE * 2 {
            return Err(Error::new());
        }

        let mut upper_case = None;

        // Ensure all characters are valid and case is not mixed
        for &byte in hex.as_bytes() {
            match byte {
                b'0'..=b'9' => (),
                b'a'..=b'z' => match upper_case {
                    Some(true) => return Err(Error::new()),
                    Some(false) => (),
                    None => upper_case = Some(false),
                },
                b'A'..=b'Z' => match upper_case {
                    Some(true) => (),
                    Some(false) => return Err(Error::new()),
                    None => upper_case = Some(true),
                },
                _ => return Err(Error::new()),
            }
        }

        let mut result = [0u8; Self::BYTE_SIZE];
        for (digit, byte) in hex.as_bytes().chunks_exact(2).zip(result.iter_mut()) {
            *byte = str::from_utf8(digit)
                .ok()
                .and_then(|s| u8::from_str_radix(s, 16).ok())
                .ok_or_else(Error::new)?;
        }

        Self::try_from(&result[..])
    }
}
