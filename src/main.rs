const GLOBAL_CONSTANT: &str = "Constants can be declared in any scope including the global scope and are valid for the duration of the program.";

fn main() {
    println!("Hello, world!");
    declare_a_variable();
    variables_are_immutable_by_default();
    declare_a_mutable_variable();
    declare_a_constant_variable();
    shadow_a_variable();
    demonstrate_type_inference();
    demonstrate_type_annotations();
    show_scalar_types();
    demonstrate_integer_type();
    demonstrate_integer_overflow();
    demonstrate_floating_point_type();
    demonstrate_boolean_type();
    demonstrate_character_type();
    demonstrate_compound_types();
    demonstrate_tuples();
    demonstrate_arrays();
}

fn print_line_break() {
    println!();
}

fn declare_a_variable() {
    print_line_break();
    println!("Declaring a variable:");

    let a_new_variable = "Rust recommends using snake_case as a naming convention for local variables.";

    println!("You declare a new variable using the 'let' keyword.");
    println!("{}", a_new_variable);
}

fn variables_are_immutable_by_default() {
    print_line_break();
    println!("Declaring an immutable variable:");

    let immutable_variable = "This variable is immutable. It's value cannot change once assigned to this label";

    // THIS WON'T WORK!
    // immutable_variable = 2;

    println!("Variable are immutable by default!");
    println!("{}", immutable_variable);
    println!("The rust compiler 'guarentees' immutability if you do not specify otherwise.");
}

fn declare_a_mutable_variable() {
    print_line_break();
    println!("Declaring a mutable variable:");

    let mut mutable_variable = "I can change!";

    println!("This is a mutable variable: {}", mutable_variable);

    mutable_variable = "SEE!";

    println!("Mutable variables can change: {}", mutable_variable);
    println!("To declare a variable as mutable use the 'mut' keyword in front of the variable name.");
}

fn declare_a_constant_variable() {
    print_line_break();
    println!("Constants:");
    
    const NAMING_CONVENTION: &str = "Rust recommends SCREAMING_SNAKE_CASE  (a.k.a Anaconda Case) as a naming convention for constant variable";
    const FIRST_DIFFERENCE: &str = "First: they can never be declared as mutable using the 'mut' keyword.";
    const SECOND_DIFFERENCE: &str = "Second: they must always be declared with a type definition.";
    const THIRD_DIFFERENCE: &str = "Third: they must be assigned a static value (nothing which is calculated at runtime)";

    println!("Constant variables can be declare using the 'const' keyword before the variable name.");
    println!("{}", NAMING_CONVENTION);
    println!("{}", GLOBAL_CONSTANT);
    println!("Constants differ from variables declared using 'let' in multiple ways.");
    println!("{}", FIRST_DIFFERENCE);
    println!("{}", SECOND_DIFFERENCE);
    println!("{}", THIRD_DIFFERENCE);
}

fn shadow_a_variable() {
    print_line_break();
    println!("Shadowing variables:");

    println!("Variables can be shadowed in Rust by re-declaring the same label with the 'let' keyword.");
    
    let this_variable_will_be_shadowed = 1;
    println!("Rust says that variable, {}...", this_variable_will_be_shadowed);
    let this_variable_will_be_shadowed = 2;
    println!("is shadowed by variable {},...", this_variable_will_be_shadowed);
    println!("if variable {} is the re-declaration of the same label to a new value.", this_variable_will_be_shadowed);

    let this_variable_will_be_shadowed = "You can change a variables value AND type by using shadowing.";
    println!("{}", this_variable_will_be_shadowed);
    
    let this_variable_will_be_shadowed = "The resultant variable is still immutable after they transformations.";
    println!("{}", this_variable_will_be_shadowed);

    let this_variable_will_be_shadowed = "This differs from a mutable variable because the type cannot change even if a variable is declare as mutable.";
    println!("{}", this_variable_will_be_shadowed);
    //this_variable_will_be_shadowed = "This won't work!";
}

fn demonstrate_type_inference() {
    print_line_break();
    println!("Type inference:");

    let inferred_string_type = "This variable is determined as a string.";
    let inferred_integer_type = 1;

    println!("The Rust language is capable of type inference.");
    println!("For example this: {}", inferred_string_type);
    println!("And: {}", inferred_integer_type);
}

fn demonstrate_type_annotations() {
    print_line_break();
    println!("Type annotations:");

    let an_integer: i32 = 1;

    println!("Variable can have their types explicitly defined using type annotations.");
    println!("An integer: {}", an_integer);
}

