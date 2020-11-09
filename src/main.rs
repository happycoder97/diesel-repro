#[macro_use]
extern crate diesel;

struct Foo;

table! {
    use diesel::sql_types::*;
    use super::Foo;

    test_table {
        id -> Integer,
        foobar -> Foo,
    }
}

fn main() {
    println!("Hello, world!");
}
