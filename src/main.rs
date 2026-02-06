mod api;
mod credentials;
mod models;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;

use api::{CannyClient, DEFAULT_API_URL};
use models::PostSort;

/// A CLI tool for interacting with the Canny API
///
/// Canny is a feedback management platform. This CLI allows you to manage
/// posts, comments, and categories from the command line.
///
/// AUTHENTICATION (in order of precedence):
///   1. --api-key flag / CANNY_API_KEY environment variable
///   2. macOS Keychain (configured via `canny auth`)
///
/// Get your API key from: https://canny.io/api-keys
///
/// EXAMPLES:
///   # Authenticate (stores API key and URL in Keychain)
///   canny auth
///
///   # List posts from a board
///   canny posts list --board-id abc123
///
///   # Create a new post
///   canny posts create --board-id abc123 --author-id user456 --title "Feature request"
///
///   # View comments on a post
///   canny comments list --post-id post789
#[derive(Parser)]
#[command(name = "canny")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Canny API key (defaults to CANNY_API_KEY env var, then Keychain)
    #[arg(long, env = "CANNY_API_KEY", global = true, hide_env_values = true)]
    api_key: Option<String>,

    /// Override the Canny API URL
    #[arg(long, global = true)]
    api_url: Option<String>,

    /// Output as JSON instead of formatted text
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage posts (feature requests, bug reports, etc.)
    ///
    /// Posts are the core content in Canny - they represent feature requests,
    /// bug reports, or any other feedback from users.
    #[command(subcommand)]
    Posts(PostsCommands),

    /// Manage comments on posts
    ///
    /// Comments allow discussion on posts. They can be top-level or replies
    /// to other comments.
    #[command(subcommand)]
    Comments(CommentsCommands),

    /// Manage categories
    ///
    /// Categories help organize posts within a board.
    #[command(subcommand)]
    Categories(CategoriesCommands),

    /// Manage users
    ///
    /// List and retrieve user information. Useful for finding your user ID
    /// which is required for creating posts and comments.
    #[command(subcommand)]
    Users(UsersCommands),

    /// Manage boards
    ///
    /// List boards in your Canny account. Useful for finding board IDs
    /// which are required for listing posts and categories.
    #[command(subcommand)]
    Boards(BoardsCommands),

    /// Manage tags
    ///
    /// Tags help organize and label posts within a board.
    #[command(subcommand)]
    Tags(TagsCommands),

    /// Manage companies
    ///
    /// Companies represent organizations associated with users. Useful for
    /// tracking feedback by customer company.
    #[command(subcommand)]
    Companies(CompaniesCommands),

    /// Manage votes on posts
    ///
    /// Votes represent user support for posts. Users can vote on posts
    /// to indicate their interest in a feature request or bug fix.
    #[command(subcommand)]
    Votes(VotesCommands),

    /// View status changes for posts
    ///
    /// Status changes track when a post's status was changed (e.g., from
    /// "open" to "planned" or "in progress" to "complete").
    #[command(subcommand)]
    StatusChanges(StatusChangesCommands),

    /// Manage changelog entries
    ///
    /// Changelog entries are announcements about new features, improvements,
    /// or fixes that you want to share with your users.
    #[command(subcommand)]
    Changelog(ChangelogCommands),

    /// List opportunities linked to posts
    ///
    /// Opportunities represent linked sales/deal information from CRMs like
    /// Salesforce that are associated with posts.
    #[command(subcommand)]
    Opportunities(OpportunitiesCommands),

    /// Manage groups
    ///
    /// Groups represent collections of users. Useful for organizing users
    /// and tracking feedback by user groups.
    #[command(subcommand)]
    Groups(GroupsCommands),

    /// Manage insights
    ///
    /// Insights are AI-generated summaries and analysis of user feedback.
    #[command(subcommand)]
    Insights(InsightsCommands),

    /// Manage ideas
    ///
    /// Ideas represent high-level concepts or themes that group related posts together.
    #[command(subcommand)]
    Ideas(IdeasCommands),

    /// Autopilot feedback processing
    ///
    /// Enqueue feedback for autopilot AI processing. Autopilot automatically
    /// categorizes and processes user feedback.
    #[command(subcommand)]
    Autopilot(AutopilotCommands),

    /// Authenticate with the Canny API
    ///
    /// If already authenticated, shows your current credentials and verifies
    /// them. Use --reset to clear stored credentials and re-authenticate.
    ///
    /// EXAMPLES:
    ///   canny auth
    ///   canny auth --reset
    Auth {
        /// Clear stored credentials and re-authenticate
        #[arg(long)]
        reset: bool,
    },
}

#[derive(Subcommand)]
enum PostsCommands {
    /// List posts from a board
    ///
    /// Retrieves posts from the specified board with optional filtering and sorting.
    ///
    /// EXAMPLES:
    ///   # List newest posts
    ///   canny posts list --board-id abc123
    ///
    ///   # List open posts sorted by score
    ///   canny posts list --board-id abc123 --status open --sort score
    ///
    ///   # Filter by multiple statuses
    ///   canny posts list --board-id abc123 --status open --status planned
    ///
    ///   # Filter by author
    ///   canny posts list --board-id abc123 --author-id user456
    ///
    ///   # Search for posts
    ///   canny posts list --board-id abc123 --search "dark mode"
    List {
        /// The ID of the board to list posts from
        #[arg(long)]
        board_id: String,

        /// Maximum number of posts to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of posts to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,

        /// Sort order for posts
        #[arg(long, value_enum, default_value = "newest")]
        sort: PostSort,

        /// Filter by status (can be specified multiple times)
        #[arg(long)]
        status: Vec<String>,

        /// Filter by author ID
        #[arg(long)]
        author_id: Option<String>,

        /// Search posts by title and content
        #[arg(long)]
        search: Option<String>,

        /// Filter by company ID
        #[arg(long)]
        company_id: Option<String>,

        /// Filter by tag IDs (can be specified multiple times)
        #[arg(long = "tag-id")]
        tag_ids: Vec<String>,
    },

    /// Retrieve a single post by ID or URL name
    ///
    /// Gets detailed information about a specific post including author,
    /// category, vote count, and more. You can retrieve by ID or by URL name
    /// (which requires the board ID).
    ///
    /// EXAMPLES:
    ///   # Get by ID
    ///   canny posts get --id post123
    ///
    ///   # Get by URL name (requires board ID)
    ///   canny posts get --url-name my-feature-request --board-id abc123
    Get {
        /// The ID of the post to retrieve
        #[arg(long)]
        id: Option<String>,

        /// The URL name of the post (requires --board-id)
        #[arg(long)]
        url_name: Option<String>,

        /// The board ID (required when using --url-name)
        #[arg(long)]
        board_id: Option<String>,
    },

    /// Create a new post
    ///
    /// Creates a new post on the specified board. The author must be a valid
    /// user ID in your Canny account.
    ///
    /// EXAMPLES:
    ///   # Create a simple post
    ///   canny posts create --board-id abc123 --author-id user456 --title "Add dark mode"
    ///
    ///   # Create a post with details and category
    ///   canny posts create --board-id abc123 --author-id user456 \
    ///     --title "Add dark mode" \
    ///     --details "It would be great to have a dark theme option" \
    ///     --category-id cat789
    Create {
        /// The ID of the board to create the post on
        #[arg(long)]
        board_id: String,

        /// The ID of the user creating the post
        #[arg(long)]
        author_id: String,

        /// Title of the post
        #[arg(long)]
        title: String,

        /// Detailed description of the post (supports markdown)
        #[arg(long)]
        details: Option<String>,

        /// Category ID to assign to the post
        #[arg(long)]
        category_id: Option<String>,

        /// The user ID performing the action (if different from author)
        #[arg(long)]
        by_id: Option<String>,

        /// Custom fields as JSON object
        #[arg(long)]
        custom_fields: Option<String>,

        /// Estimated time of arrival/completion (ISO 8601 format)
        #[arg(long)]
        eta: Option<String>,

        /// Whether the ETA should be visible to voters
        #[arg(long)]
        eta_public: Option<bool>,

        /// The ID of the admin user who owns the post
        #[arg(long)]
        owner_id: Option<String>,

        /// Image URLs to attach to the post (can be specified multiple times)
        #[arg(long = "image-url")]
        image_urls: Vec<String>,

        /// Post creation timestamp (ISO 8601 format, for imports)
        #[arg(long)]
        created_at: Option<String>,
    },

    /// Change the status of a post
    ///
    /// Updates the status of a post. Optionally notify voters of the change
    /// and add a comment explaining the status change.
    ///
    /// EXAMPLES:
    ///   # Mark a post as planned
    ///   canny posts status --id post123 --changer-id user456 --status planned
    ///
    ///   # Complete a post with a comment, notifying voters
    ///   canny posts status --id post123 --changer-id user456 --status complete \
    ///     --notify --comment "This feature is now live!"
    Status {
        /// The ID of the post to update
        #[arg(long)]
        id: String,

        /// The ID of the user making the change
        #[arg(long)]
        changer_id: String,

        /// New status for the post
        #[arg(long)]
        status: String,

        /// Notify voters about the status change
        #[arg(long, default_value = "false")]
        notify: bool,

        /// Add a comment when changing status
        #[arg(long)]
        comment: Option<String>,

        /// Image URLs to attach to the comment (can be specified multiple times)
        #[arg(long = "comment-image-url")]
        comment_image_urls: Vec<String>,
    },

    /// Change the category of a post
    ///
    /// Moves a post to a different category within the same board.
    ///
    /// EXAMPLES:
    ///   canny posts category --id post123 --category-id cat456
    Category {
        /// The ID of the post to update
        #[arg(long)]
        id: String,

        /// The ID of the new category
        #[arg(long)]
        category_id: String,
    },

