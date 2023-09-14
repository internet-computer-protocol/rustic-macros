#![allow(dead_code)]
use rustic_macros::modifiers;

#[modifiers("func_a")]
fn test_fn0() {
    // some code
    unimplemented!()
}

#[modifiers("func_a", "func_b@33")]
fn test_fn1() {
    // some code
    unimplemented!()
}

#[modifiers("func_a", "func_b@33", "func_c@33,44")]
fn test_fn2() {
    // some code
    unimplemented!()
}

#[modifiers("func_a", "func_b@33", "func_d@33,crate::TestEnum::A")]
fn test_fn3() {
    // some code
    unimplemented!()
}

#[modifiers("func_a")]
pub async fn test_fn4<'a, T, U>(
    _arg1: &'a str,
    arg2: T,
    _arg3: U,
) -> Result<T, Box<dyn std::error::Error + 'a>>
where
    T: std::fmt::Debug + 'a,
    U: std::fmt::Display,
{
    // Function body
    Ok(arg2)
}

fn func_a() -> Result<(), String> {
    Ok(())
}

fn func_b(_param: u64) -> Result<(), String> {
    Ok(())
}

fn func_c(_param0: u64, _param1: u64) -> Result<(), String> {
    Ok(())
}

enum TestEnum {
    A,
    B,
}

fn func_d(_param0: u64, _param1: TestEnum) -> Result<(), String> {
    Ok(())
}

fn main() {}
