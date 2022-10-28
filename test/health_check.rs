#[cfg(test)]
mod tests {
    use crate::health_check;

}
// `cargo expand --test health_check` (<- name of
#[tokio::test]
async fn health_check_succeeds() {

    let response = health_check().await;
    assert!(response.status().is_success())
}

fn dummy_test() {

main()
}


#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check"
                 .send()
                 .await

                 .expect("Failed to execute request."));
    // Assert

assert!(response.status().is_success());
assert_eq!(Some(0), response.content_length());

}

async fn spawn_app() -> std::io::Result<()> {
    customers::run().await
}







