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



// fn find_evem(num :i32)-> Option<i32>{
//     if num%2==0 {
//         Some(num)
//     } else{
//         None
//     }
// }

// fn validate_age(age : i32) -> Result<i32,String> {
//     if age >=18 {
//         Ok(age)
//     } else {
//         Err(String::from("Too Young"))
//     }
// }
// fn main(){
//     let num = 16;
//     let res1 = find_evem(num);
//     let res2 = validate_age(num);
//     match res1 {
//         Some(num) => println!("{}", num),
//         None => println!("Not an even number")
//     }

//     match res2 {
//         Ok(age) => println!("{}", age),
//         Err(msg) => println!("{}",msg)
//     }
// }





// use std::fmt::format;

// fn parse_age( age: &str )-> Result<i32,String>{
//     match age.parse::<i32>() {
//         Ok(age) => Ok(age),
//         Err(_) => Err(String::from("Invalid age"))
//     }
// }

// fn process_user_input(age: &str,s: &str)->Result<String,String>{
//     let res = parse_age(age)?;
//     Ok(format!("User {} is {} yrs old",s, res))
// }

// fn main(){
//     let res = process_user_input("25","John");
//     match res {
//         Ok(msg) => print!("{}",msg),
//         Err(e) => print!("Error : {}",e)
//     }
// }




// struct Book {
//     title : String,
//     author: String,
//     pages : i32
// }

// impl Book {
//     fn display_info(&self){
//         println!("Book details are : {} {} {}", self.title,self.author,self.pages);
//     }

//     fn is_long_book(&self) -> bool {
//         self.pages > 300
//     }

//     fn new (title : String, author : String, pages : i32) -> Book {
//         Book {
//             title,
//             author,
//             pages
//         }
//     }
// }
// fn main(){
//     let book = Book::new(
//         String::from("Rust Book"),
//         String:: from("Steve Klabnik"),
//         541
//     );

//     book.display_info();

//     println!("{}", book.is_long_book());

//     println!("Title : {} \n Author: {} \n Pages : {}", book.title, book.author, book.pages);
// }




// struct BankAccount {
//     account_holder : String,
//     balance : f64
// }

// impl BankAccount{
//     fn deposit(&mut self,amount: f64){
//         self.balance += amount;
//     }

//     fn withdraw(&mut self,amount: f64){
//         self.balance -= amount;
//     }

//     fn display_balance(&self){
//         println!("Current balance is {}",self.balance);
//     }
// }

// fn main(){
//     let mut account = BankAccount{
//         account_holder : String::from("John"),
//         balance : 5000.0
//     };

//     account.withdraw(1000.0);

//     account.deposit(4000.0);

//     account.display_balance();
// }



// struct Rectangle{
//     width : f64,
//     height : f64
// }

// impl Rectangle{
//     fn area(&self)-> f64 {
//         self.width * self.height
//     }

//     fn perimeter(&self) ->f64 {
//         2.0*(self.width + self.height)
//     }

//     fn is_square(&self) -> bool {
//         self.width == self.height
//     }

//     fn display(&self) {
//         println!("Width is {}\n Height is {}", self.width, self.height);
//     }
// }

// fn main(){
//     let rect = Rectangle {
//         width : 4.0,
//         height : 8.0
//     };

//     println!("Area is {}\n Perimeter is {}\n is_square?: {}", rect.area(),rect.perimeter(),rect.is_square());
//     rect.display();
// }

// enum Day {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday
// }

// fn main(){
//     let day = Day :: Friday;
//     match day {
//         Day :: Monday => println!("This is monday"),
//         Day :: Tuesday => println!("This is Tuesday"),
//         Day :: Wednesday => println!("This is Wednesday"),
//         Day :: Thursday => println!("This is thursday"),
//         Day :: Friday => println!("Friday is weekday"),
//         Day :: Saturday => println!("This is saturday"),
//         Day :: Sunday => println!("OFF day")
//     }
// }


// enum Season{
//     Spring,
//     Summer,
//     Fall,
//     Winter
// }

// impl Season {
//     fn temperature(&self)-> i32{
//         match self {
//             Season :: Spring => 15,
//             Season:: Summer => 25,
//             Season::Fall => 10,
//             Season:: Winter => 0
//         }
//     }
// }

// fn main(){
//     let season = Season :: Summer;
//     println!("Temp is {}", season.temperature());
//     let season = Season :: Winter;
//     println!("Temp is {}", season.temperature());
// }


