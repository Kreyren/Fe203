fn main() {
    // i32 & i64 is 32 and 64 bit integer data type respectively.
    let x = 69; // i32
    // to change the datatype
    let x:i64 = 69;
    let x:u64 = 1; // unsigned integers dont support negative numbers
    let f:f64 = 69.6969; // float
    let b:bool = true; // boolean

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}

/*
Length  Signed  Unsigned
8-bit	 i8	       u8
16-bit	 i16	   u16
32-bit	 i32	   u32
64-bit	 i64	   u64
128-bit	 i128     u128
arch	 isize	 usize
*/

/*
Number literals  Examples
Decimal	          98_222
Hex	              0xff
Octal	          0o77
Binary	        0b1111_0000
Byte (u8 only)	   b'A'
*/
