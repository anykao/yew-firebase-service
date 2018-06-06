use super::FirebaseService;
use stdweb::Value;
use yew::callback::Callback;

impl FirebaseService {
    // Cloud Firestore
    #[allow(non_snake_case)]
    pub fn collectionGet(&mut self, collection: &str, doc: &str, callback: Callback<Value>) {
        let callback = move |val| {
            callback.emit(val);
        };
        js! {
            var callback = @{ callback };
            var db = firebase.firestore();
            var docRef = db.collection(@{collection}).doc(@{doc});
            docRef.get().then(function(doc) {
                if (doc.exists) {
                    console.log("Document data:", doc.data());
                    callback(doc.data());
                } else {
                    // doc.data() will be undefined in this case
                    console.log("No such document!");
                }
                callback.drop();
            }).catch(function(error) {
                console.log("Error getting document:", error);
                callback.drop();
            });
        };
    }
}
