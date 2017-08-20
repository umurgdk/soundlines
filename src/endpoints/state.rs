use std::sync::Mutex;
use std::sync::MutexGuard;

use rocket_contrib::Json;

use db::Connection;
use db::models::Parameter;

pub struct ParametersManager(Mutex<Parameter>);

impl ParametersManager {
    pub fn from_db(conn: &Connection) -> ParametersManager {
        let parameters = Parameter::fetch(conn).expect("Failed to read parameters from db!");
        ParametersManager(Mutex::new(parameters))
    }

    pub fn get(&self) -> MutexGuard<Parameter> {
        self.0.lock().expect("Failed to get parameters lock!")
    }

    pub fn update_with<C>(&self, _: &Connection, mut closure: C)
        where C: FnMut(&mut Parameter) -> ()
    {
        let mut parameter = self.get();
        closure(&mut *parameter);

        //TODO: parameter.save_changes::<Parameter>(conn).expect("Failed to update parameters!");
    }

    pub fn to_json(&self) -> Json<Parameter> {
        let parameter = self.get();
        Json((*parameter).clone())
    }
}
