use std::sync::Arc;
use std::sync::Mutex;

use egg_mode;
use egg_mode::tweet::Tweet;

use soundlines_core::db::extensions::*;
use soundlines_core::db::models::Cell;
use soundlines_core::db::PooledConnection;
use soundlines_core::db::Result as DbResult;
use soundlines_core::postgis::ewkb::Point;

use config;
use config::Config;
use errors::*;

pub struct TweetRunner {
    pub conn: PooledConnection,
    pub config: Arc<Mutex<Config>>
}

impl TweetRunner {
    pub fn new(conn: PooledConnection, config: Arc<Mutex<Config>>) -> Self {
        TweetRunner { conn, config }
    }

    fn update_cell_for_coord(&self, (latitude, longitude): (f64, f64)) -> DbResult<Option<Cell>> {
        Ok(Cell::find_containing_core(&*self.conn, &Point::new(longitude, latitude, Some(4326)))?
            .map(|mut cell| {
                cell.sns += 1;
                cell
            }))
    }

    pub fn run(&mut self) -> Result<()> {
        let last_tweet_id = match self.config.lock() {
            Ok(config) => (*config).last_tweet_id.clone(),
            _          => return Err(Error::from("Lock failed"))
        };

        let egg_mode::search::SearchResult { statuses, max_id, .. } = get_tweets(last_tweet_id)?;
        let mut cells = vec![];

        if statuses.len() == 0 {
            return Ok(());
        }

        for tweet in statuses.into_iter() {
            let coordinates = tweet.coordinates.clone();
            if coordinates.is_none() {
                continue;
            }

            let updated_cell = self.update_cell_for_coord(coordinates.unwrap())?;
            if updated_cell.is_none() {
                continue;
            }

            log_tweet(&tweet);

            cells.push(updated_cell.unwrap());
        }


        let cell_ids = cells.iter().map(|cell| cell.id).collect::<Vec<_>>();
        self.conn.update_batch(&cell_ids, &cells)?;

        println!("Increased sns number for {} cells", cells.len());

        match self.config.lock() {
            Ok(mut config) => {
                (*config).last_tweet_id = Some(max_id);
                config::save(&*config)?;
            },

            _ => return Err(Error::from("Failed to acquire config mutex lock"))
        }

        Ok(())
    }
}

fn get_tweets(since: Option<u64>) -> Result<egg_mode::search::SearchResult<'static>> {
    let consumer_token = egg_mode::KeyPair::new("ZBFo3rW87dXNnLkbBg75vL7PS",
                                                "OkQwhYTVVJOjGsPw1arK51B7ziqDo8E93fB6tmyb3gzJy9RVQy");

    let access_token = egg_mode::KeyPair::new("365459783-mZEFoiD9rt8PvGMUJtS77LB1JyItBdN6t0ST2HC8",
                                              "Ujv3QlFyTr2dsqLhlAdozhoBlWV9zleeHJ675GWTl7c4E");
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let distance = egg_mode::search::Distance::Kilometers(1.2);
    let mut search = egg_mode::search::search("-")
        .geocode(37.570750, 126.997702, distance);

    if since.is_some() {
        search = search.since_tweet(since.unwrap());
    }

    search
        .call(&token)
        .map_err(|err| err.into())
        .map(|response| { response.response })
        .into()
}

fn log_tweet(tweet: &Tweet) {
    let coordinates = tweet.coordinates.map(|(lat, lng)| format!("({}, {})", lat, lng)).unwrap_or("".into());
    let username = tweet.user.as_ref().map(|user| user.name.as_str()).unwrap_or("");
    let (end_index, _) = tweet.text.char_indices().nth(30).or(tweet.text.char_indices().last()).unwrap();
    println!("@{} {} {} {}", username, tweet.created_at, coordinates, tweet.text.get(0..end_index).unwrap_or(""));
}
