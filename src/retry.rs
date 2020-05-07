use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::traits::Policy;
use std::ops::DerefMut;

pub struct RetryPolicy<'l, R> {
    pub(in crate) matchers: Vec<Arc<dyn Fn(&R) -> bool + 'l>>,
    pub(in crate) action: Arc<Mutex<dyn FnMut(R, usize) -> () + 'l>>,
    pub(in crate) durations: Vec<Duration>,
}

impl<'l, O, R> Policy<O, R> for RetryPolicy<'l, R>
    where
        O: FnMut() -> R
{
    fn execute(&self, mut operation: O) -> R {
        for (retry_count, dur) in self.durations.iter().enumerate() {
            let result = operation();

            // if all matchers return false -> return result
            if !self.matchers.iter().any(|op| op(&result)) {
                return result;
            } else {
                self.action.lock().unwrap().deref_mut()(result, retry_count + 1);
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
    pub(in crate) action: Arc<Mutex<dyn FnMut(R) -> () + 'l>>,
    pub(in crate) duration: Duration,
}

impl<'l, O, R> Policy<O, R> for RetryForeverPolicy<'l, R>
    where
        O: FnMut() -> R
{
    fn execute(&self, mut operation: O) -> R {
        loop {
            let result = operation();

            if !self.matchers.iter().any(|op| op(&result)) {
                return result;
            } else {
                self.action.lock().unwrap().deref_mut()(result);

                if self.duration.as_nanos() > 0 {
                    std::thread::sleep(self.duration)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::retry::RetryPolicy;
    use std::sync::{Arc, Mutex};
    use crate::traits::Policy;
    use std::time::Duration;

    #[test]
    fn retry_handle() {
        let mut counter = 0;
        let x = RetryPolicy {
            matchers: vec![Arc::new(|_r| true)],
            action: Arc::new(Mutex::new(|_, _| {
                counter += 1
            })),
            durations: vec![Duration::from_nanos(0); 5],
        };
        x.execute(|| ());
        drop(x);

        assert_eq!(5, counter);
    }

    #[test]
    fn retry_ok() {
        let mut counter = 0;
        let x = RetryPolicy {
            matchers: vec![Arc::new(|_r| false)],
            action: Arc::new(Mutex::new(|_, _| {
                counter += 1
            })),
            durations: vec![Duration::from_nanos(0); 5],
        };
        x.execute(|| ());
        drop(x);

        assert_eq!(0, counter);
    }


}