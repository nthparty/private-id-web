var sizes = [50, 500, 5000];
var demo_data = 'patients';  // 'https://nth.codes/private-match-web/data'
// JSON.stringify(a.map(e => [e[0], Math.random()<0.5?(Math.random()<0.5?"MGH":"BWH"):(Math.random()<0.5?"BIDMC":"SEMC"), (Math.random()<0.5?(Math.random()<0.5?"1":"2"):(Math.random()<0.5?(Math.random()<0.5?"B":"3"):(Math.random()<0.5?"4":"5"))) + String(Math.floor(Math.random()*4)) + String(Math.floor(Math.random()*10))]).map(e => [e[0], e[1], e[2], e[1]==="MGH"?Math.floor(Math.random()*8):Math.floor(Math.random()*15)]))

function Sheet(elementId, data) {
    var self = this;

    this.settings = function (data) {
        let a_or_b = $('#tab_i_am_party_a').is(".active");
        return {
            data: data,
            rowHeaders: true,
            colHeaders: ['Patient Name'].concat(a_or_b? ['Hospital', 'Room #'] : ['Stay (Days)']).concat(Array(10).fill("_")),
            colWidths: [120].concat(a_or_b? [65, 65] : [90]).concat(Array(10).fill(30)),
            preventOverflow: 'horizontal',
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
        return Math.floor((3 * n) / 5);
    };

    self.filter_demo = function (data) {
        let a_or_b = $('#tab_i_am_party_a').is(".active");
        let known_cols = e => a_or_b
            ? [e[0], e[1], e[2]]  // Party A knows location
            : [e[0], e[3]]        // Party B knows time
        ;
        return data.map(known_cols);
    }

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
                self.self.data(self.filter_demo(data));
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
                value: demo_data + '_' + sizes[i] + '.json',
                text: self.randomDataFraction(sizes[i])
            }));
        }
    };
}
