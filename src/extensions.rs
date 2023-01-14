use crate::rules::SPARule;

pub trait VecPathExt {
    fn as_paths(&self) -> Vec<&str>;
    fn is_file(&self) -> bool;
    fn is_image(&self) -> bool;
}

impl VecPathExt for &str {
    fn as_paths(&self) -> Vec<&str> {
        self.split_terminator("/").collect()
    }

    fn is_file(&self) -> bool {
        let mut path = self.split_terminator(".");

        path.next().is_some() && path.next().is_some()
    }

    fn is_image(&self) -> bool {
        let path = self.split_terminator(".");
        let extension = path.last().unwrap();

        extension == "jpeg" || extension == "jpg" || extension == "png"
    }
}

impl VecPathExt for String {
    fn as_paths(&self) -> Vec<&str> {
        self.split_terminator("/").collect()
    }

    fn is_file(&self) -> bool {
        let mut path = self.split_terminator(".");

        path.next().is_some() && path.next().is_some()
    }

    fn is_image(&self) -> bool {
        todo!()
    }
}

pub trait FilterOneExt {
    type Output;
    type Predicate;

    fn filter_one(&self, file: Self::Predicate) -> Option<Self::Output>;
}

impl FilterOneExt for core::slice::Iter<'_, SPARule> {
    type Output = SPARule;
    type Predicate = String;

    fn filter_one(&self, file: Self::Predicate) -> Option<Self::Output> {
        let path = file.split_terminator("/").collect::<Vec<_>>();
        let is_root_only = path.get(1).is_none();

        'result: {
            for rule @ SPARule { route, .. } in self.as_slice() {
                if is_root_only {
                    if *route == "" {
                        break 'result Some(rule);
                    }
                } else {
                    if *route == &path[1..].join("/") {
                        break 'result Some(rule);
                    }
                }
            }

            None
        }
        .cloned()
    }
}

pub trait DetermineFileMIMEExt {
    fn get_mime_type(&self) -> &'static str;
}

impl DetermineFileMIMEExt for &str {
    fn get_mime_type(&self) -> &'static str {
        let split_paths = self.split_terminator(".");

        match split_paths.last().unwrap() {
            "aac" => "audio/aac",
            "html" | "htm" => "text/html",
            "svg" => "image/svg+xml",
            "zip" => "application/zip",
            "txt" => "text/plain",
            "xml" => "application/xml",
            "ico" => "image/vnd.microsoft.icon",
            "json" => "application/json",
            "js" | "mjs" => "text/javascript",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "csv" => "text/csv",
            "css" => "text/css",
            _ => "",
        }
    }
}

use send_wrapper::SendWrapper;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Shared<T>(pub Option<SendWrapper<T>>);

impl<T> Shared<T> {
    pub fn new(v: T) -> Self {
        Self(Some(SendWrapper::new(v)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_deref().clone().unwrap()
    }
}
