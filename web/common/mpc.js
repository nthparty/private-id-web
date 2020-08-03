function Protocol() {
  this._key = sodium.crypto_core_ristretto255_scalar_random();
  this._mask = sodium.crypto_core_ristretto255_scalar_random();
  this._maskInv = sodium.crypto_core_ristretto255_scalar_invert(this._mask);

  var self = this;

  this.encode_small = sodium.to_base64;
  this.decode_small = sodium.from_base64;
  this.encode_fast = sodium.to_hex;
  this.decode_fast = sodium.from_hex;

  this.encode = function (point) {
    return self.encode_small(point);  // btoa(String.fromCharCode.apply(null, point));
  };

  this.decode = function (str) {
    return self.decode_small(str);  // new Uint8Array(atob(str).split("").map(function (c) { return c.charCodeAt(0); }));
  };

  this.encode_debug = function (point) {
    return point.join(",");
  };

  this.decode_debug = function (str) {
    return new Uint8Array(e.split(",").map(function (s) { return parseInt(s); }));
  };

  this.arrayOfStrToPoint = function (a) {
    var h = sodium.to_hex(sodium.crypto_generichash(64, sodium.from_string(a.join(""))));
    return sodium.crypto_core_ristretto255_from_hash(h);
  };

  this.key = function (point) {
    return sodium.crypto_scalarmult_ristretto255(self._key, point);
  };

  this.keyEncode = function (point) {
    return self.encode(self.key(point));
  };

  this.decodeKeyEncode = function (encodedPoint) {
    return self.encode(self.key(self.decode(encodedPoint)));
  };

  this.mask = function (point) {
    return sodium.crypto_scalarmult_ristretto255(self._mask, point);
  };

  this.maskEncode = function (point) {
    return self.encode(self.mask(point));
  };

  this.unmask = function (point) {
    return sodium.crypto_scalarmult_ristretto255(self._maskInv, point);
  };

  this.decodeUnmask = function (encodedPoint) {
    return self.unmask(self.decode(encodedPoint));
  };

  this.pointToReducedStr = function (point) {
    return self.encode_fast(point);
  };

  this.shuffle = function shuffle(X) {
    var k, i;
    for (k = X.length; k > 0; k--) {
      i = sodium.randombytes_uniform(k);
      [X[i], X[k-1]] = [X[k-1], X[i]];
    }
    return X;
  };
}

function Session(self_id, other_id, self_roles, self_data_clear) {
  this._protocol = new Protocol();
  var pr = this._protocol;

  // State of this contributor.
  var self = this;
  self.id = self_id;
  self.roles = self_roles;
  self.data = {
    "clear": self_data_clear,
    "points": self_data_clear.map(pr.arrayOfStrToPoint)
  };

  // State of other contributor.
  var other = {};
  other.id = other_id;
  other.data = {};

  this.stepZero = function (callback) {
    var stepResult = {
      "status": true,
      "message": {
        "from": {"id": self.id, "roles": self.roles, "data": {}},
        "to": {"id": other.id, "data": {}}
      }
    };

    // We are always a contributor; send own keyed data.
    mapCallback(pr.keyEncode, self.data.points, function (eps) {
      stepResult.message.from.data.keyed_by_from = eps;

      // Send own masked data if we are a recipient.
      if (self.roles.includes("recipient")) {
        mapCallback(pr.maskEncode, self.data.points, function (eps) {
          stepResult.message.from.data.masked_by_from = eps;
          callback(stepResult);
        });
      } else {
        callback(stepResult);
      }
    });
  };

  this.stepOne = function (message, callback) {
    var stepResult = {
      "status": false
    };

    if (message != null) {
      stepResult.status = true;
      stepResult.response = {};

      // If other party is a recipient, key their masked data
      // and return it.
      if (message.from.roles.includes("recipient")) {
        mapCallback(pr.decodeKeyEncode, message.from.data.masked_by_from, function (eps) {
          stepResult.response = {"to": {
            "id": other.id,
            "data": {
              "masked_by_to_keyed_by_from":
              $('#quantitative').is(':checked')
              ? pr.shuffle(eps)
              : eps
            }
          }};
          callback(stepResult);
        });
      } else {
        callback(stepResult);
      }
    } else {
      callback(stepResult);
    }
  };

  this.stepTwo = function (message, callback) {
    var stepResult = {
      "status": false
    };

    if (message != null) {

      // If we are a recipient, obtain the data and compute the result.
      if (self.roles.includes("recipient") &&
          message.to.data != null &&
          message.to.data.masked_by_to_keyed_by_from != null
          ) {

        // We have the message and it contains the data we expect.
        stepResult.status = true;
        stepResult.response = {"to": {"roles": message.from.roles}};

        // Unmask own masked data that has been keyed by other contributor.
        mapCallback(pr.decodeUnmask, message.to.data.masked_by_to_keyed_by_from, function (eps) {
          self.data.keyed_by_other = eps;

          // Retrieve other contributor's keyed data.
          other.data.keyed_by_other =
            message.from.data.keyed_by_from.map(pr.decode);

          // Keep only bytes that will intersect.
          other.data.keyed_by_other = other.data.keyed_by_other.map(pr.pointToReducedStr);
          self.data.keyed_by_other = self.data.keyed_by_other.map(pr.pointToReducedStr);

          // Intersect `other.data.keyed_by_other` and `self.data.keyed_by_other`.
          stepResult.intersection = [];
          for (var i = 0; i < other.data.keyed_by_other.length; i++) {
            var j = self.data.keyed_by_other.indexOf(other.data.keyed_by_other[i]);
            if (j != -1) {
              stepResult.intersection.push(self.data.clear[j]);
            }
          }

          callback(stepResult);

        });

      } else if (!self.roles.includes("recipient")) {

        // We have the message and it contains no data, as we expect.
        stepResult.status = true;
        stepResult.response = {"to": {"roles": message.from.roles}};
        callback(stepResult);

      } else {
        callback(stepResult);
      }

    } else {
      callback(stepResult);
    }
  };
}

