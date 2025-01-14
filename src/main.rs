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

//CLOSURES
// struct User {
//     name: String,
//     age: u8,
//     salary: u32
// }
// fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
// where
//     V1: FnOnce(&str) -> bool,
//     V2: Fn(u8) -> bool,
// {
//     simple_validator(name) && advance_validator(age)
// }

// fn main() {
//     let person_1 = User {
//         name: String::from("someone"),
//         age: 35,
//         salary: 40_000,
//     };

//     let validate_user = |name: &str| name.len() != 0;
//     println!("User Validity {}", validate_user(&person_1.name));
// }

// //It must not contain any generics.

// fn add(x: u32, y: u32) -> u32 {
//     x + y
// }

// fn square(x: u32) -> u32 {
//     x * x
// }

// fn sum_of_squares(num: u32, sq: fn(u32) -> u32, add: fn(u32, u32) -> u32) -> u32 {
//     let mut result = 0;
//     for i in 1..=num {
//         result = add(result, sq(i));
//     }
//     result
// }

// fn main() {
//     let num = 4;
//     let sum = sum_of_squares(num, square, add);
//     println!("Sum of squares from 1 to {} = {}", num, sum);
// }

//ITERATOR
//trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// struct Employee {
//     name: String,
//     salary: u16
// }
// struct Employee_records {
//     employee_db: Vec<Employee>,
// }

// impl Iterator for Employee_records {
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.employee_db.len() != 0 {
//             let result = self.employee_db[0].name.clone();
//             self.employee_db.remove(0);
//             Some(result)
//         } else {
//             None
//         }
//     }
// }
// fn main() {
//     let mut emp_1 = Employee{
//         name: String::from("Johny"),
//         salary: 40_000,
//     };
//     let mut emp_2 = Employee {
//         name: String::from("Papa"),
//         salary: 50_000
//     };
//     let mut emp_db = Employee_records {
//         employee_db: vec![emp_1, emp_2]
//     };
//     // println!("{:?}", emp_db.next());
//     // println!("{:?}", emp_db.next());
//     // println!("{:?}", emp_db.next());

//     for employee in emp_db {
//         println!("{employee}");
//     }
//     // for loop is smart enough to turn it into an iterator producing item of type string...
// }

// use std::vec;

// //IntoIterator
// /*
//  trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }
// */
// trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

// struct Book {
//     title: String,
//     author: String,
//     genre: String,
// }

// // struct BookIterator {
// //     properties: Vec<String>,
// // }

// // impl Iterator for BookIterator {
// //     type Item = String;

// //     fn next(&mut self) -> Option<Self::Item> {
// //         if !self.properties.is_empty() {
// //             Some(self.properties.remove(0))
// //         } else {
// //             None
// //         }
// //     }
// // }

// impl IntoIterator for Book {
//     type Item = String;
//     // type IntoIter = BookIterator;

//     // fn into_iter(self) -> Self::IntoIter {
//     //     BookIterator {
//     //         properties: vec![self.title, self.author, self.genre],
//     //     }
//     // }

//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         vec![self.title, self.author, self.genre].into_iter()
//     }
// }

// fn main() {
//     let book = Book {
//         title: "Digital Image Processing".to_string(),
//         author: "Gonzales".to_string(),
//         genre: "Science Book".to_string(),
//     };

//     let mut book_iterator = book.into_iter();

//     // println!("{:?}", book_iterator.next());
//     // println!("{:?}", book_iterator.next());
//     // println!("{:?}", book_iterator.next());
//     // println!("{:?}", book_iterator.next());

//     for book_info in book_iterator {
//         println!("{book_info}");
//     }
// }

// use std::collections::HashMap;

// // ITERATING THROUGH COLLECTIONS
// fn main() {
//     let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
//     let mut vec_1_iter = vec_1.into_iter();
//     let value_1 = vec_1_iter.next();
//     print!("{:?}", value_1);

//     let mut person: HashMap<String, i32> = HashMap::new();
//     person.insert("Hannash".to_string(), 40);
//     person.insert("Joseph".to_string(), 44);
//     person.insert("Sara".to_string(), 55);

// }

// fn main() {
//     let words = vec!["apple", "banana", "grape", "orange", "pear"];
//     // let mut result: Vec<String> = vec![];

//     // for word in words {
//     //     if word.starts_with("a") || word.starts_with("b") {
//     //         let uppercase_word = word.to_uppercase();
//     //         result.push(uppercase_word);
//     //     }
//     // }
//     // println!("Result: {:?}", result);

//     // let result: Vec<String> = words
//     //     .into_iter()
//     //     .filter(|&word| word.starts_with("a") || word.starts_with("b"))
//     //     .map(|word| word.to_uppercase())
//     //     .collect::<Vec<String>>();

//     // println!("Result: {:?}", result);

//     // let result = words.into_iter().filter(|&word| word.starts_with("a") ||
//     // word.starts_with("b")).map(|word| word.to_uppercase()).collect::<Vec<String>>();
//     // print!("{:?}",result);

//     let some_product = Some("laptop");
//     let mut products = vec!["cellphone", "battery", "charger"];

//     products.extend(some_product);
//     println!("{:?}", products);

//     let products_iter = products.iter().chain(some_product.iter());

//     for prod in products_iter {
//         println!("{:?} ", prod);
//     }
// // ------ Use Case 3 -----
// let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

// Solution 1;
// let mut prod_without_none = Vec::new();
// for p in products {
//     if p.is_some() {
//         prod_without_none.push(p.unwrap());
//     }
// }

// Solution 2:
// let prod_without_none = products
//     .into_iter()
//     .filter(|x| x.is_some())
//     .map(|x| x.unwrap())
//     .collect::<Vec<&str>>();

// Solution 3:
// let prod_wihtout_none: Vec<&str> = products.into_iter().flatten().collect();
// println!("{:?}", prod_wihtout_none);
//}

// fn main() {
//     // Example 1:
//     let i = 5;
//     let j = i;
//     println!("{i}");

//     // Example 2:
//     let str_1 = String::from("abc");
//     let str_2 = str_1;
//     //println!("str_1: {str_1}");

//     // Example 3:
//     let str_1 = String::from("abc");
//     str_fn(str_1);
//     //let str_2 = str_1;

//     // Example 4:
//     let i;
//     {
//         let j = 5;
//         i = &j;
//         println!("i: {i}");
//     }

//     // Example 5:
//     let mut vec_1 = vec![6, 5, 8, 9];
//     let ref_1 = &vec_1;
//     println!("ref 1: {:?}", ref_1);
//     let ref_2 = &mut vec_1;
//     ref_2.push(3);
//     println!("ref 2: {:?}", ref_2);
// }

// fn str_fn(s: String) {
//     println!("s: {s}");
// }

//  fn main() {
//     let int1 = 5;
//     let int2 = 10;
//     let picked_value = picking_int(&int1, &int2);
//     println!("{picked_value}");
// }

// fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
//     if rand::random() {
//         i
//     } else {
//         j
//     }
// }
// -------------------------------------------
// 	        Lifetime Elision
// -------------------------------------------

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to
    all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of self is assigned to all output lifetime parameters.
*/

// // Example 1:
// fn main() {
//     let str_1 = "some str";

//     let recieved_str = return_str(&str_1);
// }
// // Code with Lifetime Elision
// fn return_str(s_1: &str) -> &str {
//     s_1
// }

// // Code without Lifetime Elision
// fn return_str<'a>(s_1: &'a str) -> &'a str {
//     s_1
// }

// // Example 2:
//  fn main() {
//     let str_1 = "some str";
//     let str_2 = "other str";
//     let recieved_str = return_str(&str_1, &str_2);
// }

// fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
//     s_1
// }

//LIFETIME IN STRUCTS
// struct ArrayProcessor<'a>{
//     data: &'a[i32],
// }

// fn main() {}

// BOX SMART POINTER
// -------------------------------------------

//       Simple Pointer          ||         Smart Pointers
// ----------------------------------------------------------------------
// Just stores memory address    ||   Special capabilities
// Indicated by &                ||   Not just simple references
// Also called references        ||
// No special capabilities       ||

// fn main() {
//     let x = 0.625;
//     let y = Box::new(x);
// }

/*
enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk
}
*/

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     // let x = 0.625;
//     // let y = Box::new(x);
//     // let z = &x;

//     let list = List::Cons(
//         1,
//         Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
//     );

//     println!("{:?}", list);
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     let list = List::Cons(
//         1,
//         Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
//     );

//     println!("{:?}", list);
// let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];

// }

//REFERENCE COUNTING SMART POINTER

// use std::rc::Rc;
// enum List {
//     Cons(i32, Option<Rc<List>>),
// }
// fn main() {
//     let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2,None)))));
//     let b = List::Cons(3, Some(Rc::clone(&a)));
//     let c = List::Cons(4, Some(Rc::clone(&a)));
//     println!("Reference count after a: {}", Rc::strong_count(&a));
// }

// REF CELL SMART POINTER
// use std::{cell::RefCell, rc::Rc};

// fn main() {
//     // Example 1: Borrowing rules at run time
//     // let mut x = 50;
//     // let x1 = &x;
//     // let x2 = &x;
//     // let x3 = &mut x;

//     // println!("{} {} ", x1, x2);

//     let a = RefCell::new(10);

//     //{ // add the scope later on
//     let b = a.borrow();
//     let c = a.borrow();
//     //}

//     drop(b); // add later on, remove after adding the scope above
//     drop(c); // add later on
//     let d = a.borrow_mut();
//     // drop(d) // add later on
//     //println!("{} {}", b, c); // later on delete this
//     //println!("Value of a is : {:?}", a); // add later on

//     // Example 2: Interior mutability
//     // let x = 32;
//     // let x1 = &mut x;

//     let a = RefCell::new(10);
//     //let c = *a; // add later on
//     let mut b = a.borrow_mut();
//     *b = 15;

//     drop(b); // add later on
//     println!("{:?}", a);

//     // Example 3
//     let a = Rc::new(RefCell::new(String::from("c++")));
//     let b = Rc::clone(&a);

//     *b.borrow_mut() = String::from("rust");
//     println!("{:?}", a);
// }

// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug)]
// struct File {
//     active_user: u32,
// }

// #[derive(Debug)]
// struct User {
//     file: Rc<RefCell<File>>,
// }

// fn main() {
//     let mut txt_file = Rc::new(RefCell::new((File { active_user: 0 })));

//     let mut user_1 = User {
//         file: Rc::clone(&txt_file),
//     };
//     user_1.file.borrow_mut().active_user += 1;
//     println!("Active users: {:?}", txt_file.borrow().active_user);

//     let mut user_2 = User {
//         file: Rc::clone(&txt_file),
//     };
//     user_2.file.borrow_mut().active_user += 1;
//     println!("Active users: {:?}", txt_file.borrow().active_user);
// }
// use std::rc::Rc;
// #[derive(Debug)]
// struct ListNode<T> {
//     value: T,
//     next: Option<Rc<ListNode<T>>>,
// }

// fn main() {
//     let node_3 = Rc::new(ListNode {
//         value: 3,
//         next: None,
//     });

//     let node_2 = Rc::new(ListNode {
//         value: 2,
//         next: Some(Rc::clone(&node_3)),
//     });

//     let node_1 = Rc::new(ListNode {
//         value: 1,
//         next: Some(Rc::clone(&node_2)),
//     });

//     assert_eq!(Rc::strong_count(&node_1), 1); // put a value inplace of ?
//     assert_eq!(Rc::strong_count(&node_2), 2); // put a value inplace of ?
//     assert_eq!(Rc::strong_count(&node_3), 2); // put a value inplace of ?
// }

// fn main() {

//     let multiplier = 2;

//     let adder = 5;

//     let transform = |x: i32| -> i32 {

//         x * multiplier + adder

//     };

//     let result = transform(10);

//     println!("Result: {}", result);

// }

// LINKED LIST
// #[derive(Debug)]
// struct Linklist {
//     head: pointer,
// }

// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
// }

// type pointer = Option<Box<Node>>;
// fn main() {
// let list = Node {
//     element: 1,
//     next: None,
// };

// let list = Node {
//     element: 1,
//     next: Some(Box::new(Node {
//         element: 2,
//         next: None,
//     })),
// };

// let list = Linklist {
//     head: Some(Node {
//         element: 1,
//         next: None,
//     }),
// };

// let list = Linklist {
//     head: Some(Node {
//         element: 1,
//         next: Some(Box::new(Node {
//             element: 2,
//             next: Some(Box::new(Node {
//                 element: 3,
//                 next: None,
//             })),
//         })),
//     }),
// };

// let list = Linklist { head: None };

// let list = Linklist {
//     head: Some(Box::new(Node {
//         element: 100,
//         next: Some(Box::new(Node {
//             element: 200,
//             next: None,
//         })),
//     })),
// };

// println!("{:?}", &list.head);
// }

// #[derive(Debug)]
// struct LinkList {
//     head: pointer,
// }

// type pointer = Option<Box<Node>>;
// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
// }
// impl LinkList {
//     fn new() -> LinkList {
//         LinkList { head : None}
//     }
//     fn add(&mut self, element: i32) {}
// }
// fn main() {

// }
// #[derive(Debug)]
// struct Linklist {
//     head: pointer,
// }

// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
// }
// type pointer = Option<Box<Node>>;

// impl Linklist {
//     fn new() -> Linklist {
//         Linklist { head: None }
//     }

//     fn add(&mut self, element: i32) {
//         // match self.head {
//         //     None => {
//         //         let new_node = Some(Box::new(Node {
//         //             element: element,
//         //             next: None,
//         //         }));
//         //         self.head = new_node;
//         //     }
//         //     Some(previous_head) => {
//         //         let new_node = Some(Box::new(Node {
//         //             element: element,
//         //             next: Some(previous_head),
//         //         }));
//         //         self.head = new_node;
//         //     }
//         // }

//         // fn take<T>(dest: &mut T) -> T
//         let previous_head = self.head.take();
//         let new_head = Some(Box::new(Node {
//             element: element,
//             next: previous_head,
//         }));
//         self.head = new_head;
//     }

//     fn remove(&mut self) -> Option<i32> {
//         match self.head.take() {
//             Some(previous_head) => {
//                 self.head = previous_head.next;
//                 Some(previous_head.element)
//             }
//             None => None,
//         }
//     }

//     fn print(&self) {
//         let mut list_traversal = &self.head;
//         while !list_traversal.is_none() {
//             println!("{:?}", list_traversal.as_ref().unwrap().element);
//             list_traversal = &list_traversal.as_ref().unwrap().next;
//         }
//     }
// }
// fn main() {
//     let mut list = Linklist::new();
//     list.add(5);
//     list.add(7);
//     list.add(10);
//     list.add(15);
//     list.add(20);

//     //println!("List: {:?}", list);
//     list.print();
//     println!("{}", list.remove().unwrap());
// }

//DOUBLY LINKED LIST
// use std::{cell::RefCell, rc::Rc};
// #[derive(Debug)]
// struct Doubly_Linklist {
//     head: pointer,
//     tail: pointer,
// }

// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
//     prev: pointer,
// }

// type pointer = Option<Rc<RefCell<Node>>>;

// impl Doubly_Linklist {
//     fn new() -> Self {
//         Doubly_Linklist {
//             head: None,
//             tail: None,
//         }
//     }

//     fn add(&mut self, element: i32) {
//         let new_head = Node::new(element);

//         match self.head.take() {
//             Some(old_head) => {
//                 old_head.borrow_mut().prev = Some(new_head.clone());
//                 new_head.borrow_mut().next = Some(old_head.clone());
//                 self.head = Some(new_head);
//             }

//             None => {
//                 self.tail = Some(new_head.clone());
//                 self.head = Some(new_head);
//             }
//         }
//     }

//     // Case: 1
//     // -----------------------
//     //         Head        Tail
//     // None <-- 1 --> 2 --> 3 --> None
//     // None     1 <-- 2 <-- 3     None
//     // -----------------------

//     // Case: 1 (After Removal)
//     // -----------------------
//     //       Head  Tail
//     // None <-- 2 --> 3 --> None
//     // None     2 <-- 3     None
//     // -----------------------

//     // Case: 2
//     // -----------------------
//     //       Head
//     //       Tail
//     // None <-- 1 --> None
//     // -----------------------

//     // Case: 2 (After Removal)
//     // -----------------------
//     //       Head = None
//     //       Tail = None
//     // -----------------------

//     fn remove(&mut self) -> Option<i32> {
//         if self.head.is_none() {
//             println!("List is empty so we can not remove");
//             None
//         } else {
//             let removed_val = self.head.as_ref().unwrap().borrow().element;
//             self.head
//                 .take()
//                 .map(|old_head| match old_head.borrow_mut().next.take() {
//                     Some(new_head) => {
//                         new_head.borrow_mut().prev = None;
//                         self.head = Some(new_head);
//                         self.head.clone()
//                     }
//                     None => {
//                         self.tail = None;
//                         println!("List is empty after removal");
//                         None
//                     }
//                 });
//             Some(removed_val)
//         }
//     }

//     fn print(&self) {
//         let mut traversal = self.head.clone();
//         while !traversal.is_none() {
//             println!("{}", traversal.as_ref().unwrap().borrow().element);
//             traversal = traversal.unwrap().borrow().next.clone();
//         }
//     }
// }

// impl Node {
//     fn new(element: i32) -> Rc<RefCell<Node>> {
//         Rc::new(RefCell::new(Node {
//             element: element,
//             next: None,
//             prev: None,
//         }))
//     }
// }
// fn main() {
//     let mut list1 = Doubly_Linklist::new();

//     list1.add(30);
//     list1.add(32);
//     list1.add(34);
//     list1.add(36);
//     list1.print();

//     list1.remove();
//     println!("After Removal");
//     list1.print();
// }

// REFERENCE CYCLE
// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
// #[derive(Debug)]
// struct Node {
//     next: Option<Weak<RefCell<Node>>>,
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }
// fn main() {
//     let a = Rc::new(RefCell::new(Node {next: None} ));
//     println!("a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));

//     let b = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&a))}));
//     println!("B is created: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
//     println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

//     let c = Rc::new(RefCell::new(Node {next: Some(Rc::downgrade(&b))}));

//     (*a).borrow_mut().next = Some(Rc::downgrade(&c));

//     println!("After creating cycle: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
//     println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
//     println!("c strong count: {:?}, c weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c));

//     println!("a {:?}", a);

// }

// use std::borrow::Borrow;
// use std::rc::{Rc, Weak};
// use std::cell::{RefCell, Ref};

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }
// fn main() {
//     let leaf  = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// }

