use errant::prelude::*;

fn baz(_: i32) -> Result<i32, &'static str> {
    Err("Uh oh!")
}

fn bar(x: i32, y: i32) -> Result<i32, &'static str> {
    Ok(baz(x)? + y)
}

fn foo(x: i32) -> Result<i32, &'static str> {
    Ok(bar(x, 3)? * 2)
}

fn main() -> Result<(), &'static str> {
    foo(3)?;
    Ok(())
}

