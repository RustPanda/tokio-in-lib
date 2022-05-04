use async_ffi::FfiFuture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./target/debug/libplugin.so")?;
        let lib2 = libloading::Library::new("./target/debug/libplugin.so")?;
        let lib3 = libloading::Library::new("./target/debug/libplugin_http_clinet.so")?;

        {
            let init: libloading::Symbol<unsafe fn() -> safer_ffi::String> = lib.get(b"init")?;
            let err: String = init().into();
            if !err.is_empty() {
                panic!("{err}");
            }
        }

        {
            let init2: libloading::Symbol<unsafe fn() -> safer_ffi::String> = lib2.get(b"init")?;
            let err: String = init2().into();
            if !err.is_empty() {
                panic!("{err}");
            }
        }

        let test: libloading::Symbol<unsafe fn(f32) -> FfiFuture<safer_ffi::String>> =
            lib.get(b"test")?;

        let test2: libloading::Symbol<unsafe fn(f32) -> FfiFuture<safer_ffi::String>> =
            lib2.get(b"test")?;

        tokio::select! {
            res1 = test(5.) => {
                let res1: String = res1.try_into().unwrap();
                println!("res1: {}", res1);
            }
            res2 = test(2.) => {
                let res2: String = res2.try_into().unwrap();
                println!("res2: {}", res2);
            }
        }

        {
            let (res1, res2) = tokio::join!(test(2.), test(5.));

            let res1: String = res1.try_into().unwrap();
            let res2: String = res2.try_into().unwrap();

            println!("res1: {}", res1);
            println!("res2: {}", res2);
        }

        {
            let (res1, res2) = tokio::join!(test(2.), test2(5.));

            let res1: String = res1.try_into().unwrap();
            let res2: String = res2.try_into().unwrap();

            println!("res1: {}", res1);
            println!("res2: {}", res2);
        }

        let test3: libloading::Symbol<unsafe fn(f32) -> FfiFuture<safer_ffi::String>> =
            lib3.get(b"test")?;

        let res3 = test3(2.).await;

        let res3: String = res3.try_into().unwrap();
        println!("res3: {}", res3);
    }

    Ok(())
}
