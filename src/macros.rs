#[macro_export]
macro_rules! maximum {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $c:expr) => {
        if $a > $b && $a > $c {
            $a
        } else if $b > $c {
            $b
        } else {
            $c
        }
    };
}

#[macro_export]
macro_rules! multiply_all{
    ($($num:expr),*)=> {
       {
        let mut product = 1;
        $(
            product *= $num;
        )*
        product
     }
    };
}

#[macro_export]
macro_rules! string_concat {
    ($($s:expr),*) => {
        {
            let mut res = String::new();
            $(
                res.push_str(&$s.to_string());
            )*
            res
        }
    }
}

#[macro_export]
macro_rules! check_type{
    ($val:expr)=> {
        println!("{} {} Type : {}", stringify!($val), $val, std::any::type_name_of_val(&$val));
    }
}

pub fn macros_example() {
    println!("\n====CUSTOM MACROS====\n");

    // Maximum macro
    println!("Maximum of 5 and 10: {}", maximum!(5, 10));
    println!("Maximum of 5, 10, 3: {}\n", maximum!(5, 10, 3));

    // Multiply all macro
    let product = multiply_all!(2, 3, 4, 5);
    println!("Product of 2, 3, 4, 5: {}\n", product);

    // String concat macro
    let text = string_concat!("Hello", " ", "Rust", "!");
    println!("Concatenated: {}\n", text);

    // Check type macro
    check_type!(42);
    check_type!("Hello");
    check_type!(3.14);
}