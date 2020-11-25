use crate::google::storage::v1::{
    DeleteNotificationRequest, GetNotificationRequest, InsertNotificationRequest,
    ListNotificationsRequest, ListNotificationsResponse, Notification,
};
use crate::query::Query;
use crate::request::Request;
use crate::urls::Urls;
use crate::{Client, Result};
use reqwest::Method;
use url::Url;

fn notification_configs_url(base_url: Url, bucket: &str) -> Result<Url> {
    Ok(base_url
        .bucket(bucket)?
        .join_segment("notificationConfigs")?)
}

impl Query for DeleteNotificationRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for DeleteNotificationRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn request_path(&self, base_url: Url) -> Result<Url> {
        Ok(notification_configs_url(base_url, &self.bucket)?.join(&self.notification)?)
    }
}

impl Query for GetNotificationRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for GetNotificationRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = Notification;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        Ok(notification_configs_url(base_url, &self.bucket)?.join(&self.notification)?)
    }
}

impl Query for InsertNotificationRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for InsertNotificationRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = Notification;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        notification_configs_url(base_url, &self.bucket)
    }
}

impl Query for ListNotificationsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for ListNotificationsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListNotificationsResponse;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        notification_configs_url(base_url, &self.bucket)
    }
}

impl Client {
    #[doc = " Permanently deletes a notification subscription."]
    #[doc = " Note: Older, \"Object Change Notification\" push subscriptions should be"]
    #[doc = " deleted using StopChannel instead."]
    pub async fn delete_notification(
        &self,
        request: impl Into<DeleteNotificationRequest>,
    ) -> Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " View a notification configuration."]
    pub async fn get_notification(
        &self,
        request: impl Into<GetNotificationRequest>,
    ) -> Result<Notification> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Creates a notification subscription for a given bucket."]
    #[doc = " These notifications, when triggered, publish messages to the specified"]
    #[doc = " Cloud Pub/Sub topics."]
    #[doc = " See https://cloud.google.com/storage/docs/pubsub-notifications."]
    pub async fn insert_notification(
        &self,
        request: impl Into<InsertNotificationRequest>,
    ) -> Result<Notification> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Retrieves a list of notification subscriptions for a given bucket."]
    pub async fn list_notifications(
        &self,
        request: impl Into<ListNotificationsRequest>,
    ) -> Result<ListNotificationsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }
}