    /// Update a post
    ///
    /// Updates the title and/or details of an existing post.
    ///
    /// EXAMPLES:
    ///   # Update the title
    ///   canny posts update --id post123 --title "New title"
    ///
    ///   # Update the details
    ///   canny posts update --id post123 --details "Updated description"
    ///
    ///   # Update both title and details
    ///   canny posts update --id post123 --title "New title" --details "Updated description"
    Update {
        /// The ID of the post to update
        #[arg(long)]
        id: String,

        /// New title for the post
        #[arg(long)]
        title: Option<String>,

        /// New details for the post (supports markdown)
        #[arg(long)]
        details: Option<String>,

        /// Estimated time of arrival/completion (e.g., "2024-03", "Q2 2024")
        #[arg(long)]
        eta: Option<String>,

        /// Whether the ETA should be visible to voters
        #[arg(long)]
        eta_public: Option<bool>,

        /// Custom fields as JSON object (e.g., '{"priority": "high"}')
        #[arg(long)]
        custom_fields: Option<String>,
    },

    /// Delete a post
    ///
    /// Permanently deletes a post by ID.
    ///
    /// EXAMPLES:
    ///   canny posts delete --id post123
    Delete {
        /// The ID of the post to delete
        #[arg(long)]
        id: String,
    },

    /// Add a tag to a post
    ///
    /// Associates a tag with the specified post.
    ///
    /// EXAMPLES:
    ///   canny posts add-tag --id post123 --tag-id tag456
    AddTag {
        /// The ID of the post to add the tag to
        #[arg(long)]
        id: String,

        /// The ID of the tag to add
        #[arg(long)]
        tag_id: String,
    },

    /// Remove a tag from a post
    ///
    /// Removes a tag association from the specified post.
    ///
    /// EXAMPLES:
    ///   canny posts remove-tag --id post123 --tag-id tag456
    RemoveTag {
        /// The ID of the post to remove the tag from
        #[arg(long)]
        id: String,

        /// The ID of the tag to remove
        #[arg(long)]
        tag_id: String,
    },

    /// Link a Jira issue to a post
    ///
    /// Associates a Jira issue with the specified post.
    ///
    /// EXAMPLES:
    ///   canny posts link-jira --id post123 --issue-key PROJ-123
    LinkJira {
        /// The ID of the post to link
        #[arg(long)]
        id: String,

        /// The Jira issue key (e.g., PROJ-123)
        #[arg(long)]
        issue_key: String,
    },

    /// Unlink a Jira issue from a post
    ///
    /// Removes a Jira issue association from the specified post.
    ///
    /// EXAMPLES:
    ///   canny posts unlink-jira --id post123 --issue-key PROJ-123
    UnlinkJira {
        /// The ID of the post to unlink
        #[arg(long)]
        id: String,

        /// The Jira issue key (e.g., PROJ-123)
        #[arg(long)]
        issue_key: String,
    },
}

