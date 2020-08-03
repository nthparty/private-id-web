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

    this.idsFromURL = function () {
        var parts = window.location.href.split("#")[0].split("?");
        if (parts.length === 2) {
            var args = parts[1].split("&");
            var a_or_b = args[0].includes('this');
            args.sort(() => a_or_b?1:-1);  // Sort so that 'this' is first, and 'other' is second.  // -OR- `e => e.includes('this')?-1:1`
            if (args.length === 2) {
                var id_self = args[0].replace("this=", "");
                var id_other = args[1].replace("other=", "");
                document.getElementById('id-self').value = id_self;
                document.getElementById('id-other').value = id_other;
                setTimeout(function () {
                    $('#tab_i_am_party_' + (a_or_b?'a':'b')).click();
                }, 0);
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

        /*
         *
         * * */

        let a_or_b = $('#tab_i_am_party_a').is('.active');
        console.time('run');
        (a_or_b? partner : company)().then(function (id_spine) {
            console.timeEnd('run');
            // $("#sheet-other").html('<pre>' + id_spine + '</pre>');
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

            // Update interface to indicate results are posted to interface.
            $('#progress-message').text("");
            $('.modal').modal('hide');
        });
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
        setTimeout(function () { self.stages(0); }, 250);
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

var __rust__ = {
    progress_setPercent: demo.progress.setPercent,
    progress_message: str => $("#progress-message").text(str)
}