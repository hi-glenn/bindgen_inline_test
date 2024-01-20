extern crate cc;

fn main() 
{
    cc::Build::new().file("input.h").compile("input_c");
}
