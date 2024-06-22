use core::time;

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main(){
    let foo = Foo { x: (2, 2), y: 3 };
    // match foo {
    //     Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
    //     Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
    //     Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    // }

    // while let 2 = foo.x.0 {
    //     println!("x.1={},y={}",foo.x.1,foo.y);
    //     std::thread::sleep(time::Duration::from_millis(500));
    // }

// if let 表达式 else; 向 let 表达式 else；转换。
let a = 10;
if let 10 = a{
	println!("a={a}");
}else{
println!("a!=10");
};
//变成：
let a = 10 else{
    println!("a!=10");
    return ; //用let ... else 需要在else后面返回。
};
println!("a={a}");

}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

// fn main() {
//     let n = 100;
//     match divide_in_two(n) {
//         Result::Ok(half) => println!("{n} divided in two is {half}"),
//         Result::Err(msg) => println!("sorry, an error happened: {msg}"),
//     }
// }
