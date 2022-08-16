use std::sync::Arc;

use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use tokio::sync::Mutex;

use common::model::{Event, EventBody};
use common::types::{EmptyResult, TypedResult};

use crate::config::Mongo;

#[derive(Debug)]
pub struct EventCollection {
    collection: Collection<Event>,
}

impl EventCollection {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<Mutex<EventCollection>> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let event_collection = db.collection::<Event>(&mongo_config.event_collection);

        Box::leak(Box::new(Arc::new(Mutex::new(EventCollection {
            collection: event_collection,
        }))))
    }
}

impl EventCollection {
    pub async fn save(&self, event_body: EventBody) -> EmptyResult {
        let event = Event {
            emitted_at: Utc::now(),
            body: event_body,
        };
        let _ = &self.collection.insert_one(event, None).await?;
        Ok(())
    }

    pub async fn find(&self) -> TypedResult<Vec<Event>> {
        let events = self
            .collection
            .find(None, None)
            .await?
            .try_collect()
            .await?;

        Ok(events)
    }
}
