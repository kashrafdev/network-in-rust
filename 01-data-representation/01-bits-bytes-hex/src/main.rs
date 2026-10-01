use std::env::args;

enum Number {
    Signed(i32),
    Unsigned(u32),
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let Some(input_number) = args.get(1) else {
        eprintln!("Error: unexpected input");
        panic!();
    };

    let number: u32 = input_number.parse().unwrap();

    // 1. printing number in different format
    // let number = -42;
    println!("Print number in diff formats");
    println!("{number}");
    println!("{number:08b}"); // print at least 8 characters with 0 padding
    println!("{number:X}");
    println!("=================================================");

    //============================================================================================

    // 2. inspect individual bit
    println!("Inspect individual bit");
    let mut bytes: Vec<u8> = Vec::new();
    for i in (0..8).rev() {
        let bit = (number >> i) & 1;
        /*         n = 10110100
                         ↑
                     bit 5 (we start counting bits from the right 0 then 1 then 2 ...)

                     10110100 >> 5 (it shift all the bits 5 positions to the right)
                     00000101
                            ↑
                        the bit we want

            then we make bitwise AND 1 which remove all bits excebt the one we want

                             00000101
                            &00000001
                            ---------
                             00000001
        */
        bytes.push(bit as u8);
        println!("bit number {i}: {bit}");
    }
    println!("=================================================");

    //============================================================================================

    // 3. a byte printer
    println!("byte printer");
    fn byte_printer(bytes: &[u8]) {
        for byte in bytes.chunks(4) {
            for bit in byte {
                print!("{bit}");
            }
            print!(" ");
        }
        println!();
    }

    byte_printer(&bytes);
    println!("=================================================");

    //============================================================================================

    // 4. manually serialize a 32-bit to chuncks of 4-bytes
    println!("serialze 32-bit");
    /*
     00010010 00110100 01010110 01111000
    ┌────────┬────────┬────────┬────────┐
    │        │        │        │        │
    └────────┴────────┴────────┴────────┘
       b1       b2       b3       b4
    */
    // let number: u32 = 0x12345678;
    // since its 32 bit so
    // the first bit in byte1 is (8*3) = 24
    // the first bit in byte2 is (8*2) = 16
    // the first bit in byte3 is (8*1) = 8
    // the first bit in byte4 is (8*0) = 0
    // so as a formula the first bit in byte-n = 8 * (n-total - n) Note: n-total is the total number of bytes chuncks
    let byte1 = ((number >> 24) & 0xff) as u8;
    let byte2 = ((number >> 12) & 0xff) as u8;
    let byte3 = ((number >> 8) & 0xff) as u8;
    let byte4 = ((number) & 0xff) as u8;
    /*
    0xff = 11111111

    00000000 00000000 00000000 00010010
    &
    00000000 00000000 00000000 11111111
    ─────────────────────────────────────
    00000000 00000000 00000000 00010010

    so & 0xff means keep the last 8-bits

    we use as u8 to make it only the last 8-bits
    */
    println!("{byte1:02X} {byte2:02X} {byte3:02X} {byte4:02X}");
    println!("=================================================");

    //============================================================================================

    // 5. reverse the process from bytes to hex
    let original_number = (byte1 as u32) << ((4 - 1) * 8)
        | (byte2 as u32) << ((4 - 2) * 8)
        | (byte3 as u32) << 8
        | (byte4 as u32); // we use | which is bitwise OR to combine the bits together one big sequence
    println!("{original_number:02X}");
}
