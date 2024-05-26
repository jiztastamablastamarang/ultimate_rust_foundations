use aws_sdk_s3::Endpoint;
use datafusion::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    use aws_smithy_http::endpoint::Endpoint;
    use aws_types::credentials::SharedCredentialsProvider;
    use aws_types::Credentials;
    use datafusion::execution::context::SessionContext;
    use datafusion_objectstore_s3::object_store::s3::S3FileSystem;
    use http::Uri;

    const ACCESS_KEY_ID: &str = "engr";
    const SECRET_ACCESS_KEY: &str = "DXTzn4mgV8GmeavQ";
    const PROVIDER_NAME: &str = "Static";
    const ENDPOINT: &str = "http://91.107.214.250:9000";

    let s3_file_system = S3FileSystem::new(
        Some(SharedCredentialsProvider::new(Credentials::new(
            ACCESS_KEY_ID,
            SECRET_ACCESS_KEY,
            None,
            None,
            PROVIDER_NAME,
        ))),
        None,
        Some(Endpoint::immutable(Uri::from_static(ENDPOINT))),
        None,
        None,
        None,
    )
    .await;

    let ctx = SessionContext::new();

    ctx.runtime_env()
        .register_object_store("s3", Arc::new(s3_file_system));

    ctx.register_json("leads", "s3://salesnav/lead", NdJsonReadOptions::default())
        .await?;

    let df = ctx.sql("SELECT * FROM leads").await?;

    let filtered_df = df.filter(col("tag").eq(lit("RDSHOW4.0/SF")))?;

    filtered_df.show().await?;

    return Ok(());
}

fn _empty() {
    let condition = true;
    let number = if condition { 1 } else { 2 }; // if is statement.
}

fn _loop() {
    let mut counter = 0;

    let x = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };
}

fn _while() {
    let mut counter = 0;

    while counter < 10 {
        counter += 1;
    }
}

fn _for() {
    let arr = [1, 2, 3, 4, 5];
    for (i, a) in arr.iter().enumerate() {
        println!("{a}")
    }
}

fn _range() {
    for i in (0..10).rev() {
       println!("{i}"); 
    }
}

fn _type_coercion(){
    let x = 34.4321_f32;
    let y: u8 = x as u8;
}