//INITIALIZING STRUCT INSTANCE

// use learning_rust::Student;
// fn main() {
//     // let std_1 = Student {
//     //     age: 20,
//     //     name: "Joseph".to_string(),
//     // };
//     let std_1 = Student::new("Joseph".to_string()).unwrap_or_default();
//     println!("{:?}", std_1);

//     let std_2 = Student::default();
//     println!("{:?}", std_2);

//     let std_3 = Student {
//         age: 12,
//         ..Default::default()
//     };
// }

// #[derive(Debug,Default,Clone)]

// -------------------------------------------
// 			Builder Pattern
// -------------------------------------------

// #[derive(Debug, Default, Clone)]
// struct Customer {
//     name: String,
//     username: String,
//     membership: Membershiptype,
//     gender: char,
//     country: String,
//     age: u8,
// }

// #[derive(Debug, Clone)]
// enum Membershiptype {
//     new,
//     causual,
//     loyal,
// }

// impl Default for Membershiptype {
//     fn default() -> Self {
//         Membershiptype::new
//     }
// }

// impl Customer {
//     fn new(name: String) -> CustomerBuilder {
//         CustomerBuilder {
//             name: name,
//             ..Default::default() // username: None,
//                                  // membership: None,
//                                  // gender: None,
//                                  // country: None,
//                                  // age: None,
//         }
//     }
//     // fn new(name: String) -> Self {
//     //     Customer {
//     //         name: name,
//     //         ..Default::default()
//     //     }
//     // }

//     // fn new_2(name: String, username: String) -> Self {
//     //     Customer {
//     //         name: name,
//     //         username: username,
//     //         ..Default::default()
//     //     }
//     // }

