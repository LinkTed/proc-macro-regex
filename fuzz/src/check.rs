use honggfuzz::fuzz;
use proc_macro_regex::regex;
use regex::{Regex, RegexBuilder};

fn build_regex(regex: &str) -> Regex {
    RegexBuilder::new(regex).unicode(false).build().unwrap()
}

fn check(string: &str, regex: &Regex, proc_macro_regex: fn(&str) -> bool) {
    let result_regex = regex.is_match(string);
    let result_proc_macro_regex = proc_macro_regex(string);
    if result_regex != result_proc_macro_regex {
        panic!(
            "{} != {}: {}",
            result_regex, result_proc_macro_regex, string
        );
    }
}

fn main() {
    let regex_email = build_regex(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$");
    regex!(proc_macro_regex_email r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$");

    let regex_url = build_regex(
        r"^http(s)?://((\d+\.\d+\.\d+\.\d+)|(([\w-]+\.)+([a-z,A-Z][\w-]*)))(:[1-9][0-9]*)?(/([\w./:%+@&=-]+[\w ./?:%+@&=-]*)?)?(#(\s*))?$",
    );
    regex!(proc_macro_regex_url r"^http(s)?://((\d+\.\d+\.\d+\.\d+)|(([\w-]+\.)+([a-z,A-Z][\w-]*)))(:[1-9][0-9]*)?(/([\w./:%+@&=-]+[\w ./?:%+@&=-]*)?)?(#(\s*))?$");

    let regex_ipv6 = build_regex(
        r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$",
    );
    regex!(proc_macro_regex_ipv6 r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$");

    let regex_test = build_regex("^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4})$");
    regex!(proc_macro_regex_test "^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4})$");

    loop {
        fuzz!(|string: &str| {
            check(string, &regex_email, proc_macro_regex_email);
            check(string, &regex_url, proc_macro_regex_url);
            check(string, &regex_ipv6, proc_macro_regex_ipv6);
            check(string, &regex_test, proc_macro_regex_test);
        });
    }
}
