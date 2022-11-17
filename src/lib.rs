use serde_json::json;
use std::{collections::HashSet, str::FromStr};
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

struct MirrorConfig {
    anaconda_official_url: String,
    site_prefix: String,
    mirror_site: String,
    pkgs_channel: HashSet<String>,
    cloud_channel: HashSet<String>,
}

impl MirrorConfig {
    fn new_from_env(env: &RouteContext<()>) -> Self {
        // Setup information
        let anaconda_official = "https://anaconda.org/";
        let mirror_site = env.var("MIRROR_SITE").unwrap().to_string();
        let site_prefix = env.var("SITE_PREFIX").unwrap().to_string();
        let mirrored_pkgs_channel: HashSet<String> = env
            .var("MIRRORED_PKGS_CHANNEL")
            .unwrap()
            .to_string()
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mirrored_cloud_channel: HashSet<String> = env
            .var("MIRRORED_CLOUD_CHNNEL")
            .unwrap()
            .to_string()
            .lines()
            .map(|x| x.to_string())
            .collect();
        MirrorConfig {
            anaconda_official_url: anaconda_official.to_string(),
            site_prefix: site_prefix,
            mirror_site: mirror_site,
            pkgs_channel: mirrored_pkgs_channel,
            cloud_channel: mirrored_cloud_channel,
        }
    }
}

fn get_channel(url: &str, site_prefix: &str) -> String {
    let trimmed_url = url.replace(site_prefix, "");
    let channel: Vec<&str> = trimmed_url.split("/").collect();
    let channel = channel[0].to_string();
    channel
}

fn get_pkgs_channel_url(url:&str, config: &MirrorConfig) -> String{
    let result=config.mirror_site.to_string()+"pkgs/";
    let pkg_url = url.replace(&config.site_prefix, &result);
    pkg_url
}

fn get_cloud_channel_url(url: &str, config: &MirrorConfig) -> String {
    let result = config.mirror_site.to_string() + "cloud/";
    let cloud_url = url.replace(&config.site_prefix, &result);
    cloud_url
}

fn get_mirrored_package_url(url: &str, config: &MirrorConfig) -> String {
    let channel = get_channel(url, &config.site_prefix);
    let mirrored_url = match (config.pkgs_channel.contains(&channel), config.cloud_channel.contains(&channel)) {
        (true, _) => get_pkgs_channel_url(url, config),
        (_, true) => get_cloud_channel_url(url, config),
        _ => url.to_string(),
    };
    mirrored_url
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |req, ctx| {
            // config
            let config = MirrorConfig::new_from_env(&ctx);

            let req_url = req.url().unwrap();
            let req_url_str = req_url.to_string();
            let mirrored_url = get_mirrored_package_url(&req_url_str, &config);
            // a 302 redirect
            let resp = Response::redirect(Url::from_str(&mirrored_url).unwrap());
            resp
        }) 
        .run(req, env)
        .await
}
