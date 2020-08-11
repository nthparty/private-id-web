firebaseConfig = {};

__collection = null;

setTimeout(function () {
    firebase.initializeApp(self.firebaseConfig);
    __collection = firebase.database().ref().child('channel-private-id');
}, 1000);

const compress = m => m.split('|').map(x => JSON.parse(x).gz.map(n => (n).toString(36).padStart(2, '0')).join('')).join('|');
const expand = c => c.split('|').map(x => JSON.stringify({gz: x.match(new RegExp('.{1,2}', 'g')).map(s => parseInt(s, 36))})).join('|');

function give(tag, msg, json_gz = true, overwrite = true) {
    // console.log('give', tag, msg);
    if (json_gz) {
        msg = compress(msg);
    }
    // if (overwrite) {
    //     clear(tag).then(function () {
    //         __collection.push().set({tag, msg})
    //     });
    // }
    return __collection.push().set({tag, msg});
}

function get(tag, json_gz = true) {
    // console.log('get', tag);
    return new Promise(function (resolve) {
        let node = __collection.orderByChild('tag').equalTo(tag);
        node.once("value", function (response) {
            let msgs = response.val();
            if (msgs != null) {
                let keys = Object.keys(msgs);
                let last_key_idx = keys.length - 1;
                let last_key = keys[last_key_idx];
                let last_msg = msgs[last_key];

                firebase.database().ref('channel-private-id/' + last_key).remove();

                let msg = (json_gz? expand : m => m)(last_msg.msg)
                // console.log('got', tag, msg);
                resolve(msg);
            } else {
                setTimeout(function () {
                    get(tag, json_gz).then(function (msg) {
                        resolve(msg);
                    });
                }, 1000);
            }
        });
    });
}

function clear(tag) {
    // console.log('clear', tag);
    return new Promise(function (resolve) {
        let node = __collection.orderByChild('tag').equalTo(tag);
        node.once("value", function (response) {
            let msgs = response.val();
            if (msgs != null) {
                firebase.database().ref('channel-private-id/' + Object.keys(msgs)[0]).remove();

                resolve(clear(tag));
            } else {
                resolve(true);
            }
        });
    });
}
