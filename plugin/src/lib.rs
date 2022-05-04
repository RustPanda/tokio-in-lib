use async_ffi::{FfiFuture, FutureExt};
use tokio::runtime::Handle;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn test(arg: f32, handle: *const Handle) -> FfiFuture<safer_ffi::String> {
    let handle = &*handle;

    async move {
        let _enter = handle.enter();
        tokio::time::sleep(std::time::Duration::from_secs_f32(arg)).await;

        "hello".to_string().into()
    }
    .into_ffi()
}
