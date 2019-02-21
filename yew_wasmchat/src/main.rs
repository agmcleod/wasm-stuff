use wasmchat::{services::PubnubService, Model};
use web_logger;
use yew::prelude::*;

pub struct Context {
    pubnub: PubnubService,
}

impl AsMut<PubnubService> for Context {
    fn as_mut(&mut self) -> &mut PubnubService {
        &mut self.pubnub
    }
}

fn main() {
    web_logger::init();
    yew::initialize();

    let context = Context {
        pubnub: PubnubService::new(
            "pub-c-fa1ae07a-8f0d-47a1-abfa-61e0bde665d1",
            "sub-c-a6892fce-357f-11e9-99ed-dea01fda7778",
        ),
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