//     // fn new_3(name: String, username: String, membership: Membershiptype) -> Self {
//     //     Customer {
//     //         name: name,
//     //         username: username,
//     //         membership: membership,
//     //         ..Default::default()
//     //     }
//     // }
// }

// #[derive(Default)]
// struct CustomerBuilder {
//     name: String,
//     username: Option<String>,
//     membership: Option<Membershiptype>,
//     gender: Option<char>,
//     country: Option<String>,
//     age: Option<u8>,
// }

// impl CustomerBuilder {
//     fn username(&mut self, username: String) -> &mut Self {
//         self.username = Some(username);
//         self
//     }

//     fn membership(&mut self, membership: Membershiptype) -> &mut Self {
//         self.membership = Some(membership);
//         self
//     }

//     fn gender(&mut self, gender: char) -> &mut Self {
//         self.gender = Some(gender);
//         self
//     }
//     fn country(&mut self, country: String) -> &mut Self {
//         self.country = Some(country);
//         self
//     }
//     fn age(&mut self, age: u8) -> &mut Self {
//         self.age = Some(age);
//         self
//     }
//     fn build(&mut self) -> Customer {
//         Customer {
//             name: self.name.clone(),
//             username: self.username.clone().unwrap_or_default(),
//             membership: self.membership.clone().unwrap_or_default(),
//             gender: self.gender.unwrap_or_default(),
//             country: self.country.clone().unwrap_or_default(),
//             age: self.age.unwrap_or_default(),
//         }
//     }
// }
// fn main() {
//     // let new_user = Customer::new("Nouman".to_string());
//     // let user_with_login = Customer::new_2("Joseph".to_string(), "joe123".to_string());
//     // let user_with_membership = Customer::new_3(
//     //     "Micheal".to_string(),
//     //     "micheal2000".to_string(),
//     //     Membershiptype::loyal,
//     // );

//     let new_user = Customer::new("Nouman".to_string()).build();
//     let user_with_login = Customer::new("Joseph".to_string())
//         .username("joe123".to_string())
//         .build();

//     let user_with_membership = Customer::new("Micheal".to_string())
//         .username("micheal2000".to_string())
//         .membership(Membershiptype::loyal)
//         .build();
// }
// -------------------------------------------
//           	- Simplifying Structs
// -------------------------------------------
/*
// Starting code
struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}
*/

// struct A {
//     f1: u32,
//     f2: u32,
//     f3: u32,
//     // b: B,
//     // c: C,
// }

// -------------------------------------------
//           	- Simplifying structures
// -------------------------------------------

// The problem
// struct A {
//     f1: u32,
//     f2: u32,
//     f3: u32,
// }

// fn fn1(a: &mut A) -> &u32 {
//     &a.f2
// }
// fn fn2(a: &mut A) -> u32 {
//     a.f1 + a.f3
// }

// fn fn3(a: &mut A) {
//     let x = fn1(a);
//     let y = fn2(a);
//     println!("{}", x);
// }
// --------- Problem Ends ------

// ---------- Solution --------
// struct A {
//     b: B,
//     c: C,
// }
// struct B {
//     f2: u32,
// }
// struct C {
//     f1: u32,
//     f3: u32,
// }

// fn fn1(b: &mut B) -> &u32 {
//     &b.f2
// }
// fn fn2(c: &mut C) -> u32 {
//     c.f1 + c.f3
// }

// fn fn3(a: &mut A) {
//     let x = fn1(&mut a.b);
//     let y = fn2(&mut a.c);
//     println!("{}", x);
// }

// fn main() {}

// fn main() {
//     let mut stack_var = 4;
//     let heap_var = Box::new(stack_var);

//     stack_var = 5;
//     println!(
//         "The value of stack_var = {} and heap_var = {}",
//         stack_var, heap_var
//     );
// }
// fn main() {
//     let mut name = String::from("Alice");
//     let taken_name = std::mem::take(&mut name);
//     println!("Taken name: {}", taken_name);
//     println!("Remaining name: {}", name);
// }

// fn main() {
//     let count = RefCell::new(0);
//     let borrowed_count = count.borrow();
//     *borrowed_count += 1;
//     println!("Count: {}", borrowed_count);
// }

// use std::cell::RefCell;

// fn main() {
//     let count = RefCell::new(0); // Create a RefCell containing 0
//     {
//         let mut borrowed_count = count.borrow_mut(); // Mutable borrow
//         *borrowed_count += 1; // Increment the value
//     } // Mutable borrow ends here
//     println!("Count: {}", count.borrow()); // Print the value
// }

// SIZE IN RUST
//-- sized types
//-- unsized types

// use std::mem::size_of;
// fn main() {
//sized types
// println!("i32 size is: {}", size_of::<i32>());
// println!("(i32,i32) size is: {}", size_of::<(i32, i32)>());
// println!("[i32: 3] size is: {}", size_of::<[i32; 3]>());
//output
// i32 size is: 4
// (i32,i32) size is: 8
// [i32: 3] size is: 12
// struct Point {
//     x: bool,
//     y: i64,
// }
// println!("Struct size is: {}", size_of::<Point>());
// println!("i32 reference is: {}", size_of::<&i32>());
// println!("i32 mutable reference is: {}", size_of::<&mut i32>());
// /*
// Struct size is: 16
// i32 reference is: 8
// i32 mutable reference is: 8
//  */
// println!("Machine word size is: {}", size_of::<&()>());
// println!("Box<i32> is: {}", size_of::<Box<i32>>());
// println!("fn(i32) -> i32 is: {}", size_of::<fn(i32) -> i32>());
/*Machine word size is: 8
Box<i32> is: 8
fn(i32) -> i32 is: 8
     */

// // Unsized Types
// println!("[i32] size is: {}", size_of::<&[i32]>());
// let a: [i32; 3];
// //println!("str size is: {}", size_of::<str>());
// println!(
//     "The size of the trait object is: {}",
//     size_of::<&dyn Some_trait>()
// );

// -------------------------------------------
// 		- ?Sized and Generic Parameters
// -------------------------------------------

// use std::fmt::Debug;

// 1. Must have a single unsized field.
// 2. The unsized field must be the last field.
// struct UnSizedStruct<T: ?Sized> {
//     sized_field_1: i32,
//     unsized_field: T,
// }

// // fn print_fn<T: Debug>(t: T)
// fn print_fn<T: Debug + ?Sized>(t: &T) {
//     println!("{:?}", t);
// }

// fn main() {
//     let x = UnSizedStruct {
//         sized_field_1: 3,
//         unsized_field: [3],
//     };

//     let x = "my name";
//     print_fn(&x);

//     /*
//     Parameter type          T	        &T          &T
//     Function call Input     &str	        &str        &&str
//     Resolves to             T = &str	    T = str     T = &str
//     */
//}
//}

// -------------------------------------------
// 			- Unsized Coercion
// -------------------------------------------

// fn str_slice_fn(s: &str) {}

// fn array_slice_fn<T>(s: &[T]) {}

// trait Some_Trait {
//     fn method(&self);
// }

// impl<T> Some_Trait for [T] {
//     fn method(&self) {}
// can now call "method" on
// 1) any &[T]
// 2) Vec<T>
// 3) [T; N]
// }
// fn main() {
//     let some_string = String::from("String");
//     str_slice_fn(&some_string);

//     let slice: &[i32] = &[1];
//     let vec = vec![1];
//     let array = [1, 2, 3];

//     array_slice_fn(slice);
//     array_slice_fn(&vec); // deref coercion
//     array_slice_fn(&array); // Unsized coercion

//     slice.method();
//     vec.method();  // deref coercion
//     array.method(); // Unsized coercion
// }

// -------------------------------------------
// 			- Zero Sized Types
// 		        - Never Type
// -------------------------------------------
// to change to nightly use the command of rustup override set nightly
// #![feature(never_type)]

// fn unrecoverable_state() -> ! {
//     panic!("This function will never return normally with something valid");
// }

// // fn function() -> Result<i32, String> {}
// // fn function_1() -> Result<i32, !> {}
// // fn function_2() -> Result<!, String> {}

// // fn function() -> Result<NeverType, String> {}
// // fn function_1() -> Result<i32, NeverType> {}
// enum NeverType {}

// fn main() {
//     // unrecoverable_state();
//     //let x = !;
//     //let x = unrecoverable_state();
//     let x: !;

//     let x = match "123".parse::<i32>() {
//         Ok(num) => num,
//         Err(_) => panic!(),
//     };

//     let x: String = return;
//     let counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break;
//         }
//     };

//     //let x = NeverType;
// }

// -------------------------------------------
// 			- Zero Sized Types
// 			    - Unit Type
// -------------------------------------------
// fn f1() {}
// fn f1() -> () {}
// fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
//     let answer = match divisor {
//         0.0 => Err(String::from("Error: Division by zero")),
//         _ => {
//             println!("The division is invalid");
//             Ok(())
//         }
//     };
//     answer
// }
// fn main() {
//     let x = ();
//     let y = f1();

//     let z = println!("Hello, world!");

//     let mut vec: Vec<()> = Vec::with_capacity(0);
//     vec.push(());
//     vec.push(());
//     vec.push(());
//     assert_eq!(3, vec.len());
//     println!("{}", vec.capacity());

//     /*
//     Unit Type                          || Never Types
//     1. No meaningful value             || 1. Never produces a value
//     2. Function returning unit         || 2. Function returning never, will never
//     always returns normally            || returns normally
//     3. single value, which can not be  || 3. No associated value, and can be coerced
//     coerced                            || to all types.
//     */
// }

// -------------------------------------------
// 		- Zero Sized Types
// 		- Unit Struct
// -------------------------------------------

// struct Admin;
// struct User;

// trait Authenticate {
//     fn authenticate(&self, username: &str, password: &str) -> bool;
// }

// impl Authenticate for Admin {
//     fn authenticate(&self, username: &str, password: &str) -> bool {
//         username == "admin" && password == "adminpass"
//     }
// }

// impl Authenticate for User {
//     fn authenticate(&self, username: &str, password: &str) -> bool {
//         username == "user" && password == "userpass"
//     }
// }

// fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
//     role.authenticate(username, password)
// }

// fn main() {
//     let admin = Admin;
//     let user = User;

