fn main() {
    println!("Hello, world!");
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
        let more_local_scope: i32 = 3;
    }
   // println!("{}", more_local_scope);  // couldn't find `more_local_scope` in this scope
}