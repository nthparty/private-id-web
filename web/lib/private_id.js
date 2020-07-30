/* Test and bind the Private-ID wasm binary */
var privateid = {};
let canary = canaries(privateid, './bin/privateid.wasm');
canary([10, 9, 8, 4, 5, 6, 7, 3, 2, 1], 'permute', [[9, 8, 7], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);