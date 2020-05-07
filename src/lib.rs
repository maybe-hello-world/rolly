pub mod traits {
    pub trait Policy<O, R, E>
    {
        fn execute(&self, operation: O) -> Result<R, E>;
    }
}


pub mod retry {
    use super::traits::Policy;
    use std::rc::Rc;

    pub struct RetryPolicy<R, E> {
        pub(in crate) matching_operations: Vec<Rc<dyn Fn(&Result<R, E>) -> bool>>,
        pub(in crate) count: u32
    }

    impl<O, R, E> Policy<O, R, E> for RetryPolicy<R, E>
        where
            O: Fn() -> Result<R, E>
    {
        fn execute(&self, operation: O) -> Result<R, E> {
            for _ in 0..self.count {
                let result = operation();
                for op in self.matching_operations.iter() {
                    if !op(&result) {
                        return result
                    }
                }
            }
            operation()
        }
    }
}

pub mod builder {
    use super::retry::RetryPolicy;
    use std::rc::Rc;

    pub struct PolicyBuilder<R, E> {
        pub(in crate) matching_operations: Vec<Rc<dyn Fn(&Result<R, E>) -> bool>>
    }

    impl<R, E> PolicyBuilder<R, E> {
        pub fn new() -> PolicyBuilder<R, E> {
            PolicyBuilder {
                matching_operations: vec![]
            }
        }

        pub fn handle_err(&mut self) -> &mut PolicyBuilder<R, E> {
            self.matching_operations.push(Rc::new(|result| {
                match result {
                    Ok(_) => false,
                    Err(_) => true
                }
            }));
            self
        }

        pub fn handle_ok(&mut self) -> &mut PolicyBuilder<R, E> {
            self.matching_operations.push(Rc::new(|result| {
                match result {
                    Ok(_) => true,
                    Err(_) => false
                }
            }));
            self
        }

        pub fn retry(&mut self, count: u32) -> RetryPolicy<R, E> {
            if self.matching_operations.is_empty() {
                self.handle_err();
            }

            RetryPolicy {
                matching_operations: self.matching_operations.clone(),
                count
            }
        }
    }
}

pub use builder::PolicyBuilder;
