use crate::GlobalAppState;

// Nested procedures, you can also do this (path = "api.events.users")
#[taurpc::procedures(path = "events", event_trigger = ApiEventTrigger)]
pub trait Events {
    #[taurpc(event)]
    async fn ev(log: String);
}

#[derive(Clone)]
pub(crate) struct EventsImpl {
    pub _state: GlobalAppState,
}

#[taurpc::resolvers]
impl Events for EventsImpl {}
