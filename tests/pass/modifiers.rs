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
pub fn test_fn4() {
    // some code
    unimplemented!()
}

#[modifiers("func_a")]
pub async fn test_fn5() {
    // some code
    unimplemented!()
}

fn func_a() -> Result<(), String> {
    Ok(())
}

fn func_b(param: u64) -> Result<(), String> {
    Ok(())
}

fn func_c(param0: u64, param1: u64) -> Result<(), String> {
    Ok(())
}

enum TestEnum {
    A,
    B,
}

fn func_d(param0: u64, param1: TestEnum) -> Result<(), String> {
    Ok(())
}

fn main() {}
