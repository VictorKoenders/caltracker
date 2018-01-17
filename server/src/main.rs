#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate shared;
extern crate dotenv;
extern crate server;
extern crate diesel;
extern crate uuid;

use rocket::response::{content, NamedFile};
use rocket_contrib::Json;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html("
<!DOCTYPE html>
<head>
    <meta charset=\"utf-8\" />
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />
    <meta content=\"width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1\" name=\"viewport\" />
    <script>
        var Module = {};
        var __cargo_web = {};
        Object.defineProperty( Module, 'canvas', {
            get: function() {
                if( __cargo_web.canvas ) {
                    return __cargo_web.canvas;
                }

                var canvas = document.createElement( 'canvas' );
                document.querySelector( 'body' ).appendChild( canvas );
                __cargo_web.canvas = canvas;
                return canvas;
            }
        });
        </script>
    </head>
    <body>
        <script src=\"js/app.js\"></script>
    </body>
</html>")
}

#[get("/js/app.js")]
fn app_js() -> NamedFile {
    NamedFile::open("../client/target/wasm32-unknown-unknown/release/client.js").unwrap()
}

#[get("/client.wasm")]
fn client_wasm() -> NamedFile {
    NamedFile::open("../client/target/wasm32-unknown-unknown/release/client.wasm").unwrap()
}

#[get("/api/list")]
fn list(pg_pool: server::DbConn) -> Json<Vec<shared::Day>> {
    use server::schema::day::dsl::*;
    use server::schema::dayentry::dsl::*;
    use server::model::*;

    let query = dayentry
        .inner_join(server::schema::day::dsl::day)
        .order(day_of_month.desc())
        .order(month.desc())
        .order(year.desc());

    println!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&query)); 

    let entries = query.load::<DayEntryJoinedWithDay>(&*pg_pool)
        .expect("Could not load data from server");

    let mut days = vec![];
    for entry in entries {
        let index = match days.iter().position(|i| entry.matches(i)) {
            Some(i) => i,
            None => {
                let day = shared::Day {
                    date: shared::Date {
                        year: entry.day.1 as u16,
                        month: entry.day.2 as u8,
                        day: entry.day.3 as u8,
                    },
                    entries: Vec::new()
                };
                days.push(day);
                days.len() - 1
            }
        };
        days[index].entries.push(shared::Entry {
            name: entry.entry.2,
            value: entry.entry.3 as f32,
        });
    }

    Json(days)
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(server::init(&database_url))
        .mount("/", routes![index, list, app_js, client_wasm])
        .launch();
}
