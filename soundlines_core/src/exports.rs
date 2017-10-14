use jni::objects::JClass;
use jni::objects::JObject;
use jni::JNIEnv;
use jni::sys::jstring;

use db;

pub struct SoundlinesCore {
	conn: db::Connection
}

static mut CTX: Option<SoundlinesCore> = None;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Java_org_soundlines_visualization_SoundlinesCore_init(_: JNIEnv, _: JClass) {
	let conn = db::connect();
	unsafe {
		CTX = Some(SoundlinesCore { conn });
	}
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Java_org_soundlines_visualization_SoundlinesCore_getEntities(env: JNIEnv, _: JClass) -> jstring {
	let json = || {
		let conn = unsafe { &CTX.as_ref().unwrap().conn };
		let entities = db::entities::all(conn)?;
		let entities_json = entities.into_iter().map(|r| r.get::<_, String>(0)).collect::<Vec<String>>().join(",");
		Ok(format!("{{ \"entities\": [{}] }}", entities_json))
	};

	to_jstring(&env, json())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Java_org_soundlines_visualization_SoundlinesCore_getCells(env: JNIEnv, _: JClass) -> jstring {
	let json = || {
		let conn = unsafe { &CTX.as_ref().unwrap().conn };
		let cells = db::cells::all(conn)?;
		let cells_json = cells.into_iter().map(|r| r.get::<_, String>(0)).collect::<Vec<String>>().join(",");
		Ok(format!("{{ \"cells\": [{}] }}", cells_json))
	};

	to_jstring(&env, json())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Java_org_soundlines_visualization_SoundlinesCore_getSeeds(env: JNIEnv, _: JClass) -> jstring {
	let json = || {
		let conn = unsafe { &CTX.as_ref().unwrap().conn };
		let seeds = db::seeds::all(conn)?;
		let seeds_json = seeds.into_iter().map(|r| r.get::<_, String>(0)).collect::<Vec<_>>().join(",");
		Ok(format!("{{ \"seeds\": [{}] }}", seeds_json))
	};

	to_jstring(&env, json())
}

fn to_jstring(env: &JNIEnv, value: ::errors::Result<String>) -> jstring {
	match value.and_then(|json| env.new_string(json).map_err(::errors::Error::from)) {
		Ok(json) => json.into_inner(),
		Err(err) => {
			error!("{}", err);
			JObject::null().into_inner()
		}
	}
}