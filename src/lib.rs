use std::sync::{Arc, Weak};

use tinyroute::{Agent, ToAddress, Message};

// -----------------------------------------------------------------------------
//     - Entity -
// -----------------------------------------------------------------------------
pub struct Entity<T: Send + 'static> {
    inner: Weak<T>,
}

impl<T: Send + 'static> Entity<T> {
    pub fn access(&self) -> Option<impl AsRef<T>> {
        Weak::upgrade(&self.inner)
    }

    pub fn new(val: &Arc<T>) -> Self {
        Self {
            inner: Arc::downgrade(val)
        }
    }
}

// -----------------------------------------------------------------------------
//     - Store -
// -----------------------------------------------------------------------------
pub trait Store {
    type Data: Send + 'static;

    fn create_entity(&self) -> Entity<Self::Data>;
}

// -----------------------------------------------------------------------------
//     - Read only store -
// -----------------------------------------------------------------------------
pub struct Storage<T> {
    inner: Arc<T>,
}

impl<T: Send + Sync + 'static> Storage<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: Arc::new(val),
        }
    }

    pub async fn into_service<A: ToAddress>(self, mut agent: Agent<(), A>) {
        loop {
            if let Ok(msg) = agent.recv().await {
                match msg {
                    Message::Value((), sender) => log::info!("Message sent to a store from: {}", sender.to_string()),
                    Message::Fetch(req) => match req.reply_async(self.create_entity()).await {
                        Ok(()) => {}
                        Err(e) => log::error!("Failed to reply to fetch. Reason: {}", e),
                    }
                    Message::Shutdown => break,
                    Message::RemoteMessage { .. } 
                    | Message::AgentRemoved(_) => {}
                }
            }
        }
    }
}

impl<T: Send + Sync + 'static> Store for Storage<T> {
    type Data = T;

    fn create_entity(&self) -> Entity<Self::Data> {
        Entity { inner: Arc::downgrade(&self.inner) }
    }
}
