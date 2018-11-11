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

use super::super::Context;
use super::super::Database;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct DocumentForm {
    is_public: bool,
    data: String,
    title: String,
}

#[derive(Serialize)]
pub struct SampleResponse {
    error: bool,
    message: Option<String>,
    id: Option<String>,
}

/**
 * Route handle to insert a new document
 */
#[post("/new", format = "application/json", data = "<document>")]
pub fn new(ctx: State<Context>, document: Json<DocumentForm>) -> Json<SampleResponse> {
    let db: &Database = &ctx.db;
    let data = doc! {
        "is_public": document.is_public.clone(),
        "data": document.data.clone(),
        "title": document.title.clone()
    };

    // Insert document inside the database
    let docs_coll = db.get_coll("documents");

    match docs_coll.insert_one(data, None) {
        Ok(inserted) => {
            let doc_id = inserted.inserted_id.unwrap();

            Json(SampleResponse {
                error: false,
                message: None,
                id: Some(doc_id.as_object_id().unwrap().to_hex()),
            })
        }
        Err(e) => Json(SampleResponse {
            error: true,
            message: Some(e.to_string()),
            id: None,
        }),
    }
}
