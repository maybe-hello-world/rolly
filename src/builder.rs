use std::sync::Arc;

use crate::retry::RetryPolicy;

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

    pub fn retry(&self, count: u32) -> RetryPolicy<'l, R> {
        self.retry_with_action(count, |_, _| ())
    }

    pub fn retry_with_action<F>(&self, count: u32, action: F) -> RetryPolicy<'l, R>
    where F: Fn(R, u32) -> () + 'static {    // TODO: can we change static to smth?
        RetryPolicy {
            matchers: self.matchers.clone(),
            count,
            action: Arc::new(action),
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