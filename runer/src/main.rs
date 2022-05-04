use async_ffi::FfiFuture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let handle = &tokio::runtime::Handle::current() as *const tokio::runtime::Handle;

        let lib = libloading::Library::new("./target/debug/libplugin.so")?;
        let test: libloading::Symbol<
            unsafe fn(f32, *const tokio::runtime::Handle) -> FfiFuture<safer_ffi::String>,
        > = lib.get(b"test")?;

        {
            println!("res1 or res2");
            tokio::select! {
                res1 = test(5., handle) => {
                    let res1: String = res1.try_into().unwrap();
                    println!("  res1: {}", res1);
                }
                res2 = test(2., handle) => {
                    let res2: String = res2.try_into().unwrap();
                    println!("  res2: {}", res2);
                }
            }
        }

        {
            println!("res2_1 and res2_2");
            let (res1, res2) = tokio::join!(test(2., handle), test(5., handle));

            let res1: String = res1.try_into().unwrap();
            let res2: String = res2.try_into().unwrap();

            println!("  res2_1: {}", res1);
            println!("  res2_2: {}", res2);
        }

        {
            println!("res3_1 and res3_2");
            let lib2 = libloading::Library::new("./target/debug/libplugin.so")?;
            let test2: libloading::Symbol<
                unsafe fn(f32, *const tokio::runtime::Handle) -> FfiFuture<safer_ffi::String>,
            > = lib2.get(b"test")?;

            {
                let (res1, res2) = tokio::join!(test(2., handle), test2(5., handle));

                let res1: String = res1.try_into().unwrap();
                let res2: String = res2.try_into().unwrap();

                println!("  res3_1: {}", res1);
                println!("  res3_2: {}", res2);
            }
        }

        {
            let lib3 = libloading::Library::new("./target/debug/libplugin_http_clinet.so")?;

            println!("res4");
            let test3: libloading::Symbol<unsafe fn(f32) -> FfiFuture<safer_ffi::String>> =
                lib3.get(b"test")?;

            let res3 = test3(2.).await;

            let res3: String = res3.try_into().unwrap();
            println!("  res4: {}", res3);
        }
    }

    Ok(())
}
