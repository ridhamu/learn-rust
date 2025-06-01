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





