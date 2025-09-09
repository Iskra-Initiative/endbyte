#![no_std]
#![doc = include_str!("../readme.md")]

/// represents the byte order of the host system
#[derive(Debug, PartialEq, Eq)]
pub enum EndiannessType {
    /// big endian byte order (most significant byte first)
    BigEndian,
    /// little endian byte order (least significant byte first)
    LittleEndian,
}

#[inline]
fn get_local_endianness() -> EndiannessType {
    #[cfg(target_endian = "big")]
    {
        EndiannessType::BigEndian
    }
    #[cfg(target_endian = "little")]
    {
        EndiannessType::LittleEndian
    }
}

/// trait for converting between host and network byte orders
///
/// this trait provides methods to convert integer values between the host's
/// native byte order and specific endianness formats (big endian or little endian).
///
/// # examples
///
/// ```
/// use endianness::Endianness;
///
/// let value = 0x1234u16;
/// let big_endian = value.host_to_big_endian();
/// let little_endian = value.host_to_little_endian();
///
/// // round-trip conversions should always work
/// assert_eq!(big_endian.big_endian_to_host(), value);
/// assert_eq!(little_endian.little_endian_to_host(), value);
/// ```
pub trait Endianness {
    /// convert from host byte order to big endian
    fn host_to_big_endian(self) -> Self;

    /// convert from host byte order to little endian
    fn host_to_little_endian(self) -> Self;

    /// convert from big endian to host byte order
    fn big_endian_to_host(self) -> Self;

    /// convert from little endian to host byte order
    fn little_endian_to_host(self) -> Self;
}

// Macro to implement Endianness for unsigned integers
macro_rules! impl_endianness_unsigned {
    ($($t:ty),*) => {
        $(
            impl Endianness for $t {
                fn host_to_big_endian(self) -> Self {
                    match get_local_endianness() {
                        EndiannessType::BigEndian => self,
                        EndiannessType::LittleEndian => self.swap_bytes(),
                    }
                }

                fn host_to_little_endian(self) -> Self {
                    match get_local_endianness() {
                        EndiannessType::BigEndian => self.swap_bytes(),
                        EndiannessType::LittleEndian => self,
                    }
                }

                fn big_endian_to_host(self) -> Self {
                    match get_local_endianness() {
                        EndiannessType::BigEndian => self,
                        EndiannessType::LittleEndian => self.swap_bytes(),
                    }
                }

                fn little_endian_to_host(self) -> Self {
                    match get_local_endianness() {
                        EndiannessType::BigEndian => self.swap_bytes(),
                        EndiannessType::LittleEndian => self,
                    }
                }
            }
        )*
    };
}

// Special implementation for single-byte types (no byte swapping needed)
impl Endianness for u8 {
    fn host_to_big_endian(self) -> Self {
        self
    }

    fn host_to_little_endian(self) -> Self {
        self
    }

    fn big_endian_to_host(self) -> Self {
        self
    }

    fn little_endian_to_host(self) -> Self {
        self
    }
}

impl Endianness for i8 {
    fn host_to_big_endian(self) -> Self {
        self
    }

    fn host_to_little_endian(self) -> Self {
        self
    }

    fn big_endian_to_host(self) -> Self {
        self
    }

    fn little_endian_to_host(self) -> Self {
        self
    }
}

// Implement for multi-byte unsigned integers
impl_endianness_unsigned!(u16, u32, u64, u128);

// Macro to implement Endianness for signed integers
macro_rules! impl_endianness_signed {
    ($($t:ty),*) => {
        $(
            impl Endianness for $t {
                fn host_to_big_endian(self) -> Self {
                    <$t>::from_ne_bytes(
                        (self as <$t as ToUnsigned>::Unsigned)
                            .host_to_big_endian()
                            .to_ne_bytes()
                    )
                }

                fn host_to_little_endian(self) -> Self {
                    <$t>::from_ne_bytes(
                        (self as <$t as ToUnsigned>::Unsigned)
                            .host_to_little_endian()
                            .to_ne_bytes()
                    )
                }

                fn big_endian_to_host(self) -> Self {
                    <$t>::from_ne_bytes(
                        (self as <$t as ToUnsigned>::Unsigned)
                            .big_endian_to_host()
                            .to_ne_bytes()
                    )
                }

                fn little_endian_to_host(self) -> Self {
                    <$t>::from_ne_bytes(
                        (self as <$t as ToUnsigned>::Unsigned)
                            .little_endian_to_host()
                            .to_ne_bytes()
                    )
                }
            }
        )*
    };
}

// helper trait to map signed types to unsigned
trait ToUnsigned {
    type Unsigned;
}

impl ToUnsigned for i16 {
    type Unsigned = u16;
}
impl ToUnsigned for i32 {
    type Unsigned = u32;
}
impl ToUnsigned for i64 {
    type Unsigned = u64;
}
impl ToUnsigned for i128 {
    type Unsigned = u128;
}

