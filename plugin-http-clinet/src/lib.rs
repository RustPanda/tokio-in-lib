use async_ffi::{FfiFuture, FutureExt};
use http_client::{http_types, native, HttpClient};

#[no_mangle]
pub extern "C" fn test(arg: f32) -> FfiFuture<safer_ffi::String> {
    async move {
        async_std::task::sleep(std::time::Duration::from_secs_f32(arg)).await;
        let client = native::NativeClient::new();
        let req = http_types::Request::new(http_types::Method::Get, "https://httpstat.us/200");
        let res = client.send(req).await.unwrap();
        format!("status: {}", res.status()).into()
    }
    .into_ffi()
}
