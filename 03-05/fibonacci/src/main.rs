fn main() {
    let nth_fib = fibonacci::get(1).expect("Error when number < 1");
    println!("The first fibonacci number is: {nth_fib}");

    let nth_fib = fibonacci::get(2).expect("Error when number < 1");
    println!("The second fibonacci number is: {nth_fib}");

    let nth_fib = fibonacci::get(34).expect("Error when number < 1");
    println!("The 34th fibonacci number is: {nth_fib}");
}
