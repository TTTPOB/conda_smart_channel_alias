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
    anaconda_official_url: Url,
    mirror_site: Url,
    pkgs_channel: HashSet<String>,
    cloud_channel: HashSet<String>,
}

impl MirrorConfig {
    fn new_from_env(env: &RouteContext<()>) -> Self {
        // Setup information
        let anaconda_official = "https://conda.anaconda.org";
        let mirror_site = env.var("MIRROR_SITE").unwrap().to_string();
        let mirrored_pkgs_channel: HashSet<String> = env
            .var("MIRRORED_PKGS_CHANNEL")
            .unwrap()
            .to_string()
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mirrored_cloud_channel: HashSet<String> = env
            .var("MIRRORED_CLOUD_CHANNEL")
            .unwrap()
            .to_string()
            .lines()
            .map(|x| x.to_string())
            .collect();
        MirrorConfig {
            anaconda_official_url: Url::from_str(&anaconda_official).unwrap(),
            mirror_site: Url::from_str(&mirror_site).unwrap(),
            pkgs_channel: mirrored_pkgs_channel,
            cloud_channel: mirrored_cloud_channel,
        }
    }
}

fn get_channel(path: &str) -> String {
    let channel: Vec<&str> = path.split("/").collect();
    let channel = channel[1].to_string();
    console_log!("channel: {}", channel);
    channel
}

fn get_pkgs_channel_url(path: &str, config: &MirrorConfig) -> Url {
    let mut mirror_site = config.mirror_site.to_string();
    mirror_site.push_str(format!("/pkg{}", path).as_str());

    let pkg_url = Url::from_str(&mirror_site).unwrap();
    console_log!("pkg_url: {}", pkg_url);
    pkg_url
}

fn get_cloud_channel_url(path: &str, config: &MirrorConfig) -> Url {
    let mut mirror_site = config.mirror_site.to_string();
    mirror_site.push_str(format!("/cloud{}", path).as_str());

    let cloud_url = Url::from_str(&mirror_site).unwrap();
    console_log!("cloud_url: {}", cloud_url);
    cloud_url
}

fn get_anaconda_official_url(path: &str, config: &MirrorConfig) -> Url {
    let official_url = config.anaconda_official_url.join(path).unwrap();
    console_log!("official_url: {}", official_url);
    official_url
}

fn get_mirrored_package_url(url: &str, config: &MirrorConfig) -> Url {
    let channel = get_channel(url);
    let mirrored_url = match (
        config.pkgs_channel.contains(&channel),
        config.cloud_channel.contains(&channel),
    ) {
        (true, _) => {
            console_log!("belongs to pkgs channel");
            get_pkgs_channel_url(url, config)
        }
        (_, true) => {
            console_log!("belongs to cloud channel");
            get_cloud_channel_url(url, config)
        }
        _ => {
            console_log!("belongs to none of the channels, fallback to official channel");
            get_anaconda_official_url(url, config)
        }
    };
    console_log!("Mirrored url: {}", mirrored_url);
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
        .get(
            "/",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c/:d",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c/:d/:e",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c/:d/:e/:f",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c/:d/:e/:f/:g",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .get(
            "/:a/:b/:c/:d/:e/:f/:g/:h",
            |req, ctx| {
                // config
                let config = MirrorConfig::new_from_env(&ctx);

                let req_path = req.path();
                let mirrored_url = get_mirrored_package_url(&req_path, &config);
                // a 302 redirect
                let resp = Response::redirect(mirrored_url);
                resp
            },
        )
        .run(req, env)
        .await
}