var lib = privateid.lib;

function partner() {
  lib = privateid.lib;
  let data = demo.sheets.self.data().map(e => e[0]);

  return new Promise(function (resolve) {
    lib.partner_stage_1(JSON.stringify(data));

    get('u_company_json').then(function (u_company_json) {
      let u_e_v_json = lib.partner_stage_2(u_company_json);
      give('u_e_v_json', u_e_v_json);

      get('v_s_s_json').then(function (v_s_s_json) {
        let v_s_s = v_s_s_json.split('|');
        let [v_partner_json, s_prime_partner_json, s_prime_company_json] = v_s_s;
        let s_double_prime_partner_json = lib.partner_stage_3(s_prime_partner_json);
        give('s_double_prime_partner_json', s_double_prime_partner_json);
        let id_spine = lib.partner_stage_4(v_partner_json, s_prime_company_json, "—", true);

        resolve(id_spine);
      });
    });
  });
}

function company() {
  lib = privateid.lib;
  let data = demo.sheets.self.data().map(e => e[0]);

  return new Promise(function (resolve) {
    let u_company_json = lib.company_stage_1(JSON.stringify(data));
    give('u_company_json', u_company_json);

    get('u_e_v_json').then(function (u_e_v_json) {
      let v_s_s_json = lib.company_stage_2(...u_e_v_json.split('|'));
      give('v_s_s_json', v_s_s_json);

      get('s_double_prime_partner_json').then(function (s_double_prime_partner_json) {
        let id_spine = lib.company_stage_3(s_double_prime_partner_json, "—", true);

        resolve(id_spine);
      });
    });
  });
}

var uev;
function test() {
  let data = demo.sheets.self.data().map(e => e[0]);
  lib.partner_stage_1(JSON.stringify(data));
  let u_company_json = lib.company_stage_1(JSON.stringify(data));
  give('u_company_json', u_company_json);
  delete u_company_json;
  get('u_company_json').then(function (u_company_json) {
    let u_e_v_json = lib.partner_stage_2(u_company_json);
    uev = u_e_v_json;
    // give('u_e_v_json', u_e_v_json);
    // get('u_e_v_json').then(function (u_e_v_json) {
      console.log('uev', u_e_v_json);
      let v_s_s_json = lib.company_stage_2(...u_e_v_json.split('|'));
      let v_s_s = v_s_s_json.split('|');
      let [v_partner_json, s_prime_partner_json, s_prime_company_json] = v_s_s;
      let s_double_prime_partner_json = lib.partner_stage_3(s_prime_partner_json);

      let id_spine = lib.partner_stage_4(v_partner_json, s_prime_company_json, "n/a", true);
      console.log(id_spine);
      id_spine = lib.company_stage_3(s_double_prime_partner_json, "n/a", true);
      console.log(id_spine);
    // });
  });
}

// // setTimeout(test, 1000);
// setTimeout(partner, 1000);
// setTimeout(company, 1200);
