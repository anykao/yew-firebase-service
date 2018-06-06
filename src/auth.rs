use super::FirebaseService;
use super::FirebaseTask;
use stdweb::Value;
use yew::callback::Callback;

impl FirebaseService {
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
