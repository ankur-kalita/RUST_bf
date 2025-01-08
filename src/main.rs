// fn main() {
// // println!("Hello, world!");

// // Definition
// let x = 10.0;
// println!("x is: {x}");

// // Mutability
// let mut y = 69;
// y = 32;

// // Scope
// {
//     let y = 34;
// }

// // Shaodowing
// let t = 23;
// let t = t + 23;
// println!("t is : {t}");

// // Constants
// const MAX_VALUE: u32 = 100;

// // Unsigned integers
// let unsigned_num: u8 = 5;

// // Signed integers
// let signed_num: i8 = 5;

// // Floating point numbers
// let float_num: f32 = 5.0;

// // Platform specific integers
// let arch_1: usize = 5;
// let arch_2: isize = 5;

// // Characters
// let char = 'a';

// // Boolean
// let b: bool = true;

// // Type aliasing
// type Age = u8;
// let peter_age: Age = 42;

// // Type Conversion
// let a = 10;
// let b = a as f64;
// print!("{a} and {b}");

// //&str and String
// let fixed_str = "hi hello bye";
// let mut flexible_str = String::from("This string will grow");
// flexible_str.push('h')

//  // Arrays
//  let mut array_1 = [4, 5, 6, 8, 9];
//  let num = array_1[3];

//  println!("{:?}", array_1);
//  let array_2 = [0; 10];

//  // Vectors
//  let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];
//  let num = vec_1[3];

//  // Tuples
//  let my_info = ("Salary", 40000, "Age", 40);
//  let salary_value = my_info.1;
//  let (salary, salary_value, age, age_value) = my_info;

//  let unit = ();

// }

// fn main() {
//     let s = "Why are you gae ?";
//     let answer = my_fn(s);
// }

// fn my_fn(s: &str) -> &str {
//     print!("HI");
//     s
// }

// fn main() {
//     let num = 40;
//     if num < 50 {
//         println!("The number is less than 50");
//     } else {
//         println!("The number is greater than or equal to 50");
//     }

//     let marks = 95;
//     //let mut grade = 'N';

//     let grade = if marks >= 90 {
//         'A'
//     } else if marks >= 80 {
//         'B'
//     } else if marks >= 70 {
//         'C'
//     } else {
//         'F'
//     };

//     let marks = 95;
//     let grade = 'N';

//     let grade = match marks {
//         90..=100 => 'A',
//         80..=89 => 'B',
//         70..=79 => 'C',
//         _ => 'F',
//     };
//     print!("{}", grade);
// }

// fn main() {
//     'outer: loop {
//         print!("Simple loop!");
//         break 'outer;
//     }

//     let a = loop {
//         break 5;
//     };

//     let vec = vec![45, 30, 85, 90, 41, 39];

//     for i in vec {
//         println!("{i}");
//     }

//     let mut num = 0;
//     while num < 10 {
//         num = num + 1;
//     }
// }
// fn main() {
/* Escape sequences
    \n : Newline character.
    \t : Tab space.
    \r : Carriage Return.
    \" : Double quote.
    \\ : Backward slash.
*/
// println!("\n Will be printed after one empty line");
// println!("\t A tab space at the start");
// println!("This will be overwritten \r This text will only appear on the screen");

// println!("Prints double quotes \", Prints backslash \\");

// println!(
//     "I am doing {2} from {1} years and i {0} it",
//     "like", 20, "programming"
// );

// println!(
//     "{language} is a system programming language which is cool to {activity} in.",
//     activity = "code",
//     language = "Rust"
// );

// }

// use std::io::{self, stdin};

// fn main() {
//     let mut n = String::new();
//     std::io::stdin().read_line(&mut n).expect("failed to read input.");
//     let n: f64 = n.trim().parse().expect("invalid input");
//     print!("{n}");

//     let _number_one = 45.0;
//     let x = 40_000;

//     static WELCOME: &str = "Welcome to Rust";
//     const PI: f32 = 3.14;

//     let a = PI;
//     let b = PI;

//     let c = WELCOME;
//     let d = WELCOME;

// }

// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

// fn main() {
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("failed to read input.");
//     let n: i32 = n.trim().parse().expect("invalid input");

//     let mut square_of_sum = 0;
//     let mut sum_of_squares = 0;

//     let mut m = n;
//     while m != 0 {
//         square_of_sum += m;
//         sum_of_squares += m.pow(2);
//         m -= 1;
//     }

