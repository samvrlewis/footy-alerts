#[derive(Debug, thiserror::Error)]
pub enum Error {}
pub struct Notifier;

impl Notifier {
    pub fn notify(&self, _user_id: u32) -> Result<(), Error> {
        todo!()
    }
}
