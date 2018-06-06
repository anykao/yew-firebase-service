#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
extern crate yew;

mod auth;
mod database;
mod firestore;

use stdweb::Value;
use yew::services::Task;

/// A handle to cancel a firebase task.
pub struct FirebaseTask(Option<Value>);
pub struct FirebaseService {}

impl FirebaseService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new(config: Value) -> Self {
        js!{ @(no_return)
            firebase.initializeApp(@{config});
        }
        Self {}
    }
}

impl Task for FirebaseTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        // let handle = self.0.take().expect("tried to cancel firebase twice");
        // js! { @(no_return)
        //     var handle = @{handle};
        //     handle.drop();
        // }
    }
}

impl Drop for FirebaseTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
