#![allow(unused_doc_comment)]

error_chain!{
    foreign_links {
        Twitter(::egg_mode::error::Error);
        Serde(::serde_json::error::Error);
        Io(::std::io::Error);
        Db(::soundlines_core::db::Error);
        DbPool(::soundlines_core::r2d2::GetTimeout);
        Forecast(::forecast_io::Error);
    }

    errors {
        WeatherDataUnavailable(lat: f64, lng: f64) {
            description("Weather data unavailable")
            display("Current weather data is not available for '{}, {}'", lat, lng)
        }
    }
}
