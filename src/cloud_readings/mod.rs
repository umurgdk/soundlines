use futures::Future;
use futures::Stream;
use tokio_core::reactor::Core;
use twitter_stream::Token;
use twitter_stream::TwitterStreamBuilder;

pub fn subscribe_to_twitter() {
    let token = Token::new(// consumer key and secret
                           "ZBFo3rW87dXNnLkbBg75vL7PS", 
                           "OkQwhYTVVJOjGsPw1arK51B7ziqDo8E93fB6tmyb3gzJy9RVQy",
                           // access token and secret
                           "365459783-mZEFoiD9rt8PvGMUJtS77LB1JyItBdN6t0ST2HC8", 
                           "Ujv3QlFyTr2dsqLhlAdozhoBlWV9zleeHJ675GWTl7c4E");

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let future = TwitterStreamBuilder::user(&token)
        .handle(&handle)
        .timeout(None)
        .replies(false)
        //.locations()
        .listen()
        .flatten_stream()
        .for_each(|json| {
            println!("{}", json);
            Ok(())
        });

    core.run(future).unwrap();
}
