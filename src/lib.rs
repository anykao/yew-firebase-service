#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
extern crate yew;

use stdweb::Value;
use yew::callback::Callback;
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

    /// Sets interval which will call send a messages returned by a converter
    /// on every intarval expiration.
    pub fn once(&mut self, endpoint: &str, callback: Callback<Value>) -> FirebaseTask {
        let callback = move |val| {
            callback.emit(val);
        };
        let handle = js! {
            // console.log(@{endpoint});
            var callback = @{ callback };
            var db = firebase.database();
            db.ref(@{endpoint}).once("value")
            .then(function (snapshot) {
                let val = snapshot.val();
                callback(val);
                callback.drop();
            }).catch(function (err) {
                console.log(err);
                callback.drop();
            });
            // return callback;
        };
        FirebaseTask(Some(handle))
    }

    #[allow(non_snake_case)]
    pub fn signInWithEmailAndPassword(
        &mut self,
        email: &str,
        password: &str,
        callback: Callback<Result<Value, Value>>,
    ) -> FirebaseTask {
        let callback = move |success: bool, data: Value| {
            if success {
                callback.emit(Ok(data));
            } else {
                callback.emit(Err(data));
            }
        };
        let handle = js! {
            var callback = @{ callback };
            var auth = firebase.auth();
            auth.signInWithEmailAndPassword(@{email}, @{password})
                .then(userData => {
                    // console.log(userData);

                    callback(true, userData);
                    callback.drop();
                })
                .catch(error => {
                    console.log(error);
                    callback(false, error);
                    callback.drop();
                });
            // return callback;
        };
        FirebaseTask(Some(handle))
    }

    #[allow(non_snake_case)]
    pub fn onAuthStateChanged(&mut self, callback: Callback<Value>) -> FirebaseTask {
        let callback = move |val| {
            callback.emit(val);
        };
        let handle = js! {
            var callback = @{ callback };
            var auth = firebase.auth();
            auth.onAuthStateChanged(user=>{
                callback(user);
                callback.drop();
            });
            // return callback;
        };
        FirebaseTask(Some(handle))
    }

    #[allow(non_snake_case)]
    pub fn signOut(&mut self) {
        js! {
            var auth = firebase.auth();
            auth.signOut();
        };
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
