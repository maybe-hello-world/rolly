pub mod traits {
    pub trait Policy<O, R>
    {
        fn execute(&self, operation: O) -> R;
    }
}


pub mod retry {
    use super::traits::Policy;
    use std::sync::Arc;

    pub struct RetryPolicy<'l, R> {
        pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
        pub(in crate) count: u32
    }

    impl<'l, O, R> Policy<O, R> for RetryPolicy<'l, R>
        where
            O: Fn() -> R
    {
        fn execute(&self, operation: O) -> R {
            for _ in 0..self.count {
                let result = operation();

                // if all matchers return false -> return result
                if !self.matchers.iter().any(|op| op(&result)) {
                    return result
                }
            }
            operation()
        }
    }
}

pub mod builder {
    use super::retry::RetryPolicy;
    use std::sync::Arc;

    pub struct PolicyBuilder<'l, R> {
        pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>
    }

    impl<'l, R> PolicyBuilder<'l, R> {
        pub fn new() -> PolicyBuilder<'l, R> {
            PolicyBuilder {
                matchers: vec![]
            }
        }

        pub fn handle_all(mut self) -> PolicyBuilder<'l, R> {
            self.matchers.push(Arc::new(|_r| true));
            self
        }

        pub fn handle<F>(mut self, predicate: F) -> PolicyBuilder<'l, R>
        where F: Fn(&R) -> bool + 'l {
            self.matchers.push(Arc::new(predicate));
            self
        }

        pub fn retry(mut self, count: u32) -> RetryPolicy <'l, R> {
            if self.matchers.is_empty() {
                self = self.handle_all();
            }

            RetryPolicy {
                matchers: self.matchers.clone(),
                count
            }
        }
    }

    impl<'l, X, Y> PolicyBuilder<'l, Result<X, Y>> {
        pub fn handle_err(mut self) -> PolicyBuilder<'l, Result<X, Y>> {
            self.matchers.push(Arc::new(|result| {
                match result {
                    Ok(_) => false,
                    Err(_) => true
                }
            }));
            self
        }

        pub fn handle_ok(mut self) -> PolicyBuilder<'l, Result<X, Y>>{
            self.matchers.push(Arc::new(|result| {
                match result {
                    Ok(_) => true,
                    Err(_) => false
                }
            }));
            self
        }
    }
}

pub use builder::PolicyBuilder;