#[derive(Subcommand)]
enum CommentsCommands {
    /// List comments on a post
    ///
    /// Retrieves all comments for a given post, including replies.
    ///
    /// EXAMPLES:
    ///   # List comments on a post
    ///   canny comments list --post-id post123
    ///
    ///   # List comments by author
    ///   canny comments list --author-id user456
    ///
    ///   # List comments for a board
    ///   canny comments list --board-id board789
    ///
    ///   # List with pagination
    ///   canny comments list --post-id post123 --limit 50 --skip 100
    List {
        /// The ID of the post to list comments from (optional)
        #[arg(long)]
        post_id: Option<String>,

        /// Filter by author ID
        #[arg(long)]
        author_id: Option<String>,

        /// Filter by board ID
        #[arg(long)]
        board_id: Option<String>,

        /// Filter by company ID
        #[arg(long)]
        company_id: Option<String>,

        /// Maximum number of comments to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of comments to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },

    /// Create a comment on a post
    ///
    /// Adds a new comment to a post. Can optionally be a reply to another comment.
    ///
    /// EXAMPLES:
    ///   # Add a top-level comment
    ///   canny comments create --post-id post123 --author-id user456 \
    ///     --value "Great idea! We should prioritize this."
    ///
    ///   # Reply to another comment
    ///   canny comments create --post-id post123 --author-id user456 \
    ///     --value "I agree!" --parent-id comment789
    ///
    ///   # Create an internal comment
    ///   canny comments create --post-id post123 --author-id user456 \
    ///     --value "Internal note" --internal
    ///
    ///   # Create a comment with images
    ///   canny comments create --post-id post123 --author-id user456 \
    ///     --value "See attached" --image-url "https://example.com/img1.png"
    Create {
        /// The ID of the post to comment on
        #[arg(long)]
        post_id: String,

        /// The ID of the user creating the comment
        #[arg(long)]
        author_id: String,

        /// The comment text (supports markdown)
        #[arg(long)]
        value: String,

        /// Parent comment ID if this is a reply
        #[arg(long)]
        parent_id: Option<String>,

        /// Creation timestamp (ISO 8601 format)
        #[arg(long)]
        created_at: Option<String>,

        /// Image URLs to attach to the comment (can be specified multiple times)
        #[arg(long = "image-url")]
        image_urls: Vec<String>,

        /// Mark the comment as internal (only visible to admins)
        #[arg(long)]
        internal: bool,

        /// Notify voters about this comment
        #[arg(long)]
        notify_voters: bool,
    },

    /// Retrieve a single comment by ID
    ///
    /// Gets detailed information about a specific comment including author,
    /// creation date, and content.
    ///
    /// EXAMPLES:
    ///   canny comments get --id comment123
    Get {
        /// The ID of the comment to retrieve
        #[arg(long)]
        id: String,
    },

    /// Delete a comment
    ///
    /// Permanently deletes a comment by ID.
    ///
    /// EXAMPLES:
    ///   canny comments delete --id comment123
    Delete {
        /// The ID of the comment to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum CategoriesCommands {
    /// List categories for a board
    ///
    /// Retrieves all categories defined for a board.
    ///
    /// EXAMPLES:
    ///   canny categories list --board-id abc123
    List {
        /// The ID of the board to list categories from
        #[arg(long)]
        board_id: String,

        /// Maximum number of categories to return (default: 100)
        #[arg(long, default_value = "100")]
        limit: u32,

        /// Number of categories to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },

    /// Retrieve a single category by ID
    ///
    /// Gets detailed information about a specific category.
    ///
    /// EXAMPLES:
    ///   canny categories get --id cat123
    Get {
        /// The ID of the category to retrieve
        #[arg(long)]
        id: String,
    },

    /// Create a new category
    ///
    /// Creates a new category on the specified board. Optionally, you can
    /// specify a parent category to create a subcategory.
    ///
    /// EXAMPLES:
    ///   # Create a top-level category
    ///   canny categories create --board-id abc123 --name "Feature Requests"
    ///
    ///   # Create a subcategory
    ///   canny categories create --board-id abc123 --name "UI Improvements" --parent-id cat456
    Create {
        /// The ID of the board to create the category on
        #[arg(long)]
        board_id: String,

        /// Name of the category
        #[arg(long)]
        name: String,

        /// Parent category ID to create a subcategory
        #[arg(long)]
        parent_id: Option<String>,

        /// Subscribe admins to the category
        #[arg(long, default_value = "true")]
        subscribe_admins: bool,
    },

    /// Delete a category
    ///
    /// Permanently deletes a category by ID.
    ///
    /// EXAMPLES:
    ///   canny categories delete --id cat123
    Delete {
        /// The ID of the category to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum UsersCommands {
    /// List all users
    ///
    /// Retrieves all users in your Canny account (automatically fetches all pages).
    /// Use this to find user IDs for creating posts and comments.
    ///
    /// EXAMPLES:
    ///   canny users list
    ///   canny users list --json
    List,

    /// Retrieve a user by ID or email
    ///
    /// Gets detailed information about a specific user.
    ///
    /// EXAMPLES:
    ///   # Get user by ID
    ///   canny users get --id user123
    ///
    ///   # Get user by email
    ///   canny users get --email user@example.com
    Get {
        /// The ID of the user to retrieve
        #[arg(long)]
        id: Option<String>,

        /// The email of the user to retrieve
        #[arg(long)]
        email: Option<String>,
    },

    /// Create or update a user
    ///
    /// Creates a new user or updates an existing one. The user ID is a unique
    /// identifier in your system that you provide.
    ///
    /// EXAMPLES:
    ///   # Create a user with email and name
    ///   canny users create --user-id user123 --email user@example.com --name "John Doe"
    ///
    ///   # Create a user with avatar
    ///   canny users create --user-id user123 --email user@example.com --name "John Doe" \
    ///     --avatar-url "https://example.com/avatar.png"
    ///
    ///   # Create a user with custom fields
    ///   canny users create --user-id user123 --email user@example.com --name "John Doe" \
    ///     --custom-fields '{"plan": "enterprise", "role": "admin"}'
    Create {
        /// Unique identifier for the user in your system
        #[arg(long)]
        user_id: String,

        /// Email address of the user (required)
        #[arg(long)]
        email: String,

        /// Internal Canny ID (different from userID, used for updating existing users)
        #[arg(long)]
        id: Option<String>,

        /// Display name of the user
        #[arg(long)]
        name: Option<String>,

        /// URL to the user's avatar image
        #[arg(long)]
        avatar_url: Option<String>,

        /// Company ID to associate the user with
        #[arg(long)]
        company_id: Option<String>,

        /// Custom fields as a JSON string (e.g., '{"plan": "enterprise"}')
        #[arg(long)]
        custom_fields: Option<String>,
    },

    /// Delete a user
    ///
    /// Permanently deletes a user by their ID.
    ///
    /// EXAMPLES:
    ///   canny users delete --id user123
    Delete {
        /// The ID of the user to delete
        #[arg(long)]
        id: String,
    },

    /// Find a user by ID, email, or name
    ///
    /// Searches for a user using one or more criteria. At least one of
    /// --user-id, --email, or --name must be provided.
    ///
    /// EXAMPLES:
    ///   # Find user by ID
    ///   canny users find --user-id user123
    ///
    ///   # Find user by email
    ///   canny users find --email user@example.com
    ///
    ///   # Find user by name
    ///   canny users find --name "John Doe"
    Find {
        /// The user ID to search for
        #[arg(long)]
        user_id: Option<String>,

        /// The email to search for
        #[arg(long)]
        email: Option<String>,

        /// The name to search for
        #[arg(long)]
        name: Option<String>,
    },

    /// Remove a user from a company
    ///
    /// Removes the association between a user and a company.
    ///
    /// EXAMPLES:
    ///   canny users remove-from-company --user-id user123 --company-id company456
    RemoveFromCompany {
        /// The ID of the user to remove
        #[arg(long)]
        user_id: String,

        /// The ID of the company to remove the user from
        #[arg(long)]
        company_id: String,
    },
}

#[derive(Subcommand)]
enum BoardsCommands {
    /// List all boards
    ///
    /// Retrieves all boards in your Canny account.
    /// Use this to find board IDs for listing posts and categories.
    ///
    /// EXAMPLES:
    ///   canny boards list
    ///   canny boards list --json
    List,

    /// Retrieve a single board by ID
    ///
    /// Gets detailed information about a specific board.
    ///
    /// EXAMPLES:
    ///   canny boards get --id board123
    Get {
        /// The ID of the board to retrieve
        #[arg(long)]
        id: String,
    },

    /// Create a new board
    ///
    /// Creates a new board with the specified name.
    ///
    /// EXAMPLES:
    ///   canny boards create --name "Feature Requests"
    Create {
        /// Name of the board
        #[arg(long)]
        name: String,
    },

    /// Delete a board
    ///
    /// Permanently deletes a board by ID.
    ///
    /// EXAMPLES:
    ///   canny boards delete --id board123
    Delete {
        /// The ID of the board to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum TagsCommands {
    /// List tags for a board
    ///
    /// Retrieves all tags defined for a board.
    ///
    /// EXAMPLES:
    ///   canny tags list --board-id abc123
    List {
        /// The ID of the board to list tags from
        #[arg(long)]
        board_id: String,

        /// Maximum number of tags to return (default: 100)
        #[arg(long, default_value = "100")]
        limit: u32,

        /// Number of tags to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },

    /// Retrieve a single tag by ID
    ///
    /// Gets detailed information about a specific tag.
    ///
    /// EXAMPLES:
    ///   canny tags get --id tag123
    Get {
        /// The ID of the tag to retrieve
        #[arg(long)]
        id: String,
    },

    /// Create a new tag
    ///
    /// Creates a new tag on the specified board.
    ///
    /// EXAMPLES:
    ///   canny tags create --board-id abc123 --name "bug"
    Create {
        /// The ID of the board to create the tag on
        #[arg(long)]
        board_id: String,

        /// Name of the tag
        #[arg(long)]
        name: String,
    },

    /// Delete a tag
    ///
    /// Permanently deletes a tag by ID.
    ///
    /// EXAMPLES:
    ///   canny tags delete --id tag123
    Delete {
        /// The ID of the tag to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum CompaniesCommands {
    /// List companies
    ///
    /// Retrieves companies with optional filtering and pagination.
    /// Uses the v2 API with cursor-based pagination.
    ///
    /// EXAMPLES:
    ///   canny companies list
    ///   canny companies list --limit 50
    ///   canny companies list --search "Acme"
    ///   canny companies list --segment enterprise-customers
    List {
        /// Maximum number of companies to return (default: 100)
        #[arg(long, default_value = "100")]
        limit: u32,

        /// Cursor for pagination (from previous response)
        #[arg(long)]
        cursor: Option<String>,

        /// Search companies by name
        #[arg(long)]
        search: Option<String>,

        /// Filter by segment URL name
        #[arg(long)]
        segment: Option<String>,
    },

    /// Retrieve a single company by ID
    ///
    /// Gets detailed information about a specific company.
    ///
    /// EXAMPLES:
    ///   canny companies get --id company123
    Get {
        /// The ID of the company to retrieve
        #[arg(long)]
        id: String,
    },

    /// Update a company
    ///
    /// Updates company information including name, monthly spend, custom fields, and creation date.
    ///
    /// EXAMPLES:
    ///   # Update company name
    ///   canny companies update --id company123 --name "Acme Corp"
    ///
    ///   # Update monthly spend
    ///   canny companies update --id company123 --monthly-spend 5000.00
    ///
    ///   # Update custom fields (JSON)
    ///   canny companies update --id company123 --custom-fields '{"tier": "enterprise"}'
    ///
    ///   # Update creation date (ISO 8601)
    ///   canny companies update --id company123 --created "2023-01-15T10:30:00Z"
    Update {
        /// The ID of the company to update
        #[arg(long)]
        id: String,

        /// New name for the company
        #[arg(long)]
        name: Option<String>,

        /// Monthly spend amount for the company
        #[arg(long)]
        monthly_spend: Option<f64>,

        /// Custom fields as JSON object
        #[arg(long)]
        custom_fields: Option<String>,

        /// Company creation date (ISO 8601 format)
        #[arg(long)]
        created: Option<String>,
    },

    /// Delete a company
    ///
    /// Permanently deletes a company by ID.
    ///
    /// EXAMPLES:
    ///   canny companies delete --id company123
    Delete {
        /// The ID of the company to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum VotesCommands {
    /// List votes for a post or user
    ///
    /// Retrieves votes for a given post or user. At least one of --post-id or --user-id
    /// should be provided.
    ///
    /// EXAMPLES:
    ///   # List votes on a post
    ///   canny votes list --post-id post123
    ///
    ///   # List votes by a user
    ///   canny votes list --user-id user456
    ///
    ///   # List with pagination
    ///   canny votes list --post-id post123 --limit 50 --skip 100
    List {
        /// The ID of the post to list votes from
        #[arg(long)]
        post_id: Option<String>,

        /// The ID of the user to list votes from
        #[arg(long)]
        user_id: Option<String>,

        /// Maximum number of votes to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of votes to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },

    /// Retrieve a single vote by ID
    ///
    /// Gets detailed information about a specific vote.
    ///
    /// EXAMPLES:
    ///   canny votes get --id vote123
    Get {
        /// The ID of the vote to retrieve
        #[arg(long)]
        id: String,
    },

    /// Create a vote on a post
    ///
    /// Adds a vote from a user to a post.
    ///
    /// EXAMPLES:
    ///   canny votes create --post-id post123 --user-id user456
    Create {
        /// The ID of the post to vote on
        #[arg(long)]
        post_id: String,

        /// The ID of the user voting
        #[arg(long)]
        user_id: String,
    },

    /// Delete a vote
    ///
    /// Removes a vote by ID.
    ///
    /// EXAMPLES:
    ///   canny votes delete --id vote123
    Delete {
        /// The ID of the vote to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum StatusChangesCommands {
    /// List status changes for a board
    ///
    /// Retrieves all status changes for posts on a given board.
    ///
    /// EXAMPLES:
    ///   # List status changes on a board
    ///   canny status-changes list --board-id abc123
    ///
    ///   # List with pagination
    ///   canny status-changes list --board-id abc123 --limit 50 --skip 100
    List {
        /// The ID of the board to list status changes from
        #[arg(long)]
        board_id: String,

        /// Maximum number of status changes to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of status changes to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },
}

#[derive(Subcommand)]
enum OpportunitiesCommands {
    /// List opportunities for a post
    ///
    /// Retrieves opportunities (linked sales/deal info) for a given post.
    ///
    /// EXAMPLES:
    ///   # List opportunities on a post
    ///   canny opportunities list --post-id post123
    ///
    ///   # List with pagination
    ///   canny opportunities list --post-id post123 --limit 50 --skip 100
    List {
        /// The ID of the post to list opportunities from
        #[arg(long)]
        post_id: String,

        /// Maximum number of opportunities to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of opportunities to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,
    },
}

#[derive(Subcommand)]
enum ChangelogCommands {
    /// List changelog entries
    ///
    /// Retrieves changelog entries with optional filtering.
    ///
    /// EXAMPLES:
    ///   # List all changelog entries
    ///   canny changelog list
    ///
    ///   # List with pagination
    ///   canny changelog list --limit 50 --skip 10
    ///
    ///   # Filter by type
    ///   canny changelog list --type new
    List {
        /// Maximum number of entries to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Number of entries to skip (for pagination)
        #[arg(long, default_value = "0")]
        skip: u32,

        /// Filter by entry type (e.g., "new", "improved", "fixed")
        #[arg(long, name = "type")]
        entry_type: Option<String>,

        /// Filter by label IDs (can be specified multiple times)
        #[arg(long = "label-id")]
        label_ids: Vec<String>,

        /// Sort order (values: created, lastSaved, nonPublishedFirst, publishedAt)
        #[arg(long)]
        sort: Option<String>,
    },

    /// Create a changelog entry
    ///
    /// Creates a new changelog entry to announce features, improvements, or fixes.
    ///
    /// EXAMPLES:
    ///   # Create a simple entry
    ///   canny changelog create --title "New Feature: Dark Mode"
    ///
    ///   # Create with details and type
    ///   canny changelog create --title "New Feature: Dark Mode" \
    ///     --details "We added a dark theme option" --type new
    ///
    ///   # Create and publish with notification
    ///   canny changelog create --title "Bug Fix" --published --notify
    ///
    ///   # Create with linked posts
    ///   canny changelog create --title "New Feature" --post-id post123 --post-id post456
    Create {
        /// Title of the changelog entry
        #[arg(long)]
        title: String,

        /// Detailed description (supports markdown)
        #[arg(long)]
        details: Option<String>,

        /// Type of entry (e.g., "new", "improved", "fixed")
        #[arg(long, name = "type")]
        entry_type: Option<String>,

        /// Publish the entry immediately
        #[arg(long)]
        published: Option<bool>,

        /// Notify users about this entry
        #[arg(long)]
        notify: Option<bool>,

        /// Post IDs to link to this entry (can be specified multiple times)
        #[arg(long = "post-id")]
        post_ids: Vec<String>,

        /// Label IDs to assign to this entry (can be specified multiple times)
        #[arg(long = "label-id")]
        label_ids: Vec<String>,

        /// ISO 8601 date for past publication (e.g., "2024-01-15T10:00:00Z")
        #[arg(long)]
        published_on: Option<String>,

        /// ISO 8601 date for future scheduled publication (e.g., "2024-02-01T10:00:00Z")
        #[arg(long)]
        scheduled_for: Option<String>,
    },

    /// Retrieve a single changelog entry by ID
    ///
    /// Gets detailed information about a specific changelog entry.
    ///
    /// EXAMPLES:
    ///   canny changelog get --id entry123
    Get {
        /// The ID of the changelog entry to retrieve
        #[arg(long)]
        id: String,
    },

    /// Delete a changelog entry
    ///
    /// Permanently deletes a changelog entry by ID.
    ///
    /// EXAMPLES:
    ///   canny changelog delete --id entry123
    Delete {
        /// The ID of the changelog entry to delete
        #[arg(long)]
        id: String,
    },

    /// Update a changelog entry
    ///
    /// Updates an existing changelog entry. All fields are optional.
    ///
    /// EXAMPLES:
    ///   # Update the title
    ///   canny changelog update --id entry123 --title "New Title"
    ///
    ///   # Update details and type
    ///   canny changelog update --id entry123 --details "Updated description" --type improved
    ///
    ///   # Publish an entry
    ///   canny changelog update --id entry123 --published true
    ///
    ///   # Publish and notify users
    ///   canny changelog update --id entry123 --published true --notify true
    Update {
        /// The ID of the changelog entry to update
        #[arg(long)]
        id: String,

        /// New title for the entry
        #[arg(long)]
        title: Option<String>,

        /// New details/description (supports markdown)
        #[arg(long)]
        details: Option<String>,

        /// New type (e.g., "new", "improved", "fixed")
        #[arg(long, name = "type")]
        entry_type: Option<String>,

        /// Whether the entry should be published
        #[arg(long)]
        published: Option<bool>,

        /// Whether to notify users about this entry
        #[arg(long)]
        notify: Option<bool>,

        /// Label IDs to assign to this entry (can be specified multiple times)
        #[arg(long = "label-id")]
        label_ids: Vec<String>,
    },
}

#[derive(Subcommand)]
enum GroupsCommands {
    /// List groups
    ///
    /// Retrieves groups with optional pagination.
    ///
    /// EXAMPLES:
    ///   canny groups list
    ///   canny groups list --limit 50
    ///   canny groups list --cursor abc123
    List {
        /// Maximum number of groups to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Cursor for pagination (from previous response)
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Retrieve a single group by ID or URL name
    ///
    /// Gets detailed information about a specific group.
    ///
    /// EXAMPLES:
    ///   # Get group by ID
    ///   canny groups get --id group123
    ///
    ///   # Get group by URL name
    ///   canny groups get --url-name my-group
    Get {
        /// The ID of the group to retrieve
        #[arg(long)]
        id: Option<String>,

        /// The URL name of the group to retrieve
        #[arg(long)]
        url_name: Option<String>,
    },
}

#[derive(Subcommand)]
enum InsightsCommands {
    /// List insights
    ///
    /// Retrieves insights with optional pagination.
    ///
    /// EXAMPLES:
    ///   canny insights list
    ///   canny insights list --limit 50
    ///   canny insights list --idea-id idea123
    List {
        /// Maximum number of insights to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Cursor for pagination (from previous response)
        #[arg(long)]
        cursor: Option<String>,

        /// Filter insights by idea ID
        #[arg(long)]
        idea_id: Option<String>,
    },

    /// Retrieve a single insight by ID
    ///
    /// Gets detailed information about a specific insight.
    ///
    /// EXAMPLES:
    ///   canny insights get --id insight123
    Get {
        /// The ID of the insight to retrieve
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum IdeasCommands {
    /// List ideas
    ///
    /// Retrieves ideas with optional pagination.
    ///
    /// EXAMPLES:
    ///   canny ideas list
    ///   canny ideas list --limit 50
    ///   canny ideas list --parent-id parent123
    ///   canny ideas list --search "feature"
    List {
        /// Maximum number of ideas to return (default: 10, max: 10000)
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Cursor for pagination (from previous response)
        #[arg(long)]
        cursor: Option<String>,

        /// Filter by parent idea ID
        #[arg(long)]
        parent_id: Option<String>,

        /// Search term to filter ideas
        #[arg(long)]
        search: Option<String>,
    },

    /// Retrieve a single idea by ID or URL name
    ///
    /// Gets detailed information about a specific idea.
    ///
    /// EXAMPLES:
    ///   # Get idea by ID
    ///   canny ideas get --id idea123
    ///
    ///   # Get idea by URL name
    ///   canny ideas get --url-name my-idea
    Get {
        /// The ID of the idea to retrieve
        #[arg(long)]
        id: Option<String>,

        /// The URL name of the idea to retrieve
        #[arg(long)]
        url_name: Option<String>,
    },
}

#[derive(Subcommand)]
enum AutopilotCommands {
    /// Enqueue feedback for autopilot processing
    ///
    /// Submits feedback to be processed by Canny's autopilot AI. The feedback
    /// will be automatically categorized and processed.
    ///
    /// EXAMPLES:
    ///   # Enqueue simple feedback
    ///   canny autopilot enqueue --user-id user123 --feedback "Users want dark mode support"
    ///
    ///   # Enqueue feedback with source URL
    ///   canny autopilot enqueue --user-id user123 --feedback "Need better search" --source-url "https://example.com/feedback"
    Enqueue {
        /// The feedback text to enqueue for processing
        #[arg(long)]
        feedback: String,

        /// The ID of the user submitting the feedback (required)
        #[arg(long)]
        user_id: String,
        /// Optional source URL where the feedback originated
        #[arg(long)]
        source_url: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle auth before credential resolution
    if let Commands::Auth { reset } = &cli.command {
        if *reset {
            let _ = credentials::clear_stored_credentials();
            println!("  {} Credentials cleared.", "✓".green().bold());
            println!();
        }
        return handle_auth(cli.api_key, cli.api_url).await;
    }

    // Resolve API key: 1) flag/env var, 2) Keychain
    let api_key = credentials::resolve_api_key(cli.api_key)?;

    // Resolve API URL: 1) --api-url flag, 2) Keychain, 3) default
    let api_url = cli
        .api_url
        .or_else(|| credentials::resolve_api_url(None, DEFAULT_API_URL))
        .unwrap_or_else(|| DEFAULT_API_URL.to_string());

    let client = CannyClient::new(api_url, api_key);

    match cli.command {
        Commands::Posts(cmd) => handle_posts(&client, cmd, cli.json).await,
        Commands::Comments(cmd) => handle_comments(&client, cmd, cli.json).await,
        Commands::Categories(cmd) => handle_categories(&client, cmd, cli.json).await,
        Commands::Users(cmd) => handle_users(&client, cmd, cli.json).await,
        Commands::Boards(cmd) => handle_boards(&client, cmd, cli.json).await,
        Commands::Tags(cmd) => handle_tags(&client, cmd, cli.json).await,
        Commands::Companies(cmd) => handle_companies(&client, cmd, cli.json).await,
        Commands::Votes(cmd) => handle_votes(&client, cmd, cli.json).await,
        Commands::StatusChanges(cmd) => handle_status_changes(&client, cmd, cli.json).await,
        Commands::Changelog(cmd) => handle_changelog(&client, cmd, cli.json).await,
        Commands::Opportunities(cmd) => handle_opportunities(&client, cmd, cli.json).await,
        Commands::Groups(cmd) => handle_groups(&client, cmd, cli.json).await,
        Commands::Insights(cmd) => handle_insights(&client, cmd, cli.json).await,
        Commands::Ideas(cmd) => handle_ideas(&client, cmd, cli.json).await,
        Commands::Autopilot(cmd) => handle_autopilot(&client, cmd, cli.json).await,
        Commands::Auth { .. } => unreachable!(),
    }
}

async fn handle_auth(
    explicit_key: Option<String>,
    explicit_url: Option<String>,
) -> Result<()> {
    use std::io::{self, Write};

    // Check if already authenticated
    let has_key = credentials::resolve_api_key(explicit_key.clone()).is_ok();

    if has_key {
        // Already authenticated — show status
        let api_key = credentials::resolve_api_key(explicit_key)?;
        let api_url = explicit_url
            .or_else(|| credentials::resolve_api_url(None, DEFAULT_API_URL))
            .unwrap_or_else(|| DEFAULT_API_URL.to_string());

        let masked = if api_key.len() > 8 {
            format!("{}...{}", &api_key[..4], &api_key[api_key.len() - 4..])
        } else {
            "****".to_string()
        };

        println!("{}", "Canny CLI".bold());
        println!();
        println!("  {} {}", "API URL:".dimmed(), api_url);
        println!("  {} {}", "API key:".dimmed(), masked);

        // Verify credentials with a lightweight API call
        print!("  {}", "Verifying...".dimmed());
        io::stdout().flush()?;
        let client = CannyClient::new(api_url, api_key);
        match client.list_boards().await {
            Ok(boards) => {
                println!(
                    "\r  {} Authenticated ({} board{})   ",
                    "✓".green().bold(),
                    boards.len(),
                    if boards.len() == 1 { "" } else { "s" }
                );
            }
            Err(e) => {
                println!(
                    "\r  {} Authentication failed: {}",
                    "✗".red().bold(),
                    e
                );
                println!(
                    "\n  Run {} to re-authenticate.",
                    "canny auth --reset".cyan()
                );
            }
        }

        return Ok(());
    }

    // Not authenticated — prompt for credentials
    println!("{}", "Canny CLI Authentication".bold());
    println!();

    print!(
        "  {} (e.g. 'mycompany' for mycompany.canny.io) [{}]: ",
        "Subdomain".cyan().bold(),
        "clickup".dimmed()
    );
    io::stdout().flush()?;
    let mut subdomain = String::new();
    io::stdin().read_line(&mut subdomain)?;
    let subdomain = subdomain.trim();
    let subdomain = if subdomain.is_empty() {
        "clickup"
    } else {
        subdomain
    };

    let api_url = format!("https://{}.canny.io/api/v1", subdomain);

    print!("  {}: ", "API key".cyan().bold());
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    if api_key.is_empty() {
        anyhow::bail!("API key cannot be empty");
    }

    credentials::store_api_key(api_key)?;
    credentials::store_api_url(&api_url)?;

    println!();
    println!("  {} Credentials saved to Keychain.", "✓".green().bold());
    println!("  {} {}", "API URL:".dimmed(), api_url);

    Ok(())
}

async fn handle_posts(client: &CannyClient, cmd: PostsCommands, json_output: bool) -> Result<()> {
    match cmd {
        PostsCommands::List {
            board_id,
            limit,
            skip,
            sort,
            status,
            author_id,
            search,
            company_id,
            tag_ids,
        } => {
            let status_str = if status.is_empty() {
                None
            } else {
                Some(status.join(","))
            };
            // Convert Vec<String> to Vec<&str> for tag_ids
            let tag_ids_refs: Option<Vec<&str>> = if tag_ids.is_empty() {
                None
            } else {
                Some(tag_ids.iter().map(|s| s.as_str()).collect())
            };
            let response = client
                .list_posts(
                    &board_id,
                    Some(limit),
                    Some(skip),
                    Some(&sort.to_string()),
                    status_str.as_deref(),
                    author_id.as_deref(),
                    search.as_deref(),
                    company_id.as_deref(),
                    tag_ids_refs,
                )
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.posts)?);
            } else {
                if response.posts.is_empty() {
                    println!("No posts found.");
                } else {
                    for post in &response.posts {
                        print_post_summary(post);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More posts available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }

        PostsCommands::Get { id, url_name, board_id } => {
            if id.is_none() && url_name.is_none() {
                anyhow::bail!("Either --id or --url-name must be provided");
            }
            let post = client.get_post(id.as_deref(), url_name.as_deref(), board_id.as_deref()).await?;
            if let Some(post) = post {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&post)?);
                } else {
                    print_post_detail(&post);
                }
            } else {
                eprintln!("{}", "Post not found.".red());
                std::process::exit(1);
            }
        }

        PostsCommands::Create {
            board_id,
            author_id,
            title,
            details,
            category_id,
            by_id,
            custom_fields,
            eta,
            eta_public,
            owner_id,
            image_urls,
            created_at,
        } => {
            // Parse custom_fields JSON if provided
            let custom_fields_json: Option<serde_json::Value> = match custom_fields {
                Some(ref cf) => Some(serde_json::from_str(cf).context("Invalid JSON for --custom-fields")?),
                None => None,
            };
            // Convert Vec<String> to Vec<&str> for image_urls
            let image_urls_refs: Option<Vec<&str>> = if image_urls.is_empty() {
                None
            } else {
                Some(image_urls.iter().map(|s| s.as_str()).collect())
            };
            let id = client
                .create_post(
                    &board_id,
                    &author_id,
                    &title,
                    details.as_deref(),
                    category_id.as_deref(),
                    by_id.as_deref(),
                    custom_fields_json,
                    eta.as_deref(),
                    eta_public,
                    owner_id.as_deref(),
                    image_urls_refs,
                    created_at.as_deref(),
                )
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!("{} Created post with ID: {}", "✓".green(), id.cyan());
            }
        }

        PostsCommands::Status {
            id,
            changer_id,
            status,
            notify,
            comment,
            comment_image_urls,
        } => {
            let image_urls: Option<Vec<&str>> = if comment_image_urls.is_empty() {
                None
            } else {
                Some(comment_image_urls.iter().map(|s| s.as_str()).collect())
            };
            client
                .change_post_status(
                    &id,
                    &changer_id,
                    &status,
                    notify,
                    comment.as_deref(),
                    image_urls,
                )
                .await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!(
                    "{} Status changed to: {}",
                    "✓".green(),
                    status.to_string().cyan()
                );
            }
        }

        PostsCommands::Category { id, category_id } => {
            client.change_post_category(&id, &category_id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Category updated.", "✓".green());
            }
        }

        PostsCommands::Update { id, title, details, eta, eta_public, custom_fields } => {
            let custom_fields_json: Option<serde_json::Value> = custom_fields
                .as_ref()
                .map(|s| serde_json::from_str(s))
                .transpose()
                .context("Invalid JSON for custom-fields")?;
            client
                .update_post(
                    &id,
                    title.as_deref(),
                    details.as_deref(),
                    None,
                    eta.as_deref(),
                    eta_public,
                    custom_fields_json,
                )
                .await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Post updated.", "✓".green());
            }
        }

        PostsCommands::Delete { id } => {
            client.delete_post(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Post deleted.", "✓".green());
            }
        }

        PostsCommands::AddTag { id, tag_id } => {
            client.add_post_tag(&id, &tag_id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Tag added to post.", "✓".green());
            }
        }

        PostsCommands::RemoveTag { id, tag_id } => {
            client.remove_post_tag(&id, &tag_id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Tag removed from post.", "✓".green());
            }
        }

        PostsCommands::LinkJira { id, issue_key } => {
            client.link_post_jira(&id, &issue_key).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!(
                    "{} Jira issue {} linked to post.",
                    "✓".green(),
                    issue_key.cyan()
                );
            }
        }

        PostsCommands::UnlinkJira { id, issue_key } => {
            client.unlink_post_jira(&id, &issue_key).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!(
                    "{} Jira issue {} unlinked from post.",
                    "✓".green(),
                    issue_key.cyan()
                );
            }
        }
    }

    Ok(())
}

async fn handle_comments(
    client: &CannyClient,
    cmd: CommentsCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        CommentsCommands::List {
            post_id,
            author_id,
            board_id,
            company_id,
            limit,
            skip,
        } => {
            let response = client
                .list_comments(
                    post_id.as_deref(),
                    author_id.as_deref(),
                    board_id.as_deref(),
                    company_id.as_deref(),
                    Some(limit),
                    Some(skip),
                )
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.comments)?);
            } else {
                if response.comments.is_empty() {
                    println!("No comments found.");
                } else {
                    for comment in &response.comments {
                        print_comment(comment);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More comments available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }

        CommentsCommands::Create {
            post_id,
            author_id,
            value,
            parent_id,
            created_at,
            image_urls,
            internal,
            notify_voters,
        } => {
            let image_urls_refs: Option<Vec<&str>> = if image_urls.is_empty() {
                None
            } else {
                Some(image_urls.iter().map(|s| s.as_str()).collect())
            };
            let internal_opt = if internal { Some(true) } else { None };
            let notify_voters_opt = if notify_voters { Some(true) } else { None };

            let id = client
                .create_comment(
                    &post_id,
                    &author_id,
                    &value,
                    parent_id.as_deref(),
                    created_at.as_deref(),
                    image_urls_refs,
                    internal_opt,
                    notify_voters_opt,
                )
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!("{} Created comment with ID: {}", "✓".green(), id.cyan());
            }
        }

        CommentsCommands::Get { id } => {
            let comment = client.get_comment(&id).await?;
            if let Some(comment) = comment {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&comment)?);
                } else {
                    print_comment_detail(&comment);
                }
            } else {
                eprintln!("{}", "Comment not found.".red());
                std::process::exit(1);
            }
        }

        CommentsCommands::Delete { id } => {
            client.delete_comment(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Comment deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

async fn handle_categories(
    client: &CannyClient,
    cmd: CategoriesCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        CategoriesCommands::List {
            board_id,
            limit,
            skip,
        } => {
            let response = client
                .list_categories(&board_id, Some(limit), Some(skip))
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.categories)?);
            } else {
                if response.categories.is_empty() {
                    println!("No categories found.");
                } else {
                    println!("{}", "Categories:".bold());
                    for cat in &response.categories {
                        println!(
                            "  {} {} {}",
                            cat.id.dimmed(),
                            cat.name.cyan(),
                            format!("({} posts)", cat.post_count.unwrap_or(0)).dimmed()
                        );
                    }
                }
            }
        }

        CategoriesCommands::Get { id } => {
            let category = client.get_category(&id).await?;
            if let Some(category) = category {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&category)?);
                } else {
                    print_category_detail(&category);
                }
            } else {
                eprintln!("{}", "Category not found.".red());
                std::process::exit(1);
            }
        }

