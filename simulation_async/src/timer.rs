use std::time::Instant;

pub struct Timer {
	start: Instant,
	name: String
}

impl Timer {
	pub fn new<S: Into<String>>(name: S) -> Timer {
		Timer { start: Instant::now(), name: name.into() }
	}

	pub fn finish(self) -> (String, u64) {
		let since_start = Instant::now().duration_since(self.start);
		let mut millis = since_start.as_secs() * 1000;
		millis += since_start.subsec_nanos() as u64 / 1000000u64;

		(self.name, millis)
	}
}

pub fn new<S: Into<String>>(name: S) -> Timer {
	Timer::new(name)
}

macro_rules! timer_finish {
    ($v:ident) => {
		let (name, millis) = $v.finish();
        trace!("TIMER {} took {}ms", name, millis);
    };
}

macro_rules! measure {
	($n:expr, $v:expr) => {
		let timer = ::timer::new($n);
		let res = $v;
		timer_finish!(timer);
		res
	};
}