function Progress(length) {
    this.length = length;
    this.index = 0;

    var self = this;

    this.setPercent = function (percent) {
        console.log(percent+'%');
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

var self_id = '';
var other_id = '';

function Demo() {
    // Demo state.
    this.sheets = new Sheets(this);
    this.progress = new Progress();
    this.session = null;

    var self = this;

    this.idCreate = function () {
        return 'xxxyxx'.replace(/[xy]/g, function (c) {
            var r = Math.random() * 16 | 0, v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    };

    this.initialize_sheets = function () {
      // Input/output spreadsheets.
      $("#sheet-self").hide();
      self.sheets.initialize();
      self.sheets.selfSelectRandomData(); // Load the random data default selection.
      $("#sheet-self").show();
      self.clear(); // Clear results output HTML elements.
    };

    this.set_role = function (role) {  // preferred_role
      setTimeout(function () {
          get('setup', false).then(function (other_role) {
              if (other_role === 'free') {
                  give('setup', role, false);
              } else if (other_role === 'a') {
                  role = 'b';
              } else if (other_role === 'b') {
                  role = 'a';
              }
              setTimeout(function () {
                  $('#tab_i_am_party_' + role).click();
                  self.initialize_sheets();
              }, 0);
          });
      }, 1000);
    };

    this.idsFromURL = function () {
        var parts = window.location.href.split("#")[0].split("?");
        if (parts.length === 2) {
            var args = parts[1].split("&");
            var a_or_b = args[0].includes('this');
            this.set_role(a_or_b? 'a' : 'b');
            args.sort(() => a_or_b?1:-1);  // Sort so that 'this' is first, and 'other' is second.  // -OR- `e => e.includes('this')?-1:1`
            if (args.length === 2) {
                var self_id = args[0].replace("this=", "");
                var other_id = args[1].replace("other=", "");
                document.getElementById('id-self').value = self_id;
                document.getElementById('id-other').value = other_id;
                return true;
            } else {
                return false;
            }
        } else {
            this.set_role('a');  // default to party a if available
            return false;
        }
    };

    this.stages = function (stage) {
        let a_or_b = $('#tab_i_am_party_a').is('.active');
        console.time('run');
        (a_or_b? partner : company)().then(function (id_spine) {
            console.timeEnd('run');
            $("#sheet-other").show();
            let data = id_spine
                .split('-----')[2]
                .slice(1,-1)
                .split('\n')
                .map(row => row.split('\t'))
                .sort((e, f) => e[0] - f[0])
                .map(e => [e[1]]);
            console.log(id_spine);
            console.log(data);
            demo.sheets.other.data(data);
            $("#status-other").text('Rows from the joint data set which are only known to the other party are included but unlabeled.');

            // Update interface to indicate results are posted to interface.
            $('#progress-message').text("");
            $('.modal').modal('hide');

            give('setup', 'free', false);
        });
    }

    this.initialize = function () {
        // Generate or parse contributor codes.
        if (!self.idsFromURL()) {
            self_id = self.idCreate();
            document.getElementById('id-self').value = self_id;
        }

        // Button to copy contributor code to clipboard.
        new ClipboardJS('#id-self-copy');

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
    };

    this.contribute = function () {
        self.progress.setPercent(0);
        $(".modal").modal("show");
        $("#progress-message").text("Gathering data from interface.");
        setTimeout(function () { self.stages(); }, 250);
        self_id = document.getElementById('id-self').value;
        other_id = document.getElementById('id-other').value;
    };

    this.clear_all_messages = function (self_id, other_id) {
        if (self_id == null || other_id == null) {
            self_id = document.getElementById('id-self').value;
            other_id = document.getElementById('id-other').value;
        }

        pair_clear('u_company_json');
        pair_clear('u_e_v_json');
        pair_clear('v_s_s_json');
        pair_clear('s_double_prime_partner_json');

        console.log("Cleared all messages for id pair '" + self_id + "'/'" + other_id + "'.");
    };
}

var demo = new Demo();

var __rust__ = {
    progress_setPercent: demo.progress.setPercent,
    progress_message: str => $("#progress-message").text(str)
}
