use domain::something::{base_something::BaseSomething, open_something::convert_base_to_open};

fn main() {
    println!("workflow: accepted");
    accepted_workflow();

    println!("workflow: rejected");
}

fn accepted_workflow() {
    // 1. Baseオブジェクトを作る
    let base = BaseSomething::new("test_name");
    println!(" - base.name: {}!", base.name);

    // 2. BaseオブジェクトをConvert関数でOpenオブジェクトに変える
    let open = match convert_base_to_open(base, "test_open_reason") {
        Ok(open) => open,
        Err(e) => panic!("{}", e),
    };
    println!(" - open.open_reason: {}!", open.open_reason);

    // 3. OpenオブジェクトがAcceptされ、Acceptedオブジェクトに変わる
    // TODO

    // 4. Acceptedオブジェクトが役目を果たし、Closeオブジェクトになる
    // TODO
}
