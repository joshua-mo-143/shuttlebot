use octocrab::models::InstallationToken;
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::Octocrab;

pub async fn init_github_app(app_id: String, rsa_key: String) -> Result<Octocrab, anyhow::Error> {
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(rsa_key.as_bytes()).unwrap();
    let token = octocrab::auth::create_jwt(app_id.parse::<u64>().unwrap().into(), &key).unwrap();

    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    let installations = octocrab
        .apps()
        .installations()
        .send()
        .await
        .unwrap()
        .take_items();

    let mut create_access_token = CreateInstallationAccessToken::default();
    create_access_token.repositories = vec!["test".to_string()];

    let access: InstallationToken = octocrab
        .post(
            installations[0].access_tokens_url.as_ref().unwrap(),
            Some(&create_access_token),
        )
        .await
        .unwrap();

    let crab = octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .expect("Failed to build Octocrab instance");

    Ok(crab)
}