// enum Message {
//     Text(String),
//     Number(i32),
//     Empty
// }

// fn main(){
//     let text = Message::Text(String::from("Hello"));
//     let num = Message :: Number(32);
//     let empty = Message:: Empty;

//     match text {
//         Message :: Text(s)=> println!("Text is {}", s),
//         Message :: Number(n)=> println!("Number is {}", n),
//         Message::Empty => println!("Empty message")
//     }

//     match num {
//         Message :: Text(s)=> println!("Text is {}", s),
//         Message :: Number(n)=> println!("Number is {}", n),
//         Message::Empty => println!("Empty message")
//     }

//     match empty {
//         Message :: Text(s)=> println!("Text is {}", s),
//         Message :: Number(n)=> println!("Number is {}", n),
//         Message::Empty => println!("Empty message")
//     }

    
// }



// enum UserStatus{
//     Active,
//     Inactive,
//     Banned
// }

// impl UserStatus{
//     fn is_active(&self)-> bool {
//         match self {
//             UserStatus ::Active => true,
//             UserStatus :: Inactive => false,
//             UserStatus :: Banned => false
//         } 
//     }
// }
// fn main(){
//     let user1 = UserStatus:: Active;
//     let user2 = UserStatus:: Inactive;
//     let user3 = UserStatus:: Banned;

//     println!("User1 is {}\n User2 is {}\n User3 is {}\n", user1.is_active(),user2.is_active(),user3.is_active());
// }


// enum PaymentMethod{
//     CreditCard(String),
//     Paypal(String),
//     Cash(f64)
// }

// impl PaymentMethod{
//     fn process_payment(&self) -> String {
//         match self {
//             PaymentMethod :: CreditCard(card) => {
//                 format!("Processing card: {}", card)
//             }
//             PaymentMethod :: Paypal(email) => {
//                 format!("Processing Paypal: {}", email)
//             }
//             PaymentMethod:: Cash(amount) => {
//                 format!("Processing Cash: {} ", amount)
//             }
//         }
//     }
// }

// fn main (){
//     let payment1 = PaymentMethod::CreditCard(String::from("123443438937768"));
//     let payment2 = PaymentMethod::Paypal(String::from("Zyzzz@gmail.com"));
//     let payment3 = PaymentMethod::Cash(500.0);

//     println!("Payment1 is {}\n Payment2 is {}\n Payment 3 is {}", payment1.process_payment(),payment2.process_payment(),payment3.process_payment());
// }




//-----------------------------Vec--------------------------

// fn main(){
//     let mut numbers:Vec<i32> = Vec :: new();
//     for i in 1..=10 {
//         numbers.push(i);
//     }

//     for num in &numbers{
//         println!("{}",num);
//     }
// }

// fn main(){
//     let mut s: Vec<&str> = Vec :: new();

//     s.push("apple");
//     s.push("banana");
//     s.push("orange");

//     for i in &s {
//         println!("{}",i);
//     }

//     s.pop();

//     for i in &s{
//         println!("{}",i);
//     }
// }


// fn main(){
//     let mut numbers = vec![10,20,30,40,50];

//     println!("{}",numbers[0]);
//     println!("{}",numbers[numbers.len()-1]);

//     numbers[2] = 99;

//     for i in &numbers{
//         println!("{}",i);
//     }
// }


// fn main(){
//     let num = vec![1,2,3,4,5,6,7,8,9,10];

//     let new_vec : Vec<_>= num.iter().filter(|n| *n %2 ==0).copied().collect();

//     print!("Even numbers: [");
//     for i in &new_vec{
//         print!("{},",i);
//     }
//     println!("]");
// }


// fn main(){
//     let num = vec![1,2,3,4,5];

//     let doubled : Vec<_> = num.iter().map(|n| *n*2).collect();
    
//     print!("Doubled :[");
//     for i in &doubled{
//         print!("{},", i);
//     }
//     println!("]");

//     let squared: Vec<_> = num.iter().map(|n| *n * n).collect();
    
//     print!("Squared :[");
//     for i in &squared{
//         print!("{},", i);
//     }
//     println!("]");
// }


// fn main(){
//     let mut num = vec![1,2,3,4,5,6,7,8,9,10];

//     let even_num : Vec<_> = num.iter().filter(|n| *n %2==0).copied().collect();

//     let res_num : Vec<_> = even_num.iter().map(|n| *n*2).collect();

//     print!("Even numbers doubled: [");
//     for i in &res_num{
//         print!("{},", i);
//     }
//     println!("]");

