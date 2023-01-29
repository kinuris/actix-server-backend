use std::collections::HashSet;

pub enum SiteCategory<'a> {
    SinglePageApplication(&'a SPAConfig),
    StaticSite(&'a StaticConfig),
}

// RS-STRUCT
///////////////////////
//                   //
// SiteConfiguration //
//                   //
///////////////////////

pub struct SiteConfiguration {
    pub single_page_applications: HashSet<SPAConfig>,
    pub static_sites: HashSet<StaticConfig>,
}

impl SiteConfiguration {
    pub fn new() -> SiteConfiguration {
        SiteConfiguration {
            single_page_applications: HashSet::new(),
            static_sites: HashSet::new(),
        }
    }
}

impl SiteConfiguration {
    pub fn add_spa(&mut self, spa_config: SPAConfig) {
        self.single_page_applications.insert(spa_config);
    }

    pub fn add_static(&mut self, static_path: StaticConfig) {
        self.static_sites.insert(static_path);
    }

    pub fn determine_category(&self, path: &str) -> Option<SiteCategory> {
        let spa_site = self
            .single_page_applications
            .iter()
            .find(|app_config| app_config.app_name == path);

        if let Some(spa_site) = spa_site {
            return Some(SiteCategory::SinglePageApplication(spa_site));
        }

        let static_site = self
            .static_sites
            .iter()
            .find(|app_config| app_config.app_name == path);

        if let Some(static_site) = static_site {
            return Some(SiteCategory::StaticSite(static_site));
        }

        None
    }
}

// RS-STRUCT
//////////////////////
//                  //
//     SPAConfig    //
//                  //
//////////////////////

use crate::rules::{SPARule, StaticRule};

pub struct SPAConfig {
    pub app_name: &'static str,
    pub rules: Vec<SPARule>,
}

impl SPAConfig {
    pub fn new(app_name: &'static str) -> SPAConfig {
        SPAConfig {
            app_name,
            rules: vec![],
        }
    }
}

impl SPAConfig {
    pub fn restrict_route(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(SPARule::new(route_name, redirect));
    }

    pub fn disallow_logged_in(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(SPARule {
            route: route_name,
            redirect,
            on_fail: false,
            admin_only: false,
        })
    }

    pub fn restrict_route_to_admin(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(SPARule {
            route: route_name,
            redirect,
            on_fail: true,
            admin_only: true,
        })
    }
}

impl std::hash::Hash for SPAConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.app_name.hash(state);
    }
}

impl PartialEq for SPAConfig {
    fn eq(&self, other: &Self) -> bool {
        self.app_name == other.app_name
    }
}

impl Eq for SPAConfig {}

// RS-STRUCT
//////////////////////
//                  //
//   StaticConfig   //
//                  //
//////////////////////

pub struct StaticConfig {
    pub app_name: &'static str,
    pub rules: Vec<StaticRule>,
}

impl std::hash::Hash for StaticConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.app_name.hash(state);
    }
}

impl PartialEq for StaticConfig {
    fn eq(&self, other: &Self) -> bool {
        self.app_name == other.app_name
    }
}

impl Eq for StaticConfig {}

impl StaticConfig {
    pub fn new(app_name: &'static str) -> StaticConfig {
        StaticConfig {
            app_name,
            rules: vec![],
        }
    }
}

impl StaticConfig {
    pub fn restrict_route(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(StaticRule::new(route_name, redirect));
    }

    pub fn disallow_logged_in(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(StaticRule {
            route: route_name,
            redirect,
            on_fail: false,
            admin_only: false,
        })
    }

    pub fn restrict_route_to_admin(&mut self, route_name: &'static str, redirect: &'static str) {
        self.rules.retain(|rule| rule.route != route_name);
        self.rules.push(StaticRule {
            route: route_name,
            redirect,
            on_fail: true,
            admin_only: true,
        })
    }
}
