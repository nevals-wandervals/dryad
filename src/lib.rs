use crate::client::traits::AppRun;

pub mod client;
pub mod entities;
pub mod logic;
pub mod math;
pub mod render;

pub fn start<'a, C, T, E>(mut client: C)
where
    C: AppRun<'a, T, E>,
{
    if client.get_metadata().is_init() {
        client.run();
    }
}
