error_chain! {
	foreign_links {
		Db(::postgres::Error);
		Serialization(::serde_json::Error);
		Log(::log::SetLoggerError);
		Jni(::jni::errors::Error);
	}
}