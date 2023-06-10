use anyhow::anyhow;
use shuttle_secrets::SecretStore;

pub fn get_secrets(secrets: SecretStore) -> Result<(String, String), anyhow::Error> {
    let discord_token = if let Some(discord_token) = secrets.get("DISCORD_TOKEN") {
        discord_token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found"));
    };

    let github_token = if let Some(github_token) = secrets.get("GITHUB_TOKEN") {
        github_token
    } else {
        return Err(anyhow!("'GITHUB_TOKEN' was not found"));
    };

    Ok((discord_token, github_token))
}
