use domain::something::{
    accepted_something::convert_open_to_accepted, base_something::BaseSomething,
    close_something::convert_accepted_to_close, open_something::convert_base_to_open,
};

fn main() {
    println!("workflow: accepted");
    accepted_workflow();

    // println!("workflow: rejected");
    println!("End!");
}

fn accepted_workflow() {
    // 1. Baseオブジェクトを作る
    let base = BaseSomething::new("test_name");
    println!(" - base.name: {}", base.name);

    // 2. BaseオブジェクトをConvert関数でOpenオブジェクトに変える
    let open = match convert_base_to_open(&base, "test_open_reason") {
        Ok(open) => open,
        Err(e) => panic!("{e}"),
    };
    println!(" - open.open_reason: {}", open.open_reason);

    // 3. OpenオブジェクトがAcceptされ、Acceptedオブジェクトに変わる
    let accepted = match convert_open_to_accepted(&base, 15) {
        Ok(accepted) => accepted,
        Err(e) => panic!("{e}"),
    };
    println!(
        " - accepted.accepted_reason_id: {}",
        accepted.accepted_reason_id
    );

    // 4. Acceptedオブジェクトが役目を果たし、Closeオブジェクトになる
    let close = match convert_accepted_to_close(base) {
        Ok(close) => close,
        Err(e) => panic!("{e}"),
    };
    println!(" - close.close_at: {}", close.close_at);
}
