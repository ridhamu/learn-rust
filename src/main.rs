fn main() {
    println!("Hello, world!");


    // here list all the non-unit test function to avoid annoying warning 
    function_a(); 
    function_b(); 


    // all global scope var to avoid warning
    println!("{}", ANOTHER_ANOTHER_NUMBER); 
    println!("{}", GLOBAL_SCOPE); 
}

#[test]
fn hello_test() {
    println!("hello, test!");
}

#[test]
fn char() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("karakter 1: {char1}, karakter 2: {char2}");
}

#[test]
fn tuple() {
    //     tipe data tuple, fixed size dan tipe data ditetapkan diawal, bisa bervariasi :v
    let data: (i32, f64, bool) = (1, 3.14, true);
    println!("{:?}", data);

    // accessing data inside tuple from index
    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a, b, c);

    // desctructure tuple
    let (a, b, c) = data;
    println!("with destructure: ");
    println!("{} {} {}", a, b, c);

    // mutable tuple
    let mut data2: (i32, f64, bool) = (1, 1.2, true);

    let (_a, _b, _c) = data2;
    data2.0 = 32;
    data2.1 = 32.32;
    data2.2 = false;

    println!("data2 = {:?}", data2);
}

#[test]
fn unit() {
    println!("hellow, world");
}
#[test]
fn for_unit() {
    let test = unit();
    println!("{:?}", test);
}

#[test]
fn array() {
    // ketika mendeklarasikan array harus menyertakan [tipe data; ukuran]
    // #1
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("deklarasi array sederhana= {:?}", array);
    // mengakses isi dari array menggunakan [index]
    println!("value pada index ke-1 adalah = {}", array[1]);
    // mengakses panjang dari array menggunakan .len()
    println!("ukurang dari array adalah = {}", array.len());

    // array mutable  ?
    let mut mutable_array : [i32; 5] = [1, 2, 3, 4, 5];
    println!("sebelum diubah = {:?}", mutable_array);
    //mengubah isi dari array satu persatu
    mutable_array[0] = mutable_array[0] * 10;
    mutable_array[1] = mutable_array[1] * 10;
    mutable_array[2] = mutable_array[2] * 10;
    mutable_array[3] = mutable_array[3] * 10;
    mutable_array[4] = mutable_array[4] * 10;
    //setelah mengubah
    println!("setelah diubah = {:?}", mutable_array);



    // array dua dimensi
    let array_dua_dimensi: [[i32; 2]; 3] = [
        [1,2],
        [1,2],
        [1,2]
    ];
    println!("array 2 dimensi= {:?}", array_dua_dimensi);
}

// global constant
const ANOTHER_ANOTHER_NUMBER: i32 = 1;
#[test]
//constant
fn constant() {
    // berbeda denga let, konstan haru di harus ditulis dengan lengkap
    // konstan terdfinisi ketika menulis kode, buka ketika proses kompilasi // biasa nama variabel ditulis dalam huruf kapital dan kalau dua huruf maka disambung menggunakan "_" //#1(local usage)
    const NUMBER: i32 = 69;
    const ANOTHER_NUMBER: i32 = 42;

    println!("NUMBER = {} \nANOTHER_NUMBER = {}\nANOTHER_ANOTHER_NUMBER = {}", NUMBER, ANOTHER_NUMBER, ANOTHER_ANOTHER_NUMBER);
}

const GLOBAL_SCOPE: i32 = 1;
#[test]
fn variable_scope() {
     println!("{}", GLOBAL_SCOPE);

    let local_scope: i32 = 2;
    println!("{}", local_scope);
    {
        println!("{}", local_scope);
        let local_again: i32 = 3;
        println!("{}", local_again); 
    }

   // println!("{}", more_local_scope);  // couldn't find `more_local_scope` in this scope
}



#[test]
fn stack_heap() {
    function_a(); 
    function_b(); 
}

fn function_a(){
    let a = 10;
    let name = String::from("Muhammad"); 

    println!("{}, {}", a, name); 
}

