// Copyright 2021 Datafuse Labs.
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

use common_base::tokio;
use common_exception::Result;
use common_meta_types::AuthType;
use pretty_assertions::assert_eq;

use crate::configs::Config;
use crate::users::User;
use crate::users::UserManager;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_user_manager() -> Result<()> {
    let mut config = Config::default();
    config.query.tenant_id = "tenant1".to_string();

    let user = "test-user1";
    let hostname = "localhost";
    let hostname2 = "%";
    let pwd = "test-pwd";
    let user_mgr = UserManager::create_global(config).await?;

    // add user hostname.
    {
        let user_info = User::new(user, hostname, pwd, AuthType::PlainText);
        user_mgr.add_user(user_info.into()).await?;
    }

    // add user hostname2.
    {
        let user_info = User::new(user, hostname2, pwd, AuthType::PlainText);
        user_mgr.add_user(user_info.into()).await?;
    }

    // get all users.
    {
        let users = user_mgr.get_users().await?;
        assert_eq!(2, users.len());
        assert_eq!(pwd.as_bytes(), users[0].password);
    }

    // get user hostname.
    {
        let user = user_mgr.get_user(user, hostname).await?;
        assert_eq!(hostname, user.hostname);
        assert_eq!(pwd.as_bytes(), user.password);
    }

    // get user hostname2.
    {
        let user = user_mgr.get_user(user, hostname2).await?;
        assert_eq!(hostname2, user.hostname);
        assert_eq!(pwd.as_bytes(), user.password);
    }

    // drop.
    {
        user_mgr.drop_user(user, hostname).await?;
        let users = user_mgr.get_users().await?;
        assert_eq!(1, users.len());
    }

    Ok(())
}
