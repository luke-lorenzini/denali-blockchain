//! Simple Goose load test example. Duplicates the simple example on the
//! Locust project page (<https://locust.io/>).
//!
//! ## License
//!
//! Copyright 2020-2022 Jeremy Andrews
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! <http://www.apache.org/licenses/LICENSE-2.0>
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use goose::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        // In this example, we only create a single scenario, named "WebsiteUser".
        .register_scenario(
            scenario!("WebsiteUser")
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(Duration::from_secs(5), Duration::from_secs(15))?
                // This transaction only runs one time when the user first starts.
                // .register_transaction(transaction!(submit_tx).set_on_start())
                // These next two transactions run repeatedly as long as the load test is running.
                .register_transaction(transaction!(submit_hello_world)), // .register_transaction(transaction!(website_about)),
        )
        .execute()
        .await?;

    Ok(())
}

/// Demonstrates how to log in when a user starts. We flag this transaction as an
/// `on_start` transaction when registering it above. This means it only runs one time
/// per user, when the user thread first starts.
async fn _website_login(user: &mut GooseUser) -> TransactionResult {
    let params = [("username", "test_user"), ("password", "")];
    let _goose = user.post_form("/login", &params).await?;

    Ok(())
}

async fn submit_hello_world(user: &mut GooseUser) -> TransactionResult {
    let payload = json!({
        "program": "vote",
        "payload": { "candidate": "0" }
    });

    let _response = user.post_json("/submit", &payload).await?;

    // if !response.status().is_success() {
    //     return user.set_failure("non-2xx response");
    // }

    Ok(())
}

async fn _submit_tx(user: &mut GooseUser) -> TransactionResult {
    let params = [("program", "fake"), ("payload", "{\"fake\":0}")];
    let _goose = user.post_form("/submit", &params).await?;

    Ok(())
}

/// A very simple transaction that simply loads the front page.
async fn _website_index(user: &mut GooseUser) -> TransactionResult {
    let _goose = user.get("/").await?;

    Ok(())
}

/// A very simple transaction that simply loads the about page.
async fn _website_about(user: &mut GooseUser) -> TransactionResult {
    let _goose = user.get("/about/").await?;

    Ok(())
}
