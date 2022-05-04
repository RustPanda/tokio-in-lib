use async_ffi::{FfiFuture, FutureExt};

thread_local! {
    static  RUNTIME: std::cell::RefCell<Option<tokio::runtime::Runtime>> = std::cell::RefCell::new(None);
}

#[no_mangle]
pub extern "C" fn init() -> safer_ffi::String {
    match tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .worker_threads(1)
        .build()
    {
        Ok(new_runtime) => {
            RUNTIME.with(|runtime| {
                let _ = runtime.borrow_mut().insert(new_runtime);
            });
            "".to_string().into()
        }
        Err(err) => format!("{err}").into(),
    }
}

#[no_mangle]
pub extern "C" fn test(arg: f32) -> FfiFuture<safer_ffi::String> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    async move {
        RUNTIME
            .try_with(move |runtime| {
                if let Some(runtime) = runtime.borrow().as_ref() {
                    runtime.spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs_f32(arg)).await;
                        tx.send("hello".to_string()).ok();
                    });
                } else {
                    panic!("Not found Tokio runtime");
                }
            })
            .unwrap();
        rx.await.unwrap().into()
    }
    .into_ffi()
}
