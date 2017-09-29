use forecast_io;
use forecast_io::PrecipitationType;

use soundlines_core::db::extensions::*;
use soundlines_core::db::models::Weather;
use soundlines_core::db::PooledConnection;

use errors;
use errors::*;

pub struct WeatherRunner {
	conn:  PooledConnection,
}

impl WeatherRunner {
	const LATITUDE: f64 = 37.570628;
	const LONGITUDE: f64 = 126.996350;

	pub fn new(conn: PooledConnection) -> WeatherRunner {
		WeatherRunner { conn }
	}

	pub fn run(&mut self) -> errors::Result<()> {
		let forecast = forecast_io::get_forecast("fedc77e38dd033363070e349d61acd90", Self::LATITUDE, Self::LONGITUDE)?;
		let current = forecast.currently.ok_or(ErrorKind::WeatherDataUnavailable(Self::LATITUDE, Self::LONGITUDE))?;
		let temperature = current.temperature.ok_or(ErrorKind::WeatherDataUnavailable(Self::LATITUDE, Self::LONGITUDE))?;

		let precip = current.precip_type.map(|precip| match precip {
			PrecipitationType::Rain => "rain".to_string(),
			PrecipitationType::Snow => "snow".to_string(),
			PrecipitationType::Sleet => "sleet".to_string()
		});

		print!("Weather update: {}", temperature);
		if precip.is_some() {
			print!(" / {}", precip.as_ref().unwrap());
		}
		print!("\n");

		if let Ok(Some(mut weather)) = Weather::get(&*self.conn) {
			weather.precip = precip;
			weather.temperature;
			self.conn.update(weather.id, &weather)?;
		} else {
			self.conn.insert(&Weather { id: -1, temperature, precip })?;
		}

		Ok(())
	}
}