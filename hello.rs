
fn greet()
{
    let str = "Hello World!\nI'm a Rustacean!";
    println!("len is {}", str.len());
}

fn ret1() -> String
{    
    let str = "This is a ret test!";
    str.to_string()
}

fn main() {

    let str = "Hello World!\nI'm a Rustacean!";
    println!("{}", str);
    
    greet();

    let ret = ret1();
    println!("{}", ret);
    



}
