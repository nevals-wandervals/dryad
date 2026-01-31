use crate::client::MetadataApp;

pub trait AppRun<'a, T, E>: AppBuilder<'a, T> + EventHandle<E> {
    fn run(&mut self) {
        let mut event = self.event_init();
        self.get_mut_metadata().set_is_run(true);
        while self.get_metadata().is_run() {
            self.event_handler(&mut event);
            self.update();
            self.render();
        }
    }
}

pub trait AppBuilder<'a, T> {
    fn init(md: MetadataApp<'a, T>) -> Self;
    fn update(&mut self);
    fn render(&mut self);
    fn get_metadata(&self) -> &MetadataApp<'a, T>;
    fn get_mut_metadata(&mut self) -> &mut MetadataApp<'a, T>;
}

pub trait EventHandle<E> {
    fn event_handler(&mut self, event: &mut E);
    fn event_init(&self) -> E;
}
