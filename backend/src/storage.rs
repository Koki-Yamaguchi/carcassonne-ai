use aws_sdk_s3::{operation::get_object::GetObjectOutput, primitives::ByteStream, Client};
use rocket::State;

pub async fn put_object(storage_client: &State<Client>, key: &str, body: ByteStream) {
    let _ = storage_client
        .put_object()
        .bucket("top-carcassonner")
        .key(key)
        .body(body)
        .send()
        .await
        .unwrap();
}

pub async fn get_object_url(storage_client: &State<Client>, key: &str) -> String {
    let obj = storage_client
        .get_object()
        .bucket("top-carcassonner")
        .key(key)
        .send()
        .await;
    match obj {
        Ok(o) => {
            let t = o.last_modified.unwrap().to_millis().unwrap();
            format!(
                "https://top-carcassonner.s3.ap-northeast-1.amazonaws.com/{}?t={}",
                key, t,
            )
        }
        Err(_) => "".to_string(),
    }
}
