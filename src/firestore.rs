use super::FirebaseService;
use stdweb::Value;
use yew::callback::Callback;

impl FirebaseService {
    // Cloud Firestore
    #[allow(non_snake_case)]
    pub fn collection_doc(&mut self, collection: &str, doc: &str, callback: Callback<Value>) {
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

    // Cloud Firestore
    #[allow(non_snake_case)]
    pub fn collection(&mut self, collection: &str, callback: Callback<Value>) {
        let callback = move |val| {
            callback.emit(val);
        };
        js! {
            var callback = @{ callback };
            var db = firebase.firestore();
            var docRef = db.collection(@{collection});
            docRef.get().then(function(querySnapshot) {
                var videos = [];
                querySnapshot.forEach(function(doc) {
                    // doc.data() is never undefined for query doc snapshots
                    // console.log(doc.id, " => ", doc.data());
                    videos.push[doc.data()["url"]];
                });
                callback(videos);
                // callback.drop()
            });
        };
    }
}