fn function_b(){
    let a = 10;
    let name = String::from("Ridha"); 

    println!("{}, {}", a, name); 
}




#[test]
#[allow(unused_assignments)]
fn string(){
    // this is string slice, which is stored in stack memory
    // by default is set to immutable
    let name: &str = "  Muhammad Ridha  "; 
    let trim: &str = name.trim();

    println!("name: &str = |{}|\ntrim: &str = |{}|", name, trim);
    
    // when we try to make it immutable, the data is still there the only thing change is
    // the binding to which data in stack memory/binary form
    let mut mutable_str: &str = "Muhammad";  
    mutable_str = "ridha"; 
    
    println!("mutable_str: &str = {}", mutable_str); 
}


#[test]
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn string_type(){
    let mut nama: String = String::from("Muhammad"); 
    println!("nama: {}", nama); 
    
    nama.push_str(" Ridha"); 
    println!("nama: {}", nama); 

    let namabaru: String = nama.replace("Ridha", "Budi"); 
    println!("nama: {}", namabaru); 
}


#[test]
fn ownership(){
    // variabel a tidak bisa diakses disini karena belum di deklarasikan
    let a = 10; // variabel a bisa diakses mulai disini 
    {
        let b = 20; // variabel b mulai diakses dari ini 
        println!("b = {}", b); 
    } // variabel b selesai, dihapus & tidak bisa diakses mulai disini
    println!("a = {}", a); 
} // variabel a selesai, dihapus, tidak bisa diakses mulai dari sini


#[test]
#[allow(unused_mut)]
fn data_copy() {
    let mut a = 10; 
    let mut b = a;// here rust make copy of value inside a then bind it to b 

    println!("a = {}\nb = {}", a, b); 

    //changing b values here won't change the value that binds to a
    //proving that each of a and b not pointing to the same value proving that one owner at a time
    b = 20; 
    println!("a = {}\nb = {}", a, b); 
}


#[test]
fn ownership_movement(){
    let firstowner: String = String::from("!!!!!some heap alocated value here!!!!!"); 
    let secondowner: String = firstowner;  // since the owner of the value is changed, firstowner
    // is no longer valid here

   println!("secondowner: {}", secondowner);  
// this line no longer valid   println!("firstowner: {}", firstowner);  
}


#[test]
fn clone(){
    let a: String = String::from("SOME VALUE IN HELP");
    // let b: String = a; -> this is move the ownership
    let b: String = a.clone();
    
    println!("a = {}\nb = {}", a, b); 
}


#[test]
#[allow(unused_variables)]
fn if_else(){
   let value = 1; 

  // this is normal use case of fi statement
  //  if value >= 8 {
  //      println!("good!"); 
  //  } else if value >= 6{
  //      println!("not bad!"); 
  //  } else if value >= 3{
  //      println!("bad!"); 
  //  }else {
  //      println!("very bad !"); 
  //  }

// how to assing conditional value to a variable    
//    let result: &str; 

//    if value >= 8 {
//        result = "good!" 
//    } else if value >= 6{
//        result = "not bad!" 
//    } else if value >= 3{
//        result = "bad!" 
//    }else {
//        result = "very bad !" 
//    }

//    println!("result is '{}' ", result); 
      
    let result: &str = if value >= 8 {
        "good!" 
    } else if value >= 6{
        "not bad!" 
    } else if value >= 3{
        "bad!" 
    }else {
        "very bad !" 
    }; 
      
    println!("result = '{}'", result); 
}



#[test]
fn loop_expression(){
   let mut counter = 0;

    loop {
        counter += 1; 
        if counter > 10 {
            break; 
        }else if counter % 2 == 0 {
            continue; 
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return_value(){
    let mut counter = 0; 
    let result = loop {
        counter += 1; 
        if counter >= 10 {
            break counter * 2; 
        }
    }; 
    println!("result: {}", result); 
}


#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i );
            i += 1; 
            if i > 10 {
                break; 
            }
        }

        number += 1; 
    }

}

