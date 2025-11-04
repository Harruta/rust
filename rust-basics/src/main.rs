// main.rs
fn main() {
    println!("=== Rust Basic Data Types Demo ===\n");

    // 1. Integer Types
    println!("1. Integer Types:");
    let _i8: i8 = -128;
    let _i16: i16 = -32768;
    let _i32: i32 = -2_147_483_648;
    let _i64: i64 = -9_223_372_036_854_775_808;
    let _i128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

    let _u8: u8 = 255;
    let _u16: u16 = 65_535;
    let _u32: u32 = 4_294_967_295;
    let _u64: u64 = 18_446_744_073_709_551_615;
    let _u128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    let _isize: isize = -1; // depends on architecture (32 or 64 bit)
    let _usize: usize = 100;

    println!("   i8 max: {}, min: {}", i8::MAX, i8::MIN);
    println!("   u8 max: {}", u8::MAX);
    println!("   usize example: {}", _usize);
    println!("   isize example: {}\n", _isize);

    // 2. Floating-Point Types
    println!("2. Floating-Point Types:");
    let _f32: f32 = 3.1;
    let _f64: f64 = 3.14159289793;

    println!("   f32: {}", _f32);
    println!("   f64: {}\n", _f64);

    // 3. Boolean Type
    println!("3. Boolean Type:");
    let is_rust_fun: bool = true;
    let is_boring: bool = false;
    println!("   Is Rust fun? {}", is_rust_fun);
    println!("   Is it boring? {}\n", is_boring);

    // 4. Character Type
    println!("4. Character Type (Unicode scalar values):");
    let heart: char = '❤';
    let letter_a: char = 'A';
    let emoji: char = '🚀';
    println!("   Heart: {}", heart);
    println!("   Letter A: {}", letter_a);
    println!("   Rocket: {}\n", emoji);

    // 5. String Types
    println!("5. String Types:");

    // String (owned, growable)
    let mut name: String = String::from("Alice");
    name.push_str(" & Bob");
    println!("   String (owned): {}", name);

    // &str (string slice, borrowed)
    let greeting: &str = "Hello, Rust!";
    println!("   &str (slice): {}\n", greeting);

    // 6. Arrays
    println!("6. Arrays (fixed size):");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let same_value: [u8; 3] = [9; 3]; // [9, 9, 9]
    println!("   numbers: {:?}", numbers);
    println!("   same_value: {:?}\n", same_value);

    // 7. Tuples
    println!("7. Tuples (heterogeneous, fixed size):");
    let person: (&str, i32, bool) = ("Charlie", 30, true);
    let point: (f64, f64, f64) = (1.0, 2.5, -3.7);
    println!("   person: {:?}", person);
    println!("   point: {:?}", point);
    println!(
        "   Name: {}, Age: {}, Is active: {}\n",
        person.0, person.1, person.2
    );

    // 8. Unit Type
    println!("8. Unit Type:");
    let unit: () = ();
    println!("   The unit type `()` has one value: {:?}", unit);
    println!("   Often used as return type when no value is returned.\n");

    // 9. Never Type (rarely seen directly)
    println!("9. Never Type (!):");
    println!("   The `!` type (never) is used in functions that never return.");
    println!("   Example: `panic!()` or infinite loops.\n");

    // 10. Raw Pointers (unsafe, but can be shown)
    println!("10. Raw Pointers (in unsafe context):");
    let x = 42;
    let raw_ptr: *const i32 = &x;
    unsafe {
        println!("   Value via raw pointer: {}", *raw_ptr);
    }
    println!("   Raw pointer address: {:p}\n", raw_ptr);

    // 11. References (borrowed pointers)
    println!("11. References:");
    let value = 100;
    let ref_to_value: &i32 = &value;
    println!("   Original value: {}", value);
    println!("   Reference points to: {}\n", *ref_to_value);

    // 12. Slices
    println!("12. Slices (dynamic view into array/vector):");
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4];
    println!("   Array: {:?}", arr);
    println!("   Slice [1..4]: {:?}\n", slice);

    // 13. Option<T> - Represents optional values
    println!("13. Option<T> (handles null-like cases safely):");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!("   Some(42): {:?}", some_value);
    println!("   None: {:?}\n", none_value);

    // 14. Result<T, E> - For error handling
    println!("14. Result<T, E>:");
    let success: Result<&str, &str> = Ok("Operation succeeded!");
    let failure: Result<&str, &str> = Err("Something went wrong");
    println!("   Ok result: {:?}", success);
    println!("   Err result: {:?}\n", failure);

    println!("=== End of Rust Data Types Demo ===");
}
