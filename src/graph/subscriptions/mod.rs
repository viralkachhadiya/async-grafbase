use std::time::Duration;

use async_graphql::{futures_util::Stream, Subscription};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    pub async fn get_unread_notification_count(&self) -> impl Stream<Item = i32> {
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                yield 1;
            }
        }
    }
}
