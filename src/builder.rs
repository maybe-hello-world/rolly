use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::retry::{RetryForeverPolicy, RetryPolicy};

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

    pub fn retry(&self, count: usize) -> RetryPolicy<'l, R> {
        self.retry_with_action(count, |_, _| ())
    }

    pub fn retry_with_action<F>(&self, count: usize, action: F) -> RetryPolicy<'l, R>
        where F: FnMut(R, usize) -> () + 'l {
        RetryPolicy {
            matchers: self.matchers.clone(),
            action: Arc::new(Mutex::new(action)),
            durations: vec![Duration::from_nanos(0); count],
        }
    }

    pub fn retry_forever(&self) -> RetryForeverPolicy<'l, R> {
        self.retry_forever_with_action(|_| ())
    }

    pub fn retry_forever_with_action<F>(&self, action: F) -> RetryForeverPolicy<'l, R>
        where F: FnMut(R) -> () + 'l {
        RetryForeverPolicy {
            matchers: self.matchers.clone(),
            action: Arc::new(Mutex::new(action)),
            duration: Duration::from_nanos(0),
        }
    }

    pub fn wait_and_retry(&self, durations: Vec<Duration>) -> RetryPolicy<'l, R> {
        self.wait_and_retry_with_action(durations, |_, _| ())
    }

    pub fn wait_and_retry_with_action<F>(&self, durations: Vec<Duration>, action: F) -> RetryPolicy<'l, R>
        where F: FnMut(R, usize) -> () + 'l {
        RetryPolicy {
            matchers: self.matchers.clone(),
            action: Arc::new(Mutex::new(action)),
            durations,
        }
    }

    pub fn wait_and_retry_forever(&self, duration: Duration) -> RetryForeverPolicy<'l, R> {
        self.wait_and_retry_forever_with_action(duration, |_| ())
    }

    pub fn wait_and_retry_forever_with_action<F>(&self, duration: Duration, action: F) -> RetryForeverPolicy<'l, R>
        where F: FnMut(R) -> () + 'l {
        RetryForeverPolicy {
            matchers: self.matchers.clone(),
            action: Arc::new(Mutex::new(action)),
            duration,
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

    pub fn handle_ok(mut self) -> PolicyBuilder<'l, Result<X, Y>> {
        self.matchers.push(Arc::new(|result| {
            match result {
                Ok(_) => true,
                Err(_) => false
            }
        }));
        self
    }
}