#[test]
fn while_loop() {
    let mut counter = 1; 
    while counter <= 10 {
        if counter % 2 != 0 {
            println!("counter = {}", counter);
        }
        counter += 1; 
    }
}

#[test]
fn array_iteration_using_while(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E" ]; 
    let mut index = 0; 
    while index < array.len() {
        println!("array[{index}] = {}", array[index]);
        index += 1; 
    } 
}

#[test]
fn array_iteration_using_for_loop(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E" ]; 

    for value in array {
        println!("{}", value);
    }
}



#[test]
fn range(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E" ]; 
    let range = 0..array.len(); 
    println!("Range Start: {}", range.start); 
    println!("Range End: {}", range.end); 

    for i in range{
        println!("value: {}", i); 
    }
}

#[test]
fn range_inclisive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E" ];    
    let range = 0..=4; // this same as 0..5  which is exclusive

    println!("range start: {}", range.start()); 
    println!("range end: {}", range.end()); 


    for i in range {
        println!("value: {}", array[i]); 
    }
}

#[allow(dead_code)]
fn say_hello(){
    println!("hello!!!");
}

#[test]
fn test_say_hello(){
    say_hello(); 
    say_hello(); 
    say_hello(); 
}


#[allow(dead_code)]
fn say_goodbye(first_name: &str, last_name: &str){
    println!("good bye {} {} !", first_name, last_name); 
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Muhammad", "Ridha"); 
    say_goodbye("Muhammad", "Ridha"); 
    say_goodbye("Muhammad", "Ridha"); 
}


#[allow(dead_code)]
fn factorial_loop(n: i32)-> i32 {
    if n < 0 {
        return 1; 
    }
    let mut value = 1; 
    for i in 1..=n {
        value *= i;  
    }
    value  
}


#[test]
fn test_factoriar_loop() {
    let n = factorial_loop(5); 
    println!("n = {}", n); 


    let m = factorial_loop(-10); 
    println!("n = {}", m); 
    
}


#[allow(dead_code)]
fn print_using_recursive(message: String, times: u32) {
    if times < 1 {
        return;
    } else {
        println!("{}", message);
    }

    print_using_recursive(message, times - 1); 
}



#[test]
fn test_print_using_recursive() {
    print_using_recursive(String::from("hello"), 10 ); 
}

#[allow(dead_code)]
fn factorial_recursive(n: i32)-> i32 {
    if n < 1 {
        return 1; 
    }
    n * factorial_recursive(n - 1) 
} 


#[test]
fn test_factorial_recursive() {
    let recursive_result: i32 = factorial_recursive(5); 
    println!("recursive result = {}", recursive_result); 


    let negative: i32 = factorial_recursive(0); 
    println!("negative recursive = {}", negative); 
}


#[allow(dead_code)]
fn print_number(number: i32) {
    println!("print_number => {}", number); 
}

#[allow(dead_code)]
fn hi(name: String) {
    println!("Hi {} !", name); 
}

#[test]
fn test_function() {

    let initial_number = 10; 
    print_number(initial_number); 
    println!("initial number: {}", initial_number); 


    let initial_name = String::from("Muhammad Ridha"); 
    hi(initial_name); 
    // println!("initial_name = {}", initial_name); 
} 

#[allow(dead_code)]
fn full_name(first_name: String, last_name: String)-> (String, String, String) {
    let fullname = format!("{} {}", first_name, last_name); 
    (first_name, last_name, fullname)
}

#[allow(dead_code)]
fn full_name_reference(first_name: &String, last_name: &String)-> String {
    format!("{} {}", first_name, last_name) 
}

#[test]
fn test_fullname() {
    let initial_first_name: String = String::from("Muhammad"); 
    let initial_last_name: String = String::from("Ridha"); 

    // using tuple to return back what passdown to the function parameter
    let (initial_first_name, initial_last_name, fullname) = full_name(initial_first_name, initial_last_name); 
    println!("{}", initial_first_name);
    println!("{}", initial_last_name);
    println!("{}", fullname);


    let initial_first_name_for_reference: String = String::from("Muhammad"); 
    let initial_last_name_for_reference: String = String::from("Ridha"); 
    // alternatif solutions would be using references to
    let fullname_by_references = full_name_reference(&initial_first_name_for_reference, &initial_last_name_for_reference); 
    println!("initial first name for reference: {}", initial_first_name_for_reference);
    println!("initial last name for reference: {}", initial_last_name_for_reference);
    println!("fullname for reference: {}", fullname_by_references);
}