        CategoriesCommands::Create {
            board_id,
            name,
            parent_id,
            subscribe_admins,
        } => {
            let id = client
                .create_category(&board_id, &name, parent_id.as_deref(), subscribe_admins)
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!("{} Created category with ID: {}", "✓".green(), id.cyan());
            }
        }

        CategoriesCommands::Delete { id } => {
            client.delete_category(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Category deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_post_summary(post: &models::CannyPost) {
    let status = post.status.as_deref().unwrap_or("unknown").to_uppercase();

    let status_colored = match status.to_lowercase().as_str() {
        "open" => status.yellow(),
        "planned" | "in progress" => status.blue(),
        "complete" => status.green(),
        "closed" => status.red(),
        _ => status.white(),
    };

    println!("\n{} {}", post.id.dimmed(), post.title.bold());
    println!(
        "  {} | {} votes | {} comments",
        status_colored,
        post.score.to_string().cyan(),
        post.comment_count.to_string().cyan()
    );
    if let Some(ref cat) = post.category {
        println!("  Category: {}", cat.name.magenta());
    }
}

fn print_post_detail(post: &models::CannyPost) {
    println!("\n{}", post.title.bold());
    println!("{}", "─".repeat(60).dimmed());

    if let Some(ref status) = post.status {
        let status_upper = status.to_uppercase();
        let status_colored = match status.to_lowercase().as_str() {
            "open" => status_upper.yellow(),
            "planned" | "in progress" => status_upper.blue(),
            "complete" => status_upper.green(),
            "closed" => status_upper.red(),
            _ => status_upper.white(),
        };
        println!("Status: {}", status_colored);
    }

    println!("Votes: {}", post.score.to_string().cyan());
    println!("Comments: {}", post.comment_count.to_string().cyan());

    if let Some(ref author) = post.author {
        println!("Author: {}", author.name);
    }

    if let Some(ref cat) = post.category {
        println!("Category: {}", cat.name.magenta());
    }

    if let Some(ref created) = post.created {
        println!("Created: {}", created.dimmed());
    }

    println!("URL: {}", post.url.underline());
    println!("ID: {}", post.id.dimmed());

    if let Some(ref details) = post.details {
        if !details.is_empty() {
            println!("\n{}", "Description:".bold());
            println!("{}", details);
        }
    }
}

fn print_comment(comment: &models::CannyComment) {
    let author_name = comment
        .author
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("Unknown");

    let prefix = if comment.parent_id.is_some() {
        "  ↳ "
    } else {
        ""
    };

    let pinned = if comment.pinned.unwrap_or(false) {
        " [PINNED]".yellow().to_string()
    } else {
        String::new()
    };

    println!(
        "\n{}{} {}{}",
        prefix,
        author_name.cyan(),
        comment.created.dimmed(),
        pinned
    );
    println!("{}{}", prefix, comment.value);
    println!("{}{}", prefix, format!("ID: {}", comment.id).dimmed());
}

fn print_comment_detail(comment: &models::CannyComment) {
    let author_name = comment
        .author
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("Unknown");

    println!("\n{}", "Comment".bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", comment.id.cyan());
    println!("Author: {}", author_name);
    println!("Created: {}", comment.created.dimmed());

    if let Some(ref post) = comment.post {
        println!("Post ID: {}", post.id.dimmed());
        println!("Post: {}", post.title);
    }

    if let Some(ref parent_id) = comment.parent_id {
        println!("Parent ID: {} (reply)", parent_id.dimmed());
    }

    if comment.pinned.unwrap_or(false) {
        println!("Pinned: {}", "Yes".yellow());
    }

    println!("\n{}", "Content:".bold());
    println!("{}", comment.value);
}

fn print_category_detail(category: &models::CannyCategory) {
    println!("\n{}", category.name.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", category.id.cyan());
    println!(
        "Posts: {}",
        category.post_count.unwrap_or(0).to_string().cyan()
    );

    if let Some(ref url) = category.url {
        println!("URL: {}", url.underline());
    }
}

async fn handle_users(client: &CannyClient, cmd: UsersCommands, json_output: bool) -> Result<()> {
    use std::io::Write;

    match cmd {
        UsersCommands::List => {
            let users = if json_output {
                client.list_users(None::<fn(usize)>).await?
            } else {
                client
                    .list_users(Some(|count: usize| {
                        print!("\rFetching users... {}", count);
                        let _ = std::io::stdout().flush();
                    }))
                    .await?
            };

            // Clear the progress line
            if !json_output {
                print!("\r\x1b[K"); // Clear line
                let _ = std::io::stdout().flush();
            }

            if json_output {
                println!("{}", serde_json::to_string_pretty(&users)?);
            } else {
                if users.is_empty() {
                    println!("No users found.");
                } else {
                    println!("{} ({} total)", "Users:".bold(), users.len());
                    for user in &users {
                        print_user(user);
                    }
                }
            }
        }

        UsersCommands::Get { id, email } => {
            if id.is_none() && email.is_none() {
                anyhow::bail!("Either --id or --email must be provided");
            }

            let user = client.get_user(id.as_deref(), email.as_deref()).await?;
            if let Some(user) = user {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&user)?);
                } else {
                    print_user_detail(&user);
                }
            } else {
                eprintln!("{}", "User not found.".red());
                std::process::exit(1);
            }
        }

        UsersCommands::Create {
            user_id,
            email,
            id: canny_id,
            name,
            avatar_url,
            company_id,
            custom_fields,
        } => {
            // Parse custom fields from JSON string if provided
            let custom_fields_value = match custom_fields {
                Some(ref json_str) => {
                    Some(serde_json::from_str(json_str).context("Failed to parse custom-fields as JSON")?)
                }
                None => None,
            };

            let id = client
                .create_or_update_user(
                    &user_id,
                    &email,
                    canny_id.as_deref(),
                    name.as_deref(),
                    avatar_url.as_deref(),
                    None,
                    company_id.as_deref(),
                    custom_fields_value,
                )
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!(
                    "{} Created/updated user with ID: {}",
                    "✓".green(),
                    id.cyan()
                );
            }
        }

        UsersCommands::Delete { id } => {
            client.delete_user(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} User deleted.", "✓".green());
            }
        }

        UsersCommands::Find {
            user_id,
            email,
            name,
        } => {
            if user_id.is_none() && email.is_none() && name.is_none() {
                anyhow::bail!("At least one of --user-id, --email, or --name must be provided");
            }

            let user = client
                .find_user(user_id.as_deref(), email.as_deref(), name.as_deref())
                .await?;
            if let Some(user) = user {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&user)?);
                } else {
                    print_user_detail(&user);
                }
            } else {
                eprintln!("{}", "User not found.".red());
                std::process::exit(1);
            }
        }

        UsersCommands::RemoveFromCompany {
            user_id,
            company_id,
        } => {
            client
                .remove_user_from_company(&user_id, &company_id)
                .await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} User removed from company.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_user(user: &models::CannyUserFull) {
    let name = user.name.as_deref().unwrap_or("(no name)");
    let email = user.email.as_deref().unwrap_or("");
    let admin_badge = if user.is_admin.unwrap_or(false) {
        " [ADMIN]".magenta().to_string()
    } else {
        String::new()
    };

    println!("\n  {} {}{}", user.id.dimmed(), name.cyan(), admin_badge);
    if !email.is_empty() {
        println!("    Email: {}", email);
    }
}

