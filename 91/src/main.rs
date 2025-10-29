use stack::*;

fn main() {
    let mut s = init();
    dbg!(&s);
    s = push(String::from("0"), s);
    dbg!(&s);
    s = push(String::from("1"), s);
    dbg!(&s);
    let (popped, mut s) = pop(s);
    dbg!(popped);
    dbg!(&s);
    s = push(String::from("n"), s);
    dbg!(&s);
}
