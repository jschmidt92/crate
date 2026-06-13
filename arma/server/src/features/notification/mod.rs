use forge_lib::{
    models::Notification, repositories::NotificationRepository, services::NotificationService,
    shared::NotificationError,
};

#[derive(Clone)]
pub(crate) struct NotificationFeature<R> {
    service: NotificationService<R>,
}

impl<R> NotificationFeature<R>
where
    R: NotificationRepository,
{
    pub(crate) const fn new(service: NotificationService<R>) -> Self {
        Self { service }
    }

    pub(crate) fn list(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        self.service.list(uid)
    }

    pub(crate) fn unread(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        self.service.unread(uid)
    }

    pub(crate) fn mark_read(&self, uid: &str, id: &str) -> Result<Notification, NotificationError> {
        self.service.mark_read(uid, id)
    }

    pub(crate) fn mark_all_read(&self, uid: &str) -> Result<Vec<Notification>, NotificationError> {
        self.service.mark_all_read(uid)
    }
}
