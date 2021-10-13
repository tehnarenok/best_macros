
#[test]
fn test_public_struct() {

    mod tehn_test {
        use best_macros::*;

        #[public_struct]
        struct HelloWorld<'a, T> {
            text: &'a str,
            number: T,
        }
    }

    let hello_world = tehn_test::HelloWorld::<i32> {
        text: "Hello",
        number: 12,
    };

    assert_eq!("Hello", hello_world.text);
}