// fn main() {
//     println!("====RUST FUNDAMENTALS===\n");

//     //Immutable variables
//     let name = "Rust Learner";
//     println!("Hello, {}!", name);

//     //Mutable variables
//     let mut age = 25;
//     println!("Age is: {}", age);
//     age = 26;
//     println!("Birthday! New age: {}\n" , age);

//     //SHadowing
//     let x=5;
//     let x=x+1;
//     println!("Shadowed x: {}\n" , x);

//     let name = "Rust Learner";
//     println!("{}", name);

//     let name = name.len();
//     print!("{}", name);

//     //Data Types
//      let integer: i32 = 42;
//     let float: f64 = 3.14;
//     let boolean: bool = true;
//     let character: char = 'R';

//     println!("Integer: {}", integer);
//     println!("Float: {}", float);
//     println!("Boolean: {}", boolean);
//     println!("Character: {}\n", character);

//     // 5. Basic math operations
//     let sum = 10 + 5;
//     let difference = 10 - 5;
//     let product = 10 * 5;
//     let quotient = 10 / 5;
//     let remainder = 10 % 3;

//     println!("10 + 5 = {}", sum);
//     println!("10 - 5 = {}", difference);
//     println!("10 * 5 = {}", product);
//     println!("10 / 5 = {}", quotient);
//     println!("10 % 3 = {}", remainder);
// }

//

// fn main(){
//     let num = -4;
//     if num>0 && num%2==0 {
//         print!("{} is positive and even", num);
//     } else if num <0 && num %2==0 {
//         println!("{} is negative and even", num);
//     } else if num >0 && num%2 !=0 {
//         println!("{} is positive and odd", num);
//     }else if num<0 && num%2!=0 {
//         println!("{} is negative and odd", num);
//     } else {
//         println!("{} is zero", num);
//     }

//     let temp = 30;

//     if temp<0 {
//         println!("Freezing");
//     } else if temp>0 && temp <15 {
//         println!("Cold");
//     }else if temp > 16 && temp<25{
//         println!("Warm");
//     }else {
//         println!("Hot");
//     }

//     let username = "admin";
//     let password = "password123";

//     if username == "admin" && password =="password123" {
//         println!("Login Successful");
//     } else if username !="admin" && password == "password123" {
//         println!("Invalid username");
//     }else if username == "admin" && password != "password123" {
//         println!("Wrong password");
//     }

// }

// fn main(){
// // Loop Exercise 1
// for i in 1..=10{
//     println!("{}", i);
// }

// // Loop Exercise 2
// for i in 1..=10 {
//     println!("{}", 5*i);
// }

// // Loop exercise 3
// let mut sum =0;
// let mut i =1;

// while i<=10 {
//     sum += i;
//     i+=1;
// }

// println!("{}", sum);

// // Match Exercise 1
// let grade = 49;

// match grade {
//     90..=100 => println!("Grade A"),
//     70..=89 => println!("Grade B"),
//     50..=69=> println!("Grade C"),
//     _ => println!("Fail Grade")
// }

// // Match Exercise 2

// let fruit = String:: from("apple");

// match fruit.as_str() {
//     "apple" => println!("Color : Red"),
//     "banana" => println!("Color: Yellow"),
//     "grape" => println!("Color :Purple"),
//     _ => println!("Unknown fruit"),
// }

// Last Exercise
// let mut fact =1;

// for i in 1..=5{
//     fact *= i;
// }
// println!("{}", fact);

// }

// fn main() {
//         let s = String::from("hello");
//         takes_ownership(&s);
//         println!("{}", s); // What's the problem?
//         // Ans: Problem here is that the function takes_ownership has already taken ownership of s, so now basically its lifetime is over
// }

// fn takes_ownership(s: &String) {
//         println!("{}", s);
// }


// fn main(){
//     let mut s = String:: from("Hello");
//     append_fn(&mut s);
//     println!("{}", s);
// }

// fn append_fn(s : &mut String){
//     s.push_str(" Rust");
// }

// fn main(){
//     let str = String :: from("hello");
//     let r1 = &str;
//     let r2 = &str;

//     println!("{} {}", r1 ,r2);
// }

// fn main(){
//     let mut s = String :: from("hello");

//     let r3 = &mut s;
//     println!("{}",r3);

//     let r1 = &s;
//     let r2 = &s;
    

//     println!("{}", r1);
//     println!("{}", r2);
    
// }

// fn main(){
//     let s1 = String:: from("hello");
//     let result ;
//     {
//         let s2 = String :: from ("world");
//         result = get_first( &s1, &s2); // it wont compile because the lifetime of s2 ends in this block.
//     }

//     println!("{}", result);
// }

// fn get_first<'a>(s1 :&'a String, s2 : &'a String)-> &'a String {
//     s1
// }


// fn main(){
//     let s1 = String:: from("hellooo");
//     let s2 = String :: from("world");

//     let res_str = shorter_str(&s1,&s2);
//     println!("{}", res_str);
// }

// fn shorter_str<'a>(s1 : &'a String, s2: &'a String)-> &'a String{
//     if s1.len() >s2.len(){
//         return s2;
//     }
//     s1
// }

// struct Message<'a>{
//     text: &'a str
// }
// fn main(){
//     let s = String:: from("hello rust");
//     let msg = Message {text : &s};

//     println!("{}", msg.text);
// }

// fn combine<'a,'b>(s1: &'a String, s2: &'b String)-> &'a String{
//     s1 // we are using 2 different lifespans because they are under different blocks and have different lifespans
// }
// fn main(){
//     let s1 = String:: from("Hello");
//     let result;
//     {
//         let s2 = String:: from("World");
//         result = combine(&s1,&s2);

//     }
//     println!("{}", result);
// }

// struct Message<'a> {
//     text : &'a String
// }

// fn shorter_str<'a>(s1: &'a String, s2: &'a String) -> &'a String {
//     if s1.len()< s2.len() {
//         return s1;
//     }
//     s2
// }
// fn main(){
//     let s1 = String:: from("Hellooo");
//     let s2= String :: from("world");

//     let result = shorter_str(&s1, &s2);

//     let m = Message{text : &result};

//     println!("Shortest string: {}", m.text);
// }



// fn safe_sqrt(num: i32)-> Result<f64,String>{
//     if num>0 {
//          Ok((num as f64).sqrt())
//     } else{
//         Err(String::from("Cannot take sqrt of negative number"))
//     }

// }
// fn main (){
//     let num : i32 = 10;
//     let res = safe_sqrt(num);

//     match res {
//         Ok(num) => println!("{}",num),
//         Err(msg) => println!("{}", msg)
//     }
// }

// 



fn find_evem(num :i32)-> Option<i32>{
    if num%2==0 {
        Some(num)
    } else{
        None
    }
}

fn validate_age(age : i32) -> Result<i32,String> {
    if age >=18 {
        Ok(age)
    } else {
        Err(String::from("Too Young"))
    }
}
fn main(){
    let num = 16;
    let res1 = find_evem(num);
    let res2 = validate_age(num);
    match res1 {
        Some(num) => println!("{}", num),
        None => println!("Not an even number")
    }

    match res2 {
        Ok(age) => println!("{}", age),
        Err(msg) => println!("{}",msg)
    }
}