fn print_user_detail(user: &models::CannyUserFull) {
    let name = user.name.as_deref().unwrap_or("(no name)");

    println!("\n{}", name.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", user.id.cyan());

    if let Some(ref email) = user.email {
        println!("Email: {}", email);
    }

    if let Some(is_admin) = user.is_admin {
        if is_admin {
            println!("Role: {}", "Admin".magenta());
        }
    }

    if let Some(ref created) = user.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref last_activity) = user.last_activity {
        println!("Last Activity: {}", last_activity.dimmed());
    }

    if let Some(ref url) = user.url {
        println!("URL: {}", url.underline());
    }

    if let Some(ref user_id) = user.user_id {
        println!("User ID: {}", user_id.dimmed());
    }
}

async fn handle_boards(client: &CannyClient, cmd: BoardsCommands, json_output: bool) -> Result<()> {
    match cmd {
        BoardsCommands::List => {
            let boards = client.list_boards().await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&boards)?);
            } else {
                if boards.is_empty() {
                    println!("No boards found.");
                } else {
                    println!("{} ({} total)", "Boards:".bold(), boards.len());
                    for board in &boards {
                        print_board(board);
                    }
                }
            }
        }

        BoardsCommands::Get { id } => {
            let board = client.get_board(&id).await?;
            if let Some(board) = board {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&board)?);
                } else {
                    print_board(&board);
                }
            } else {
                eprintln!("{}", "Board not found.".red());
                std::process::exit(1);
            }
        }

        BoardsCommands::Create { name } => {
            let id = client.create_board(&name).await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!("{} Created board with ID: {}", "✓".green(), id.cyan());
            }
        }

        BoardsCommands::Delete { id } => {
            client.delete_board(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Board deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_board(board: &models::CannyBoard) {
    let private_badge = if board.is_private.unwrap_or(false) {
        " [PRIVATE]".yellow().to_string()
    } else {
        String::new()
    };

    let post_count = board.post_count.unwrap_or(0);

    println!(
        "\n  {} {}{}",
        board.id.dimmed(),
        board.name.cyan(),
        private_badge
    );
    println!("    Posts: {}", post_count);
    if let Some(ref url) = board.url {
        println!("    URL: {}", url.underline());
    }
}

async fn handle_tags(client: &CannyClient, cmd: TagsCommands, json_output: bool) -> Result<()> {
    match cmd {
        TagsCommands::List {
            board_id,
            limit,
            skip,
        } => {
            let response = client.list_tags(&board_id, Some(limit), Some(skip)).await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.tags)?);
            } else {
                if response.tags.is_empty() {
                    println!("No tags found.");
                } else {
                    println!("{}", "Tags:".bold());
                    for tag in &response.tags {
                        print_tag(tag);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More tags available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }

        TagsCommands::Get { id } => {
            let tag = client.get_tag(&id).await?;
            if let Some(tag) = tag {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&tag)?);
                } else {
                    print_tag_detail(&tag);
                }
            } else {
                eprintln!("{}", "Tag not found.".red());
                std::process::exit(1);
            }
        }

        TagsCommands::Create { board_id, name } => {
            let id = client.create_tag(&board_id, &name).await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!("{} Created tag with ID: {}", "✓".green(), id.cyan());
            }
        }

        TagsCommands::Delete { id } => {
            client.delete_tag(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Tag deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_tag(tag: &models::CannyTag) {
    println!(
        "  {} {} {}",
        tag.id.dimmed(),
        tag.name.cyan(),
        format!("({} posts)", tag.post_count.unwrap_or(0)).dimmed()
    );
}

fn print_tag_detail(tag: &models::CannyTag) {
    println!("\n{}", tag.name.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", tag.id.cyan());
    println!("Posts: {}", tag.post_count.unwrap_or(0).to_string().cyan());

    if let Some(ref board_id) = tag.board_id {
        println!("Board ID: {}", board_id.dimmed());
    }

    if let Some(ref created) = tag.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref url) = tag.url {
        println!("URL: {}", url.underline());
    }
}

async fn handle_companies(
    client: &CannyClient,
    cmd: CompaniesCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        CompaniesCommands::List {
            limit,
            cursor,
            search,
            segment,
        } => {
            let response = client
                .list_companies(Some(limit), cursor.as_deref(), search.as_deref(), segment.as_deref())
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.companies)?);
            } else {
                if response.companies.is_empty() {
                    println!("No companies found.");
                } else {
                    println!(
                        "{} ({} returned)",
                        "Companies:".bold(),
                        response.companies.len()
                    );
                    for company in &response.companies {
                        print_company(company);
                    }
                    if response.has_next_page.unwrap_or(false) {
                        if let Some(ref next_cursor) = response.cursor {
                            println!(
                                "\n{} Use --cursor {} to see more.",
                                "More companies available.".dimmed(),
                                next_cursor
                            );
                        }
                    }
                }
            }
        }

        CompaniesCommands::Update {
            id,
            name,
            monthly_spend,
            custom_fields,
            created,
        } => {
            // Parse custom_fields from JSON string if provided
            let custom_fields_value = if let Some(cf_str) = custom_fields {
                Some(serde_json::from_str(&cf_str).context("Invalid JSON for custom_fields")?)
            } else {
                None
            };

            client
                .update_company(&id, name.as_deref(), monthly_spend, custom_fields_value, created.as_deref())
                .await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Company updated.", "✓".green());
            }
        }

        CompaniesCommands::Get { id } => {
            let company = client.get_company(&id).await?;
            if let Some(company) = company {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&company)?);
                } else {
                    print_company_detail(&company);
                }
            } else {
                eprintln!("{}", "Company not found.".red());
                std::process::exit(1);
            }
        }

        CompaniesCommands::Delete { id } => {
            client.delete_company(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Company deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_company(company: &models::CannyCompany) {
    let name = company.name.as_deref().unwrap_or("(no name)");
    let user_count = company.user_count.unwrap_or(0);

    println!("\n  {} {}", company.id.dimmed(), name.cyan());
    println!("    Users: {}", user_count);
    if let Some(monthly_spend) = company.monthly_spend {
        println!("    Monthly Spend: ${:.2}", monthly_spend);
    }
    if let Some(ref created) = company.created {
        println!("    Created: {}", created.dimmed());
    }
}

fn print_company_detail(company: &models::CannyCompany) {
    let name = company.name.as_deref().unwrap_or("(no name)");

    println!("\n{}", name.bold());
    println!("{}", "-".repeat(60).dimmed());

    println!("ID: {}", company.id.cyan());
    println!(
        "Users: {}",
        company.user_count.unwrap_or(0).to_string().cyan()
    );

    if let Some(monthly_spend) = company.monthly_spend {
        println!("Monthly Spend: ${:.2}", monthly_spend);
    }

    if let Some(ref created) = company.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref custom_fields) = company.custom_fields {
        if !custom_fields.is_null() {
            println!(
                "Custom Fields: {}",
                serde_json::to_string_pretty(custom_fields).unwrap_or_default()
            );
        }
    }
}

async fn handle_votes(client: &CannyClient, cmd: VotesCommands, json_output: bool) -> Result<()> {
    match cmd {
        VotesCommands::List {
            post_id,
            user_id,
            limit,
            skip,
        } => {
            let response = client
                .list_votes(post_id.as_deref(), user_id.as_deref(), Some(limit), Some(skip))
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.votes)?);
            } else {
                if response.votes.is_empty() {
                    println!("No votes found.");
                } else {
                    println!("{}", "Votes:".bold());
                    for vote in &response.votes {
                        print_vote(vote);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More votes available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }

        VotesCommands::Get { id } => {
            let vote = client.get_vote(&id).await?;
            if let Some(vote) = vote {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&vote)?);
                } else {
                    print_vote_detail(&vote);
                }
            } else {
                eprintln!("{}", "Vote not found.".red());
                std::process::exit(1);
            }
        }

        VotesCommands::Create { post_id, user_id } => {
            client.create_vote(&post_id, &user_id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Vote created.", "✓".green());
            }
        }

        VotesCommands::Delete { id } => {
            client.delete_vote(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Vote deleted.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_vote(vote: &models::CannyVote) {
    let voter_name = vote
        .voter
        .as_ref()
        .map(|v| v.name.as_str())
        .unwrap_or("Unknown");

    let created = vote.created.as_deref().unwrap_or("");

    println!(
        "\n  {} {} {}",
        vote.id.dimmed(),
        voter_name.cyan(),
        created.dimmed()
    );
}

fn print_vote_detail(vote: &models::CannyVote) {
    println!("\n{}", "Vote".bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", vote.id.cyan());

    if let Some(ref voter) = vote.voter {
        println!("Voter: {}", voter.name);
        if let Some(ref email) = voter.email {
            println!("Email: {}", email);
        }
    }

    if let Some(ref post_id) = vote.post_id {
        println!("Post ID: {}", post_id.dimmed());
    }

    if let Some(ref created) = vote.created {
        println!("Created: {}", created.dimmed());
    }
}

async fn handle_status_changes(
    client: &CannyClient,
    cmd: StatusChangesCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        StatusChangesCommands::List {
            board_id,
            limit,
            skip,
        } => {
            let response = client
                .list_status_changes(&board_id, Some(limit), Some(skip))
                .await?;

            if json_output {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&response.status_changes)?
                );
            } else {
                if response.status_changes.is_empty() {
                    println!("No status changes found.");
                } else {
                    println!("{}", "Status Changes:".bold());
                    for status_change in &response.status_changes {
                        print_status_change(status_change);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More status changes available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_status_change(status_change: &models::CannyStatusChange) {
    let changer_name = status_change
        .changer
        .as_ref()
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let status = status_change.status.as_deref().unwrap_or("unknown");
    let status_upper = status.to_uppercase();
    let status_colored = match status.to_lowercase().as_str() {
        "open" => status_upper.yellow(),
        "planned" | "in progress" => status_upper.blue(),
        "complete" => status_upper.green(),
        "closed" => status_upper.red(),
        _ => status_upper.white(),
    };

    let created = status_change.created.as_deref().unwrap_or("");

    println!(
        "\n  {} {} -> {} by {} {}",
        status_change.id.dimmed(),
        "Status".dimmed(),
        status_colored,
        changer_name.cyan(),
        created.dimmed()
    );

    if let Some(ref post_id) = status_change.post_id {
        println!("    Post ID: {}", post_id.dimmed());
    }
}

async fn handle_changelog(
    client: &CannyClient,
    cmd: ChangelogCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        ChangelogCommands::List {
            limit,
            skip,
            entry_type,
            label_ids,
            sort,
        } => {
            let label_ids_refs: Option<Vec<&str>> = if label_ids.is_empty() {
                None
            } else {
                Some(label_ids.iter().map(|s| s.as_str()).collect())
            };

            let response = client
                .list_entries(
                    Some(limit),
                    Some(skip),
                    entry_type.as_deref(),
                    label_ids_refs,
                    sort.as_deref(),
                )
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.entries)?);
            } else {
                if response.entries.is_empty() {
                    println!("No changelog entries found.");
                } else {
                    println!("{}", "Changelog Entries:".bold());
                    for entry in &response.entries {
                        print_entry(entry);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More entries available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }

        ChangelogCommands::Create {
            title,
            details,
            entry_type,
            published,
            notify,
            post_ids,
            label_ids,
            published_on,
            scheduled_for,
        } => {
            let post_ids_refs: Option<Vec<&str>> = if post_ids.is_empty() {
                None
            } else {
                Some(post_ids.iter().map(|s| s.as_str()).collect())
            };
            let label_ids_refs: Option<Vec<&str>> = if label_ids.is_empty() {
                None
            } else {
                Some(label_ids.iter().map(|s| s.as_str()).collect())
            };

            let id = client
                .create_entry(
                    &title,
                    details.as_deref(),
                    entry_type.as_deref(),
                    published,
                    notify,
                    post_ids_refs,
                    label_ids_refs,
                    published_on.as_deref(),
                    scheduled_for.as_deref(),
                )
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!(
                    "{} Created changelog entry with ID: {}",
                    "✓".green(),
                    id.cyan()
                );
            }
        }

        ChangelogCommands::Get { id } => {
            let entry = client.get_entry(&id).await?;
            if let Some(entry) = entry {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&entry)?);
                } else {
                    print_entry_detail(&entry);
                }
            } else {
                eprintln!("{}", "Changelog entry not found.".red());
                std::process::exit(1);
            }
        }

        ChangelogCommands::Delete { id } => {
            client.delete_entry(&id).await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Changelog entry deleted.", "✓".green());
            }
        }

        ChangelogCommands::Update {
            id,
            title,
            details,
            entry_type,
            published,
            notify,
            label_ids,
        } => {
            let label_ids_refs: Option<Vec<&str>> = if label_ids.is_empty() {
                None
            } else {
                Some(label_ids.iter().map(|s| s.as_str()).collect())
            };

            client
                .update_entry(
                    &id,
                    title.as_deref(),
                    details.as_deref(),
                    entry_type.as_deref(),
                    published,
                    notify,
                    label_ids_refs,
                )
                .await?;

            if json_output {
                println!(r#"{{"success": true}}"#);
            } else {
                println!("{} Changelog entry updated.", "✓".green());
            }
        }
    }

    Ok(())
}

fn print_entry(entry: &models::CannyEntry) {
    let title = entry.title.as_deref().unwrap_or("(no title)");
    let status = entry.status.as_deref().unwrap_or("draft").to_uppercase();
    let entry_type = entry.entry_type.as_deref().unwrap_or("");

    let status_colored = match status.to_lowercase().as_str() {
        "published" => status.green(),
        "draft" => status.yellow(),
        _ => status.white(),
    };

    let type_badge = if !entry_type.is_empty() {
        format!(" [{}]", entry_type.to_uppercase())
            .magenta()
            .to_string()
    } else {
        String::new()
    };

    println!("\n  {} {}{}", entry.id.dimmed(), title.cyan(), type_badge);
    println!("    Status: {}", status_colored);
    if let Some(ref published_at) = entry.published_at {
        println!("    Published: {}", published_at.dimmed());
    }
    if let Some(ref created) = entry.created {
        println!("    Created: {}", created.dimmed());
    }
    if let Some(ref url) = entry.url {
        println!("    URL: {}", url.underline());
    }
}

fn print_entry_detail(entry: &models::CannyEntry) {
    let title = entry.title.as_deref().unwrap_or("(no title)");

    println!("\n{}", title.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", entry.id.cyan());

    if let Some(ref entry_type) = entry.entry_type {
        println!("Type: {}", entry_type.magenta());
    }

    if let Some(ref status) = entry.status {
        let status_upper = status.to_uppercase();
        let status_colored = match status.to_lowercase().as_str() {
            "published" => status_upper.green(),
            "draft" => status_upper.yellow(),
            _ => status_upper.white(),
        };
        println!("Status: {}", status_colored);
    }

    if let Some(ref published_at) = entry.published_at {
        println!("Published: {}", published_at.dimmed());
    }

    if let Some(ref created) = entry.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref url) = entry.url {
        println!("URL: {}", url.underline());
    }

    if let Some(ref details) = entry.details {
        if !details.is_empty() {
            println!("\n{}", "Details:".bold());
            println!("{}", details);
        }
    }
}

async fn handle_opportunities(
    client: &CannyClient,
    cmd: OpportunitiesCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        OpportunitiesCommands::List {
            post_id,
            limit,
            skip,
        } => {
            let response = client
                .list_opportunities(&post_id, Some(limit), Some(skip))
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.opportunities)?);
            } else {
                if response.opportunities.is_empty() {
                    println!("No opportunities found.");
                } else {
                    println!("{}", "Opportunities:".bold());
                    for opportunity in &response.opportunities {
                        print_opportunity(opportunity);
                    }
                    if response.has_more {
                        println!(
                            "\n{} Use --skip {} to see more.",
                            "More opportunities available.".dimmed(),
                            skip + limit
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_opportunity(opportunity: &models::CannyOpportunity) {
    let name = opportunity.name.as_deref().unwrap_or("(no name)");

    let status = if opportunity.closed.unwrap_or(false) {
        if opportunity.won.unwrap_or(false) {
            "WON".green()
        } else {
            "LOST".red()
        }
    } else {
        "OPEN".yellow()
    };

    println!("\n  {} {}", opportunity.id.dimmed(), name.cyan());
    println!("    Status: {}", status);

    if let Some(value) = opportunity.value {
        println!("    Value: ${:.2}", value);
    }

    if let Some(ref opp_id) = opportunity.opportunity_id {
        println!("    Opportunity ID: {}", opp_id.dimmed());
    }

    if let Some(ref sf_id) = opportunity.salesforce_opportunity_id {
        println!("    Salesforce ID: {}", sf_id.dimmed());
    }
}

async fn handle_groups(client: &CannyClient, cmd: GroupsCommands, json_output: bool) -> Result<()> {
    match cmd {
        GroupsCommands::List { limit, cursor } => {
            let response = client
                .list_groups(Some(limit), cursor.as_deref())
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.groups)?);
            } else {
                if response.groups.is_empty() {
                    println!("No groups found.");
                } else {
                    println!("{}", "Groups:".bold());
                    for group in &response.groups {
                        print_group(group);
                    }
                    if response.has_more {
                        if let Some(ref next_cursor) = response.cursor {
                            println!(
                                "\n{} Use --cursor {} to see more.",
                                "More groups available.".dimmed(),
                                next_cursor
                            );
                        }
                    }
                }
            }
        }

        GroupsCommands::Get { id, url_name } => {
            if id.is_none() && url_name.is_none() {
                anyhow::bail!("Either --id or --url-name must be provided");
            }

            let group = client.get_group(id.as_deref(), url_name.as_deref()).await?;
            if let Some(group) = group {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&group)?);
                } else {
                    print_group_detail(&group);
                }
            } else {
                eprintln!("{}", "Group not found.".red());
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn print_group(group: &models::CannyGroup) {
    let name = group.name.as_deref().unwrap_or("(no name)");
    let member_count = group.member_count.unwrap_or(0);

    println!("\n  {} {}", group.id.dimmed(), name.cyan());
    println!("    Members: {}", member_count);
    if let Some(ref url) = group.url {
        println!("    URL: {}", url.underline());
    }
    if let Some(ref created) = group.created {
        println!("    Created: {}", created.dimmed());
    }
}

fn print_group_detail(group: &models::CannyGroup) {
    let name = group.name.as_deref().unwrap_or("(no name)");

    println!("\n{}", name.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", group.id.cyan());
    println!(
        "Members: {}",
        group.member_count.unwrap_or(0).to_string().cyan()
    );

    if let Some(ref url) = group.url {
        println!("URL: {}", url.underline());
    }

    if let Some(ref created) = group.created {
        println!("Created: {}", created.dimmed());
    }
}

async fn handle_insights(
    client: &CannyClient,
    cmd: InsightsCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        InsightsCommands::List {
            limit,
            cursor,
            idea_id,
        } => {
            let response = client
                .list_insights(Some(limit), cursor.as_deref(), idea_id.as_deref())
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.insights)?);
            } else {
                if response.insights.is_empty() {
                    println!("No insights found.");
                } else {
                    println!("{}", "Insights:".bold());
                    for insight in &response.insights {
                        print_insight(insight);
                    }
                    if response.has_more {
                        if let Some(ref next_cursor) = response.cursor {
                            println!(
                                "\n{} Use --cursor {} to see more.",
                                "More insights available.".dimmed(),
                                next_cursor
                            );
                        }
                    }
                }
            }
        }

        InsightsCommands::Get { id } => {
            let insight = client.get_insight(&id).await?;
            if let Some(insight) = insight {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&insight)?);
                } else {
                    print_insight_detail(&insight);
                }
            } else {
                eprintln!("{}", "Insight not found.".red());
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn print_insight(insight: &models::CannyInsight) {
    let title = insight.title.as_deref().unwrap_or("(no title)");

    println!("\n  {} {}", insight.id.dimmed(), title.cyan());
    if let Some(ref url) = insight.url {
        println!("    URL: {}", url.underline());
    }
    if let Some(ref created) = insight.created {
        println!("    Created: {}", created.dimmed());
    }
}

fn print_insight_detail(insight: &models::CannyInsight) {
    let title = insight.title.as_deref().unwrap_or("(no title)");

    println!("\n{}", title.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", insight.id.cyan());

    if let Some(ref url) = insight.url {
        println!("URL: {}", url.underline());
    }

    if let Some(ref created) = insight.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref description) = insight.description {
        if !description.is_empty() {
            println!("\n{}", "Description:".bold());
            println!("{}", description);
        }
    }
}

async fn handle_ideas(client: &CannyClient, cmd: IdeasCommands, json_output: bool) -> Result<()> {
    match cmd {
        IdeasCommands::List {
            limit,
            cursor,
            parent_id,
            search,
        } => {
            let response = client
                .list_ideas(
                    Some(limit),
                    cursor.as_deref(),
                    parent_id.as_deref(),
                    search.as_deref(),
                )
                .await?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&response.ideas)?);
            } else {
                if response.ideas.is_empty() {
                    println!("No ideas found.");
                } else {
                    println!("{}", "Ideas:".bold());
                    for idea in &response.ideas {
                        print_idea(idea);
                    }
                    if response.has_more {
                        if let Some(ref next_cursor) = response.cursor {
                            println!(
                                "\n{} Use --cursor {} to see more.",
                                "More ideas available.".dimmed(),
                                next_cursor
                            );
                        }
                    }
                }
            }
        }

        IdeasCommands::Get { id, url_name } => {
            if id.is_none() && url_name.is_none() {
                anyhow::bail!("Either --id or --url-name must be provided");
            }

            let idea = client.get_idea(id.as_deref(), url_name.as_deref()).await?;
            if let Some(idea) = idea {
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&idea)?);
                } else {
                    print_idea_detail(&idea);
                }
            } else {
                eprintln!("{}", "Idea not found.".red());
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn print_idea(idea: &models::CannyIdea) {
    let name = idea.name.as_deref().unwrap_or("(no name)");

    println!("\n  {} {}", idea.id.dimmed(), name.cyan());
    if let Some(post_count) = idea.post_count {
        println!("    Posts: {}", post_count);
    }
    if let Some(ref url) = idea.url {
        println!("    URL: {}", url.underline());
    }
    if let Some(ref created) = idea.created {
        println!("    Created: {}", created.dimmed());
    }
}

fn print_idea_detail(idea: &models::CannyIdea) {
    let name = idea.name.as_deref().unwrap_or("(no name)");

    println!("\n{}", name.bold());
    println!("{}", "─".repeat(60).dimmed());

    println!("ID: {}", idea.id.cyan());

    if let Some(post_count) = idea.post_count {
        println!("Posts: {}", post_count.to_string().cyan());
    }

    if let Some(ref url) = idea.url {
        println!("URL: {}", url.underline());
    }

    if let Some(ref created) = idea.created {
        println!("Created: {}", created.dimmed());
    }

    if let Some(ref description) = idea.description {
        if !description.is_empty() {
            println!("\n{}", "Description:".bold());
            println!("{}", description);
        }
    }
}

async fn handle_autopilot(
    client: &CannyClient,
    cmd: AutopilotCommands,
    json_output: bool,
) -> Result<()> {
    match cmd {
        AutopilotCommands::Enqueue {
            feedback,
            user_id,
            source_url,
        } => {
            let id = client
                .enqueue_autopilot_feedback(&feedback, &user_id, source_url.as_deref())
                .await?;

            if json_output {
                println!(r#"{{"id": "{}"}}"#, id);
            } else {
                println!(
                    "{} Enqueued feedback with ID: {}",
                    "✓".green(),
                    id.cyan()
                );
            }
        }
    }

    Ok(())
}
