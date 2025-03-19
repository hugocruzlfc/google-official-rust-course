
fn main() {
    assign_value();
    println!("Hello, world!");

     println!("resultado: {}", interproduct(120, 100, 248));

     takes_u32(10);
     takes_i8(10);

    let n = 10;
    let result = fibonacci(n);
    println!("fib({n}) = {}", result);

    using_if();
    if_with_expression();

    boucle_while();
    boucle_for();
    boucle_loop();
    continue_break();
    label_argument();

    ambit_shadowing_variables();

     println!("gcd: {}", gcd(143, 52));

    let n = 4;
    println!("{n}! = {}", factorial(n));

    //fizzbuzz(15);  => genera un error de pánico debido al todo!()

    println!("gccollatz_length: {}", collatz_length(3));
    
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

/// bucles y condicionales

fn using_if() {
    let x = 10;

    if x == 0 {
        println!("cero!");
    } else if x < 100 {
        println!("muy grande");
    } else {
        println!("enorme");
    }
}



fn if_with_expression() {
    let x = 10;
    let size = if x < 20 { "pequeño" } else { "grande" };

    println!("tamaño del número: {}", size);
}



fn boucle_while() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("x final: {x}");
}


fn boucle_for() {
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }
}


fn boucle_loop() {
    let mut i = 0;

    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}

fn continue_break() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}


fn label_argument() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;

    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elementos travesados: {elements_searched}");
}


fn ambit_shadowing_variables() {
    let a = 10;
    println!("antes: {a}");
    {
        let a = "hola";
        println!("ámbito interno: {a}");

        let a = true;
        println!("sombreado en el ámbito interno: {a}");
    }

    println!("después: {a}");
}

///

// functions

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}


// macros
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

///////////



//Ejercicio: secuencia de Collatz

fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
  
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
