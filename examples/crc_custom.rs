use crc::Crc;

fn main() {
    simple();
    marco();
}

fn simple() {
    // use custom to calculate CRC-12/DECT
    let mut alg = crc::Algorithm::<u16> {
        width: 12,
        poly: 0x80f,
        init: 0x000,
        refin: false,
        refout: true,
        xorout: 0x000,
        check: 0xdaf,
        residue: 0x000,
    };
    let cs = Crc::<u16>::new(&alg).checksum(b"123456789");
    println!("CRC-12/DECT : {cs}");

    // re use alg for CRC-14/DARC
    alg.width = 14;
    alg.poly = 0x0805;
    alg.init = 0x0000;
    alg.refin = true;
    alg.refout = true;
    alg.xorout = 0x0000;
    alg.check = 0x082d;
    alg.residue = 0x000;
    let cs = Crc::<u16>::new(&alg).checksum(b"123456789");
    println!("CRC-14/DARC : {cs}");
}

// use a simple marco to gengrate crc functions
fn marco() {
    macro_rules! crc_checksum {
        ($generic:ident,$func:ident) => {
            #[allow(dead_code)]
            fn $func(
                width: u8,
                poly: $generic,
                init: $generic,
                refin: bool,
                refout: bool,
                xorout: $generic,
                buf: &[u8],
            ) -> $generic {
                let alg = crc::Algorithm::<$generic> {
                    width,
                    poly,
                    init,
                    refin,
                    refout,
                    xorout,
                    check: 0,   // no use for calculation
                    residue: 0, // no use for calculation
                };
                Crc::<$generic>::new(&alg).checksum(buf)
            }
        };
    }

    crc_checksum!(u8, crc_u8);
    crc_checksum!(u16, crc_u16);
    crc_checksum!(u32, crc_u32);
    crc_checksum!(u64, crc_u64);
    crc_checksum!(u128, crc_u128);

    let cs = crc_u16(12, 0x80f, 0x000, false, true, 0x000, b"123456789");
    println!("CRC-12/DECT : {cs}");

    let cs = crc_u128(
        82,
        0x0308c0111011401440411,
        0x000000000000000000000,
        true,
        true,
        0x000000000000000000000,
        b"123456789",
    );
    println!("CRC-82/DARC : {cs}");
}
