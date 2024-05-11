use anyhow::Result;
use z_macro::my_vec;

fn main() -> Result<()>{
    let v: Vec<i32> = my_vec![];
    println!("{:?}", v);
    Ok(())
}