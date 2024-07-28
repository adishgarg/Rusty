struct MyClass {
    field1: i32,
    field2: String,
}

impl MyClass {
    fn new(field1: i32, field2: String) -> Self {
        Self {
            field1,
            field2,
        }
    }

    fn some_method(&self) {
    }
}

fn main() {
    let my_object = MyClass::new(42, String::from("Hello"));

    my_object.some_method();
}