//     let difference = square_of_sum.pow(2) - sum_of_squares;
//     println!(
//         "The difference of the square_of_sum and sum of Squares for N = {} is {}",
//         n, difference
//     );

//     /* Complete the code after this line */
// }

// fn main() {
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("failed to read input.");
//     let n: i32 = n.trim().parse().expect("invalid input");

//     let mut sum = 0;
//     for i in 1..n {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum += i;
//         }
//     }
//     print!("{sum}");

//     /* Add your code below this line */
// }

// fn main() {
//     let input = String::from("1211");
//     println!(
//         "It is {:?} that the given string is palindrome",
//         palindrome(input)
//     );
// }

// fn palindrome(input: String) -> bool {
//     let mut is_palindrome = true;
//     if input.len() == 0 {
//         is_palindrome = true;
//     } else {
//         let mut last = input.len() - 1;
//         let mut first = 0;

//         let my_vec = input.as_bytes();

//         while first < last {
//             if my_vec[first] != my_vec[last] {
//                 is_palindrome = false;
//                 break;
//             }

//             first += 1;
//             last -= 1;
//         }
//     }
//     is_palindrome
// }

// fn main() {
//     let x: u64 = 4_294_967_296;
//     let y = x as u32;
//     if x == y as u64 {
//         println!("x equals y.");
//     } else {
//         println!("x does not equal y.");
//     }
// }

// fn main() {
//     let three = 3;

//     if three {
//         println!("Number was three");
//     }
// }

// fn main() {
//     let marks = 98;
//     let grade = match marks {
//         90..=100 => 'A',
//         80..=89 => "B",
//         70..=79 => "C",
//         _ => 'F',
//     };
// }

//OWNERSHIP
// -------------------------------------------
// 	Ownership Basics
// -------------------------------------------

/*
1.	Each value has a variable that's its "owner."
2.	A value can have only one owner at a time.
3.	If the owner goes out of scope, the value is cleaned up.
*/
// fn main() {
//     let s1 = String::from("world");
//     let s2 = s1;
//     println!("s1 is: {s1}");
// }

// fn main() {
//     let vec_1 = vec![1, 2, 3];
//     takes_ownership(vec_1.clone());
//     println!("vec 1 is: {:?}", vec_1);

//     let vec_2 = gives_onwership();
//     println!("vec 2 is: {:?}", vec_2);

//     let vec_3 = takes_and_gives_ownership(vec_2);
//     //println!("vec 2 is: {:?}", vec_2);
//     println!("vec 3 is: {:?}", vec_3);

//     let x = 10;
//     stack_function(x);
//     println!("In main, x is: {x}");
// }

// fn takes_ownership(vec: Vec<i32>) {
//     println!("vec is: {:?}", vec);
// }

// fn gives_onwership() -> Vec<i32> {
//     vec![4, 5, 6]
// }

// fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
//     vec.push(10);
//     vec
// }

// fn stack_function(mut var: i32) {
//     var = 56;
//     println!("In func, var is: {var}");
// }

// -------------------------------------------
// 	    Borrowing
// -------------------------------------------
/*
- Borrrowing Rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.

- Solve out two problems
    - Data race
    - Dangling references
*/

// fn main() {
//     let mut vec_1 = vec![4, 5, 6];
//     let ref1 = &vec_1;
//     let ref2 = &vec_1;
//     println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
//     let ref3 = &mut vec_1;

//     let vec_2 = {
//         let vec_3 = vec![1, 2, 3];
//         &vec_3
//     };
// }

// fn main() {
//     let mut vec_1 = vec![1, 2, 3];
//     let ref1 = &vec_1;
//     borrows_vec(ref1);
//     let ref2 = &mut vec_1;
//     mutably_borrows_vec(ref2);
//     println!("vec 1 is: {:?}", vec_1);
// }
// fn borrows_vec(vec: &Vec<i32>) {
//     println!("vec is: {:?}", vec);
// }

// fn mutably_borrows_vec(vec: &mut Vec<i32>) {
//     vec.push(10);
// }

// fn gives_onwership() -> Vec<i32> {
//     vec![4, 5, 6]
// }

// Deferencing

// fn main() {
//     let mut some_data = 42;
//     let ref_1 = &mut some_data;
//     let deref_copy = *ref_1;
//     *ref_1 = 13;
//     println!("some_data is: {some_data}, deref_copy is: {deref_copy}");

//     let mut heap_data = vec![5, 6, 7];
//     let ref_1 = &heap_data;
//     let ref_2 = ref_1;
//     let ref_3 = ref_1;
//     let deref_copy = ref_1.clone();

