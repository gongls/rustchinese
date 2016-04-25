use std::io;
fn main(){
	println!("input your name,please!");
	let mut name=String::new();
	io::stdin().read_line(&mut name).expect("failed!");
	println!("hello {}",name);
}
