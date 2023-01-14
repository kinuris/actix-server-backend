#[cfg(test)]
use crate::configs::SPAConfig;

#[test]
fn spa_config_hash_test() {
    let mut config1 = SPAConfig::new("world");
    config1.disallow_logged_in("/", "/other");
    config1.restrict_route("/", "/other");

    assert_eq!(config1.rules.len(), 1);

    config1.disallow_logged_in("/other", "/some-other");
    assert_eq!(config1.rules.len(), 2);
}
