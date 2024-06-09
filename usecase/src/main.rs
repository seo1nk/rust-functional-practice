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
    let open = match convert_base_to_open(base, "test_open_reason") {
        Ok(open) => {
            println!(" - open.open_reason: {}", open.open_reason);
            open
        }
        Err(e) => panic!("{e}"),
    };

    // 3. OpenオブジェクトがAcceptされ、Acceptedオブジェクトに変わる
    let accepted = match convert_open_to_accepted(open, 15) {
        Ok(accepted) => {
            println!(
                " - accepted.accepted_reason_id: {}",
                accepted.accepted_reason_id
            );
            accepted
        }
        Err(e) => panic!("{e}"),
    };

    // 4. Acceptedオブジェクトが役目を果たし、Closeオブジェクトになる
    let _close = match convert_accepted_to_close(accepted) {
        Ok(close) => {
            println!(" - close.close_at: {}", close.close_at);
            close
        }
        Err(e) => panic!("{e}"),
    };
}
