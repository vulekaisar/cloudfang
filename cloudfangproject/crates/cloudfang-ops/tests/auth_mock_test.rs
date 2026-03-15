use cloudfang_ops::{OpenStackCredentials, OpenStackSession};
use mockito::Server;

#[tokio::test]
async fn test_authentication_mock() {
    let mut server = Server::new_async().await;
    let auth_url = server.url();

    // Fake OpenStack Identity (Keystone) response
    let fake_token = "fake-token-12345";
    let mock = server.mock("POST", "/auth/tokens")
        .with_status(201)
        .with_header("X-Subject-Token", fake_token)
        .with_body(r#"{
            "token": {
                "expires_at": "2030-01-01T00:00:00.000000Z",
                "catalog": [
                    {
                        "type": "compute",
                        "endpoints": [
                            {"url": "http://fake-compute/v2.1", "interface": "public", "region": "RegionOne"}
                        ]
                    }
                ]
            }
        }"#)
        .create_async()
        .await;

    let credentials = OpenStackCredentials {
        auth_url,
        username: "admin".to_string(),
        password: "password".to_string(),
        project_name: "admin".to_string(),
        domain_name: "Default".to_string(),
    };

    let session = OpenStackSession::new(credentials).await;
    
    assert!(session.is_ok(), "Authentication failed: {:?}", session.err());
    let session = session.unwrap();

    mock.assert_async().await;

    assert_eq!(session.token, fake_token);
    assert_eq!(session.endpoint("compute").unwrap(), "http://fake-compute/v2.1");
    assert!(session.is_token_valid());
}
