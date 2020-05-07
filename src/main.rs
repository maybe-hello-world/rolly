use std::collections::HashSet;

use rolly::Policy;
use rolly::PolicyBuilder;

fn ok_fn(x: i32) -> Result<i32, i32> {
    println!("I always return Ok");
    Ok(x)
}

fn faulty_fn(x: i32) -> Result<i32, i32> {
    println!("I always return Err");
    Err(x)
}

fn random_fn() -> i32 {
    println!("Random here!");
    42 // truly random chosen number
}


fn main() {

    // retry any time ok is received
    let retry_policy = PolicyBuilder::new()
        .handle_ok()
        .retry(3);

    retry_policy.execute(|| faulty_fn(33)).err().unwrap();
    retry_policy.execute(|| ok_fn(42)).unwrap();

    println!();

    // retry any time err is received
    let retry_policy = PolicyBuilder::new()
        .handle_err()
        .retry(3);

    retry_policy.execute(|| faulty_fn(33)).err().unwrap();
    retry_policy.execute(|| ok_fn(42)).unwrap();

    println!();

    let values: HashSet<i32> = vec![1, 3, 5, 7, 42].drain(..).collect();
    let retry_policy = PolicyBuilder::new()
        .handle(|&x| values.contains(&x))
        .retry_with_action(
            3,
            |x, retry_count| {
                println!("Answer {} found in hashset {:?}, retry count: {}",
                         x, &values, retry_count
                )
            },
        );
    retry_policy.execute(|| random_fn());

    println!();

    // action is FnMut, so you can mutate outside objects
    // but you need to drop retry_policy as it mutably borrow
    let mut history_vec = vec![];
    PolicyBuilder::new()
        .handle_all()
        .retry_with_action(3, |x, retry_counter| {
            history_vec.push((x, retry_counter))
        })
        .execute(|| random_fn());

    println!("{:?}", history_vec);
    println!();

    let _retry_policy = PolicyBuilder::new()
        .handle(|&x: &i32| x == 42)
        .retry_forever();

    // Are you gonna live forever?
    // _retry_policy.execute(|| random_fn());
}