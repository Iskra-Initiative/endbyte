#![no_std]
#![no_main]

use endbyte::Endianness;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test the endbyte library in a no_std environment

    // test unsigned integers
    let val_u8 = 0x42u8;
    let val_u16 = 0x1234u16;
    let val_u32 = 0x12345678u32;
    let val_u64 = 0x123456789abcdef0u64;
    let val_u128 = 0x123456789abcdef0fedcba9876543210u128;

    // test all endianness conversion methods
    let _ = val_u8.host_to_big_endian();
    let _ = val_u8.host_to_little_endian();
    let _ = val_u8.big_endian_to_host();
    let _ = val_u8.little_endian_to_host();

    let _ = val_u16.host_to_big_endian();
    let _ = val_u16.host_to_little_endian();
    let _ = val_u16.big_endian_to_host();
    let _ = val_u16.little_endian_to_host();

    let _ = val_u32.host_to_big_endian();
    let _ = val_u32.host_to_little_endian();
    let _ = val_u32.big_endian_to_host();
    let _ = val_u32.little_endian_to_host();

    let _ = val_u64.host_to_big_endian();
    let _ = val_u64.host_to_little_endian();
    let _ = val_u64.big_endian_to_host();
    let _ = val_u64.little_endian_to_host();

    let _ = val_u128.host_to_big_endian();
    let _ = val_u128.host_to_little_endian();
    let _ = val_u128.big_endian_to_host();
    let _ = val_u128.little_endian_to_host();

    // test signed integers
    let val_i8 = 0x42i8;
    let val_i16 = 0x1234i16;
    let val_i32 = 0x12345678i32;
    let val_i64 = 0x123456789abcdef0i64;
    let val_i128 = 0x123456789abcdef0fedcba9876543210i128;

    let _ = val_i8.host_to_big_endian();
    let _ = val_i8.host_to_little_endian();
    let _ = val_i8.big_endian_to_host();
    let _ = val_i8.little_endian_to_host();

    let _ = val_i16.host_to_big_endian();
    let _ = val_i16.host_to_little_endian();
    let _ = val_i16.big_endian_to_host();
    let _ = val_i16.little_endian_to_host();

    let _ = val_i32.host_to_big_endian();
    let _ = val_i32.host_to_little_endian();
    let _ = val_i32.big_endian_to_host();
    let _ = val_i32.little_endian_to_host();

    let _ = val_i64.host_to_big_endian();
    let _ = val_i64.host_to_little_endian();
    let _ = val_i64.big_endian_to_host();
    let _ = val_i64.little_endian_to_host();

    let _ = val_i128.host_to_big_endian();
    let _ = val_i128.host_to_little_endian();
    let _ = val_i128.big_endian_to_host();
    let _ = val_i128.little_endian_to_host();

    // test round-trip conversions to ensure correctness
    assert_eq!(val_u16.host_to_big_endian().big_endian_to_host(), val_u16);
    assert_eq!(
        val_u16.host_to_little_endian().little_endian_to_host(),
        val_u16
    );

    assert_eq!(val_i32.host_to_big_endian().big_endian_to_host(), val_i32);
    assert_eq!(
        val_i32.host_to_little_endian().little_endian_to_host(),
        val_i32
    );

    #[allow(clippy::empty_loop)]
    loop {}
}
