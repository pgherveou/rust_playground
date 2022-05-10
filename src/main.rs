use rust_playground::my_feature;

struct Foo {
    bar: my_feature!(String, u8),
}

fn main() {
    let foo = Foo {
        bar: my_feature!(String::from("foo"), 44),
    };

    println!("{}", foo.bar);
}
