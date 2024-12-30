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

