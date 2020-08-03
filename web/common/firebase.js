var mapCallbackBase = function (fn, arr, callback) {
    var arr_ = [];
    for (var i = 0; i < arr.length; i++) {
        arr_.push(fn(arr[i]));
    }
    callback(arr_);
}

var mapCallbackProgress = function (fn, arr, callback, index, arr_) {
    index = (index == null) ? 0 : index;
    arr_ = (arr_ == null) ? [] : arr_;
    var portion = 1000;
    var i = null;

    for (i = index; i < arr.length && (i-index) < portion; i++) {
        arr_.push(fn(arr[i]));
    }
    demo.progress.advanceByLength(i - index);

    if (i < arr.length) {
        setTimeout(function () { mapCallbackProgress(fn, arr, callback, i, arr_); }, 100);
    } else {
        callback(arr_);
    }
}

var mapCallback = mapCallbackProgress;

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

/* * * * * * * */

function Sheet(elementId, data) {
    var self = this;

    this.settings = function (data) {
        return {
            data: data,
            rowHeaders: true,
            colHeaders: false,
            filters: false,
            dropdownMenu: false,
            licenseKey: 'non-commercial-and-evaluation'
        };
    };

    this.data = function (data) {
        if (data == null) {
            // No data was supplied; return current data.
            return self.sheet.getData();
        } else {
            // Update data with supplied data.
            self.sheet.updateSettings(self.settings(data));
        }
    }

    // Create spreadsheet object and populate HTML element.
    data = data == null ? [[""]] : data; // Default data: one cell.
    self.sheet = new Handsontable(
        document.getElementById(elementId),
        self.settings(data)
    );
}

function Sheets(demo) {
    this.demo = demo;

    var self = this;

    this.randomDataFraction = function (n) {
        return 2;//Math.floor((3 * n) / 5);
    };

    this.selfSelectRandomData = function () {
        fetch($("#data-random").val())
            .then(response => {
                if (!response.ok) {
                    throw new Error("HTTP error " + response.status);
                }
                return response.json();
            })
            .then(data => { // The `data` result is a JSON object.
                // Sample the raw data (above 50% ensures a non-empty intersection).
                data = _.sample(data, self.randomDataFraction(data.length));
                self.self.data(data);
                $("#this_rows").val(data.length);
                $("#this_cols").val((data.length > 0) ? data[0].length : 1);
            })
            .catch(function () {
                // Should report a connectivity issue.
            });
    };

    this.selfResize = function () {
        // Unselect random data drop-down list element.
        $("#data-random").val("");

        // Get size information from sheet dimension
        // interface elements.
        var rows = Math.max(1, $("#this_rows").val());
        var cols = Math.max(1, $("#this_cols").val());

        // Fill sheet with empty data set of specified size.
        self.self.data(
            _.range(rows).map(function (__) {
                return _.range(cols).map(function (__) {
                    return "";
                });;
            })
        );
    };

    this.initialize = function () {
        // Create spreadsheets.
        self.self = new Sheet("sheet-self");
        self.other = new Sheet("sheet-other");

        // Add event handlers.
        $(".this_data_dim").keyup(self.selfResize);
        $(".this_data_dim").change(self.selfResize);
        $("#data-random").change(function () {
            self.selfSelectRandomData();
            self.demo.clear();
        });

        // Populate random data selection options.
        for (var i = 0; i < sizes.length; i++) {
            $('#data-random').append($('<option>', {
                value: 'data' + sizes[i] + '.json',
                text: self.randomDataFraction(sizes[i])
            }));
        }
    };
}

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

function Progress(length) {
    this.length = length;
    this.index = 0;

    var self = this;

    this.setPercent = function (percent) {
        $('#progress-overall-bar').css('width', percent+'%').attr('aria-valuenow', percent);
    };

    this.setLength = function (length) {
        self.length = length;
        self.index = 0;
        self.setPercent(0);
    };

    this.advanceByLength = function (delta) {
        delta =  (delta == null) ? 1 : delta;
        self.index += delta;
        self.setPercent(Math.floor((100 * self.index) / self.length));
    };

    this.advanceByPercent = function (delta) {
        delta =  (delta == null) ? 1 : delta;
        self.index += (delta * self.length) / 100;
        percent = Math.floor((100 * self.index) / self.length);
        self.setPercent(Math.floor((100 * self.index) / self.length));
    };
}

