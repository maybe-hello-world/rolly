use std::sync::Arc;

use crate::traits::Policy;

pub struct RetryPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) count: u32,
    pub(in crate) action: Arc<dyn Fn(R, u32) -> () + 'l>,
}

impl<'l, O, R> Policy<O, R> for RetryPolicy<'l, R>
    where
        O: Fn() -> R
{
    fn execute(&self, operation: O) -> R {
        for retry_count in 0..self.count {
            let result = operation();

            // if all matchers return false -> return result
            if !self.matchers.iter().any(|op| op(&result)) {
                return result;
            } else {
                (self.action)(result, retry_count);
            }
        }
        operation()
    }
}


pub struct RetryForeverPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) action: Arc<dyn Fn(R) -> () + 'l>,
}

impl<'l, O, R> Policy<O, R> for RetryForeverPolicy<'l, R>
    where
        O: Fn() -> R
{
    fn execute(&self, operation: O) -> R {
        loop {
            let result = operation();

            if !self.matchers.iter().any(|op| op(&result)) {
                return result;
            } else {
                (self.action)(result);
            }
        }
    }
}