fn show_scalar_types() {
    print_line_break();
    println!("Scalar types:");

    let an_integer: i32 = 1;
    let a_float: f32 = 1.1;
    let a_boolean: bool = true;
    let a_character: char = 'a';
    
    println!("Rust has four scalar types, integers, floating point numbers, booleans and characters.");
    println!("An integer: {}", an_integer);
    println!("A float: {}", a_float);
    println!("A boolean: {}", a_boolean);
    println!("A character: {}", a_character);
}

fn demonstrate_integer_type() {
    print_line_break();
    println!("Integer type:");

    let default_integer = 1;
    let unsigned_integer: u32 = 12;
    let signed_integer: i32 = -13;
    
    println!("The integer type represents whole numbers.");
    println!("If the type is inferred from a whole number literal it will default to a 32-bit signed integer (e.g. {}).", default_integer);
    println!("Integers can be typed as unsigned positive values (e.g. {}) and signed values capable of representing negative values (e.g. {}).", unsigned_integer, signed_integer);
    println!("Integers can be types with specific bit lengths. 8, 16, 32, 64 and usize/isize are all valid types annotations.");
    println!("Integers types as usize or isize will have the bit length of the system architecture (e.g. 32-bit or 64-bit).");
    println!("usize and isize are typically used to store indexed addresses in memory");
}

fn demonstrate_integer_overflow() {
    print_line_break();
    println!("Integer overflows:");

    let overflowing_integer: i8 = 127;
    println!("Integers can overflow if you assign them a value beyond their bit length.");
    println!("In debug mode the rust compiler will panic and point this overflow out.");
    println!("In release mode the rust compiler will not complain but instead wrap the value.");

    println!("Before: {}", overflowing_integer);
    println!("After: {}", overflowing_integer+1);
}

fn demonstrate_floating_point_type() {
    print_line_break();
    println!("Floating point type:");
    
    let float_64: f64 = 1.1;
    let default_float = 1.23456789;
    let float_32: f32 = 1.2;

    println!("Floating point numbers can be declared in a similar way to integers.");
    println!("Floating point numbers are capable of representing numbers with a fractional component (e.g. {}).", float_64);
    println!("If a floating point number is inferred from a literal then it will be typed as a 'f64' double precision value (e.g. {}).", default_float);
    println!("Single precision floating point numbers can be represented using the 'f32' type annotation (e.g. {}).", float_32);
}

fn demonstrate_boolean_type() {
    print_line_break();
    println!("Boolean type:");

    let true_value = true;
    let false_value = false;

    println!("Boolean values represent either true or false.");
    println!("A true value: {}", true_value);
    println!("A false value: {}", false_value);
    println!("They occupy a single byte in memory.");
    println!("A true value: {}", format!("{:#08b}", true_value as u8));
    println!("A false value: {}", format!("{:#08b}", false_value as u8));
}

fn demonstrate_character_type() {
    print_line_break();
    println!("Character type:");

    let character: char = 'a';

    println!("Character variables can be declared using the 'char' type annotations.");
    println!("Character literals are declared between single quotes '{}'.", character);
    println!("Characters are stored using Unicode representation.");
}

fn demonstrate_compound_types() {
    print_line_break();
    println!("Compund types:");

    let tuple = (1, 'a');
    let array = ['a', 'b', 'c'];

    println!("Compound types are used to store collections of values.");
    println!("Two of these compound types are tuples {:?} and arrays {:?}", tuple, array);
    println!("Both tuples and arrays are static in length and stack allocated.");
}

fn demonstrate_tuples() {
    print_line_break();
    println!("Tuples:");

    let tuple = ('a', 1, true, 2.2);
    let (character, integer, boolean, float) = tuple;

    println!("Tuples can be used to store a collection of data of different types.");
    println!("Elements in a tuple can either be accessed using destructuring syntax: {} {} {} {}", character, integer, boolean, float);

    let a = tuple.0;
    let one = tuple.1;
    let boolean = tuple.2;
    let float = tuple.3;

    println!("Or directly using index access: {} {} {} {}", a, one, boolean, float);
}

fn demonstrate_arrays() {
    print_line_break();
    println!("Arrays:");

    let array = [1, 2, 3, 4];
    let initialised_array: [i32; 4] = [0; 4];

    println!("Arrays can be used to store multiple elements of the same type in a fixed length collection.");
    println!("Accessible via index using the '[]' syntax: array[0] = {}, array[1] = {}, array[2] = {}, array[3] = {}", array[0], array[1], array[2], array[3]);
    println!("An entire array can be initialised with values using this syntax 'let initialised_array = [0; 4]': {:?}", initialised_array);
    println!("An array can be typed using a similar syntax 'let initialised_array: [i32, 4] = [0; 4]': {:?}", initialised_array);
    println!("Attempting to access an array index which is out of bounds will result in a panic at runtime.");
    //println!("Panic at runtime: {}", initialised_array[4]);
}