//     let move_out = ref_1;
//     // let move_out_again = ref_1;
// }

// Structs

// struct Car {
//     owner: String,
//     year: u32,
//     fuel_level: f32,
//     price: u32
// }
// fn main() {
//     let mut my_car = Car {
//         owner: String::from("ABC"),
//         year: 2010,
//         fuel_level: 0.0,
//         price: 5_000,
//     };
//     let car_year = my_car.year;
//     my_car.fuel_level = 30.0;
//     let extracted_owner = my_car.owner.clone();
//     println!("Owner is: {}", my_car.owner);

//     let another_car = Car {
//         owner: "new_name".to_string(),
//         ..my_car
//     };

//     //println!("Owner is: {}", my_car.owner);

//     // Tuple Structs
//     let point_2D = (1, 3);
//     let point_3D = (4, 10, 13);

//     struct Point_2D(i32, i32);
//     struct Point_3D(i32, i32, i32);

//     let point1 = Point_2D(1, 3);
//     let point2 = Point_3D(4, 10, 13);

//     // Unit Struct
//     struct ABC;
// }

// -------------------------------------------
// 	Adding Functionality to Structs
// -------------------------------------------

// struct Car {
//     owner: String,
//     year: u32,
//     fuel_level: f32,
//     price: u32,
// }
// impl Car {
//     fn monthly_insurance() -> u32 {
//         123
//     }

//     fn selling_price(&self) -> u32 {
//         self.price + Car::monthly_insurance()
//     }

//     fn new(name: String, year: u32) -> Self {
//         Self {
//             owner: name,
//             year: year,
//             fuel_level: 0.0,
//             price: 0,
//         }
//     }

//     fn display_car_info(&self) {
//         println!(
//             "Owner: {}, Year: {}, Price: {}",
//             self.owner, self.year, self.price
//         );
//     }

//     fn refuel(&mut self, gallons: f32) {
//         self.fuel_level += gallons;
//     }

//     fn sell(self) -> Self {
//         self
//     }
// }

// fn main() {
//     let mut my_car = Car {
//         owner: String::from("ABC"),
//         year: 2010,
//         fuel_level: 0.0,
//         price: 5_000,
//     };

//     // my_car.display_car_info();
//     // display_car_info(&my_car);

//     // my_car.refuel(10.5);
//     // let new_owner = my_car.sell();
//     // my_car.refuel(10.5);

//     // let new_car = Car::new("XYZ".to_string(), 2020);
//     print!("{}",my_car.selling_price());
// }

// Enums

// enum WeekDay {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sundary,
// }
// fn main() {
//     let mut day = "Saturday".to_string();

//     let week_day = vec![
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//         "Sundary".to_string(),
//     ];
//     day = week_day[1].clone();

//     let day = WeekDay::Saturday;
// }

// enum TravelType {
//     Car(f32),
//     Train(f32),
//     Aeroplane(f32),
// }

// impl TravelType {
//     fn travel_allowance(&self) -> f32 {
//         let allowance = match self {
//             TravelType::Car(miles) => miles * 2.0,
//             TravelType::Train(miles) => miles * 3.0,
//             TravelType::Aeroplane(miles) => miles * 5.0,
//         };
//         allowance
//     }
// }
// fn main() {
//     let participant = TravelType::Car(60.0);
//     println!(
//         "Allowance of participant is: {}",
//         participant.travel_allowance()
//     );
// }

// Option

// struct Student {
//     name: String,
//     grade: Option<u32>,
// }

// fn main() {
//     let student_db = vec![
//         Student {
//             name: String::from("Alice"),
//             grade: Some(95),
//         },
//         Student {
//             name: String::from("Bob"),
//             grade: Some(87),
//         },
//         Student {
//             name: String::from("Charlie"),
//             grade: None,
//         },
//     ];
//     let student_name = String::from("Bob");
//     let student_grade = get_grade(&student_name, &student_db);

//     // match student_grade {
//     //     Some(grade) => println!("Grade is: {grade}"),
//     //     None => {}
//     // }

//     if let Some(grade) = student_grade {
//         println!("Grade is: {grade}");
//     }
// }

// struct Student {
//     name: String,
//     grade: Option<u32>,
// }
// fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
//     for student in student_db {
//         if student.name == *student_name {
//             return student.grade;
//         }
//     }
//     None // not reachable
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
//     for student in student_db {
//         if student.name == *student_name {
//             return Ok(());
//         }
//     }
//     Err(String::from("Student not found"))
// }

