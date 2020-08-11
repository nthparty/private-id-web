__collection = null;

setTimeout(function () {
  sodium.ready.then(function () {
    const __key = window.location.search.substring(1).split('&').map(e => e.split('=')).filter(e => e[0] === 'access_key')[0][1];
    const __storage = [
      'I8xyy-YSHleWq-RfrE7aq5_dDkjcqxQa6vxnnh1BJIguU9qCVcQIsg' +
      '_-e7wXaVtj_jMpzMvOUhWHAKkyiImKEBNXXRLYTLnMuY9TTMv2Yk2U' +
      'u1kMt-dmGXOhVMBIugSHKfJXHGt70Kc5lueVqi37S5uubybg8RZBON' +
      'bV5STRpp54ec1Cq0eMveW-hKZjyHFoQh-pVk44_BqYLaDQrHOg1qTq' +
      'KSTpC3mUSp9mPXy-2CZbq0NZernsWKKgMH-4430inHFpKhJrLOPAcg' +
      'MZOjl-cr88uipZFTDopjZJPFxeV6WTykzBEFyr7RNkFr7Q3UpCxj0l' +
      'hs-JWgXSo1Uwrjicf6ZdC1baihbL_QR3qtzGC7_AF4yuCl8ymFgr0G' +
      '7lNzrIncHvPJug2oJMDbjMJm4Y3jqImVXJJGmX1yHSGUFxxtOvQ68h' +
      'vcX2DoQSanFNNZuMTA8HuUaKQB0RYpdncTlgmho', 'hHPj5xRKAvr' +
      'SuuhtwIGJ9aXdQBKmRs_eT8'
    ];

    try {
      const firebaseConfig = JSON.parse(
        sodium.to_string(
          sodium.crypto_aead_chacha20poly1305_decrypt(null,
            sodium.from_base64(__storage[0]), null, new Uint8Array(8),
            sodium.from_base64(__key + __storage[1])
          )
        )
      );

      firebase.initializeApp(firebaseConfig);
      __collection = firebase.database().ref().child('channel-private-id');

      $('#ContributeButton').prop('disabled', false);
    } catch (e) {
      if (e.toString() === 'Error: invalid input') {
        e = new Error('Failed to authenticate firebase for communications.  Bad API key?');
      }
      throw e;
    }
  });
}, 1000);

const compress = m => m.split('|').map(x => JSON.parse(x).gz.map(n => (n).toString(36).padStart(2, '0')).join('')).join('|');
const expand = c => c.split('|').map(x => JSON.stringify({gz: x.match(new RegExp('.{1,2}', 'g')).map(s => parseInt(s, 36))})).join('|');

function give(tag, msg, json_gz = true, overwrite = true) {
    // console.log('give', tag, msg);
    if (json_gz) {
        msg = compress(msg);
    }
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

function pair_get(tag, json_gz = true) { return get(tag + '@p' + other_id, json_gz); }
function pair_give(tag, msg, json_gz = true) { return give(tag + '@p' + self_id, msg, json_gz); }
function pair_clear(tag) { return Promise.all(clear(tag + '@p' + self_id), clear(tag + '@p' + other_id)); }
