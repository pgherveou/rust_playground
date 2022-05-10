use rust_playground::my_feature;

fn main() {
    let foo = Foo {
        type_name: my_feature!(String::from("foo"), 44),
    };

    println!("{}", foo.type_name);
}

#[derive(Debug)]
struct Foo {
    type_name: my_feature!(String, u8),
}
