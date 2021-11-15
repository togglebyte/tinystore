use std::sync::{Arc, RwLock};

use tinyroute::{Agent, ToAddress, Message};

// * Be notified of a change, but not what changed
// * Have a reference, not to mirror the data (that would be silly)

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Updated,
    Removed,
    // Created,
}

struct StoreOne<T, A: ToAddress> {
    inner: Arc<RwLock<T>>,
    agent: Agent<(), A>,
}

impl<T, A: ToAddress> StoreOne<T, A> {
    pub fn new(inner: T, agent: Agent<(), A>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
            agent,
        }
    }

    pub async fn run(mut self) {
        while let Ok(msg) = self.agent.recv().await {
        }
    }
}
