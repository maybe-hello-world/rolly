use std::sync::Arc;

use crate::traits::Policy;

pub struct RetryPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) count: u32,
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
                return result;
            }
        }
        operation()
    }
}
