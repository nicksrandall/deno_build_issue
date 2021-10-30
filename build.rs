use deno_core::{JsRuntime, RuntimeOptions};
use std::fs::File;
use std::io::prelude::*;

// Example custom build script.
fn main() -> std::io::Result<()> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        will_snapshot: true,
        extensions: vec![
            deno_webidl::init(),
            deno_console::init(),
            deno_url::init(),
            deno_web::init(deno_web::BlobStore::default(), Default::default()),
            // deno_fetch::init::<deno_fetch::NoFetchPermissions>(
            //     "trade-rhythm-1".to_owned(),
            //     None,
            //     None,
            //     None,
            //     None,
            //     None,
            // ),
            deno_crypto::init(None),
            // deno_timers::init::<deno_timers::NoTimersPermission>(),
        ],
        ..Default::default()
    });
    let snapshot = runtime.snapshot();
    let mut file = File::create("runtime.snap")?;
    file.write_all(&snapshot)?;
    Ok(())
}
