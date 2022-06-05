use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use proc_macro_regex::regex;
use regex::Regex;

const INPUT_EMAIL: &str = "example@example.org";
const INPUT_URL: &str = "https://www.example.org/page?param=value";
const INPUT_IPV6: &str = "fe80::1ff:fe23:4567:890a";

fn regex(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex");

    let regex_email = Regex::new("^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$").unwrap();
    let throughput = Throughput::Bytes(INPUT_EMAIL.len() as u64);
    let benchmark_id = BenchmarkId::new("email", INPUT_EMAIL.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_EMAIL, |b, input| {
        b.iter(|| regex_email.is_match(input))
    });

    let regex_url = Regex::new(
        r"^http(s)?://(([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+)|(([0-9A-Za-z-]+\.)+([a-z,A-Z][0-9A-Za-z_-]*)))(:[1-9][0-9]*)?(/([0-9A-Za-z_./:%+@&=-]+[0-9A-Za-z_ ./?:%+@&=-]*)?)?(#([\t\n\v\f\r ]*))?$",
    ).unwrap();
    let throughput = Throughput::Bytes(INPUT_URL.len() as u64);
    let benchmark_id = BenchmarkId::new("url", INPUT_URL.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_URL, |b, input| {
        b.iter(|| regex_url.is_match(input))
    });

    let regex_ipv6 = Regex::new(
        r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$",
    ).unwrap();
    let throughput = Throughput::Bytes(INPUT_IPV6.len() as u64);
    let benchmark_id = BenchmarkId::new("ipv6", INPUT_IPV6.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_IPV6, |b, input| {
        b.iter(|| regex_ipv6.is_match(input))
    });
}

fn proc_macro_regex(c: &mut Criterion) {
    let mut group = c.benchmark_group("proc-macro-regex");

    regex!(regex_email "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$");
    let throughput = Throughput::Bytes(INPUT_EMAIL.len() as u64);
    let benchmark_id = BenchmarkId::new("email", INPUT_EMAIL.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_EMAIL, |b, input| {
        b.iter(|| regex_email(input))
    });

    regex!(regex_url r"^http(s)?://(([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+)|(([0-9A-Za-z-]+\.)+([a-z,A-Z][0-9A-Za-z_-]*)))(:[1-9][0-9]*)?(/([0-9A-Za-z_./:%+@&=-]+[0-9A-Za-z_ ./?:%+@&=-]*)?)?(#([\t\n\v\f\r ]*))?$");
    let throughput = Throughput::Bytes(INPUT_URL.len() as u64);
    let benchmark_id = BenchmarkId::new("url", INPUT_URL.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_URL, |b, input| {
        b.iter(|| regex_url(input))
    });

    regex!(regex_ipv6 r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$" 1048576);
    let throughput = Throughput::Bytes(INPUT_IPV6.len() as u64);
    let benchmark_id = BenchmarkId::new("ipv6", INPUT_IPV6.len());
    group.throughput(throughput);
    group.bench_with_input(benchmark_id, INPUT_IPV6, |b, input| {
        b.iter(|| regex_ipv6(input))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = regex, proc_macro_regex
}
criterion_main!(benches);
