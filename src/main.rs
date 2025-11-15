// ===== RUST PRACTICE: MODULARIZED =====
// Entry point for all learning exercises

#[macro_use]
mod macros;

mod fundamentals;
mod control_flow;
mod ownership;
mod error_handling;
mod structs;
mod enums;
mod vectors;
mod generics;
mod file_io;

fn main() {
    println!("🦀 RUST LEARNING JOURNEY 🦀");
    println!("============================\n");

    // HOUR 1: FUNDAMENTALS
    println!("\n📚 HOUR 1: FUNDAMENTALS");
    println!("=======================");
    fundamentals::variables_example();
    fundamentals::data_types_example();

    // HOUR 2: CONTROL FLOW
    println!("\n📚 HOUR 2: CONTROL FLOW");
    println!("======================");
    control_flow::conditional_exercise_1();
    control_flow::loop_exercise_1();
    control_flow::match_exercise_1();
    control_flow::match_exercise_2();
    control_flow::run_factorial();

    // HOUR 3: OWNERSHIP & LIFETIMES
    println!("\n📚 HOUR 3: OWNERSHIP & LIFETIMES");
    println!("================================");
    ownership::borrowing_example();
    ownership::mutable_borrow_example();
    ownership::lifetime_example();
    ownership::shorter_lifetime();
    ownership::struct_with_lifetime();
    ownership::combined_lifetimes();

    // HOUR 4: ERROR HANDLING
    println!("\n📚 HOUR 4: ERROR HANDLING");
    println!("========================");
    error_handling::result_example_1();
    error_handling::option_example_1();
    error_handling::option_example_2();
    error_handling::question_mark_operator_1();
    error_handling::question_mark_operator_2();

    // HOUR 5: STRUCTS
    println!("\n📚 HOUR 5: STRUCTS");
    println!("=================");
    structs::book_example();
    structs::bank_account_example();
    structs::rectangle_example();

    // HOUR 5: ENUMS
    println!("\n📚 HOUR 5: ENUMS");
    println!("================");
    enums::day_example();
    enums::season_example();
    enums::message_example();
    enums::user_status_example();
    enums::payment_method_example();

    // VECTORS
    println!("\n📚 VECTORS");
    println!("==========");
    vectors::vec_exercise_1();
    vectors::vec_exercise_2();
    vectors::vec_exercise_3();
    vectors::vec_exercise_4();
    vectors::vec_exercise_5();
    vectors::vec_exercise_6();
    vectors::vec_exercise_7();
    vectors::vec_exercise_8();

    // GENERICS & TRAITS
    println!("\n📚 GENERICS & TRAITS");
    println!("====================");
    generics::stack_example();
    generics::stack_strings_example();
    generics::pair_example();
    generics::pair_float_example();
    generics::drawable_example();

    // CUSTOM MACROS
    macros::macros_example();

    println!("\n============================");
    println!("✅ ALL EXERCISES COMPLETED!");
    println!("============================");

    //FILE I/O

    println!("\n I/O exercises");
    file_io ::write_to_file_exercise();
    file_io::read_from_file_exercise();
    file_io::append_to_file_exercise();
    file_io::list_files_exercise();
    file_io ::create_directory_exercise();
    file_io ::file_metadata_exercise();
    file_io::copy_file_exercise();
    file_io ::delete_file_exercise();
    file_io::read_lines_exercise();
    file_io::count_words_exercise();
}
