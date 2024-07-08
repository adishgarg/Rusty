use std::io;
// fn main(){
//     let _x: u32 = 5;
//     let _x = 6;
// }

// fn main(){
//     let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let size = x.len();
//     println!("Size of x: {}", size);
//     println!("value of x is: {}", &x);
// }


fn main() {
    let mut x = [0; 10]; 
    for i in 0..10 {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string)
            .expect("Failed to read line");
        x[i] = input_string.trim().parse()
            .expect("Please type a number");
    }
    swapadjacentelements(&x::i32);
    for i in 0..10 {
        println!("{}", x[i]);
    }
}

fn swapadjacentelements(x: &mut [i32]) {
    for i in (0..x.len()).step_by(2) {
        if i + 1 < x.len() {
            x.swap(i, i + 1);
        }
    }
}

// fn maxinarray(arr: &[i32; 10]) -> i32 {
//     let mut max = arr[0];
//     for &val in arr.iter() {
//         if val > max {
//             max = val;
//         }
//     }
//     max
// }

