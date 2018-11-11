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
use bson::oid::ObjectId;
use bson::ordered::OrderedDocument;
use rocket::State;
use rocket_contrib::Json;

#[derive(Serialize)]
pub struct SampleResponse {
    error: bool,
    message: Option<String>,
    data: Option<OrderedDocument>,
}

#[get("/<id>")]
pub fn show(ctx: State<Context>, id: String) -> Json<SampleResponse> {
    let coll = ctx.db.get_coll("documents");
    match ObjectId::with_string(&id) {
        Ok(object_id) => {
            let filter = doc! {
                "_id":object_id
            };

            match coll.find_one(Some(filter), None) {
                Ok(document) => {
                    if let Some(data) = document {
                        Json(SampleResponse {
                            error: false,
                            message: None,
                            data: Some(data),
                        })
                    } else {
                        format_error(String::from("Not found"))
                    }
                }
                Err(e) => format_error(e.to_string()),
            }
        }
        Err(e) => format_error(String::from("Invalid Id")),
    }
}

fn format_error(msg: String) -> Json<SampleResponse> {
    Json(SampleResponse {
        error: true,
        message: Some(msg),
        data: None,
    })
}
