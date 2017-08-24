use env_logger;
use egg_mode;

pub fn subscribe_to_twitter() {
    env_logger::init().expect("Coludn't initialize env_logger");

    let consumer_token = egg_mode::KeyPair::new("ZBFo3rW87dXNnLkbBg75vL7PS", 
                                                "OkQwhYTVVJOjGsPw1arK51B7ziqDo8E93fB6tmyb3gzJy9RVQy");

    let access_token = egg_mode::KeyPair::new("365459783-mZEFoiD9rt8PvGMUJtS77LB1JyItBdN6t0ST2HC8", 
                                              "Ujv3QlFyTr2dsqLhlAdozhoBlWV9zleeHJ675GWTl7c4E");
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let distance = egg_mode::search::Distance::Kilometers(0.2);
    let search = egg_mode::search::search("")
        //.geocode(37.569436, 127.001926, distance)
        .geocode(37.546007, 126.921622, distance)
        .call(&token)
        .unwrap();


    for tweet in &search.statuses {
        println!("(@{}) {} {} at {:?}", tweet.user.as_ref().unwrap().screen_name, tweet.text, tweet.created_at, tweet.coordinates);
    }
}

