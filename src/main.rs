fn main() {

    let first:i8 = 64;
    let second:u8 = 255;
    let third:f32 = 5.5;
    let undefined = 300;
    let boolval:bool = false;
    let letter:char = 'Z';

    println!("8 bit integer:\t{}\t|\tSize:\t{} bytes", first, std::mem::size_of_val(&first));
    println!("8 bit unsigned:\t{}\t|\tSize:\t{} bytes", second, std::mem::size_of_val(&second));
    println!("32 bit float:\t{}\t|\tSize:\t{} bytes", third, std::mem::size_of_val(&third));
    println!("undefined:\t{}\t|\tSize:\t{} bytes", undefined, std::mem::size_of_val(&undefined));
    println!("Boolean:\t{}\t|\tSize:\t{} bytes", boolval, std::mem::size_of_val(&boolval));
    print!("Character:\t{}\t|\tSize:\t{} bytes\n", letter, std::mem::size_of_val(&letter))
}