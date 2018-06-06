use super::FirebaseService;
use super::FirebaseTask;
use stdweb::Value;
use yew::callback::Callback;

impl FirebaseService {
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
}
