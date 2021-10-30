fn main() {
    println!("Hello, world!");
    let mut _rt = JsRuntime::new(RuntimeOptions {
        startup_snapshot: Some(Snapshot::Static(include_bytes!("../runtime.snap"))),
        extensions: vec![extension],
        ..Default::default()
    });
}
