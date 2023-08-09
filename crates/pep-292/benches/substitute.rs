use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pep_292::Template;

pub fn criterion_benchmark(c: &mut Criterion) {
    let map = HashMap::from([
        ("identifier", "identifier"),
        ("noun", "noun"),
        ("string", "substitution"),
    ]);
    let input = black_box(
        r#"
This is an example text of what can be done with string substition. Each
A $identifier can be used, also $$ scapes and brackets ${identifier} 
"#,
    );
    let template = Template::new(input);
    c.bench_function("substitute", |b| b.iter(|| template.substitute(&map)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
