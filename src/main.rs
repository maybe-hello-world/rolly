use rolly::PolicyBuilder;
use rolly::traits::Policy;

fn ok_fn(x: i32) -> Result<i32, i32> {
    println!("I always return Ok");
    Ok(x)
}

fn faulty_fn(x: i32) -> Result<i32, i32> {
    println!("I always return Err");
    Err(x)
}


fn main() {

    // retry any time ok is received
    let retry_policy = PolicyBuilder::new()
        .handle_ok()
        .retry(3);

    retry_policy.execute(|| faulty_fn(33));
    retry_policy.execute(|| ok_fn(42));

    println!("");

    // retry any time err is received
    let retry_policy = PolicyBuilder::new()
        .handle_err()
        .retry(3);

    retry_policy.execute(|| faulty_fn(33));
    retry_policy.execute(|| ok_fn(42));

}