fn bubblesort(){
    
}

fn main(){
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let n = arr.len();
    for i in 0..n-1{
        for j in 0..n-i-1{
            if arr[j] > arr[j+1]{
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    for i in 0..n{
        print!("{} ", arr[i]);
    }
    println!();
}