#[allow(dead_code)]
fn change_value(value: &mut String) {
    value.push_str("Test")
}

#[test]
fn test_change_value() {
    let mut name: String = String::from("Ridha");

    //creating mutable reference
    let value_borrow = &mut name; 
    // here cannot assign another mutable reference until value_borrow not use anymore/out of scope

    change_value(value_borrow); 
    change_value(value_borrow); 
    change_value(value_borrow); 

    println!("name: {}", name); 
}

#[allow(dead_code)]
fn get_full_name(first_name: &String, last_name: &String)-> String {
    let full_name = format!("{} {}", first_name, last_name); 
    full_name
}

#[test]
fn test_get_full_name() {
    let firstname: String = String::from("Muhammad");
    let lastname: String = String::from("Ridha");

    let fullname: String = get_full_name(&firstname, &lastname); 

    println!("firstname: {}", firstname);
    println!("lastname: {}", lastname);
    println!("fullname: {}", fullname);

}

#[test]
#[allow(unused_variables)]
fn slice_referece() {
    let arr: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    let slice1: &[i32] = &arr[..]; // slice the whole array
    println!("slice1: {:?}", slice1); // {:?} means that this implement Debug Trait

    let slice2: &[i32] = &arr[0..5]; // exclusive from index 0, 1, 2, 3, 4
    println!("slice2: {:?}", slice2);

    let slice3: &[i32] = &arr[5..];
    let slice4 = slice3; 
    println!("slice3: {:?}", slice3);
    
    println!("arr: {:?}", arr); 
}


#[test]
fn string_slice() {
    let name: String = String::from("Muhammad Ridha"); 

    // make a reference for the first name from name
    let first_name: &str = &name[..8];
    println!("first_name: {}", first_name);


    // make a reference for the last_name from name
    let last_name: &str = &name[9..];
    println!("last_name: {}", last_name);
}

#[allow(dead_code)]
struct Person {
    first_name: String, 
    last_name: String,
    address: String,
    age: u8
}
#[allow(dead_code)]
impl Person {
    fn say_hello(&self, name: &str) {
       println!("Hello {}, my name is {}!", name, self.first_name); 
    }
}

#[allow(dead_code)]
fn print_struct_person(person: &Person) {
    println!("person.first_name: {}", person.first_name);
    println!("person.last_name: {}", person.last_name);
    println!("person.address: {}", person.address);
    println!("person.age: {}", person.age);
}


#[test]
fn test_struct_person() {
    let first_name: String = String::from("Muhammad");
    let last_name: String = String::from("Ridha");
    let person1: Person = Person {
        //first_name: String::from("Muhammad"), 
        //last_name: String::from("Ridha"),
        first_name,  // init shorthand, first_name no longer available here, since it
        // already move to person1.first_name
        last_name, // init shorthand
        address: String::from("Gajayana gg.6"),
        age: 22
    };
    print_struct_person(&person1);


    println!("============= below is person1_copy =============");
    // here we gonna make a copy of person 1, but the problem is all field inside person1 that
    // store inside heap is gonna move the ownership to the new owner
    // let person1_copy: Person = Person {..person1}; // here person1.first_name is no longer
    // available
    // the solution is to use clone function
    let person1_copy: Person = Person {
        // here we handle all value that stored inside heap, and let the rest handle by itself
        first_name: person1.first_name.clone(),
        last_name: person1.last_name.clone(),
        address: person1.address.clone(),
        ..person1
    };


    print_struct_person(&person1_copy);
    
    println!("============= test is person1 field still available =============");
    println!("person1.first_name: {}", person1.first_name);
}

