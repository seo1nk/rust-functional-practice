use domain::something::{
    base_something::BaseSomething,
    open_something::{convert_base_to_open, OpenSomething},
};

fn main() {
    // Baseオブジェクトを作る
    let base_something = BaseSomething::new("test_name");
    println!("base_something.name: {}!", base_something.name);

    // BaseオブジェクトをConvert関数でOpenオブジェクトに変える
    let open_something_convert: OpenSomething =
        convert_base_to_open(&base_something, "test_reason");
    println!(
        "open_something.open_reason: {}!",
        open_something_convert.open_reason
    );

    // BaseオブジェクトをTryIntoでOpenオブジェクトに変える
    let open_something_try_into: OpenSomething = base_something.try_into().unwrap();
    println!(
        "open_something.open_reason: {}!",
        open_something_try_into.open_reason
    );
}
