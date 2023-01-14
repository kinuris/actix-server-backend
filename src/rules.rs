#[derive(Clone, Debug)]
pub struct StaticRule {
    pub route: &'static str,
    pub redirect: &'static str,
    pub on_fail: bool,
    pub admin_only: bool,
}

impl StaticRule {
    pub fn new(route: &'static str, redirect: &'static str) -> StaticRule {
        StaticRule {
            route,
            redirect,
            on_fail: true,
            admin_only: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SPARule {
    pub route: &'static str,
    pub redirect: &'static str,
    pub on_fail: bool,
    pub admin_only: bool,
}

impl SPARule {
    pub fn new(route: &'static str, redirect: &'static str) -> SPARule {
        SPARule {
            route,
            redirect,
            on_fail: true,
            admin_only: false,
        }
    }
}