//     let admin_login = login(admin, "admin", "adminpass");
//     let user_login = login(user, "user", "userpass");

//     if admin_login {
//         println!("Admin login successful!");
//     } else {
//         println!("Admin login failed!");
//     }

//     if user_login {
//         println!("User login successful!");
//     } else {
//         println!("User login failed!");
//     }
//     struct ABC;
//     let x = ();
//     let y = x;
//     let z = x;

//     let x = ABC;
//     let y = x;
//     //let z = x;
// }

// Extra examples:
/*
// Type-Level Constraints: Unit structs can be used as type-level constraints to enforce certain conditions or behaviors at the type level. For example, you can define a unit struct called NonEmpty to enforce that a collection or container type must always have at least one element.
#[derive(Debug)]
struct NonEmpty<T>(T);

impl<T> NonEmpty<Vec<T>> {
    fn new(data: T) -> Self {
        NonEmpty(vec![data])
    }
}

fn main() {
    let non_empty_vec = NonEmpty::new(10);
    println!("{:?}", non_empty_vec);
}
*/

/*
//Namespace Organization: Unit structs can be used to organize related functions or methods into a single namespace. By defining them within a unit struct, you can group related functionality together and access them using the :: syntax.
mod math {
    pub struct Operations;

    impl Operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

fn main() {
    let result = math::Operations::add(2, 3);
    println!("Addition: {}", result);
}
*/

/*
// Code Documentation: Unit structs can be used to document certain concepts or intentions in your code. By giving a meaningful name to a unit struct, you can convey the purpose or intention of a particular code block or construct.
mod logging {
    pub struct Log;

    impl Log {
        pub fn info(message: &str) {
            println!("INFO: {}", message);
        }

        pub fn error(message: &str) {
            println!("ERROR: {}", message);
        }
    }
}

fn main() {
    logging::Log::info("This is an informational message");
    logging::Log::error("An error occurred!");
}
*/

/*
// Future Extensibility: Unit structs can be used as placeholders for potential future enhancements or extensions. You can define a unit struct with the intention of adding more fields or functionality to it in future versions of your code.
struct FutureExtension;

impl FutureExtension {
    fn new() -> Self {
        FutureExtension
    }

    // Additional methods or fields can be added in the future
    fn additional_feature(&self) {
        println!("Performing additional feature...");
    }
}

fn main() {
    let future = FutureExtension::new();
    future.additional_feature();
}
*/

//THREAD BASICS
// use::std::thread;
// use::std::time::Duration;

// fn main() {
//     println!("This will be printed");
//     println!("This will also be printed");
//     println!("The concurrenct will start after this line");

//     let t = thread::spawn(|| {
//         println!("Hello 1 from the thread");
//         println!("Hello 2 from the thread");
//         println!("Hello 3 from the thread");
//         println!("Hello 4 from the thread");
//         println!("Hello 5 from the thread");
//         println!("Hello 6 from the thread");
//         println!("Hello 7 from the thread");
//         println!("Hello 8 from the thread");
//     });
//     // thread::sleep(Duration::from_millis(1));
//     t.join();
//     println!("Hello 9 from the main");
//     println!("Hello 10 from the main");

// }

//OWNERSHIP IN THREADS
// -------------------------------------------
// 	Ownership and Threads
//              - Prerequiste: Closures
// -------------------------------------------

// use std::thread;
// fn main() {
//     let x = "some string".to_string();

//     thread::spawn(move || {
//         // let y = x;
//         println!("{x}");
//     });
//     //println!("{x}");
// }

// use std::thread;

// fn main() {

//     let handle_1 = thread::spawn(|| {
//         let mut sum = 0;
//         let range = 0..=1_000;
//         for num in range {
//             sum += num;
//         }
//         sum
//     });
//     let handle_2 = thread::spawn(|| {
//         let mut sum = handle_1.join().unwrap();
//         let range = 1001..=2000;
//         for num in range {
//             sum += num;
//         }
//         sum
//     });
//     let handle_3 = thread::spawn(|| {
//         let mut sum = handle_2.join().unwrap();
//         let range = 2001..=3000;
//         for num in range {
//             sum += num;
//         }
//         sum
//     });

//     let mut sum = 0;

//     sum = handle_3.join().unwrap();

//     println!("Final Summation Result {sum}");
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];
//     let x = 5;
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//         println!("Here's a variable : {:?}", x);
//     });

//     println!("The variable x is still alive {}", x);    // Note: premitives are not moved but copied
//                                                         // so no issues here
//     // println!("The variable v is not alive {}", v);   // Note: Heap allocated data are moved so no more usable
//     handle.join();
// }

//MESSAGE PASSING THROUGH CHANNELS

// use std::thread;
// use std::sync::mpsc;
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     //  let rx_clone = rx.clone();
//      for i in 0..10 {
//         let tx_clone = tx.clone();
//         thread::spawn(move || {
//             println!("Sending value {i}");
//             tx_clone.send(i).unwrap();
//         });
//     }
// }
// // Example 1
// /*
// use std::sync::mpsc;
// use std::thread;
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     // let rx_clone = rx.clone();
//     for i in 0..10 {
//         let tx_clone = tx.clone();
//         thread::spawn(move || {
//             println!("Sending value {i}");
//             tx_clone.send(i).unwrap();
//         });
//     }

//     drop(tx);
//     let recv_val = rx.recv().unwrap();
//     println!("Recieved {recv_val}");
//     let recv_val = rx.recv().unwrap();
//     println!("Received {recv_val}");