// }

// fn main(){
//     let num = vec![3,7,2,9,1,8,4];

//     println!("Length : {}",num.len());

//     let mut sum =0;
//     for i in &num{
//         sum += i;
//     }

//     println!("Sum :{}",sum);

//     let mut cnt =0;
//     for i in &num {
//         if *i>5{
//             cnt +=1;
//         }
//     }

//     println!("Numbers>5 : {}",cnt);
// }


// fn main(){
//     let mut num = vec![1,2,3,4,5];

//     num.remove(2);

//     print!("After removal: [");
//     for i in &num{
//         print!("{},",i);
//     }
//     println!("]");

//     num.clear();

//     print!("After clear: ");
//     println!("{:?}",num);
// }



//------------------------Todo App-------------------------

// #[derive(Debug,Clone)]
// struct Todo {
//     id : i32,
//     title : String,
//     description : String,
//     is_done : bool
// }

// impl Todo {
//     fn new (id:i32, title: String,description: String) -> Todo{
//         Todo {id,title,description,is_done:false,}
//     }

//     fn mark_todo(&mut self){
//         self.is_done = true;
//     }

//     fn display_todo (&self){
//         let status = if self.is_done {"Done"}else{"pending"};
//         println!("[{}] ID:{} - {} ({})", status, self.id,self.title,self.description);
//     }
// }

// struct TodoApp{
//     todos : Vec<Todo>,
//     next_id : i32
// }

// impl TodoApp{
//     fn new() -> TodoApp{
//         TodoApp {todos: Vec::new(),next_id:1}
//     }

//     fn add_Todo(&mut self, title:String, description: String){
//         let todo = Todo ::new(self.next_id, title, description);
//         self.todos.push(todo);
//         self.next_id +=1;
//     }

//     fn list_todos(&self){
//         if self.todos.is_empty(){
//             println!("No todos to display");
//         }

//         for todo in &self.todos{
//             todo.display_todo();
//         }
//     }

//     fn mark_todo_done (&mut self, id:i32)-> Result<(),String> {
//         for todo in &mut self.todos{
//             if todo.id == id{
//                 todo.mark_todo();
//                 return Ok(());
//             }
//         }
//         Err(format!("Todo with {} not found",id))
//     }

//     fn delete_todo9(&mut self, id:i32) -> Result<(), String> {
//         let original_len = self.todos.len();

//         self.todos.retain(|t| t.id !=id);

//         if self.todos.len() < original_len {
//             println!("Deleted Todo {}", id);
//             return Ok(());
//         } else {
//             Err(format!("Todo with {} not found", id))
//         }
//     }

//     fn show_pending (&self){
//         let pending_todos: Vec<&Todo> = self.todos.iter().filter(|t| !t.is_done).collect();

//         if pending_todos.is_empty(){
//             println!("All todos completed");
//             return;
//         }

//         for todo in &pending_todos {
//             todo.display_todo();
//         }
//         println!();
//     }

//     fn show_completed (&self){
//         let completed: Vec<&Todo> = self.todos.iter().filter(|t| t.is_done).collect();

//         if completed.is_empty(){
//             println!("No completed Todos yet!");
//         }
//         for todo in &completed{
//             todo.display_todo();
//         }

//         println!();
//     }

// }

// fn main(){
//     let mut app = TodoApp:: new();

//     app.add_Todo(String::from("Go to gym"), String::from("Today is push day"));

//     app.add_Todo(String::from("Do maths"), String::from("Quadratic equation"));

//     app.add_Todo(String::from("Practice Rust"), String::from("Complete the mini project"));

//     // app.list_todos();

//     app.mark_todo_done(3);

//     // app.list_todos();

//     app.delete_todo9(3);

//     // app.list_todos();

//     app.show_pending();

//     app.mark_todo_done(1);
//     app.mark_todo_done(2);

//     app.list_todos();

//     app.show_completed();
// }


struct Stack<T>{
    items: Vec<T>
}

impl<T> Stack<T>{

    fn new()-> Stack<T>{
        Stack {items: Vec::new()}
    }
    fn push(&mut self, item:T){
        self.items.push(item);
    }

    fn pop(&mut self)-> Option<T>{
        self.items.pop()
    }

    fn peek(&self) -> Option<&T>{
        self.items.last()
    }

    fn is_empty(&self) -> bool{
        self.items.is_empty()
    }

    fn size(&self) -> usize{
        self.items.len()
    }
}
fn main(){

}