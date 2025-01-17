use crate::auth::utils::insert_secret_to_vault;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use serde_json::Value;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slugify::slugify;
use std::env;
use std::sync::Arc;
use urlencoding;
use uuid::Uuid;

use crate::auth::utils::{
    generate_code_challenge, generate_code_verifier, generate_random_string,
    generate_unique_account_slug,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountAuthProviderAccount {
    pub account_auth_provider_account_id: Uuid,
    pub account_id: Uuid,
    pub auth_provider_id: String,
    pub auth_provider: Option<Value>,
    pub account_auth_provider_account_label: String,
    pub account_auth_provider_account_slug: String,
    pub account_data: Option<Value>,
    pub access_token: String,
    pub access_token_vault_id: String,
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
    pub refresh_token_vault_id: String,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failed: bool,
    pub failed_reason: Option<String>,
    pub failure_retries: i32,
    pub last_failure_retry: Option<DateTime<Utc>>
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub state: String,
    pub code_verifier: String,
    pub account_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct OAuthResponse {
    url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProvider {
    pub auth_provider_id: String,
    pub provider_name: String,
    pub provider_label: String,
    pub provider_icon: String,
    pub provider_description: String,
    pub provider_readme: String,
    pub auth_type: String,
    pub auth_url: String,
    pub token_url: String,
    pub provider_data: Option<serde_json::Value>,
    pub access_token_lifetime_seconds: Option<String>,
    pub refresh_token_lifetime_seconds: Option<String>,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_id_vault_id: Uuid,
    pub client_secret_vault_id: Option<Uuid>,
    pub scopes: String,
    pub public: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountAuthProviderAccount {
    pub account_id: String,
    pub auth_provider_id: String,
    pub account_auth_provider_account_label: String,
    pub account_auth_provider_account_slug: String,
    pub access_token_vault_id: String,
    pub refresh_token_vault_id: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token_expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

pub async fn handle_provider_callback(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<OAuthCallbackParams>,
) -> impl IntoResponse {
    println!(
        "[AUTH INIT] Handling auth callback for provider: {:?}",
        provider_name
    );
    println!("[AUTH INIT] Params: {:?}", params);

    let client = &state.anything_client;
    let auth_states = &state.auth_states;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get Provider details
    let response = match client
        .rpc(
            "get_decrypted_auth_provider_by_name",
            json!({"provider_name_param": &provider_name}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    println!(
        "[AUTH INIT] get_decryped_auth_provider Response: {:?}",
        response
    );

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let auth_providers: Vec<AuthProvider> = match serde_json::from_str(&body) {
        Ok(providers) => providers,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    let auth_provider = match auth_providers.into_iter().next() {
        Some(provider) => provider,
        None => return (StatusCode::NOT_FOUND, "Auth provider not found").into_response(),
    };

    println!("[AUTH INIT] AuthProvider: {:?}", auth_provider);

    // Verify state from the database
    // Retrieve the stored AuthState using the received state
    let auth_state = auth_states
        .read()
        .await
        .get(&params.state.unwrap())
        .cloned();

    let auth_state = match auth_state {
        Some(state) => state,
        None => return (StatusCode::BAD_REQUEST, "Invalid state").into_response(),
    };

    // Exchange code for token
    // Use the stored code_verifier in the token exchange
    let token = match exchange_code_for_token(
        &auth_provider,
        &params.code.as_deref().unwrap_or(""),
        &auth_provider.redirect_url,
        &auth_state.code_verifier, // Use the stored code_verifier here
    )
    .await
    {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to exchange code for token",
            )
                .into_response()
        }
    };

    println!("[AUTH INIT] Token: {:?}", token);

    let (account_slug, account_label) = generate_unique_account_slug(
        client,
        auth_provider.provider_label.as_str(),
        auth_state.account_id.as_str(),
    )
    .await;

    let refresh_token_expires_at = if let Some(refresh_token_lifetime) =
        auth_provider.refresh_token_lifetime_seconds.as_deref()
    {
        let refresh_token_lifetime: i64 = refresh_token_lifetime.parse().unwrap_or(0);
        Some(Utc::now() + chrono::Duration::seconds(refresh_token_lifetime))
    } else {
        None
    };

    let access_token_expires_at = if let Some(access_token_lifetime) =
        auth_provider.access_token_lifetime_seconds.as_deref()
    {
        let access_token_lifetime: i64 = access_token_lifetime.parse().unwrap_or(0);
        Some(Utc::now() + chrono::Duration::seconds(access_token_lifetime))
    } else {
        None
    };

    //Add access-token to vault
    let vault_access_token_name = slugify!(
        format!(
            "access_token_for_{}_for_account_{}",
            account_slug.clone(), //change so we can have multiple per account //its not used for fetching just unique id
            auth_state.account_id.clone()
        )
        .as_str(),
        separator = "_"
    );

    println!("Access Token Vault Name: {}", vault_access_token_name);

    let access_token_vault_id = insert_secret_to_vault(
        client,
        &vault_access_token_name,
        &token.access_token,
        &format!(
            "Access Token for {} for Account {}",
            auth_provider.auth_provider_id, auth_state.account_id
        ),
    )
    .await
    .unwrap();

    //Add refresh token secret in vault
    let vault_refresh_token_name = slugify!(
        format!(
            "refresh_token_for_{}_for_account_{}",
            account_slug.clone(), //
            auth_state.account_id.clone()
        )
        .as_str(),
        separator = "_"
    );

    println!("Refresh Token Vault Name: {}", vault_refresh_token_name);

    // let refresh_token_vault_id = body.trim_matches('"');
    let refresh_token_vault_id = insert_secret_to_vault(
        client,
        &vault_refresh_token_name,
        &token.refresh_token.unwrap_or_default(),
        &format!(
            "Refresh Token for {} for Account {}",
            auth_provider.auth_provider_id, auth_state.account_id
        ),
    )
    .await
    .unwrap();

    let input = CreateAccountAuthProviderAccount {
        account_id: auth_state.account_id.clone(),
        auth_provider_id: auth_provider.auth_provider_id.clone(),
        account_auth_provider_account_label: account_label,
        account_auth_provider_account_slug: account_slug,
        access_token_vault_id: access_token_vault_id.to_string(),
        access_token_expires_at: access_token_expires_at.unwrap_or_else(Utc::now),
        refresh_token_vault_id: refresh_token_vault_id.to_string(),
        refresh_token_expires_at: refresh_token_expires_at.unwrap_or_else(Utc::now),
    };

    println!("[AUTH INIT] Create Account Input: {:?}", input);
    // Store token in the database
    let create_account_response = match client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    println!(
        "[AUTH INIT] Create Account Response: {:?}",
        create_account_response
    );

    // Invalidate the bundler secrets cache for this account after creating a new secret
    // Only lock for the minimum time needed
    {
        let mut cache = state.bundler_accounts_cache.write().await;
        cache.invalidate(&auth_state.account_id);
    }

    // Return success response
    if create_account_response.status().is_success() {
        // Successful response
        let html = r#"
        <html>
        <body>
            <script>
            if (window.opener) {
                window.opener.postMessage('auth_success', '*');
                window.close();
            } else {
                document.body.innerHTML = 'Authentication successful. You can close this window.';
            }
            </script>
        </body>
        </html>
        "#;

        Html(html).into_response()
    } else {
        // Error response
        let html = r#"
        <html>
        <body>
            <h1>Authentication Failed</h1>
            <p>There was an error during authentication. Please try again.</p>
            <script>
            if (window.opener) {
                window.opener.postMessage('auth_failed', '*');
            }
            </script>
        </body>
        </html>
        "#;

        (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
    }
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
}

pub async fn exchange_code_for_token(
    provider: &AuthProvider,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> Result<OAuthToken, (StatusCode, String)> {
    let client = Client::new();

    let request = client
        .post(provider.token_url.clone())
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

    // // Add Authorization header if client_secret is present
    // if let Some(client_secret) = &provider.client_secret {
    //     let credentials = format!("{}:{}", provider.client_id, client_secret);
    //     let encoded_credentials = URL_SAFE_NO_PAD.encode(credentials);
    //     request = request.header(
    //         header::AUTHORIZATION,
    //         format!("Basic {}", encoded_credentials),
    //     );
    // }

    let form_params = [
        ("code", code),
        ("client_id", &provider.client_id),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
        ("code_verifier", code_verifier),
    ];

    println!("[AUTH INIT] token exchange form_params: {:?}", form_params);

    let response = request
        .form(&form_params)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status = response.status();
    println!("[AUTH INIT] Response status: {:?}", status);

    let body = response.text().await.map_err(|e| {
        println!("[AUTH INIT] Error reading response body: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("[AUTH INIT] Response body: {:?}", body);

    if status.is_success() {
        serde_json::from_str::<OAuthToken>(&body).map_err(|e| {
            println!("[AUTH INIT] Failed to parse token response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse token response: {}", e),
            )
        })
    } else {
        let error: ErrorResponse = serde_json::from_str(&body).map_err(|e| {
            println!("[AUTH INIT] Failed to parse error response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse error response: {}", e),
            )
        })?;
        // println!("Parsed error response: {:?}", error);

        let status_code = if error.error == "invalid_client" {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::BAD_REQUEST
        };

        println!(
            "[AUTH INIT] Returning error with status code: {:?}, description: {:?}",
            status_code, error.error_description
        );
        Err((status_code, error.error_description))
    }
}

pub async fn initiate_auth(
    Path((account_id, provider_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let auth_states = &state.auth_states;
    // Generate a unique state parameter
    let state_string = generate_random_string(32);
    let code_verifier = generate_code_verifier();
    // Use the account_id from the path parameter
    let account_id = account_id.clone();

    let auth_state = AuthState {
        state: state_string.clone(),
        code_verifier: code_verifier.clone(),
        account_id: account_id.clone(),
        created_at: Utc::now(),
    };

    println!("[AUTH INIT] Auth State: {:?}", auth_state);

    // Store the state in memory
    let mut auth_states_lock = auth_states.write().await;
    auth_states_lock.insert(state_string.clone(), auth_state);

    let client = &state.anything_client;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get Provider details
    let response = match client
        .rpc(
            "get_decrypted_auth_provider_by_name",
            json!({"provider_name_param": &provider_name}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find that provider",
            )
                .into_response()
        }
    };

    println!(
        "[AUTH INIT] get_decrypted_auth_provider_by_name Response: {:?}",
        response
    );

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("[AUTH INIT] Body: {:?}", body);

    let auth_providers: Vec<AuthProvider> = match serde_json::from_str(&body) {
        Ok(providers) => providers,
        Err(e) => {
            println!(
                "[AUTH INIT] Failed to parse JSON for auth_providers: {:?}",
                e
            );
            println!("[AUTH INIT] Raw body: {}", body);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse JSON for auth_providers",
            )
                .into_response();
        }
    };

    if auth_providers.is_empty() {
        println!("[AUTH INIT] No auth providers found in parsed JSON");
        return (StatusCode::NOT_FOUND, "No auth providers found").into_response();
    }

    println!("[AUTH INIT] Parsed {} auth providers", auth_providers.len());

    let auth_provider = match auth_providers.into_iter().next() {
        Some(provider) => provider,
        None => {
            println!("[AUTH INIT] No auth provider found after parsing");
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
    };

    println!("[AUTH INIT] Selected auth provider: {:?}", auth_provider);

    // Build the OAuth URL
    let client_id = auth_provider.client_id.clone();
    let redirect_uri = auth_provider.redirect_url.clone();
    let auth_url = auth_provider.auth_url.clone();

    let scope = auth_provider.scopes.clone();
    let code_challenge = generate_code_challenge(&code_verifier).await; // Assuming you have a function to generate code challenge

    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        auth_url,
        client_id,
        urlencoding::encode(redirect_uri.as_str()),
        urlencoding::encode(scope.as_str()),
        urlencoding::encode(&state_string),
        urlencoding::encode(&code_challenge)
    );

    println!("[AUTH INIT] Auth URL: {}", auth_url);

    Json(OAuthResponse { url: auth_url }).into_response()
}
