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
