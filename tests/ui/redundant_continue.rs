#![warn(clippy::redundant_continue)]

fn main() {
    let x = 42;

    loop {
        if x == 0 {
            println!("foo");
        }
    }

    loop {
        //~ ERROR trailing continue
        continue;
    }

    loop {
        if x == 0 {
            println!("foo");
        }
        //~ ERROR trailing continue
        continue;
    }
}
