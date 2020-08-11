/* JS Canaries */
function canaries(mutable, path) {
    const cough = c => {throw new Error(c)};
    const chirp = function () {
        mutable.lib = {...wasm_bindgen};
        delete mutable.lib.__wbindgen_wasm_module;
    };

    return function (treat, breath, air) {
        let breathe = air => wasm_bindgen[breath](...air);
        wasm_bindgen(path).then(function () {
            if (String(breathe(air)) === String(treat)) {
                // Got back a treat!
                chirp();
            } else {
                // Can't breathe-
                cough("Rust code returned an unexpected result.");
            }
        });
    };
}