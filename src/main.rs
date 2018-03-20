use std::mem;

fn main()
{
	// u -> unsigned integer 0...255; i -> signed integer -127...128
	let a:u8 = 123; //8 bits
	println!("a = {}", a);

	//a = 456;

	// mutable = changable
	let mut b:i8 = 0;
	println!("b = {}", b);
	b = 42;
	println!("b = {}", b);

	let mut c = 123456789; //32-bit signed i32
	println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
	c = -1;
	println!("c = {} after modification", c);

	//i8 u8 ... i64 u64
	let z:isize = 123; // isize usize
	let size_of_z = mem::size_of_val(&z);
	println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

	
}