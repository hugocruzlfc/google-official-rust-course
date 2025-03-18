
fn main() {
    assign_value();
    println!("Hello, world!");

     println!("resultado: {}", interproduct(120, 100, 248));

     takes_u32(10);
     takes_i8(10);

    let n = 10;
    let result = fibonacci(n);
    println!("fib({n}) = {}", result);
    
}


fn assign_value() {
    let x: i32 = 10;
    println!("x: {x}");
    
    let mut y: i32 = 20;
    println!("y: {y}");

    y = 25;
    println!("y muted: {y}");
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}


// Inferencia de tipos

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

// fn make_error() {
//     let x = 3.14;
//     let y = 20;
//    // assert_eq!(x, y);
//     // ERROR: no hay implementación para `{float} == {integer}`
// }



fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        // El caso base.
       return n;
    } else {
        // El caso recursivo.
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}



