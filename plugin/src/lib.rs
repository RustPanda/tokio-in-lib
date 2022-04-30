use async_ffi::{FfiFuture, FutureExt};
use lazy_static::lazy_static;

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[no_mangle]
pub extern "C" fn test(arg: f32) -> FfiFuture<safer_ffi::String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    RUNTIME.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs_f32(arg)).await;
        tx.send("hello".to_string()).ok();
    });

    async move { rx.await.unwrap().into() }.into_ffi()
}
