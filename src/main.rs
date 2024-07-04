use std::io;
// fn main(){
//     let _x: u32 = 5;
//     let _x = 6;
// }

fn main(){
    let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let size = x.len();
    println!("Size of x: {}", size);
    println!("value of x is: {}", &x);
}