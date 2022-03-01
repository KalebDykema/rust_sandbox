fn main() {
    // ------------
    // Scalar Types
    // ------------

    /*
        Integers
            - Can be 8, 16, 32, 64, or 128 bit. Arch (size) is the last option, which is either 32 or 64 bit and designated by the computer and/or OS.
            - Signed intergers are used to designate number that may be positive or negative. Designated by i.
            - Unsigned intergers are used to designate number that may be positive or negative. Designated by u.
            - The limits of a number are designated from -(2^(n-1)) to 2^n-1, n being the bits. For example, 8 bit would be from -256 to 255.
            - Defaults to i32.
            - Integer literals can be used to display number data in other format, such as hexidecimal, binary, or byte.
    */

    let unsigned_int: u8 = 5;
    let signed_int: i8 = -5;

    println!("\nUnsigned Int: {}", unsigned_int);
    println!("Signed Int: {}", signed_int);

    /*
        Floats
            - Used to display decimals
            - Can either be f32 or f63 (default)
    */
    
    let float_thirty_two = 3.0; // implicitly set as f32
    let float_sixty_four: f64 = 4.5;

    println!("\nf32 Float: {}", float_thirty_two);
    println!("f64 Float: {}", float_sixty_four);

    /*
        Operations
            - Similar to other languages, you can add (+), subtract (-), multiplication (*), divide (/), or get the remainder (%).
    */

    let added_ints = unsigned_int + 5;

    println!("\nAdded Ints: {}", added_ints);

    /* 
        Boolean
            - Must I say more? True or false
    */

    let boolean_var: bool = true;

    println!("\nBoolean: {}", boolean_var);

    /*
        Char
            - Holds a single character
    */
    
    let char_var: char = 'b';

    println!("\nChar: {}", char_var);


    // --------------
    // Compound Types
    // --------------

    /*
        Tuple
            - Way of grouping multiple values with a variety of types
            - Fixed length onced define
    */

    let tup_var: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup_var;

    println!("\nTuple: {}, {}, {}", x, y, z);

    /* 
        Array
            - List of variables of the same type
            - Unlike other languages, has a fixed lenght
            - Useful to define your variables on the stack instead of the heap
    */

    let array_var: [i32; 5] = [1, 2, 3, 4, 5];

    println!("\nArray Index 0: {}", array_var[0]);
}
