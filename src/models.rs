use serde::{Deserialize, Serialize};

/// Represents a Canny company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyCompany {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub monthly_spend: Option<f64>,
    #[serde(default)]
    pub user_count: Option<i32>,
    #[serde(default)]
    pub custom_fields: Option<serde_json::Value>,
}

/// Response from companies/list endpoint (v2 API)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompaniesListResponse {
    #[serde(default)]
    pub has_next_page: Option<bool>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub companies: Vec<CannyCompany>,
}

/// Response from companies/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct CompanyRetrieveResponse {
    pub company: Option<CannyCompany>,
}

/// Represents a Canny user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyUser {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// Represents a Canny category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyCategory {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub post_count: Option<i32>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Represents a Canny post
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyPost {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub details: Option<String>,
    pub url: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub comment_count: i32,
    #[serde(default)]
    pub score: i32,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub author: Option<CannyUser>,
    #[serde(default)]
    pub category: Option<CannyCategory>,
}

/// Represents a Canny comment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyComment {
    pub id: String,
    pub value: String,
    pub created: String,
    #[serde(default)]
    pub author: Option<CannyUser>,
    #[serde(default)]
    pub post: Option<CannyPost>,
    #[serde(default, rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub pinned: Option<bool>,
}

/// Response from posts/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsListResponse {
    pub has_more: bool,
    pub posts: Vec<CannyPost>,
}

/// Response from posts/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct PostRetrieveResponse {
    pub post: Option<CannyPost>,
}

/// Response from comments/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsListResponse {
    pub has_more: bool,
    pub comments: Vec<CannyComment>,
}

/// Response from comments/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct CommentRetrieveResponse {
    pub comment: Option<CannyComment>,
}

/// Response from categories/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CategoriesListResponse {
    pub has_more: bool,
    pub categories: Vec<CannyCategory>,
}

/// Response from categories/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct CategoryRetrieveResponse {
    pub category: Option<CannyCategory>,
}

/// Represents a Canny board
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyBoard {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub post_count: Option<i32>,
    #[serde(default)]
    pub is_private: Option<bool>,
    #[serde(default)]
    pub private_comments: Option<bool>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
}

/// Full user details returned by users/retrieve and users/list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyUserFull {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub is_admin: Option<bool>,
    #[serde(default)]
    pub last_activity: Option<String>,
    #[serde(default, rename = "userID")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Response from users/find endpoint
#[derive(Debug, Deserialize)]
pub struct UserFindResponse {
    pub user: Option<CannyUserFull>,
}

/// Response from boards/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct BoardRetrieveResponse {
    pub board: Option<CannyBoard>,
}

/// Response from create operations
#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub id: String,
}

/// Sort options for posts
#[derive(Debug, Clone, clap::ValueEnum, Default)]
pub enum PostSort {
    #[default]
    Newest,
    Oldest,
    Relevance,
    Score,
    StatusChanged,
    Trending,
}

impl std::fmt::Display for PostSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostSort::Newest => write!(f, "newest"),
            PostSort::Oldest => write!(f, "oldest"),
            PostSort::Relevance => write!(f, "relevance"),
            PostSort::Score => write!(f, "score"),
            PostSort::StatusChanged => write!(f, "statusChanged"),
            PostSort::Trending => write!(f, "trending"),
        }
    }
}

/// Represents a Canny tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyTag {
    pub id: String,
    pub name: String,
    #[serde(default, rename = "boardID")]
    pub board_id: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub post_count: Option<i32>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Response from tags/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsListResponse {
    pub has_more: bool,
    pub tags: Vec<CannyTag>,
}

/// Response from tags/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct TagRetrieveResponse {
    pub tag: Option<CannyTag>,
}

/// Represents a Canny vote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyVote {
    pub id: String,
    #[serde(default, rename = "postID")]
    pub post_id: Option<String>,
    #[serde(default)]
    pub voter: Option<CannyUser>,
    #[serde(default)]
    pub created: Option<String>,
}

/// Response from votes/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotesListResponse {
    pub has_more: bool,
    pub votes: Vec<CannyVote>,
}

/// Response from votes/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct VoteRetrieveResponse {
    pub vote: Option<CannyVote>,
}

/// Represents a Canny status change
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyStatusChange {
    pub id: String,
    #[serde(default, rename = "postID")]
    pub post_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub changer: Option<CannyUser>,
}

/// Response from status_changes/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusChangesListResponse {
    pub has_more: bool,
    pub status_changes: Vec<CannyStatusChange>,
}

/// Represents a Canny changelog entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyEntry {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "type")]
    pub entry_type: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Response from entries/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntriesListResponse {
    pub has_more: bool,
    pub entries: Vec<CannyEntry>,
}

/// Response from entries/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct EntryRetrieveResponse {
    pub entry: Option<CannyEntry>,
}

/// Represents a Canny opportunity (linked sales/deal info)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyOpportunity {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "opportunityID")]
    pub opportunity_id: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub won: Option<bool>,
    #[serde(default)]
    pub closed: Option<bool>,
    #[serde(default, rename = "salesforceOpportunityID")]
    pub salesforce_opportunity_id: Option<String>,
}

/// Response from opportunities/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpportunitiesListResponse {
    pub has_more: bool,
    pub opportunities: Vec<CannyOpportunity>,
}

/// Represents a Canny group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyGroup {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub member_count: Option<i32>,
}

/// Response from groups/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsListResponse {
    pub has_more: bool,
    #[serde(default)]
    pub cursor: Option<String>,
    pub groups: Vec<CannyGroup>,
}

/// Response from groups/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct GroupRetrieveResponse {
    pub group: Option<CannyGroup>,
}

/// Represents a Canny idea
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyIdea {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub post_count: Option<i32>,
}

/// Response from ideas/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeasListResponse {
    pub has_more: bool,
    #[serde(default)]
    pub cursor: Option<String>,
    pub ideas: Vec<CannyIdea>,
}

/// Response from ideas/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct IdeaRetrieveResponse {
    pub idea: Option<CannyIdea>,
}

/// Represents a Canny insight
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CannyInsight {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
}

/// Response from insights/list endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsightsListResponse {
    pub has_more: bool,
    #[serde(default)]
    pub cursor: Option<String>,
    pub insights: Vec<CannyInsight>,
}

/// Response from insights/retrieve endpoint
#[derive(Debug, Deserialize)]
pub struct InsightRetrieveResponse {
    pub insight: Option<CannyInsight>,
}

/// Response from autopilot/enqueue endpoint
#[derive(Debug, Deserialize)]
pub struct AutopilotEnqueueResponse {
    pub id: String,
}