// fn check_student_get_grade(
//     student_name: &String,
//     student_db: &Vec<Student>,
// ) -> Result<Option<u32>, String> {
//     for student in student_db {
//         if student.name == *student_name {
//             return Ok(student.grade);
//         }
//     }
//     Err(String::from("Student not found"))
// }
// fn main() {
//     let student_db = vec![
//         Student {
//             name: String::from("Alice"),
//             grade: Some(95),
//         },
//         Student {
//             name: String::from("Bob"),
//             grade: Some(87),
//         },
//         Student {
//             name: String::from("Charlie"),
//             grade: None,
//         },
//     ];

//     let student_name = String::from("Adam");
//     let student_status = check_student_get_grade(&student_name, &student_db);

//     match student_status {
//         Ok(option_grade) => {
//             if let Some(grade) = option_grade {
//                 println!("Grade is: {grade}");
//             }
//         }
//         Err(error_msg) => println!("{error_msg}"),
//     }
//     // let student_grade = get_grade(&student_name, &student_db);

//     // match student_grade {
//     //     Some(grade) => println!("Grade is: {grade}"),
//     //     None => {}
//     // }

//     // if let Some(grade) = student_grade {
//     //     println!("Grade is: {grade}");
//     // }
// }

// HashMaps

//  use std::{collections::HashMap};
// fn main() {
//     let mut person:HashMap<&str, i32> = HashMap::new();
//     person.insert("Nouman", 69);
//     person.insert("Ankur", 21);
//     println!("I am {:?}", person.get("Nouman").unwrap());

//     if person.contains_key("Nouman") {
//         println!("The value exist.");
//     } else {
//         print!("Insallah");
//     }
//     match person.get("Nouman") {
//         Some(value) => println!("The value exists: {}", value),
//         None => println!("The value does not exist"),
//     }

//     for (name, age) in &person {
//         println!("The person {} has an age of {}", name, age);
//     }

//     let mut likes: HashMap<&str, &str> = HashMap::new();

//     // likes.insert("Nouman", "apple");
//     // likes.insert("Nouman", "mango");
//     // println!("The fruit which is being liked is {:?}", likes);

//     likes.entry("Nouman").or_insert("apple");
//     likes.entry("Nouman").or_insert("mango");
//     println!("The fruit which is being liked is {:?}", likes);
//}

// fn main() {
//     let some_vec: Vec<i32> = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
//     let mut freq_vec: HashMap<i32, u32> = HashMap::new();

//     for i in &some_vec {
//         let freq = freq_vec.entry(*i).or_insert(0);
//         *freq += 1;
//     }

//     println!("{:?}", freq_vec);
// }

// struct S {
//     x: i32,
// }

// fn main() {
//     let mut s1 = S { x: 2 };
//     let v = &mut s1;
//     v.x += 1;
//     // s1.x += 1;
//     print!("{}{}", v.x, s1.x);
// }

// mod product {
//     pub use category::Category;
//         pub struct Product {
//             id: u64,
//             name: String,
//             price: f64,
//             category: Category,
//         }

//     impl Product {
//         pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
//             Product {
//                 id,
//                 name,
//                 price,
//                 category,
//             }
//         }
//     }
// }

// GENERICS
// struct Point<T, U> {
//     x: T,
//     y: U
// }

// impl Point<i32, i32> {
//     fn printing(&self) {
//         println!("The values of the coordinates are {}, {}", self.x, self.y);
//     }

//     fn new_1(x: i32, y: i32) -> Point<i32, i32> {
//         Point { x, y }
//     }
// }

// fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
//     unimplemented!();
// }

// impl<T, U> Point<T,U> {
//     fn new(x: T, y: U) -> Point<T, U> {
//         Point {x, y}
//     }
// }
// fn main() {
//     let origin = Point::new(0, 0);
//     let p1 = Point::new(1.0, 2.0);

//     let p2 = Point::new(2, 3.0);

// }

// TRAITS
// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
//     //info: drawing_info,
// }

// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
//     // info: drawing_info,
// }
// trait Shape {
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32 {
//         println!("Perimeter not implemented, returning dummy value");
//         0.0
//     }
// }
// impl Shape for Rectangle {
//     fn area(&self) -> f32 {
//         let area_of_rect = self.length * self.width;
//         println!("Rectangle area: {}", area_of_rect);
//         area_of_rect
//     }
// }
// impl Shape for Square {
//     fn area(&self) -> f32 {
//         let area_of_square = self.side * self.side;
//         println!("Square area: {}", area_of_square);
//         area_of_square
//     }
// }
// fn main() {
//     let r1 = Rectangle {
//         width: 5.0,
//         length: 4.0,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     let s1 = Square {
//         side: 3.2,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     r1.area();
//     s1.area();

//     r1.perimeter();
//     s1.perimeter();
// }

// struct drawing_info {
//     line_width: u8,
//     color: String,
// }
// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
//     //info: drawing_info,
// }

// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
//     // info: drawing_info,
// }

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

// TRAIT BOUNDS
// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
// }

// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
// }

// trait Shape {
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32 {
//         println!("Perimeter not implemented, returning dummy value");
//         0.0
//     }
// }

// impl Shape for Rectangle {
//     fn area(&self) -> f32 {
//         let area_of_rect = self.length * self.width;
//         println!("Rectangle area: {}", area_of_rect);
//         area_of_rect
//     }
//     fn perimeter(&self) -> f32 {
//         let perimeter_of_rect = 2.0 * (self.length + self.width);
//         println!("Rectangle Perimeter: {}", perimeter_of_rect);
//         perimeter_of_rect
//     }
// }

// impl Shape for Square {
//     fn area(&self) -> f32 {
//         let area_of_square = self.side * self.side;
//         println!("Square area: {}", area_of_square);
//         area_of_square
//     }
// }

// fn shape_properties<T>(object: T)
// where
//     T: Shape,
// {
//     object.area();
//     object.perimeter();
// }

// fn returns_shape() -> impl Shape {
//     let sq = Square {
//         side: 5.0,
//         line_width: 5,
//         color: String::from("Red"),
//     };
//     sq
//     // let rect = Rectangle {
//     //     length: 5.0,
//     //     width: 10.0,
//     //     line_width: 5,
//     //     color: String::from("Red"),
//     // };

//     // let x = false;
//     // if x {
//     //     sq
//     // } else {
//     //     rect
//     // }
// }

// struct Circle {
//     radius: f32,
// }
// fn main() {
//     let r1 = Rectangle {
//         width: 5.0,
//         length: 4.0,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     let s1 = Square {
//         side: 3.2,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     let c1 = Circle { radius: 5.0 };
//     shape_properties(r1);
//     shape_properties(s1);
//     // shape_properties(c1); // Trait bound not satisfied
// }

// SUPER TRAIT
// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
// }

// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
// }

// trait Draw {
//     fn draw_object(&self);
// }
// trait Shape: Draw + OtherTrait + SomeOtherTrait {
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32 {
//         println!("Perimeter not implemented, returning dummy value");
//         0.0
//     }
// }

// trait OtherTrait {}
// impl OtherTrait for Rectangle {}
// impl OtherTrait for Square {}

// trait SomeOtherTrait {}
// impl SomeOtherTrait for Rectangle {}
// impl SomeOtherTrait for Square {}

// impl Shape for Rectangle {
//     fn area(&self) -> f32 {
//         let area_of_rect = self.length * self.width;
//         println!("Rectangle area: {}", area_of_rect);
//         area_of_rect
//     }
//     fn perimeter(&self) -> f32 {
//         let perimeter_of_rect = 2.0 * (self.length + self.width);
//         println!("Rectangle Perimeter: {}", perimeter_of_rect);
//         perimeter_of_rect
//     }
// }

// impl Shape for Square {
//     fn area(&self) -> f32 {
//         let area_of_square = self.side * self.side;
//         println!("Square area: {}", area_of_square);
//         area_of_square
//     }
// }

// impl Draw for Square {
//     fn draw_object(&self) {
//         println!("Drawing Square");
//     }
// }
// impl Draw for Rectangle {
//     fn draw_object(&self) {
//         println!("Drawing Rectangle");
//     }
// }
// fn shape_properties<T>(object: T)
// where
//     T: Shape,
// {
//     object.area();
//     object.perimeter();
// }

// fn returns_shape() -> impl Shape {
//     let sq = Square {
//         side: 5.0,
//         line_width: 5,
//         color: String::from("Red"),
//     };
//     sq
// }

// fn main() {
//     let r1 = Rectangle {
//         width: 5.0,
//         length: 4.0,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     let s1 = Square {
//         side: 3.2,
//         line_width: 1,
//         color: String::from("Red"),
//     };

//     shape_properties(r1);
//     shape_properties(s1);
// }
// fn shape_properties_dynamic(object: Box<dyn Shape>) {
//     object.area();
//     object.perimeter();
//     }
// fn returns_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
//     if dimension.len() == 1 {
//         let sq = Square {
//             side: dimension[0],
//             line_width: 5,
//             color: String::from("Red"),
//         };
//         Box::new(sq)
//     } else {
//         let rect = Rectangle {
//             length: dimension[0],
//             width: dimension[1],
//             line_width: 5,
//             color: String::from("Red"),
//         };
//         Box::new(rect)
//     }
// }
// fn main() {
//     let r1 = Rectangle {
//         width: 5.0,
//         length: 4.0,
//         line_width: 1,
//         color: String::from("Red"),
//     };
//     let s1 = Square {
//         side: 3.2,
//         line_width: 1,
//         color: String::from("Red"),
//     };
//     shape_properties_dynamic(Box::new(r1));
//     shape_properties_dynamic(Box::new(s1));
// }

// trait Vehicle {
//     fn start_engine(&self) -> String;
//     fn drive(&self) -> String;
// }

// struct Car;

// struct Bicycle;

// impl Vehicle for Car {
//     fn start_engine(&self) -> String {
//         "🚗 Engine started".to_string()
//     }

//     fn drive(&self) -> String {
//         "🚗 Driving the car".to_string()
//     }
// }

// impl Vehicle for Bicycle {
//     fn start_engine(&self) -> String {
//         "🚲 No engine to start for a bicycle".to_string()
//     }

//     fn drive(&self) -> String {
//         "🚲 Pedaling the bicycle".to_string()
//     }
// }

// fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
//     // This line needs a fix
//     match vehicle_type {
//         "car" => Box::new(Car),
//         "bicycle" => Box::new(Bicycle),
//         _ => panic!("No vehicle of that type available"),
//     }
// }

// fn operate_vehicle(driver: &dyn Vehicle) {
//     // This line needs a fix
//     println!("{}", driver.start_engine());
//     println!("{}", driver.drive());
// }

// fn main() {
//     // Do not change code in main
//     let my_vehicle = get_vehicle("car");
//     operate_vehicle(my_vehicle.as_ref());

//     let my_vehicle = get_vehicle("bicycle");
//     operate_vehicle(my_vehicle.as_ref());
// }

// DERIVED TRAITS and MARKER TRAITS
// trait Properties: PartialEq + Default + Clone {}
// #[derive(Debug, PartialEq, Default, Clone)]
// struct Student {
//     name: String,
//     age: u8,
//     sex: char,
// }
// impl Properties for Student {}
// fn main() {
//     let s_1 = Student {
//         name: String::from("ABC"),
//         age: 35,
//         sex: 'M',
//     };

//     let s_2 = Student {
//         name: String::from("XYZ"),
//         age: 40,
//         sex: 'M',
//     };

//     println!("Student: {:?}", s_1);
//     println!("s_1 and s_2 are equal: {}", s_1 == s_2);
// }

//ASSOCIATED TRAITS

// #[derive(Debug)]
// struct Km {
//     value: u32,
// }

// #[derive(Debug)]
// struct Kmh {
//     value: u32,
// }

// #[derive(Debug)]
// struct Miles {
//     value: u32,
// }

// #[derive(Debug)]
// struct Mph {
//     value: u32,
// }


// trait DistanceThreeHours {
//     type Distance;
//     fn distance_in_three_hours(&self) -> Self::Distance;
// }

// impl DistanceThreeHours for Kmh {
//     type Distance = Km;
//     fn distance_in_three_hours(&self) -> Self::Distance {
//         Self::Distance {
//             value: self.value * 3,
//         }
//     }
// }

// impl DistanceThreeHours for Mph {
//     type Distance = Miles;
//     fn distance_in_three_hours(&self) -> Self::Distance {
//         Self::Distance {
//             value: self.value * 3,
//         }
//     }
// }
// fn main() {
//     let speed_Kmh = Kmh { value: 90 };
//     let distance_Km = speed_Kmh.distance_in_three_hours();

//     println!(
//         "At {:?}, you will travel {:?} in 3 hours",
//         speed_Kmh, distance_Km
//     );

//     let speed_Mph = Mph { value: 90 };
//     let distance_Miles = speed_Mph.distance_in_three_hours();
//     println!(
//         "At {:?}, you will travel {:?}, in 3 hours",
//         speed_Mph, distance_Miles
//     );
// }

struct User {
    name: String,
    age: u8,
    salary: u32
}

fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };

    let validate_user = |name: &str| name.len() != 0;
    println!("User Validity {}", validate_user(&person_1.name));
}