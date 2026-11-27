use base::config::keys;
use hbb_common::config;

pub fn apply() {
    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    for (key, value) in [
        (
            keys::OPTION_CUSTOM_RENDEZVOUS_SERVER,
            "rust.ljmam.com:21116",
        ),
        (keys::OPTION_RELAY_SERVER, "rust.ljmam.com:21117"),
        (keys::OPTION_API_SERVER, "http://rust.ljmam.com"),
        (
            keys::OPTION_KEY,
            "X7cyME6Ro9s18xTI8ePwIvnOmZL5Zhy6lXeUigFjJZE=",
        ),
    ] {
        defaults
            .entry(key.to_owned())
            .or_insert_with(|| value.to_owned());
    }
}
