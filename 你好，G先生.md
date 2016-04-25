##源码
```
fn main(){
	let x='G';
	println!("hello {}",x);
}
```
##编译
```
rustc g.rs
```
##执行
```
g
```

##输入/输出
```
use std::io;
fn main(){
	println!("input your name,please!");
	let mut name=String::new();
	io::stdin().read_line(&mut name).expect("failed!");
	println!("hello {}",name);
}
```

```
rustc uname.rs
```

```
uname
```
