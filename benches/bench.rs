#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use ksql::parser::Parser;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    for (name, src, expression) in [
        ("num_num", "", "1 + 1"),
        ("ident_num", r#"{"field1":1}"#, ".field + 1"),
        (
            "ident_ident",
            r#"{"field1":1,"field2":1}"#,
            ".field + .field2",
        ),
        (
            "fname_lname",
            r#"{"first_name":"Joey","last_name":"Bloggs"}"#,
            r#".first_name + " " + .last_name"#,
        ),
    ]
    .iter()
    {
        let ex = Parser::parse(expression.as_bytes()).unwrap();
        group.throughput(Throughput::Bytes(src.len() as u64));
        group.bench_function(*name, |b| {
            b.iter(|| {
                let _res = ex.calculate(src);
            })
        });
    }
    group.finish();

    let mut group = c.benchmark_group("complex");
    for (name, src, expression) in [
        ("paren_div", "", "(1 + 1) / 2"),
        (
            "paren_div_idents",
            r#"{"field1":1,"field2":1,"field3":2}"#,
            "(.field1 + .field2) / .field3",
        ),
        (
            "company_employees",
            r#"{"name":"Company","properties":{"employees":50}}"#,
            ".properties.employees > 20",
        ),
    ]
    .iter()
    {
        let ex = Parser::parse(expression.as_bytes()).unwrap();
        group.throughput(Throughput::Bytes(src.len() as u64));
        group.bench_function(*name, |b| {
            b.iter(|| {
                let _res = ex.calculate(src);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);