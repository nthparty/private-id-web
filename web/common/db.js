function DB() {
    this.firebaseConfig = {
        apiKey: "***REMOVED***",
        authDomain: "solutions-demo-d9591.firebaseapp.com",
        databaseURL: "https://solutions-demo-d9591.firebaseio.com",
        projectId: "solutions-demo-d9591",
        storageBucket: "solutions-demo-d9591.appspot.com",
        messagingSenderId: "248363884794",
        appId: "1:248363884794:web:effdf64a3be94397cde943"
    };
    this._collection = null;

    var self = this;

    this.initialize = function () {
        firebase.initializeApp(self.firebaseConfig);
        self._collection = firebase.database().ref().child('channel');
    };

    this.collection = function () {
        return self._collection;
    };

    this.newDocument = function (document) {
        var entryRef = self._collection.push();
        entryRef.set(document);
        return entryRef;
    };

    this.query = function (key, value, callback) {
        self._collection.orderByChild(key)
            .equalTo(value)
            .once("value", function (snapshot) {
                // If any data is found, invoke the callback on only
                // the last result (or on null if there is no result).
                var result = null;
                snapshot.forEach(function (childSnapshot) {
                    result = childSnapshot.val();
                });
                callback(result);
            });
    };

    this.remove = function (key, value) {
        self._collection.orderByChild(key)
            .equalTo(value)
            .once("value", function (snapshot) {
                snapshot.forEach(function (childSnapshot) {
                    firebase.database().ref('channel/' + childSnapshot.key).remove();
                });
            });
    };
}
