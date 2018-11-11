// Copyright 2018 webd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use(doc, bson)]
extern crate bson;
extern crate mongodb;

// modules
mod database;
mod handlers;

pub use self::database::Database;
use self::handlers::new;

pub struct Context {
    pub db: Database,
}

fn main() {
    rocket::ignite()
        .manage(Context {
            db: Database::connect("localhost", 27017, "sharoit"),
        })
        .mount("/", routes![new::new])
        .launch();
}