impl_endianness_signed!(i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_endianness() {
        assert_eq!(
            get_local_endianness(),
            if cfg!(target_endian = "big") {
                EndiannessType::BigEndian
            } else {
                EndiannessType::LittleEndian
            }
        );
    }

    #[test]
    fn test_swap_bytes() {
        assert_eq!(0x1234u16.swap_bytes(), 0x3412);
        assert_eq!(0x12345678u32.swap_bytes(), 0x78563412);

        // calculate the correct u64 swap
        let original = 0x123456789abcdef0u64;
        let expected = 0xf0debc9a78563412u64;
        assert_eq!(original.swap_bytes(), expected);
    }

    #[test]
    fn test_endianness_conversions_u16() {
        let value = 0x1234u16;

        // test round-trip conversions
        assert_eq!(value.host_to_big_endian().big_endian_to_host(), value);
        assert_eq!(value.host_to_little_endian().little_endian_to_host(), value);

        // test that big and little endian representations are swapped
        assert_eq!(
            value.host_to_big_endian(),
            value.host_to_little_endian().swap_bytes()
        );

        // test specific behavior based on host endianness
        if cfg!(target_endian = "little") {
            assert_eq!(value.host_to_big_endian(), value.swap_bytes());
            assert_eq!(value.host_to_little_endian(), value);
        } else {
            assert_eq!(value.host_to_big_endian(), value);
            assert_eq!(value.host_to_little_endian(), value.swap_bytes());
        }
    }

    #[test]
    fn test_endianness_conversions_u32() {
        let value = 0x12345678u32;

        // test round-trip conversions
        assert_eq!(value.host_to_big_endian().big_endian_to_host(), value);
        assert_eq!(value.host_to_little_endian().little_endian_to_host(), value);

        // test that big and little endian representations are swapped
        assert_eq!(
            value.host_to_big_endian(),
            value.host_to_little_endian().swap_bytes()
        );

        // test specific behavior based on host endianness
        if cfg!(target_endian = "little") {
            assert_eq!(value.host_to_big_endian(), value.swap_bytes());
            assert_eq!(value.host_to_little_endian(), value);
        } else {
            assert_eq!(value.host_to_big_endian(), value);
            assert_eq!(value.host_to_little_endian(), value.swap_bytes());
        }
    }

    #[test]
    fn test_endianness_conversions_u64() {
        let value = 0x123456789abcdef0u64;

        // test round-trip conversions
        assert_eq!(value.host_to_big_endian().big_endian_to_host(), value);
        assert_eq!(value.host_to_little_endian().little_endian_to_host(), value);

        // test that big and little endian representations are swapped
        assert_eq!(
            value.host_to_big_endian(),
            value.host_to_little_endian().swap_bytes()
        );

        // test specific behavior based on host endianness
        if cfg!(target_endian = "little") {
            assert_eq!(value.host_to_big_endian(), value.swap_bytes());
            assert_eq!(value.host_to_little_endian(), value);
        } else {
            assert_eq!(value.host_to_big_endian(), value);
            assert_eq!(value.host_to_little_endian(), value.swap_bytes());
        }
    }

    #[test]
    fn test_u8_endianness() {
        let value = 0x42u8;
        // u8 should always return the same value regardless of endianness
        assert_eq!(value.host_to_big_endian(), value);
        assert_eq!(value.host_to_little_endian(), value);
        assert_eq!(value.big_endian_to_host(), value);
        assert_eq!(value.little_endian_to_host(), value);
    }

    #[test]
    fn test_signed_integers() {
        let value_i16 = 0x1234i16;
        let value_i32 = 0x12345678i32;
        let value_i64 = 0x123456789abcdef0i64;

        // test round-trip conversions for signed integers
        assert_eq!(
            value_i16.host_to_big_endian().big_endian_to_host(),
            value_i16
        );
        assert_eq!(
            value_i16.host_to_little_endian().little_endian_to_host(),
            value_i16
        );

        assert_eq!(
            value_i32.host_to_big_endian().big_endian_to_host(),
            value_i32
        );
        assert_eq!(
            value_i32.host_to_little_endian().little_endian_to_host(),
            value_i32
        );

        assert_eq!(
            value_i64.host_to_big_endian().big_endian_to_host(),
            value_i64
        );
        assert_eq!(
            value_i64.host_to_little_endian().little_endian_to_host(),
            value_i64
        );
    }

    #[test]
    fn test_128bit_integers() {
        let value_u128 = 0x123456789abcdef0fedcba9876543210u128;
        let value_i128 = 0x123456789abcdef0fedcba9876543210i128;

        // test round-trip conversions for 128-bit integers
        assert_eq!(
            value_u128.host_to_big_endian().big_endian_to_host(),
            value_u128
        );
        assert_eq!(
            value_u128.host_to_little_endian().little_endian_to_host(),
            value_u128
        );

        assert_eq!(
            value_i128.host_to_big_endian().big_endian_to_host(),
            value_i128
        );
        assert_eq!(
            value_i128.host_to_little_endian().little_endian_to_host(),
            value_i128
        );

        // test that big and little endian representations are swapped
        assert_eq!(
            value_u128.host_to_big_endian(),
            value_u128.host_to_little_endian().swap_bytes()
        );
    }

    #[test]
    fn test_single_byte_integers() {
        let value_u8 = 0x42u8;
        let value_i8 = 0x42i8;

        // single-byte integers should always return the same value
        assert_eq!(value_u8.host_to_big_endian(), value_u8);
        assert_eq!(value_u8.host_to_little_endian(), value_u8);
        assert_eq!(value_u8.big_endian_to_host(), value_u8);
        assert_eq!(value_u8.little_endian_to_host(), value_u8);

        assert_eq!(value_i8.host_to_big_endian(), value_i8);
        assert_eq!(value_i8.host_to_little_endian(), value_i8);
        assert_eq!(value_i8.big_endian_to_host(), value_i8);
        assert_eq!(value_i8.little_endian_to_host(), value_i8);
    }
}
