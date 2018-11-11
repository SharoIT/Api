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

use mongodb::coll::Collection;
use mongodb::db::{Database as MongoDatabase, ThreadedDatabase};
use mongodb::{Client, ThreadedClient};

pub struct Database {
    instance: MongoDatabase,
}

impl Database {
    pub fn connect(host: &str, port: u16, db_name: &str) -> Database {
        let instance = Client::connect(host, port)
            .expect("Failed to connect to mongodb")
            .db(db_name);
        Database { instance }
    }

    pub fn get_coll(&self, name: &str) -> Collection {
        self.instance.collection(name)
    }
}