function Demo() {
    // Demo state.
    this.sheets = new Sheets(this);
    this.db = new DB();
    this.progress = new Progress();
    this.session = null;

    var self = this;

    this.idCreate = function () {
        return 'xxxyxx'.replace(/[xy]/g, function (c) {
            var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    };

    this.idsFromURL = function () {
        var parts = window.location.href.split("#")[0].split("?");
        if (parts.length == 2) {
            var args = parts[1].split("&");
            if (args.length == 2) {
                var id_self = args[0].replace("this=", "");
                var id_other = args[1].replace("other=", "");
                document.getElementById('id-self').value = id_self;
                document.getElementById('id-other').value = id_other;
                return true;
            }
            return false;
        }
        return false;
    };

    this.stages = function (stage) {
        var id_self = document.getElementById('id-self').value;
        var id_other = document.getElementById('id-other').value;
        var roles = // Determine if this contributor is also a recipient.
            ["contributor"] +
            ($("#receive").is(':checked') ? ["recipient"] : []);

        // Add the initial data from this contributor to the database if this
        // is the first time this function has been invoked.
        if (stage == 0) {

            // Reset the results panel.
            self.clear();

            // Create a protocol session data structure and
            // initialize progress tracking UI element.
            var data = self.sheets.self.data();
            self.session = new Session(id_self, id_other, roles, data);
            self.progress.setLength(data.length * 4);
            $("#progress-message").text("Preparing data for contribution.");

            // Write first message to database.
            self.session.stepZero(function (stepResult) {
                self.session.document = self.db.newDocument(stepResult.message);

                // Update the interface to indicate we are waiting
                // and begin polling for a response.
                $("#progress-message").text("Waiting for other party to contribute.");
                setTimeout(function () { self.stages(1); }, 500);
            });

        } else if (stage == 1 || stage == 2) {

            // Obtain data from other contributor.
            self.db.query("to/id", id_self, function (message) {

                if (stage == 1) {

                    self.session.stepOne(message, function (stepResult) {
                        if (!stepResult.status) {
                            // Poll again if we did not have the expected message.
                            setTimeout(function () { self.stages(1); }, 500);
                        } else {
                            // Key the masked data from the other contributor and
                            // post it back; move on to the next protocol stage.
                            $('#progress-message').text("Waiting for other party to send final response.");
                            self.session.document.update(stepResult.response);
                            setTimeout(function () { self.stages(2); }, 500);
                        }
                    });

                } else if (stage == 2) {

                    self.session.stepTwo(message, function (stepResult) {
                        if (!stepResult.status) {
                            // Keep polling if other contributor has not yet posted an
                            // expected response.
                            setTimeout(function () { self.stages(2); }, 500);
                        } else {
                            // We received a response but still need to compute the
                            // intersection.
                            $('#progress-message').text("Computing the intersection.");

                            // If we have an intersection, display it.
                            if (Array.isArray(stepResult.intersection)) {
                                $("#sheet-other").show();
                                if ($('#quantitative').is(':checked')) {
                                    // The rows were randomized so that only their count is meaningful.
                                    const result_count = stepResult.intersection.length;
                                    const my_total_count = self.session.data.clear.length;
                                    const percentage = 100 * result_count / my_total_count;
                                    $("#sheet-other").html(
                                        "<h1>" + percentage.toFixed(3) + "%</h1>" +
                                        "<h4>or " + result_count + " rows in common</h4>"
                                    );
                                } else {
                                    self.sheets.other.data(stepResult.intersection);
                                }
                            }

                            // Display whether the other contributor received data.
                            $("#status-other").show();
                            if (stepResult.response.to.roles.includes("recipient")) {
                                $("#status-other").text("Other contributor has received the results.");
                            } else {
                                $("#status-other").text("Other contributor did not request the results.");
                            }

                            // Clear out entries addressed to this contributor once finished.
                            self.db.remove("to/id", id_self);

                            // Update interface to indicate results are posted to interface.
                            $('#progress-message').text("");
                            $('.modal').modal('hide');
                        } // End if second stage data received.
                    });

                } // End if `stage` is `2`.

            }); // End database query.
        } // End if `stage` is `1` or `2`.
    }

    this.initialize = function () {
        // Generate or parse contributor codes.
        if (!self.idsFromURL()) {
            var id_self = self.idCreate();
            document.getElementById('id-self').value = id_self;
        }

        // Input/output spreadsheets.
        $("#sheet-self").hide();
        self.sheets.initialize();
        self.sheets.selfSelectRandomData(); // Load the random data default selection.
        $("#sheet-self").show();
        self.clear(); // Clear results output HTML elements.

        // Database API wrapper.
        self.db.initialize();

        // Button to copy contributor code to clipboard.
        new ClipboardJS('#id-self-copy');

        // Input box for other party's identification code.
        $("#id-other").focus(function () {
            $("#id-other").css("background-color", "#FFFFFF");
        });

        // Tab interface.
        $(".nav-link").click(function () {
            $(".nav-link").removeClass("active");
            $(this).addClass("active");

            if ($(this).attr('id') == 'tab_i_am_party_b') {
                $('#card_other').insertBefore('#card_this');
            } else {
                $('#card_this').insertBefore('#card_other');
            }
        });
    };

    this.clear = function () {
        // Clear the results panel.
        $("#sheet-other").hide();
        $("#status-other").text(
            $("#receive").is(':checked') ?
                "The results will be displayed here." :
                "It will be indicated here if/when the other party receives the results."
        );
        if (!$("#receive").is(':checked')) {
            $("#quantitative").prop("checked", false);
        }
    };

    this.contribute = function () {
        if (document.getElementById('id-other').value.length != 6) {
            $("#id-other").css("background-color", "pink");
        } else {
            self.progress.setPercent(0);
            $(".modal").modal("show");
            $("#progress-message").text("Gathering data from interface.");
            setTimeout(function () { self.stages(0); }, 250);
        }
    };

    this.clear_all_messages = function (self_id, other_id) {
        if (self_id == null || other_id == null) {
            self_id = document.getElementById('id-self').value;
            other_id = document.getElementById('id-other').value;
        }
        console.log("Cleared all messages for id pair '" + self_id + "'/'" + other_id + "'.");
    };
}

var demo = new Demo();