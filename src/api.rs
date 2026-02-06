use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

use crate::models::*;

/// Default Canny API base URL (generic — configure your subdomain via `canny auth`)
pub const DEFAULT_API_URL: &str = "https://canny.io/api/v1";

/// Canny API client
pub struct CannyClient {
    client: Client,
    api_url: String,
    api_key: String,
}

impl CannyClient {
    /// Create a new Canny API client
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            api_key,
        }
    }

    /// List posts from a board
    pub async fn list_posts(
        &self,
        board_id: &str,
        limit: Option<u32>,
        skip: Option<u32>,
        sort: Option<&str>,
        status: Option<&str>,
        author_id: Option<&str>,
        search: Option<&str>,
        company_id: Option<&str>,
        tag_ids: Option<Vec<&str>>,
    ) -> Result<PostsListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }
        if let Some(s) = sort {
            body["sort"] = json!(s);
        }
        if let Some(s) = status {
            body["status"] = json!(s);
        }
        if let Some(a) = author_id {
            body["authorID"] = json!(a);
        }
        if let Some(s) = search {
            body["search"] = json!(s);
        }
        if let Some(c) = company_id {
            body["companyID"] = json!(c);
        }
        if let Some(tags) = tag_ids {
            body["tagIDs"] = json!(tags);
        }

        let response = self
            .client
            .post(format!("{}/posts/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single post by ID, URL name (with board ID), or both
    pub async fn get_post(
        &self,
        id: Option<&str>,
        url_name: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<Option<CannyPost>> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(i) = id {
            body["id"] = json!(i);
        }
        if let Some(name) = url_name {
            body["urlName"] = json!(name);
        }
        if let Some(b) = board_id {
            body["boardID"] = json!(b);
        }

        let response = self
            .client
            .post(format!("{}/posts/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: PostRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.post)
    }

    /// Create a new post
    pub async fn create_post(
        &self,
        board_id: &str,
        author_id: &str,
        title: &str,
        details: Option<&str>,
        category_id: Option<&str>,
        by_id: Option<&str>,
        custom_fields: Option<serde_json::Value>,
        eta: Option<&str>,
        eta_public: Option<bool>,
        owner_id: Option<&str>,
        image_urls: Option<Vec<&str>>,
        created_at: Option<&str>,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
            "authorID": author_id,
            "title": title,
        });

        if let Some(d) = details {
            body["details"] = json!(d);
        }
        if let Some(c) = category_id {
            body["categoryID"] = json!(c);
        }
        if let Some(b) = by_id {
            body["byID"] = json!(b);
        }
        if let Some(cf) = custom_fields {
            body["customFields"] = cf;
        }
        if let Some(e) = eta {
            body["eta"] = json!(e);
        }
        if let Some(ep) = eta_public {
            body["etaPublic"] = json!(ep);
        }
        if let Some(o) = owner_id {
            body["ownerID"] = json!(o);
        }
        if let Some(urls) = image_urls {
            body["imageURLs"] = json!(urls);
        }
        if let Some(ca) = created_at {
            body["createdAt"] = json!(ca);
        }

        let response = self
            .client
            .post(format!("{}/posts/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Change the status of a post
    pub async fn change_post_status(
        &self,
        post_id: &str,
        changer_id: &str,
        status: &str,
        notify_voters: bool,
        comment: Option<&str>,
        comment_image_urls: Option<Vec<&str>>,
    ) -> Result<()> {
        let mut body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "changerID": changer_id,
            "status": status,
            "shouldNotifyVoters": notify_voters,
        });

        if let Some(c) = comment {
            body["commentValue"] = json!(c);
        }
        if let Some(urls) = comment_image_urls {
            body["commentImageURLs"] = json!(urls);
        }

        let response = self
            .client
            .post(format!("{}/posts/change_status", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status_code = response.status();
        let text = response.text().await?;

        if !status_code.is_success() {
            anyhow::bail!("API error ({}): {}", status_code, text);
        }

        Ok(())
    }

    /// Update a post
    pub async fn update_post(
        &self,
        post_id: &str,
        title: Option<&str>,
        details: Option<&str>,
        image_urls: Option<Vec<&str>>,
        eta: Option<&str>,
        eta_public: Option<bool>,
        custom_fields: Option<serde_json::Value>,
    ) -> Result<()> {
        let mut body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
        });

        if let Some(t) = title {
            body["title"] = json!(t);
        }
        if let Some(d) = details {
            body["details"] = json!(d);
        }
        if let Some(urls) = image_urls {
            body["imageURLs"] = json!(urls);
        }
        if let Some(e) = eta {
            body["eta"] = json!(e);
        }
        if let Some(ep) = eta_public {
            body["etaPublic"] = json!(ep);
        }
        if let Some(cf) = custom_fields {
            body["customFields"] = cf;
        }

        let response = self
            .client
            .post(format!("{}/posts/update", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Delete a post
    pub async fn delete_post(&self, post_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
        });

        let response = self
            .client
            .post(format!("{}/posts/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Change the category of a post
    pub async fn change_post_category(&self, post_id: &str, category_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "categoryID": category_id,
        });

        let response = self
            .client
            .post(format!("{}/posts/change_category", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Add a tag to a post
    pub async fn add_post_tag(&self, post_id: &str, tag_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "tagID": tag_id,
        });

        let response = self
            .client
            .post(format!("{}/posts/add_tag", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Remove a tag from a post
    pub async fn remove_post_tag(&self, post_id: &str, tag_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "tagID": tag_id,
        });

        let response = self
            .client
            .post(format!("{}/posts/remove_tag", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Link a Jira issue to a post
    pub async fn link_post_jira(&self, post_id: &str, issue_key: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "issueKey": issue_key,
        });

        let response = self
            .client
            .post(format!("{}/posts/link_jira", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Unlink a Jira issue from a post
    pub async fn unlink_post_jira(&self, post_id: &str, issue_key: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "issueKey": issue_key,
        });

        let response = self
            .client
            .post(format!("{}/posts/unlink_jira", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List comments for a post
    pub async fn list_comments(
        &self,
        post_id: Option<&str>,
        author_id: Option<&str>,
        board_id: Option<&str>,
        company_id: Option<&str>,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<CommentsListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(p) = post_id {
            body["postID"] = json!(p);
        }
        if let Some(a) = author_id {
            body["authorID"] = json!(a);
        }
        if let Some(b) = board_id {
            body["boardID"] = json!(b);
        }
        if let Some(c) = company_id {
            body["companyID"] = json!(c);
        }
        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/comments/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Create a comment on a post
    pub async fn create_comment(
        &self,
        post_id: &str,
        author_id: &str,
        value: &str,
        parent_id: Option<&str>,
        created_at: Option<&str>,
        image_urls: Option<Vec<&str>>,
        internal: Option<bool>,
        should_notify_voters: Option<bool>,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "authorID": author_id,
            "value": value,
        });

        if let Some(p) = parent_id {
            body["parentID"] = json!(p);
        }
        if let Some(c) = created_at {
            body["createdAt"] = json!(c);
        }
        if let Some(urls) = image_urls {
            body["imageURLs"] = json!(urls);
        }
        if let Some(i) = internal {
            body["internal"] = json!(i);
        }
        if let Some(n) = should_notify_voters {
            body["shouldNotifyVoters"] = json!(n);
        }

        let response = self
            .client
            .post(format!("{}/comments/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Retrieve a single comment by ID
    pub async fn get_comment(&self, comment_id: &str) -> Result<Option<CannyComment>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": comment_id,
        });

        let response = self
            .client
            .post(format!("{}/comments/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CommentRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.comment)
    }

    /// Delete a comment by ID
    pub async fn delete_comment(&self, comment_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "commentID": comment_id,
        });

        let response = self
            .client
            .post(format!("{}/comments/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List categories for a board
    pub async fn list_categories(
        &self,
        board_id: &str,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<CategoriesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/categories/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single category by ID
    pub async fn get_category(&self, category_id: &str) -> Result<Option<CannyCategory>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": category_id,
        });

        let response = self
            .client
            .post(format!("{}/categories/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CategoryRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.category)
    }

    /// Create a new category
    pub async fn create_category(
        &self,
        board_id: &str,
        name: &str,
        parent_id: Option<&str>,
        subscribe_admins: bool,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
            "name": name,
            "subscribeAdmins": subscribe_admins,
        });

        if let Some(p) = parent_id {
            body["parentID"] = json!(p);
        }

        let response = self
            .client
            .post(format!("{}/categories/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Delete a category
    pub async fn delete_category(&self, category_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "categoryID": category_id,
        });

        let response = self
            .client
            .post(format!("{}/categories/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List all users (automatically depaginates using cursor pagination)
    /// Note: Uses API v2 which requires cursor-based pagination
    /// If `on_progress` is provided, it will be called with the current count after each page
    pub async fn list_users<F>(&self, mut on_progress: Option<F>) -> Result<Vec<CannyUserFull>>
    where
        F: FnMut(usize),
    {
        let mut all_users: Vec<CannyUserFull> = Vec::new();
        let mut cursor: Option<String> = None;
        let limit: u32 = 100; // API max is 100

        loop {
            let (users, next_cursor, has_next) =
                self.fetch_users_page(cursor.as_deref(), limit).await?;

            if users.is_empty() {
                break;
            }

            all_users.extend(users);

            if let Some(ref mut progress) = on_progress {
                progress(all_users.len());
            }

            if !has_next || next_cursor.is_none() {
                break;
            }

            cursor = next_cursor;

            // Safety limit to prevent infinite loops
            if all_users.len() > 100000 {
                break;
            }
        }

        Ok(all_users)
    }

    /// Fetch a single page of users using cursor pagination
    /// Returns (users, next_cursor, has_next_page)
    async fn fetch_users_page(
        &self,
        cursor: Option<&str>,
        limit: u32,
    ) -> Result<(Vec<CannyUserFull>, Option<String>, bool)> {
        let mut body = json!({
            "apiKey": self.api_key,
            "limit": limit,
        });

        if let Some(c) = cursor {
            body["cursor"] = json!(c);
        }

        // Users endpoint uses v2 API
        let base_url = self.api_url.replace("/v1", "/v2");
        let response = self
            .client
            .post(format!("{}/users/list", base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let value: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse response as JSON")?;

        let obj = value
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Expected object response"))?;

        // v2 API returns "items" array
        let users_value = obj
            .get("items")
            .or_else(|| obj.get("users"))
            .cloned()
            .unwrap_or(json!([]));

        let users: Vec<CannyUserFull> =
            serde_json::from_value(users_value).context("Failed to parse users from response")?;

        let has_next = obj
            .get("hasNextPage")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let next_cursor = obj
            .get("cursor")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok((users, next_cursor, has_next))
    }

    /// Retrieve a user by ID or email
    pub async fn get_user(
        &self,
        id: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<CannyUserFull>> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(i) = id {
            body["id"] = json!(i);
        }
        if let Some(e) = email {
            body["email"] = json!(e);
        }

        let response = self
            .client
            .post(format!("{}/users/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        // The API returns the user object directly, or an error
        let result: Option<CannyUserFull> = serde_json::from_str(&text).ok();
        Ok(result)
    }

    /// Create or update a user
    pub async fn create_or_update_user(
        &self,
        user_id: &str,
        email: &str,
        id: Option<&str>,
        name: Option<&str>,
        avatar_url: Option<&str>,
        created: Option<&str>,
        company_id: Option<&str>,
        custom_fields: Option<serde_json::Value>,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "userID": user_id,
            "email": email,
        });

        if let Some(i) = id {
            body["id"] = json!(i);
        }
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(a) = avatar_url {
            body["avatarURL"] = json!(a);
        }
        if let Some(c) = created {
            body["created"] = json!(c);
        }
        if let Some(cid) = company_id {
            body["companyID"] = json!(cid);
        }
        if let Some(cf) = custom_fields {
            body["customFields"] = cf;
        }

        let response = self
            .client
            .post(format!("{}/users/create_or_update", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Delete a user by ID
    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "userID": user_id,
        });

        let response = self
            .client
            .post(format!("{}/users/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Find a user by ID, email, or name
    pub async fn find_user(
        &self,
        user_id: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<Option<CannyUserFull>> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(id) = user_id {
            body["userID"] = json!(id);
        }
        if let Some(e) = email {
            body["email"] = json!(e);
        }
        if let Some(n) = name {
            body["name"] = json!(n);
        }

        let response = self
            .client
            .post(format!("{}/users/find", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: UserFindResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.user)
    }

    /// Remove a user from a company
    pub async fn remove_user_from_company(&self, user_id: &str, company_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "userID": user_id,
            "companyID": company_id,
        });

        let response = self
            .client
            .post(format!("{}/users/remove_from_company", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List all boards
    pub async fn list_boards(&self) -> Result<Vec<CannyBoard>> {
        let body = json!({
            "apiKey": self.api_key,
        });

        let response = self
            .client
            .post(format!("{}/boards/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let value: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse response as JSON")?;

        let boards_value = value.get("boards").cloned().unwrap_or(json!([]));

        let boards: Vec<CannyBoard> =
            serde_json::from_value(boards_value).context("Failed to parse boards from response")?;

        Ok(boards)
    }

    /// Retrieve a single board by ID
    pub async fn get_board(&self, board_id: &str) -> Result<Option<CannyBoard>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": board_id,
        });

        let response = self
            .client
            .post(format!("{}/boards/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: BoardRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.board)
    }

    /// Create a new board
    pub async fn create_board(&self, name: &str) -> Result<String> {
        let body = json!({
            "apiKey": self.api_key,
            "name": name,
        });

        let response = self
            .client
            .post(format!("{}/boards/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Delete a board by ID
    pub async fn delete_board(&self, board_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "id": board_id,
        });

        let response = self
            .client
            .post(format!("{}/boards/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List tags for a board
    pub async fn list_tags(
        &self,
        board_id: &str,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<TagsListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/tags/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single tag by ID
    pub async fn get_tag(&self, tag_id: &str) -> Result<Option<CannyTag>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": tag_id,
        });

        let response = self
            .client
            .post(format!("{}/tags/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: TagRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.tag)
    }

    /// Create a new tag
    pub async fn create_tag(&self, board_id: &str, name: &str) -> Result<String> {
        let body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
            "name": name,
        });

        let response = self
            .client
            .post(format!("{}/tags/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Delete a tag
    pub async fn delete_tag(&self, tag_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "tagID": tag_id,
        });

        let response = self
            .client
            .post(format!("{}/tags/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List companies using v2 API with cursor-based pagination
    pub async fn list_companies(
        &self,
        limit: Option<u32>,
        cursor: Option<&str>,
        search: Option<&str>,
        segment: Option<&str>,
    ) -> Result<CompaniesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(c) = cursor {
            body["cursor"] = json!(c);
        }
        if let Some(s) = search {
            body["search"] = json!(s);
        }
        if let Some(seg) = segment {
            body["segment"] = json!(seg);
        }

        // Companies endpoint uses v2 API
        let base_url = self.api_url.replace("/v1", "/v2");
        let response = self
            .client
            .post(format!("{}/companies/list", base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let value: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse response as JSON")?;

        let obj = value
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Expected object response"))?;

        // v2 API returns "items" array
        let companies_value = obj
            .get("items")
            .or_else(|| obj.get("companies"))
            .cloned()
            .unwrap_or(json!([]));

        let companies: Vec<CannyCompany> = serde_json::from_value(companies_value)
            .context("Failed to parse companies from response")?;

        let has_next_page = obj.get("hasNextPage").and_then(|v| v.as_bool());

        let cursor = obj
            .get("cursor")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(CompaniesListResponse {
            has_next_page,
            cursor,
            companies,
        })
    }

    /// Update a company
    pub async fn update_company(
        &self,
        company_id: &str,
        name: Option<&str>,
        monthly_spend: Option<f64>,
        custom_fields: Option<serde_json::Value>,
        created: Option<&str>,
    ) -> Result<()> {
        let mut body = json!({
            "apiKey": self.api_key,
            "id": company_id,
        });

        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(ms) = monthly_spend {
            body["monthlySpend"] = json!(ms);
        }
        if let Some(cf) = custom_fields {
            body["customFields"] = cf;
        }
        if let Some(c) = created {
            body["created"] = json!(c);
        }

        let response = self
            .client
            .post(format!("{}/companies/update", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Delete a company by ID
    pub async fn delete_company(&self, company_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "id": company_id,
        });

        let response = self
            .client
            .post(format!("{}/companies/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Retrieve a single company by ID
    pub async fn get_company(&self, company_id: &str) -> Result<Option<CannyCompany>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": company_id,
        });

        let response = self
            .client
            .post(format!("{}/companies/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CompanyRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.company)
    }

    /// List votes for a post or user
    pub async fn list_votes(
        &self,
        post_id: Option<&str>,
        user_id: Option<&str>,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<VotesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(p) = post_id {
            body["postID"] = json!(p);
        }
        if let Some(u) = user_id {
            body["userID"] = json!(u);
        }
        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/votes/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single vote by ID
    pub async fn get_vote(&self, vote_id: &str) -> Result<Option<CannyVote>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": vote_id,
        });

        let response = self
            .client
            .post(format!("{}/votes/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: VoteRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.vote)
    }

    /// Create a vote on a post
    pub async fn create_vote(&self, post_id: &str, user_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
            "userID": user_id,
        });

        let response = self
            .client
            .post(format!("{}/votes/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Delete a vote by ID
    pub async fn delete_vote(&self, vote_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "voteID": vote_id,
        });

        let response = self
            .client
            .post(format!("{}/votes/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List status changes for a board
    pub async fn list_status_changes(
        &self,
        board_id: &str,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<StatusChangesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
            "boardID": board_id,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/status_changes/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// List changelog entries
    pub async fn list_entries(
        &self,
        limit: Option<u32>,
        skip: Option<u32>,
        entry_type: Option<&str>,
        label_ids: Option<Vec<&str>>,
        sort: Option<&str>,
    ) -> Result<EntriesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }
        if let Some(t) = entry_type {
            body["type"] = json!(t);
        }
        if let Some(ids) = label_ids {
            body["labelIDs"] = json!(ids);
        }
        if let Some(s) = sort {
            body["sort"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/entries/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single changelog entry by ID
    pub async fn get_entry(&self, entry_id: &str) -> Result<Option<CannyEntry>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": entry_id,
        });

        let response = self
            .client
            .post(format!("{}/entries/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: EntryRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.entry)
    }

    /// Delete a changelog entry by ID
    pub async fn delete_entry(&self, entry_id: &str) -> Result<()> {
        let body = json!({
            "apiKey": self.api_key,
            "entryID": entry_id,
        });

        let response = self
            .client
            .post(format!("{}/entries/delete", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Create a changelog entry
    pub async fn create_entry(
        &self,
        title: &str,
        details: Option<&str>,
        entry_type: Option<&str>,
        published: Option<bool>,
        notify: Option<bool>,
        post_ids: Option<Vec<&str>>,
        label_ids: Option<Vec<&str>>,
        published_on: Option<&str>,
        scheduled_for: Option<&str>,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "title": title,
        });

        if let Some(d) = details {
            body["details"] = json!(d);
        }
        if let Some(t) = entry_type {
            body["type"] = json!(t);
        }
        if let Some(p) = published {
            body["published"] = json!(p);
        }
        if let Some(n) = notify {
            body["notify"] = json!(n);
        }
        if let Some(ids) = post_ids {
            body["postIDs"] = json!(ids);
        }
        if let Some(ids) = label_ids {
            body["labelIDs"] = json!(ids);
        }
        if let Some(p) = published_on {
            body["publishedOn"] = json!(p);
        }
        if let Some(s) = scheduled_for {
            body["scheduledFor"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/entries/create", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: CreateResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }

    /// Update a changelog entry
    pub async fn update_entry(
        &self,
        entry_id: &str,
        title: Option<&str>,
        details: Option<&str>,
        entry_type: Option<&str>,
        published: Option<bool>,
        notify: Option<bool>,
        label_ids: Option<Vec<&str>>,
    ) -> Result<()> {
        let mut body = json!({
            "apiKey": self.api_key,
            "entryID": entry_id,
        });

        if let Some(t) = title {
            body["title"] = json!(t);
        }
        if let Some(d) = details {
            body["details"] = json!(d);
        }
        if let Some(t) = entry_type {
            body["type"] = json!(t);
        }
        if let Some(p) = published {
            body["published"] = json!(p);
        }
        if let Some(n) = notify {
            body["notify"] = json!(n);
        }
        if let Some(ids) = label_ids {
            body["labelIDs"] = json!(ids);
        }

        let response = self
            .client
            .post(format!("{}/entries/update", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// List opportunities for a post
    pub async fn list_opportunities(
        &self,
        post_id: &str,
        limit: Option<u32>,
        skip: Option<u32>,
    ) -> Result<OpportunitiesListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
            "postID": post_id,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(s) = skip {
            body["skip"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/opportunities/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// List groups
    pub async fn list_groups(
        &self,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GroupsListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(c) = cursor {
            body["cursor"] = json!(c);
        }

        let response = self
            .client
            .post(format!("{}/groups/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single group by ID or URL name
    pub async fn get_group(
        &self,
        group_id: Option<&str>,
        url_name: Option<&str>,
    ) -> Result<Option<CannyGroup>> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(id) = group_id {
            body["id"] = json!(id);
        }
        if let Some(name) = url_name {
            body["urlName"] = json!(name);
        }

        let response = self
            .client
            .post(format!("{}/groups/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: GroupRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.group)
    }

    /// List insights
    pub async fn list_insights(
        &self,
        limit: Option<u32>,
        cursor: Option<&str>,
        idea_id: Option<&str>,
    ) -> Result<InsightsListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(c) = cursor {
            body["cursor"] = json!(c);
        }
        if let Some(i) = idea_id {
            body["ideaID"] = json!(i);
        }

        let response = self
            .client
            .post(format!("{}/insights/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single insight by ID
    pub async fn get_insight(&self, insight_id: &str) -> Result<Option<CannyInsight>> {
        let body = json!({
            "apiKey": self.api_key,
            "id": insight_id,
        });

        let response = self
            .client
            .post(format!("{}/insights/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: InsightRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.insight)
    }

    /// List ideas
    pub async fn list_ideas(
        &self,
        limit: Option<u32>,
        cursor: Option<&str>,
        parent_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<IdeasListResponse> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(l) = limit {
            body["limit"] = json!(l);
        }
        if let Some(c) = cursor {
            body["cursor"] = json!(c);
        }
        if let Some(p) = parent_id {
            body["parentID"] = json!(p);
        }
        if let Some(s) = search {
            body["search"] = json!(s);
        }

        let response = self
            .client
            .post(format!("{}/ideas/list", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response")
    }

    /// Retrieve a single idea by ID or URL name
    pub async fn get_idea(
        &self,
        idea_id: Option<&str>,
        url_name: Option<&str>,
    ) -> Result<Option<CannyIdea>> {
        let mut body = json!({
            "apiKey": self.api_key,
        });

        if let Some(id) = idea_id {
            body["id"] = json!(id);
        }
        if let Some(name) = url_name {
            body["urlName"] = json!(name);
        }

        let response = self
            .client
            .post(format!("{}/ideas/retrieve", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: IdeaRetrieveResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.idea)
    }

    /// Enqueue feedback for autopilot processing
    pub async fn enqueue_autopilot_feedback(
        &self,
        feedback: &str,
        user_id: &str,
        source_url: Option<&str>,
    ) -> Result<String> {
        let mut body = json!({
            "apiKey": self.api_key,
            "feedback": feedback,
            "userID": user_id,
        });

        if let Some(url) = source_url {
            body["sourceURL"] = json!(url);
        }

        let response = self
            .client
            .post(format!("{}/autopilot/enqueue", self.api_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let result: AutopilotEnqueueResponse =
            serde_json::from_str(&text).context("Failed to parse response")?;
        Ok(result.id)
    }
}
