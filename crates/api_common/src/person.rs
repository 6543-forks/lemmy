use lemmy_db_views::{
  comment_view::CommentView,
  post_view::PostView,
  private_message_view::PrivateMessageView,
};
use lemmy_db_views_actor::{
  community_moderator_view::CommunityModeratorView,
  person_mention_view::PersonMentionView,
  person_view::PersonViewSafe,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
  pub username_or_email: String,
  pub password: String,
}
use lemmy_db_schema::newtypes::{CommunityId, PersonId, PersonMentionId, PrivateMessageId};

#[derive(Serialize, Deserialize)]
pub struct Register {
  pub username: String,
  pub password: String,
  pub password_verify: String,
  pub show_nsfw: bool,
  pub email: Option<String>,
  pub captcha_uuid: Option<String>,
  pub captcha_answer: Option<String>,
  pub honeypot: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetCaptcha {}

#[derive(Serialize, Deserialize)]
pub struct GetCaptchaResponse {
  pub ok: Option<CaptchaResponse>, // Will be None if captchas are disabled
}

#[derive(Serialize, Deserialize)]
pub struct CaptchaResponse {
  pub png: String, // A Base64 encoded png
  pub wav: String, // A Base64 encoded wav audio
  pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveUserSettings {
  pub show_nsfw: Option<bool>,
  pub show_scores: Option<bool>,
  pub theme: Option<String>,
  pub default_sort_type: Option<i16>,
  pub default_listing_type: Option<i16>,
  pub lang: Option<String>,
  pub avatar: Option<String>,
  pub banner: Option<String>,
  pub display_name: Option<String>,
  pub email: Option<String>,
  pub bio: Option<String>,
  pub matrix_user_id: Option<String>,
  pub show_avatars: Option<bool>,
  pub send_notifications_to_email: Option<bool>,
  pub bot_account: Option<bool>,
  pub show_bot_accounts: Option<bool>,
  pub show_read_posts: Option<bool>,
  pub show_new_post_notifs: Option<bool>,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
  pub new_password: String,
  pub new_password_verify: String,
  pub old_password: String,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
  pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPersonDetails {
  pub person_id: Option<PersonId>, // One of these two are required
  /// Example: dessalines , or dessalines@xyz.tld
  pub username: Option<String>,
  pub sort: Option<String>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub community_id: Option<CommunityId>,
  pub saved_only: Option<bool>,
  pub auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPersonDetailsResponse {
  pub person_view: PersonViewSafe,
  pub comments: Vec<CommentView>,
  pub posts: Vec<PostView>,
  pub moderates: Vec<CommunityModeratorView>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRepliesResponse {
  pub replies: Vec<CommentView>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPersonMentionsResponse {
  pub mentions: Vec<PersonMentionView>,
}

#[derive(Serialize, Deserialize)]
pub struct MarkAllAsRead {
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddAdmin {
  pub person_id: PersonId,
  pub added: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddAdminResponse {
  pub admins: Vec<PersonViewSafe>,
}

#[derive(Serialize, Deserialize)]
pub struct BanPerson {
  pub person_id: PersonId,
  pub ban: bool,
  pub remove_data: Option<bool>,
  pub reason: Option<String>,
  pub expires: Option<i64>,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BanPersonResponse {
  pub person_view: PersonViewSafe,
  pub banned: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BlockPerson {
  pub person_id: PersonId,
  pub block: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockPersonResponse {
  pub person_view: PersonViewSafe,
  pub blocked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetReplies {
  pub sort: Option<String>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub unread_only: Option<bool>,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPersonMentions {
  pub sort: Option<String>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub unread_only: Option<bool>,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct MarkPersonMentionAsRead {
  pub person_mention_id: PersonMentionId,
  pub read: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonMentionResponse {
  pub person_mention_view: PersonMentionView,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAccount {
  pub password: String,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
  pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordResetResponse {}

#[derive(Serialize, Deserialize)]
pub struct PasswordChange {
  pub token: String,
  pub password: String,
  pub password_verify: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePrivateMessage {
  pub content: String,
  pub recipient_id: PersonId,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditPrivateMessage {
  pub private_message_id: PrivateMessageId,
  pub content: String,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePrivateMessage {
  pub private_message_id: PrivateMessageId,
  pub deleted: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct MarkPrivateMessageAsRead {
  pub private_message_id: PrivateMessageId,
  pub read: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPrivateMessages {
  pub unread_only: Option<bool>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateMessagesResponse {
  pub private_messages: Vec<PrivateMessageView>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateMessageResponse {
  pub private_message_view: PrivateMessageView,
}

#[derive(Serialize, Deserialize)]
pub struct GetReportCount {
  pub community_id: Option<CommunityId>,
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetReportCountResponse {
  pub community_id: Option<CommunityId>,
  pub comment_reports: i64,
  pub post_reports: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetUnreadCount {
  pub auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetUnreadCountResponse {
  pub replies: i64,
  pub mentions: i64,
  pub private_messages: i64,
}