// here we implement tuple struct
#[allow(dead_code)]
struct GeoPoint(f64, f64); 

#[allow(dead_code)]
impl GeoPoint {
    //creating associated function
    fn new(coordinates: f64) -> GeoPoint {
        GeoPoint (coordinates, coordinates)
    }
}

#[test]
fn test_struct_tuple() {
    // kita bisa mengakses isi dari tuple struct sama seperti mengakses tuple, yaitu menggunakan
    // index
    let random_geopoint: GeoPoint = GeoPoint(-1.6969, 69.1111);
    
    println!("latitude: {}", random_geopoint.0);
    println!("longitude: {}", random_geopoint.1);
}

#[test]
fn test_associated_function() {
    let coordinates: GeoPoint = GeoPoint::new(69.69); 

    println!("the longitude is: {}", coordinates.0);
    println!("the latitude is: {}", coordinates.1);
}

// kita bisa menginisiasi struct tanpa field/empty struct
#[allow(dead_code)]
struct Nothing;

#[test]
fn test_struct_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

#[test]
fn test_person_method() {
    let first_name: String = String::from("Muhammad");
    let last_name: String = String::from("Ridha");
    let person1: Person = Person {
        //first_name: String::from("Muhammad"), 
        //last_name: String::from("Ridha"),
        first_name,  // init shorthand, first_name no longer available here, since it
        // already move to person1.first_name
        last_name, // init shorthand
        address: String::from("Gajayana gg.6"),
        age: 22
    };
    person1.say_hello("alejandro");
}

// enum
#[allow(dead_code)]
enum Level {
    Regular, 
    Premium,
    Platinum
}

#[test]
fn test_level() {
    let level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}


//example payment kind with enum

#[allow(dead_code)]
enum Payment {
    //card number
    CreditCard(String),
    //bank name, account number
    BankTransfer(String, String),
    //ewallet name, ewallet number
    EWallet(String, String)
}
#[allow(dead_code)]
impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {}, total amount: {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with Bank Transfer {} {}, total amount: {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with EWallet {} {}, total amount: {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("12345678"));
    _payment1.pay(1000000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("12345678"));
    _payment2.pay(2000000);

    let _payment3: Payment = Payment::EWallet(String::from("GoPay"), String::from("12345678"));
    _payment3.pay(3000000);
}

#[test]
fn test_pattern_value() {
    let name: &str = "Eko";

    match name {
        "Eko" => {
            println!("Hello Eko!");
        }
        "Budi" => {
            println!("Hello Budi!");
        }
        other => {
            println!("Hello {}!", other);
        }
    }
    match name {
        "Eko" | "Budi" | "Joko" => {
            println!("Hello Bos!");
        }
        other => {
            println!("Hello {}!", other);
        }
    }
}

#[test]
fn test_pattern_range() {
    let value: i32 = -80;


    match value {
        75..=100 => {
            println!("Great!");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}




#[test]
fn test_struct_pattern() {
    let geo: GeoPoint = GeoPoint(0.0, 0.0);

    match geo {
        GeoPoint(0.0, lat) => {
            println!("the longitude is 0.0, but the lat is {}", lat);
        }
        GeoPoint(long, 0.0) => {
            println!("the latitude is 0.0, but the long is {}", long);
        }
        GeoPoint(long, lat) => {
            println!("{} {}", long, lat);
        }
    }



    let person: Person = Person {
        first_name: String::from("Ridha"),
        last_name: String::from("Muhammad"),
        address: String::from("Gajayana Gg. 6 no. 577C"),
        age: 22,
    };

    match person {
        Person {first_name, last_name, ..} => {
            println!("hi {} {} !", first_name, last_name);
        }
    }
}


#[test]
fn test_match_return_value() {
    let number = 3; 
    

    let number_in_string: &str = match number {
        0 => "nol",
        1 => "satu",
        2 => "dua", 
        3 => "tiga", 
        4 => "empat",
        5 => "lima",
        _ => "invalid"
    };


    println!("{}", number_in_string);
}




