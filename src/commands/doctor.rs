use anyhow::Result;
use serde_json::json;

use crate::api::LinearClient;
use crate::cache;
use crate::config;
use crate::output::{print_json, OutputOptions};

pub async fn run(output: &OutputOptions, check_api: bool) -> Result<()> {
    let config_path = config::config_file_path()?;
    let config_data = config::load_config()?;
    let profile = config::current_profile().ok();
    let env_key = std::env::var("LINEAR_API_KEY").ok().filter(|k| !k.is_empty());
    let env_profile = std::env::var("LINEAR_CLI_PROFILE").ok().filter(|p| !p.is_empty());
    let cache_dir = cache::cache_dir_path()?;

    let configured = profile
        .as_ref()
        .and_then(|p| config_data.workspaces.get(p))
        .map(|w| !w.api_key.is_empty())
        .unwrap_or(false);

    let mut api_ok = None;
    let mut api_error = None;
    if check_api {
        match validate_api().await {
            Ok(()) => api_ok = Some(true),
            Err(err) => {
                api_ok = Some(false);
                api_error = Some(err.to_string());
            }
        }
    }

    if output.is_json() || output.has_template() {
        print_json(
            &json!({
                "config_path": config_path.to_string_lossy(),
                "profile": profile,
                "configured": configured,
                "env_api_key": env_key.is_some(),
                "env_profile": env_profile,
                "cache_dir": cache_dir.to_string_lossy(),
                "cache_ttl_seconds": output.cache.effective_ttl_seconds(),
                "api_ok": api_ok,
                "api_error": api_error,
            }),
            output,
        )?;
        return Ok(());
    }

    println!("Config path: {}", config_path.display());
    println!("Profile: {}", profile.unwrap_or_else(|| "none".to_string()));
    println!("Configured: {}", if configured { "yes" } else { "no" });
    println!(
        "Env API key override: {}",
        if env_key.is_some() { "yes" } else { "no" }
    );
    println!(
        "Env profile override: {}",
        env_profile.unwrap_or_else(|| "none".to_string())
    );
    println!("Cache dir: {}", cache_dir.display());
    println!(
        "Cache TTL: {}s",
        output.cache.effective_ttl_seconds()
    );
    if let Some(api_ok) = api_ok {
        println!("API check: {}", if api_ok { "ok" } else { "failed" });
        if let Some(err) = api_error {
            println!("API error: {}", err);
        }
    }

    Ok(())
}

async fn validate_api() -> Result<()> {
    let client = LinearClient::new()?;
    let query = r#"
        query {
            viewer {
                id
            }
        }
    "#;
    let result = client.query(query, None).await?;
    let viewer = &result["data"]["viewer"];
    if viewer.is_null() {
        anyhow::bail!("Viewer query failed");
    }
    Ok(())
}
