use std::sync::Arc;
use std::time::Duration;

use crate::traits::Policy;

pub struct RetryPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) action: Arc<dyn Fn(R, usize) -> () + 'l>,
    pub(in crate) durations: Vec<Duration>,
}

impl<'l, O, R> Policy<O, R> for RetryPolicy<'l, R>
    where
        O: Fn() -> R
{
    fn execute(&self, operation: O) -> R {
        for (retry_count, dur) in self.durations.iter().enumerate() {
            let result = operation();

            // if all matchers return false -> return result
            if !self.matchers.iter().any(|op| op(&result)) {
                return result;
            } else {
                (self.action)(result, retry_count);
                if dur.as_nanos() > 0 {
                    std::thread::sleep(*dur);
                }
            }
        }
        operation()
    }
}


pub struct RetryForeverPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) action: Arc<dyn Fn(R) -> () + 'l>,
    pub(in crate) duration: Duration,
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
                if self.duration.as_nanos() > 0 {
                    std::thread::sleep(self.duration)
                }
            }
        }
    }
}