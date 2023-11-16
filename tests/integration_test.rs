#[test]
fn test_public_struct() {
    mod tehn_test {
        use best_macros::*;

        #[public_struct]
        struct HelloWorld<'a, T> {
            text: &'a str,
            number: T,
        }

        #[derive(Getters)]
        struct HelloWord2 {
            text: String,
        }
    }

    let hello_world = tehn_test::HelloWorld::<i32> {
        text: "Hello",
        number: 12,
    };

    // let hello_world2 = tehn_test::HelloWorld2 {
    //     text: "Hello",
    //     number: 12,
    // };

    assert_eq!("Hello", hello_world.text);
}
