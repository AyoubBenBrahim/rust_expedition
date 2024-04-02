
fn greet()
{
    // let len = str::len("Hello World!\nI'm a Rustacean!");
    println!("len is {}", str::len("Hello World!\nI'm a Rustacean!"));
}

fn ret1() -> String
{    
    let str = "This is a ret test!";
    str.to_string()
}

fn match_test(nbr: i32)
{
    match nbr {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 | 7 | 8 => println!("six, seven or eight"),
        _ => println!("something else"),
    }
}

fn main() {

    let str = "Hello World!\nI'm a Rustacean!";
    println!("{}", str);
    
    greet();

    let ret = ret1();
    println!("{}", ret);
    
    match_test(77);



}
