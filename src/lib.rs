#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use]
extern crate rocket;

#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod webinterface {
    #[derive(serde::Serialize)]
    pub struct Nav {
        name: &'static str,
        link: &'static str,
    }

    impl Nav {
        pub fn new(name: &'static str, link: &'static str) -> Nav {
            Nav { name, link }
        }
    }

    pub trait WebPage {
        fn get_navs() -> Vec<Nav> {
            vec![
                Nav::new("Home", "/"),
                Nav::new("Today's events", "/today"),
                Nav::new("Map", "/map"),
            ]
        }
    }
}

pub mod context {
    use crate::theme::ThemeInfo;
    use crate::webinterface::Nav;
    use crate::webinterface::WebPage;

    #[derive(serde::Serialize)]
    pub struct PageContext {
        title: &'static str,
        description: &'static str,
        events: Vec<&'static str>,
        navs: Vec<Nav>,
        darkmode: bool,
    }

    impl PageContext {
        pub fn new_events_page(
            title: &'static str,
            description: &'static str,
            events: Vec<&'static str>,
            theme: ThemeInfo,
        ) -> PageContext {
            PageContext {
                title,
                description,
                events,
                navs: PageContext::get_navs(),
                darkmode: theme.0,
            }
        }
    }

    impl WebPage for PageContext {}

    #[derive(serde::Serialize)]
    pub struct ErrorContext {
        title: &'static str,
        description: &'static str,
        message: String,
        navs: Vec<Nav>,
        darkmode: bool,
    }

    impl ErrorContext {
        pub fn new(title: &'static str, message: String, theme: ThemeInfo) -> ErrorContext {
            ErrorContext {
                title,
                description: "Oops!",
                message,
                navs: ErrorContext::get_navs(),
                darkmode: theme.0,
            }
        }
    }

    impl WebPage for ErrorContext {}
}

pub mod theme {
    use rocket::request::{self, FromRequest, Request};

    #[derive(Debug)]
    pub struct ThemeInfo(pub bool);

    impl<'a, 'r> FromRequest<'a, 'r> for ThemeInfo {
        type Error = ThemeInfo;
        fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<ThemeInfo, ThemeInfo> {
            match request
                .cookies()
                .get("darkmode")
                .and_then(|cookie| cookie.value().parse().ok())
                {
                    Some(value) => request::Outcome::Success(ThemeInfo(value)),
                    None => request::Outcome::Success(ThemeInfo(false))
                }
                
        }
    }
}

pub mod resources {
    use rocket::response::NamedFile;
    use std::path::{Path, PathBuf};

    #[get("/img/<path..>")]
    pub fn img(path: PathBuf) -> Option<NamedFile> {
        NamedFile::open(Path::new("img/").join(path)).ok()
    }
}
