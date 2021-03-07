
use serde::{Deserialize, Serialize};
use warp::{reject, Filter, Rejection, Reply};
use warp::http::StatusCode;
use ed25519_dalek::{
     PublicKey, Sha512, Digest
};
use std::convert::Infallible;
use std::env;
use super::crypto;
use super::filesystem::managed_dns;
use super::filesystem::wireguard_config;

const SSL_PEM_CERT :&str = "SSL_PEM_CERT";
const SSL_KEY :&str = "SSL_KEY";
const PORT :&str = "PORT";
const DEFAULT_PORT :&str = "3030";
const HOST :&str = "HOST";
const DEFAULT_HOST :&str = "0.0.0.0";
const ALLOWED_ORIGIN :&str = "ALLOWED_ORIGIN";


#[derive(Debug)]
struct IncorrectSignature;
impl reject::Reject for IncorrectSignature {}

#[derive(Debug)]
struct FailWritingConfig;
impl reject::Reject for FailWritingConfig {}

#[derive(Debug)]
struct FeatureDisabled;
impl reject::Reject for FeatureDisabled {}

#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
    signature: String,
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}



// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "";
    } else if let Some(IncorrectSignature) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "Not authorized to update configuration";
    } else if let Some(FailWritingConfig) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Could not write config. Please check the server logs";
    } else {
        // We should have expected this... Just log and say its a 500
        error!("Unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }
    info!("{}", message);

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}

fn ok() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get().and(warp::path!("ok").map(|| format!("OK")))
}
fn update(
    public_key: PublicKey,
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::post()
        .and(warp::path("update"))
        .and(warp::body::json())
        .and(warp::any().map(move || public_key.clone()))
        .and_then(|message: Message, public_key: PublicKey| async move {
            let signature = crypto::decode_signature_base64(message.signature);
            let message_as_bytes = message.message.as_bytes();
            let mut prehashed: Sha512 = Sha512::default();
            prehashed.update(message_as_bytes);
            if public_key
                .verify_prehashed(prehashed, Some(b"wirtbot"), &signature)
                .is_ok()
            {
                Ok(message.message)
            } else {
                Err(reject::custom(IncorrectSignature))
            }
        })
        .and_then(|config: String| async {
            match wireguard_config::write_config_file(config) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    error!("Error when writing config file: {}", e);
                    return Err(reject::custom(FailWritingConfig));
                }
            };
        })
        .map(|_| format!("Config updated"))
}

fn update_device_dns_entries(
    public_key: PublicKey,
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::post()
        .and(warp::path("update-device-dns-entries"))
        .and(warp::body::json())
        // Drop out early if the MANAGED_DNS feature is not enabled
        .and_then(|message: Message| async move {
            if managed_dns::enabled() {
                Ok(message)
            } else {
                Err(reject::custom(FeatureDisabled))
            }
        })
        .and(warp::any().map(move || public_key.clone()))
        .and_then(|message: Message, public_key: PublicKey| async move {
            let signature = crypto::decode_signature_base64(message.signature);
            let message_as_bytes = message.message.as_bytes();
            let mut prehashed: Sha512 = Sha512::default();
            prehashed.update(message_as_bytes);
            if public_key
                .verify_prehashed(prehashed, Some(b"wirtbot"), &signature)
                .is_ok()
            {
                Ok(message.message)
            } else {
                Err(reject::custom(IncorrectSignature))
            }
        })
        .and_then(|device_list: String| async {
            match managed_dns::write_device_file(device_list) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    error!("{}", e);
                    return Err(reject::custom(FailWritingConfig));
                }
            }
        })
        .map(|_| {
            format!(
                "Updated {} with new devices",
                managed_dns::get_device_file_path()
            )
        })
}

pub async fn start_api(){
    let log = warp::log("wirt::api");
    let public_key_base64 = crypto::get_key();
    info!("Loaded public key: {}", public_key_base64);
    let public_key = crypto::decode_public_key_base64(public_key_base64);

    let allowed_origin: String = env::var(ALLOWED_ORIGIN).unwrap();
    let cors = warp::cors()
        .allow_origin(&allowed_origin[..])
        .allow_methods(vec!["POST"])
        .allow_header("content-type");

    let update_options = warp::options().and(warp::path("update")).map(warp::reply);
    let update_dns_options = warp::options()
        .and(warp::path("update-device-dns-entries"))
        .map(warp::reply);

    let routes = ok()
        .or(update(public_key))
        .or(update_options)
        .or(update_device_dns_entries(public_key))
        .or(update_dns_options)
        .with(log)
        .with(cors)
        .recover(handle_rejection);

    let port: String = env::var(PORT).unwrap_or(DEFAULT_PORT.into());
    let port: u16 = port.parse().unwrap();
    let host: Vec<u8> = env::var(HOST)
        .unwrap_or(DEFAULT_HOST.into())
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.to_string())
        .map(|value| value.parse().expect("Invalid Hostname specified"))
        .collect();

    let host: [u8; 4] = [host[0], host[1], host[2], host[3]];

    match env::var(SSL_PEM_CERT) {
        Ok(cert_path) => match env::var(SSL_KEY) {
            Ok(key_path) => {
                info! {"Running server in HTTPS mode with certificate: {} and key: {}", cert_path, key_path};
                warp::serve(routes)
                    .tls()
                    .cert_path(cert_path)
                    .key_path(key_path)
                    .run((host, port))
                    .await
            }
            Err(_e) => {
                info! {"Running server in HTTP mode"};
                warp::serve(routes).run((host, port)).await
            }
        },

        Err(_e) => {
            info! {"Running server in HTTP mode"};
            warp::serve(routes).run((host, port)).await
        }
    }

}