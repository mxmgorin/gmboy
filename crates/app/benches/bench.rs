use app::input::{combo::ComboHandler, config::InputConfig};
use criterion::{criterion_group, criterion_main, Criterion};
use sdl2::controller::Button;

fn criterion_benchmark(c: &mut Criterion) {
    let conf = InputConfig::default();
    let mut handler = ComboHandler::new();
    let button = Button::Back;

    c.bench_function("handle_combo", |b| {
        b.iter(|| {
            handler.handle(button, true, &conf);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