//     // for message in rx {
//     //     println!("Received {message}");
//     // }
// }
// */
// // Example 2
// use std::{sync::mpsc, thread};
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || {
//         let mut i = "5".to_string();
//         println!("Sending value {i}");
//         tx.send(i).unwrap();
//     });

//     // let received_val = rx.recv().unwrap();
//     // println!("Received {received_val}");

//     // t.join();
//     let mut received_status = false;
//     while received_status != true {
//         match rx.try_recv() {
//             Ok(received_value) => {
//                 println!("Received value is: {received_value}");
//                 received_status = true;
//             }
//             Err(_) => println!("I am doing some other stuff"),
//         }
//     }
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let handle = thread::spawn(move || {
//         let x = "some_value".to_string();
//         println!("Sending value {x}");
//         // thread::sleep(Duration::from_secs(3));
//         tx.send(x).unwrap();
//     });

//     // rx.recv().unwrap();
//     // println!("I am blocked");

//     let mut received_status = false;
//     while received_status != true {
//         match rx.try_recv() {
//             Ok(received_value) => {
//                 println!("Received value is {:?}", received_value);
//                 received_status = true;
//             }
//             Err(_) => println!("I am doing some other stuff"),
//         }
//     }
// }

// use std::sync::mpsc;
// use std::thread;

// fn thread_fn(d: i32, tx: mpsc::Sender<i32>) {
//     thread::spawn(move || {
//         println!("{} send!", d);
//         // Add code for sending d
//         tx.send(d).unwrap();
//     });
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     for i in 0..5 {
//         // Add code for calling the function with value i and a clone of tx
//         thread_fn(i, tx.clone());
//     }

//     drop(tx);

//     for recieving_val in rx {
//         println!("{} received!", recieving_val);
//     }
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || {
//         let x = "some_value".to_string();
//         println!("Sending value {x}");
//         tx.send(x).unwrap();
//     });

//     let mut received_status = false;
//     while !received_status {
//         match rx.try_recv() {
//             Ok(received_value) => {
//                 println!("Received value is: {received_value}");
//                 received_status = true;
//             }
//             Err(_) => {
//                 println!("I am doing some other stuff"); // This will now execute
//                 thread::sleep(Duration::from_millis(50)); // Add a small delay
//             }
//         }
//     }

//     t.join().unwrap(); // Properly handle the thread join
// }

//SHARING STATES

// use std::sync::Mutex;
// use::std::thread;

// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 10;
//     }

//     let lock_m = m.lock().unwrap();
//     println!("m is: {:?}", *lock_m);
//     drop(lock_m);   // add later on

//     let lock_m1 = m.lock().unwrap();
//     println!("This code is blocked");
// }

// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// struct File {
//     text: Vec<String>,
// }
// fn main() {

// }

// use std::rc::Rc;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread;

// struct File {
//     text: Vec<String>,
// }
// fn main() {
//     let file = Arc::new(Mutex::new(File { text: vec![] }));
//     let mut thread_vec = vec![];

//     for i in 0..10 {
//         let file = Arc::clone(&file);
//         let handle = thread::spawn(move || {
//             let mut file_lock = file.lock().unwrap();
//             file_lock.text.push(format!("Hello from Thread {i}"));
//         });
//         thread_vec.push(handle);
//     }
//     for handle in thread_vec {
//         handle.join().unwrap();
//     }

//     let file_lock = file.lock().unwrap();
//     for t in &file_lock.text {
//         println!("{t}");
//     }
// }

//BARRIERS
// use std::sync::{Arc, Barrier, Mutex};
// use std::thread;

// fn main() {
//     let mut threads_vec = Vec::new();
//     let tasks = Arc::new(Mutex::new(vec![]));
//     let barrier = Arc::new(Barrier::new(5));

//     for i in 0..5 {
//         let tasks = tasks.clone();
//         let barrier = barrier.clone();
//         let handle = thread::spawn(move || {
//             // Tasks 1
//             tasks
//                 .lock()
//                 .unwrap()
//                 .push(format!("Thread {i}, Completed its part on Task 1"));

//             barrier.wait();
//             // Task 2
//             tasks
//                 .lock()
//                 .unwrap()
//                 .push(format!("Thread {i}, Completed its part on Task 2"));

//             barrier.wait();
//             // Task 3
//             tasks
//                 .lock()
//                 .unwrap()
//                 .push(format!("Thread {i}, Completed its part on Task 3"));
//         });
//         threads_vec.push(handle);
//     }

//     for handle in threads_vec {
//         handle.join().unwrap();
//     }

//     let task_lock = &*tasks.lock().unwrap();
//     for contents in task_lock {
//         println!("{contents}");
//     }
// }

// -------------------------------------------
// 	 Scoped Threads
// -------------------------------------------

// use std::thread;

// fn main() {
//     let mut vec = vec![1, 2, 3];

//     thread::scope(|some_scope| {
//         some_scope.spawn(|| {
//             println!("Thread inside scope");
//             println!("vec: {:?}", vec);
//         });

//         some_scope.spawn(|| {
//             println!("Another Thread inside scope");
//             //vec.push(4);
//             println!("vec: {:?}", vec);
//         });
//     });

//     println!("The scope finished");
//     vec.push(5);
//     println!("vec: {:?}", vec);
// }

//THREAD PARKING
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;
// fn main() {
//     let data = Arc::new(Mutex::new(5));
//     let data_clone = data.clone();
//     let thread_1 = thread::spawn(move || {
//         println!("Thread 1: I am doing some work");
//         println!("Thread 1: I am doing some more work");
//         // thread::sleep(Duration::from_secs(2));

//         println!("Thread 1: Parked");
//         thread::park();
//         //thread::park_timeout(Duration::from_secs(4));

//         println!("Thread 1: Printing the updated data");
//         println!("Thread 1: Data: {:?}", *data.lock().unwrap());
//     });

//     let thread_2 = thread::spawn(move || {
//         println!("Thread 2: I am working on updating the data");
//         thread::sleep(Duration::from_secs(1));
//         *data_clone.lock().unwrap() = 10;
//         println!("Thread 2: Data updated completed");
//     });
//     thread_2.join();
//     thread_1.thread().unpark();
//     thread_1.join();
// }

// ASYNC AWAIT
// async fn printing() {
//     println!("I am async function")
// }
// #[tokio::main]
// async fn main() {
//     let x = printing().await;
// }

//TOKIO
// async fn printing(i: i32) {
//     sleep(Duration::from_secs(1)).await;
//     println!("Task {i}");
// }
// use std::time::Duration;
// use tokio::time::sleep;
// #[tokio::main]
// //#[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let mut handles = vec![];
//     for i in 0..3 {
//         let handle = tokio::spawn(async move {
//             println!("Task {i}, printing, first time");
//             printing(i).await;
//             println!("Task {i}, printing, second time");
//             printing(i).await;
//             println!("Task {i}, completed");
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("All Tasks are now completed");
// }

// //      Computationally Intensive Functions
// // -------------------------------------------

// /*
// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let mut handles = vec![];

//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("All tasks are now completed");
// }
// */
// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let mut handles = vec![];

//     handles.push(tokio::spawn(async {
//         // heavy_computations();
//         let _result = tokio::task::spawn_blocking(|| {
//             heavy_computations();
//         })
//         .await;
//     }));

//     handles.push(tokio::spawn(async move {
//         simpler_computations().await;
//     }));

//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("All tasks are now completed");
// }

// fn heavy_computations() {
//     println!("fn heavy_computations started");
//     let mut d = 0;
//     for i in 0..300_000_000 {
//         d = d + 1;
//     }
//     println!("fn heavy_computations completed");
// }

// async fn simpler_computations() {
//     println!("fn simpler_computations started");
//     println!("fn simpler_computations completed");
// }

//WEB SCRAPPING
// use std::sync::{mpsc,Arc,Mutex};
// use std::time::{Duration, Instant};
// use std::thread;
// use ureq::{Agent, AgentBuilder};
// fn main() -> Result<(), ureq::Error>{
//  let webpages = vec![
//      "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
//      "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/460157afc6a7492555bb",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/c9bc4130af995c36176d",
//      "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
//      "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/460157afc6a7492555bb",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/c9bc4130af995c36176d",
//      "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
//      "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/460157afc6a7492555bb",
//      "https://gist.github.com/recluze/5051735efe3fc189b90d",
//      "https://gist.github.com/recluze/c9bc4130af995c36176d",
//  ];

//  let agent = ureq::AgentBuilder::new().build();
//  let now = Instant::now();

//  for web_page in &webpages {
//      let web_body = agent.get(web_page).call()?.into_string()?;
//  }
//  println!("Time taken wihtout Threads: {:.2?}", now.elapsed());

//  let now = Instant::now();
//  let agent = Arc::new(agent);
//  let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

//  for web_page in webpages {
//      let agent_thread = agent.clone();
//      let t = thread::spawn(move || {
//          let web_body = agent_thread
//          .get(web_page)
//          .call()?
//          .into_string()?;

//          Ok(())
//      });
//      handles.push(t);
//  }

//  for handle in handles {
//      handle.join().unwrap();
//  }

//  println!("Time taken using Threads: {:.2?}", now.elapsed());
//  Ok(())

// }

// let mut vec_1 = vec![1, 2, 3];
// let mut vec_2 = vec![4, 5, 6];
// let mut reference = &vec_1;
// reference = &vec_2;

// let mut reference = &mut vec_1;
// reference.push(10);
// reference = &mut vec_2;

// let mut reference = &mut &vec_1;
// reference = &mut &vec_2;

// // Summary
// //     ref: &T;       // immutable binding of an immutable reference
// // mut ref: &T;       // mutable binding of an immutable reference
// //     ref: &mut T;   // immutable binding of a mutable reference
// // mut ref: &mut T;   // mutable binding of a mutable reference
// //     ref: &mut &T;  // immutable binding of a mutable reference to an immutable reference.
// // mut ref: &mut &T;  // mutable binding of a mutable reference to an immutable reference.

// fn main() {
//     let str = String::from("");
//     let ref_str_1 = &str;
//     let ref_str_2 = ref_str_1;
//     println!("{ref_str_1}");

//     let mut str_slice_1: &str;
//     {
//         let str_slice_2 = "";
//         str_slice_1 = "";
//     }
//     println!("{str_slice_1}");

//     let str_slice_from_string: &str;
//     {
//         let string = String::from("");
//         //str_slice_from_string = &*string;
//     }

//     //println!("{str_slice_from_string}");

//     let mut x = 45;
//     let z = &mut x;
//     let y = &*z;
// }

// fn largest<'a, 'b>(x: &'a u8, y: &'b u8) -> &'a u8
// where
//     'b: 'a,
// {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

// -------------------------------------------
// 	        Implied Bounds
// -------------------------------------------

// struct GenericWithLifetime<'a, T> {
//     b: &'a T, // implied T: 'a   // 'a: 'b => 'a outlives 'b and now T: 'a => T outlives 'a
// }

// fn lifetime_bound_implied_fn<'a, T>(x: GenericWithLifetime<T>) {} // implied T: 'a

// fn lifetime_bound_implied_fn_1<'b, U>(x: &'b U) {} // implied U: 'b

// fn requires_lifetime_bound<'a, T>() {} // bound not not implied

// trait Trait<'a, T: 'a> {}
// //impl<'a, T> Trait<'a, T> for () {} // not implied T: 'a

// // This compiles as `T: 'a` is implied by the self type `&'a T`.
// impl<'a, T> Trait<'a, T> for &'a T {} // implied T: 'a

// struct A<T: Clone>(T);
// fn doesnt_specify_T_Clone<T>(x: A<T>) {} // add the lifetime bound of T: Clone
// fn main() {}

// // -------------------------------------------
// // 	        Subtyping and Variance (Covariance)
// //
// //          Prereqs: Subtyping, Lifetime Elision
// // -------------------------------------------

// // Subtype
// // X: Y
// // 'x: 'y => 'x outlives 'y

// // Variance
// // 'x: 'y = > T<'x> relationship with T<'y> ?

// // Types of variance
// //          1. Covariance:  if 'x: 'y then T<'x>: T<'y>
// //                          if 'static: 'a then 'static T: 'a T, assume T = &str
// //                          &'static str: &'a str

// //          2. Contravariance
// //          3. Invariance

// fn accepts_str(s: &str) {}
// // fn accepts_str(s: &'a str)

// // We can provide
// // fn(&'a str) but also fn(&'static str)

// fn main() {
//     let s_1 = String::from("");
//     let s_2 = ""; // 'static

//     // let s_4: &str;
//     // {
//     //     let s_2 = "";
//     //     s_4 = s_2;
//     // }
//     // println!("{s_4}");
//     let s_3 = &*s_1; // 'a

//     accepts_str(s_2); // 'static
//     accepts_str(s_3); // 'a

//     let mut x = &*s_1; // 'a
//     let mut y: &'static str = ""; // 'static

//     x = y; // y: &'a str however 'static: 'a => &'static: &'a str
//     //y = x; // not valid
// }

// // -------------------------------------------
// // 	        Contravariance
// //
// //          Prereqs: Lifetime Elision, Subtyping
// // -------------------------------------------

// // Subtype
// // 'x: 'y  => T<'x> relationship with T<'y>?

// // Types of variance
// //      1. Covariance: If 'x: 'y then T<'x>: T<'y>
// //      2. Contravariance: if 'x: 'y  then T<'y>: T<'x>
// //                         if 'static: 'a then 'a T: 'static T, assume T = fn(&str)
// //                         fn(&'a str): fn(&'static str)
// //      3. Invariance

// // function expects:
// // fn(fn(&'a str))

// // However, we provided:
// // fn(fn(&'static str))

// fn str_fn(f: fn(&'static str)) {}

// fn accepts_str_1(s: &'static str) {}
// fn accepts_str_2(s: &str) {}
// fn main() {
//     str_fn(accepts_str_1);
//     str_fn(accepts_str_2);

//     let mut x: fn(&str) = accepts_str_2; // fn(&'a str)
//     let mut y: fn(&'static str) = accepts_str_1; // fn(&'static str)

//     //x = y;
//     y = x;
// }

// // -------------------------------------------
// // 	        Invariance
// //
// //          Prereqs: Lifetime Elision, Subtyping
// // -------------------------------------------

// /* // Subtype
// // 'x: 'y  => T<'x> relationship with T<'y>?

// // Types of variance
// //      1. Covariance: If 'x: 'y then T<'x>: T<'y>
// //      2. Contravariance: if 'x: 'y then T<'y>: T<'x>
// //      3. Invariance: if 'x: 'y then T<'x> has no relationship T<'y>
// */
// // function expects: fn(&mut Vec<&'a str>,      &'a str)
// // We provided:      fn(&mut Vec<&'static str>, &'a str)

// fn accept_vec(str_vec: &mut Vec<&str>, s: &str) {
//     str_vec.push(s)
// }
// fn main() {
//     let mut vec: Vec<&'static str> = vec!["", ""];
//     let non_static_str = &*String::from("");
//     accept_vec(vec, non_static_str);
// }

// // -------------------------------------------
// // 	        Invariance
// //              - Mutable references are invariant in their Type or T
// //                but covariant in their lifetime
// // -------------------------------------------
// // &'a mut T
// fn update_reference<'longer, 'shorter>(mut u_1: &'longer mut u8, mut u_2: &'shorter mut u8)
// where
//     'longer: 'shorter,
// {
//     u_2 = u_1; // &'shorter mut u8 = &'longer mut u8
// }
// fn main() {
//     // &'a mut &'b str  // (Note: The 2nd reference: &'b str was considered in the previous lesson on invariance)
//                         // In this lesson we are focusing and explaining the first reference: &'a mut

//     let mut x1: u8 = 8;
//     let mut x2: u8 = 10;
//     let mut y = &mut x1;
//     {
//         let mut z = &mut x2;
//         update_reference(y, z);
//     }
// }

//

// macro_rules! Insallah_macro {
//     () => { 1 + 1

//     };
// }
// fn main() {
//     Insallah_macro!();
//     println!("{}",Insallah_macro!());
// }

// -------------------------------------------------
// 			Declarative Macros
// -------------------------------------------------

/* General Syntax

macro_rules! macro_name {
//   |--- Match rules
    (...) => { ... };
    (...) => { ... };
    (...) => { ... };    // the semicolon at the last rule is optional
}
*/

// macro_rules! our_macro {
//     () => { 1+1;
//     };

//     (something 4 u dear u32 @_@) => {
//         println!("You found nonsense here")
//     };

//     ($e1:expr, $e2:expr) => {
//         $e1 + $e2
//     };

//     ($a:expr, $b:expr; $c:expr) => {
//         $a * ($b + $c)
//     }
// }
// fn main() {

//     println!("{}",our_macro!());
//     our_macro!();
//     println!("{}", our_macro!(2,2));
//     println!("{}", our_macro!(5,6;3));
//     // println!("{}", our_macro!("something",2;"nothing"));

//     our_macro!();
//     our_macro![];

//     our_macro!{};

// 1. cargo install cargo-expand
// 2. rustup install nightly
// 3. rustup component add rustfmt
// 4. rustup component add rustfmt --toolchain nightly
//}

// -------------------------------------------------
// 			Capturing Types
// -------------------------------------------------
/*
macro_rules! input {
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");

        let n: $t = n.trim().parse().expect("invalid input");
        n
    }};
}

macro_rules! add_as {
    ($a: expr, $b: expr, $typ: ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1;
    };
}
fn main() {
    /*
    println!("Please enter a floating point number");
    let some_input_0 = input!(f32);
    */

    // println!("{}", add_as!(15,2.3,f32));
    let mut x = 4;
    some_macro!(x);
}
*/
// macro_rules! create_function {
//     ($func_name:ident, $input: ident, $type_input: ty, $type_output: ty) => {
//         fn $func_name($input: $type_input) -> $type_output {
//             println!(
//                 "You called {:?}() with the input of {:?}",
//                 stringify!($func_name),
//                 stringify!($input1)
//             );
//             $input
//         }
//     };
// }

// create_function!(f1, x, i32, i32);
// fn main() {
//     //f1(15);
//     let y = f1(15);
// }

// -------------------------------------------------
//          	- Repeating Patterns
// -------------------------------------------------

// macro_rules! string_concat {
//     /*
//     () => {
//         String::new();
//     };

//     ($some_str: expr) => {{
//         let mut temp_str = String::new();
//         temp_str.push_str($some_str);
//         temp_str
//     }
//     };

//     ($some_s1: expr, $some_s2:expr) => {{
//         let mut temp_str = String::new();
//         temp_str.push_str($some_s1);
//         temp_str.push_str($some_s2);

//         temp_str
//     }
// };

// */

// ($($some_str:expr,) *) => {{
//     let mut temp_str = String::new();
//     $(temp_str.push_str($some_str);)*
//     temp_str
// }
// };


// }

// macro_rules! vec_mac {
//     ( $($element: expr),*) => {{
//         let mut some_vec = Vec::new();
//         $(some_vec.push($element);)*
//         some_vec
//     }
// };
// }
// fn main() {
//     let str_null = string_concat!();
//     let str_single = string_concat!("First",);

//     let str_double = string_concat!("First", "Second",);

//     let string_vec = vec_mac!("Nouman", "Azam");
// }

// -------------------------------------------
// 			Question Marks Operator
// -------------------------------------------

/*
use std::num::ParseIntError;
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let integer = input.parse::<i32>()?;

    println!("The value {:?} is integer {:?}", input, integer);
    Ok(integer)
}
fn main() {
    let some_values = vec!["123", "some1", "some(123)", "abc", "53"];
    for value in some_values {
        println!("{:?}", parse_str(value));
    }
}
*/

/*
fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    //fn division(dividend: f64, divisor: f64) -> Result<f64, u8>  {

    let answer = match divisor {
        0.0 => Err(String::from("Error: Division by zero")), // Err(8),
        _ => Ok(dividend / divisor),
    };

    let correct = answer?; // it is also going to unwrap the result
                           // A small note: this operator will take the ownership
                           //println!("{:?}",answer);
    println!("This line will not print in case of error {:?}", correct);
    Ok(correct)
}

fn main() {
    println!(
        "Call from main with result equals to {:?}\n",
        division(9.0, 3.0)
    );
    println!(
        "Call from main with result equals to {:?}\n",
        division(4.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}\n",
        division(0.0, 2.0)
    );
}
*/

/*
fn division(dividend: f64, divisor: f64) -> Option<f64> {
    let answer = match divisor {
        0.0 => None,
        _ => Some(dividend / divisor),
    };

    let correct = answer?; // A small note: this operator will take the ownership

    // println!("{:?}",answer);
    println!("This line will not print in case of error {:?}", correct);
    Some(correct)
}

fn main() {
    //let x = division(1.0,2.0);
    println!(
        "Call from main with result equals to {:?}\n",
        division(9.0, 3.0)
    );
    println!(
        "Call from main with result equals to {:?}\n",
        division(4.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}\n",
        division(0.0, 2.0)
    );
}
*/

/*
#[derive(Debug)]
enum MathError {
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogarithm,
    SqrtError_NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn division(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionError_DivisionByZero)
    } else {
        println!("The division is successful and has a result of {}", x / y);
        Ok(())
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else {
        println!(
            "The square root is successful and has a result of {}",
            x.sqrt()
        );
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::LogError_NonPositiveLogarithm)
    } else {
        println!("The log was successful and has a result of {}", x.ln());
        Ok(())
    }
}
fn operations(x: f64, y: f64) -> MathResult {
    division(x, y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

fn main() {
    let result = operations(0.0, 10.0);
    if result.is_ok() {
        println!("All the functions executed successfully");
    } else {
        println!("{:?}", result);
    }
}
*/

// Another example not in the video lectures but simple to understand and very useful
// use std::env;

// fn current_dir1() -> std::io::Result<()> {
//     let path = env::current_dir()?; // for unwrapping the current path
//     println!("\n\n The current path is {:?}\n\n ", path);
//     Ok(())
// }

// fn main() {
//     current_dir1();
// }

// // -------------------------------------------
// // 			Correct Search Results Using Word Grouping
// //           	- Description
// //           	    - Given a list of words, group the words that are anagrams
// //           	- Tools
// //           	    - Hashmaps, Nested Loops
// // -------------------------------------------

// use std::collections::HashMap;
// fn word_grouping(words_list: Vec<String>) -> Vec<Vec<String>> {
//     let mut word_hash = HashMap::new();
//     let mut char_freq = vec![0; 26];

//     for current_word in words_list {
//         for c in current_word.to_lowercase().chars() {
//             char_freq[(c as u32 - 'a' as u32) as usize] += 1;
//         }

//         let key: String = char_freq
//             .into_iter()
//             .map(|i| i.to_string())
//             .collect::<String>();
//         word_hash
//             .entry(key)
//             .or_insert(Vec::new())
//             .push(current_word);

//         char_freq = vec![0; 26];
//     }

//     // Just for hte sake of output and confirming the (key, value) pairs
//     for (key, value) in &word_hash {
//         println!("key # {:?} value {:?}", key, value);
//     }

//     word_hash.into_iter().map(|(_, v)| v).collect()
// }

// fn main() {
//     let words = vec![
//         "The".to_string(),
//         "teh".to_string(),
//         "het".to_string(),
//         "stupid".to_string(),
//         "studpi".to_string(),
//         "apple".to_string(),
//         "appel".to_string(),
//     ];

//     let grouping = word_grouping(words);
//     println!("{:?}\n\n\n", grouping);

//     let input_word = String::from("teh");

//     for i in grouping.into_iter() {
//         if i.contains(&input_word) {
//             println!("The group of the word is {:?}", i);
//         }
//     }
// }

// -------------------------------------------
// 			Popularity Scores
//           	- Description
//           	    - Given some products along with its respectively popularity scores,
//                     We want to determine if the popularity is fulctuating, increasing or decreasing
//           	- Tools
//           	    - Hashmaps, Loops, conditional if
// -------------------------------------------

// fn popularity_analysis(scores: Vec<i32>) -> bool {
//     let mut increasing = true;
//     let mut decreasing = true;

//     for i in 0..scores.len() - 1 {
//         if scores[i] > scores[i + 1] {
//             increasing = false;
//         }
//         if scores[i] < scores[i + 1] {
//             decreasing = false;
//         }
//     }
//     return increasing || decreasing;
// }

// use std::collections::HashMap;

// fn main() {
//     let mut products = HashMap::new();

//     products.insert("Product 1", vec![1, 2, 2, 3]);
//     products.insert("Product 2", vec![4, 5, 6, 3, 4]);
//     products.insert("Product 3", vec![8, 8, 7, 6, 5, 4, 4, 1]);

//     for (product_id, popularity) in products {
//         if popularity_analysis(popularity) {
//             println!("{} popularity is increasing or decreasing", product_id);
//         } else {
//             println!("{} popularity is Fluctuating", product_id);
//         }
//     }
// }

// // -------------------------------------------
// // 			Highest Price Stock
// //           	- Description
// //           	    - We have weekwise stock prices. we want to retrieve the highest
// //                     stock price in any in any week in little to no time.
// //           	- Tools
// //           	    - Maxstacks, Structures, Vectors
// // -------------------------------------------

// struct MaxStack {
//     main_stack: Vec<i32>,
//     maximum_stack: Vec<i32>,
// }
// impl MaxStack {
//     fn new() -> Self {
//         MaxStack {
//             main_stack: Vec::new(),
//             maximum_stack: Vec::new(),
//         }
//     }

//     fn push(&mut self, value: i32) {
//         self.main_stack.push(value);
//         if !self.maximum_stack.is_empty() && self.maximum_stack.last().unwrap() > &value {
//             self.maximum_stack.push(*self.maximum_stack.last().unwrap());
//         } else {
//             self.maximum_stack.push(value);
//         }
//     }
//     fn pop(&mut self) {
//         self.main_stack.pop();
//         self.maximum_stack.pop();
//     }

//     fn max_value(&self) -> i32 {
//         *self.maximum_stack.last().unwrap()
//     }
// }

// fn main() {
//     let mut stack = MaxStack::new();
//     stack.push(55);
//     stack.push(80);
//     stack.push(120);
//     stack.push(99);
//     stack.push(22);
//     stack.push(140);
//     stack.push(145);

//     print!("Maximum value of stock: ");
//     println!("{:}", stack.max_value());

//     println!("After going one week back");
//     print!("Maximum value of stock: ");
//     stack.pop();

//     println!("{:}", stack.max_value());
// }

// // -------------------------------------------
// // 			Employee with no Meeting
// //           	- Description
// //           	    - Given meeting schedule of employees, we want to determine the overlappign time.
// //           	- Tools
// //           	    - MultiDimensional Arrays, Nested Loops.
// // -------------------------------------------

// // essential condition is to check for two ranges of the form [x1, x2] and [y1,y2] is max(x1,y1) <= min(x2,y2)
// use std::cmp;
// fn overlapping_meetings(meetings_a: Vec<Vec<i32>>, meetings_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut intersection: Vec<Vec<i32>> = Vec::new();
//     for i in 0..meetings_a.len() {
//         for j in 0..meetings_b.len() {
//             let (st_a, st_b) = (meetings_a[i][0], meetings_b[j][0]);
//             let (ed_a, ed_b) = (meetings_a[i][1], meetings_b[j][1]);
//             let overlap_status = overlap(st_a, st_b, ed_a, ed_b);
//             if overlap_status != None {
//                 intersection.push(overlap_status.unwrap());
//             }
//         }
//     }
//     intersection
// }

// fn overlap(start_a: i32, start_b: i32, end_a: i32, end_b: i32) -> Option<Vec<i32>> {
//     let mut intersection_time: Vec<i32> = Vec::new();
//     if cmp::max(start_a, start_b) < cmp::min(end_a, end_b) {
//         intersection_time.push(cmp::max(start_a, start_b));
//         intersection_time.push(cmp::min(end_a, end_b));
//         Some(intersection_time)
//     } else {
//         None
//     }
// }

// fn main() {
//     let meetings_sec_a: Vec<Vec<i32>> = vec![vec![13, 15], vec![15, 16], vec![7, 9]];
//     let meetings_sec_b: Vec<Vec<i32>> = vec![vec![14, 15], vec![5, 10]];

//     let intersection = overlapping_meetings(meetings_sec_a, meetings_sec_b);
//     println!("The overlapping timings are {:?}", intersection);
// }

// // -------------------------------------------
// // 			Longest Non-Stop Work
// //           	- Description
// //           	    - Given time slots numbers, we want to determine the longest consective time slots.
// //           	- Tools
// //           	    - HashSet, Vectors, Loops
// // -------------------------------------------

// use std::collections::HashSet;
// fn longest_busy_time(working_slots: Vec<Vec<u8>>) -> u8 {
//     let mut employee_longest_nonstop_work: Vec<u8> = Vec::new();
//     for i in working_slots {
//         employee_longest_nonstop_work.push(longest_period(i));
//     }

//     for i in 0..employee_longest_nonstop_work.len() {
//         println!(
//             "Employee number {} has worked nonstop for {} slots",
//             i, employee_longest_nonstop_work[i]
//         );
//     }

//     let max_val = employee_longest_nonstop_work.iter().max();
//     employee_longest_nonstop_work
//         .iter()
//         .position(|x| *x == *max_val.unwrap())
//         .unwrap() as u8
// }

// fn longest_period(working_slots: Vec<u8>) -> u8 {
//     let mut longest_busy_period = 0;
//     let slot_set: HashSet<_> = working_slots.into_iter().collect();

//     for slot in &slot_set {
//         if !slot_set.contains(&(slot - 1)) {
//             let mut current_slot = slot.to_owned();
//             let mut current_consecutive_slot = 1;
//             while slot_set.contains(&(current_slot + 1)) {
//                 current_slot += 1;
//                 current_consecutive_slot += 1;
//             }
//             if current_consecutive_slot > longest_busy_period {
//                 longest_busy_period = current_consecutive_slot;
//             }
//         }
//     }

//     return longest_busy_period;
// }

// fn main() {
//     let schedule: Vec<Vec<u8>> = vec![
//         vec![4, 1, 2, 5, 6, 8, 10, 11],
//         vec![3, 1, 2, 5, 7, 10, 11, 14],
//         vec![3, 1, 15, 5, 13, 12, 10, 14, 15, 16, 17, 18, 8, 9],
//     ];

//     println!(
//         "Employee number: {} has the highest number of nonstop working slots",
//         longest_busy_time(schedule)
//     );
// }

// // -------------------------------------------
// // 			Suggesting Items for Special Shopping Card
// //           	- Description
// //           	    - Given a list of prices, return a couple of items with their sum matching the given price.
// //           	- Tools
// //           	    - Hashsets, Vectors
// // -------------------------------------------

// use std::collections::HashSet;
// fn product_suggestions(product_prices: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
//     let mut prices_hash = HashSet::new();
//     let mut offers = Vec::new();

//     //vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
//     for i in product_prices {
//         let diff = amount - i;
//         if prices_hash.get(&diff).is_none() {
//             prices_hash.insert(i);
//         } else {
//             offers.push(vec![i, diff]);
//         }
//     }

//     offers
// }

// fn main() {
//     let product = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];

//     let suggestions = product_suggestions(product, 50);
//     println!("{:?}", suggestions);
// }

// // -------------------------------------------
// // 			Fetching Top Products
// //           	- Description
// //           	    - We are given link lists corresponding to top ranked products in different countries.
// //           	      We need to combine all these link lists into one consolidated list containing the ranks in an descneder order
// //           	- Tools
// //           	    - Linklist + Iterators
// // -------------------------------------------

// #[derive(Debug)]

// struct Linklist<T: std::fmt::Debug> {
//     head: pointer<T>,
// }

// #[derive(Debug)]

// struct Node<T> {
//     element: T,
//     next: pointer<T>,
// }

// type pointer<T> = Option<Box<Node<T>>>;

// impl<T: std::fmt::Debug> Linklist<T> {
//     fn create_empty_list() -> Linklist<T> {
//         Linklist { head: None }
//     }

//     fn add(&mut self, element: T) {
//         let previous_head = self.head.take();
//         let new_head = Box::new(Node {
//             element: element,
//             next: previous_head,
//         });
//         self.head = Some(new_head);
//     }

//     fn reverse(&mut self) {
//         if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
//             return;
//         }

//         let mut prev = None;
//         let mut current_node = self.head.take();
//         while current_node.is_some() {
//             let next = current_node.as_mut().unwrap().next.take();
//             current_node.as_mut().unwrap().next = prev.take();
//             prev = current_node.take();
//             current_node = next;
//         }

//         self.head = prev.take();
//     }

//     fn remove(&mut self) -> Option<T> {
//         let previous_head = self.head.take();
//         match previous_head {
//             Some(old_head) => {
//                 self.head = old_head.next;
//                 Some(old_head.element)
//             }
//             None => None,
//         }
//     }
//     fn peek(&self) -> Option<&T> {
//         match &self.head {
//             Some(H) => Some(&H.element),
//             None => None,
//         }
//     }
//     fn printing(&self) {
//         let mut list_traversal = &self.head;
//         println!();

//         while true {
//             match list_traversal {
//                 Some(Node) => {
//                     print!("{:?} ", Node.element);
//                     list_traversal = &list_traversal.as_ref().unwrap().next;
//                 }
//                 None => break,
//             }
//         }
//     }
// }

// fn sorting_lists(vec_list: &mut Vec<Linklist<i32>>) -> Linklist<i32> {
//     let mut sortted_list: Linklist<i32> = Linklist::create_empty_list();

//     let mut values: Vec<i32> = Vec::new();

//     while true {
//         let values = vec_list
//             .into_iter()
//             .map(|x| x.head.as_ref().unwrap().element)
//             .collect::<Vec<i32>>();

//         let min_val = *values.iter().min().unwrap();
//         let min_index = values.iter().position(|x| *x == min_val).unwrap();

//         sortted_list.add(min_val);
//         vec_list[min_index].remove();

//         if vec_list[min_index].head.is_none() {
//             vec_list.remove(min_index);
//         }

//         if vec_list.len() == 0 {
//             break;
//         }
//     }
//     sortted_list
// }

// fn main() {
//     let mut list1 = Linklist::create_empty_list();
//     list1.add(45);
//     list1.add(40);
//     list1.add(35);
//     list1.add(23);
//     list1.add(11);

//     let mut list2 = Linklist::create_empty_list();
//     list2.add(60);
//     list2.add(44);

//     let mut list3 = Linklist::create_empty_list();
//     list3.add(85);
//     list3.add(20);
//     list3.add(15);

//     let mut result = sorting_lists(&mut vec![list1, list2, list3]);
//     result.printing();

//     result.reverse();
//     result.printing();
// }

// // -------------------------------------------
// // 			Efficient Storage and Retrieval of Words
// // -------------------------------------------

// use std::collections::HashMap;
// #[derive(Default, Debug, PartialEq, Eq, Clone)]
// struct Node {
//     children: HashMap<char, Node>,
//     is_word: bool,
// }

// impl Node {
//     fn new() -> Self {
//         Node {
//             is_word: false,
//             children: HashMap::new(),
//         }
//     }
// }
// #[derive(Default, Debug, PartialEq, Eq, Clone)]
// struct WordDictionary {
//     root: Node,
// }

// impl WordDictionary {
//     fn new() -> Self {
//         Self::default()
//     }

//     fn insert(&mut self, word: &String) {
//         let mut current = &mut self.root;
//         for w in word.chars() {
//             current = current.children.entry(w).or_insert(Node::new());
//         }

//         if !current.is_word {
//             current.is_word = true;
//         }
//     }

//     fn search(&self, word: &String) -> bool {
//         let mut current = &self.root;
//         for w in word.chars() {
//             if current.children.get(&w).is_some() {
//                 current = current.children.get(&w).unwrap();
//             } else {
//                 return false;
//             }
//         }
//         current.is_word
//     }
// }

// fn main() {
//     let words = vec![
//         "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
//     ]
//     .into_iter()
//     .map(|x| String::from(x))
//     .collect::<Vec<String>>();

//     let mut d = WordDictionary::new();

//     for i in 0..words.len() {
//         d.insert(&words[i]);
//     }
//     println!(
//         "Searching 'there' in the dictionary results: {}",
//         d.search(&"ther".to_string())
//     );
//     // search for 'that' and finally ther
// }


// -------------------------------------------
// 			Most Recently Used Products
//           	- Description
//           	    - A business is interesting in knowing the products that has been
//                    purchased most recently by a customer.
//           	- Tools
//           	    - Hashmaps + Doubly Link List
// -------------------------------------------

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;
// #[derive(Debug)]
// struct Node {
//     prod_id: i32,
//     prev: Link,
//     next: Link,
// }

// impl Node {
//     fn new(elem: i32) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Node {
//             prod_id: elem,
//             prev: None,
//             next: None,
//         }))
//     }
// }

// type Link = Option<Rc<RefCell<Node>>>;
// #[derive(Default, Debug)]
// struct List {
//     head: Link,
//     tail: Link,
// }

// impl List {
//     fn new() -> List {
//         List {
//             head: None,
//             tail: None,
//         }
//     }

//     // a slightly changed version where the function returns the a link now
//     pub fn push_back(&mut self, elem: i32) -> Link {
//         let new_tail = Node::new(elem);
//         match self.tail.take() {
//             Some(old_tail) => {
//                 old_tail.borrow_mut().next = Some(new_tail.clone());
//                 new_tail.borrow_mut().prev = Some(old_tail);
//                 self.tail = Some(new_tail);
//             }
//             None => {
//                 self.head = Some(new_tail.clone());
//                 self.tail = Some(new_tail);
//             }
//         }
//         self.tail.clone()
//     }

//     // a slightly changed version of the remove_head and now in this case it returns the head which is removed
//     pub fn remove_front(&mut self) -> Option<Link> {
//         self.head
//             .take()
//             .map(|old_head| match old_head.borrow_mut().next.take() {
//                 Some(new_head) => {
//                     new_head.borrow_mut().prev.take();
//                     self.head = Some(new_head);
//                     self.head.clone()
//                 }
//                 None => {
//                     self.tail.take();
//                     None
//                 }
//             })
//     }

//     fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
//         let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
//         let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
//         match (prev, next) {
//             (None, None) => {
//                 // This is the only node in list, so do nothng
//             }
//             (Some(_), None) => {
//                 // This node is already the tail, so do nothing
//             }
//             (None, Some(next)) => {
//                 // This node is at the head, so we need to move it to the tail
//                 node.borrow_mut().next = None;
//                 next.borrow_mut().prev = None;
//                 self.head = Some(next.clone());

//                 let prev_tail = self.tail.as_ref().unwrap();
//                 prev_tail.borrow_mut().next = Some(node.clone());
//                 node.borrow_mut().prev = Some(prev_tail.clone());
//                 self.tail = Some(node.clone());
//             }
//             (Some(prev), Some(next)) => {
//                 node.borrow_mut().next = None;

//                 prev.borrow_mut().next = Some(next.clone());
//                 next.borrow_mut().prev = Some(prev.clone());

//                 let prev_tail = self.tail.as_ref().unwrap();
//                 prev_tail.borrow_mut().next = Some(node.clone());
//                 node.borrow_mut().prev = Some(prev_tail.clone());
//                 self.tail = Some(node.clone());
//             }
//         }
//     }
// }

// #[derive(Debug)]
// struct MRP_Item {
//     map: HashMap<i32, Rc<RefCell<Node>>>,
//     item_list: List,
//     size: i32,
//     capacity: i32,
// }

// impl MRP_Item {
//     fn new(capacity: i32) -> Self {
//         Self {
//             map: HashMap::new(),
//             item_list: List::new(),
//             size: 0,
//             capacity: capacity,
//         }
//     }

//     fn purchased(&mut self, prod_id: i32) {
//         // First check if we have the entry for the movie already in the hashmap. If yes move to the tail
//         if let Some(node) = self.map.get(&prod_id) {
//             self.item_list.move_to_tail(node);
//         } else {
//             // This means that the movie is not in the hashmap
//             if self.size >= self.capacity {
//                 // If the capacity is exhausted then remove head and insert at the tail
//                 let prev_head = self.item_list.remove_front().unwrap();
//                 self.map.remove(&prev_head.unwrap().borrow().prod_id);
//             }
//             // Add node to list head
//             let node = self.item_list.push_back(prod_id).unwrap();
//             // Update hashmap

//             self.map.insert(prod_id, node);
//             // update size
//             self.size += 1;
//         }
//     }
//     fn print(&self) {
//         let mut traversal = self.item_list.head.clone();
//         while !traversal.is_none() {
//             let temp = traversal.clone().unwrap();
//             print!("{} ", temp.borrow().prod_id);
//             traversal = temp.borrow().next.clone();
//         }
//         println!("");
//     }
// }
// fn main() {
//     let mut items_list = MRP_Item::new(3);

//     items_list.purchased(10);
//     items_list.print();

//     items_list.purchased(15);
//     items_list.print();

//     items_list.purchased(20);
//     items_list.print();

//     items_list.purchased(25);
//     items_list.print();

//     items_list.purchased(20);
//     items_list.print();
//     /*
//     items_list.purchased(25);
//     items_list.print();

//     items_list.purchased(20);
//     items_list.print();

//     items_list.purchased(5);
//     items_list.print();
//      */
// }

// // -------------------------------------------
// // 			Displaying Participant of an Online Meeting
// //           	- Description
// //           	    - Retrieving list of paginated view of the list participants in an online meeting

// //           	- Tools
// //           	    - BST + Stack
// // -------------------------------------------

// use std::cell::RefCell;
// use std::rc::Rc;
// #[derive(Debug, Default, PartialEq, Eq, Clone)]
// struct Node {
//     val: String,
//     left: Link,
//     right: Link,
// }

// type Link = Option<Rc<RefCell<Node>>>;
// impl Node {
//     fn new(val: String) -> Self {
//         Node {
//             val,
//             left: None,
//             right: None,
//         }
//     }

//     fn insert(&mut self, val: String) {
//         if val > self.val {
//             match &self.right {
//                 None => self.right = Some(Rc::new(RefCell::new(Self::new(val)))),
//                 Some(node) => node.borrow_mut().insert(val.to_string()),
//             }
//         } else {
//             match &self.left {
//                 None => self.left = Some(Rc::new(RefCell::new(Self::new(val)))),
//                 Some(node) => node.borrow_mut().insert(val.to_string()),
//             }
//         }
//     }
// }

// #[derive(Debug, Default, PartialEq, Eq)]
// struct BinarySearchTree {
//     root: Node,
// }
// impl BinarySearchTree {
//     fn new(val: String) -> Self {
//         BinarySearchTree {
//             root: Node::new(val.to_string()),
//         }
//     }
//     fn insert(&mut self, val: String) {
//         self.root.insert(val.to_string());
//     }
// }

// struct DisplayLobby {
//     stack: Vec<Rc<RefCell<Node>>>,
// }

// impl DisplayLobby {
//     fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
//         let mut stack = Vec::new();
//         Self::push_all_left(root.clone(), &mut stack);
//         DisplayLobby { stack }
//     }

//     fn push_all_left(mut p: Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
//         while let Some(link) = p.clone() {
//             stack.push(p.clone().unwrap());
//             p = link.borrow().left.clone();
//         }
//     }

//     fn next_name(&mut self) -> String {
//         let node = self.stack.pop().unwrap();
//         let name = &node.borrow().val;
//         let mut next_node = node.borrow().right.clone();

//         Self::push_all_left(next_node, &mut self.stack);
//         name.to_string()
//     }

//     fn next_page(&mut self) -> Vec<String> {
//         let mut resultant_names: Vec<String> = Vec::new();
//         for i in 0..10 {
//             if !self.stack.is_empty() {
//                 resultant_names.push(self.next_name());
//             } else {
//                 break;
//             }
//         }
//         resultant_names
//     }
// }

// fn main() {
//     let mut bst = BinarySearchTree::new("Jeanette".to_string());
//     let names: Vec<String> = vec![
//         "Latasha",
//         "Elvira",
//         "Caryl",
//         "Antoinette",
//         "Cassie",
//         "Charity",
//         "Lyn",
//         "Lia",
//         "Anya",
//         "Albert",
//         "Cherlyn",
//         "Lala",
//         "Kandice",
//         "Iliana",
//         "Nouman",
//         "Azam",
//     ]
//     .into_iter()
//     .map(String::from)
//     .collect();
//     for name in names.into_iter() {
//         bst.insert(name.to_string());
//     }

//     let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
//     println!("Participants on first page: {:?}", display.next_page());

//     println!("Participants on second page: {:?}", display.next_page());
// }

//WEB PROGRAMMING BASICS

// use std::net::{TcpListener};

// use tokio::stream;

// fn main() {
//     let listener= TcpListener::bind("127.0.0.1:8000").unwrap();
//     let stream = listener.accept();

//     println!("The stream {:?} The socket{:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);
// }

