// Widget module
//
// Custom widgets for displaying notifications.

pub mod notification_card;
pub mod notification_list;

// Re-export commonly used functions
pub use notification_card::notification_card;
pub use notification_list::notification_list;
