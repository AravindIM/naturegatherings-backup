#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;
use naturegatherings::*;

mod forms {
    use naturegatherings::theme::ThemeInfo;
    use rocket::http::{Cookie, Cookies};
    use rocket::response::Redirect;

    #[post("/toggledarktheme")]
    pub fn toggle_dark_theme(theme: ThemeInfo, mut cookies: Cookies) -> Redirect {
        cookies.remove(Cookie::named("darkmode"));
        cookies.add(Cookie::new("darkmode", format!("{:?}", !theme.0)));
        Redirect::to("/..")
    }
}

mod pages {
    use naturegatherings::context::{ErrorContext, PageContext};
    use naturegatherings::theme::ThemeInfo;
    use rocket_contrib::templates::Template;

    #[get("/")]
    pub fn index(theme: ThemeInfo) -> Template {
        Template::render(
            "events",
            PageContext::new_events_page(
                "Home",
                "Popular events:",
                vec![
                    "A way for people to organise peaceful environment events online!",
                    "This website is under development!",
                ],
                theme,
            ),
        )
    }

    #[get("/today")]
    pub fn today(theme: ThemeInfo) -> Template {
        Template::render(
            "events",
            PageContext::new_events_page(
                "Today's events",
                "Events happening today:",
                vec![
                    "Made with Rust, Tera Template, and Material Bootstrap!",
                    "Hosted on Heroku!",
                ],
                theme,
            ),
        )
    }

    #[get("/<error>")]
    pub fn error(error: String, theme: ThemeInfo) -> Template {
        Template::render(
            "error",
            ErrorContext::new("404", format!("the link {} does not exist!", error), theme),
        )
    }
}

mod errors {
    use rocket::request::Request;
    use rocket::response::Redirect;
    use crate::pages;
    
    #[catch(404)]
    pub fn not_found(req: &Request<'_>) -> Redirect {
        Redirect::to(uri!(pages::error: req.uri().path()))
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                pages::index,
                pages::today,
                pages::error,
                forms::toggle_dark_theme,
                resources::img
            ],
        )
        .attach(Template::fairing())
        .register(catchers![errors::not_found])
        .launch();
}
