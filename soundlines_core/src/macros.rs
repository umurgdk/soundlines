#[macro_export]
macro_rules! timer_finish {
    ($v:ident) => {
		let (name, millis) = $v.finish();
        trace!("{} took {}ms", name, millis);
    };
}

#[macro_export]
macro_rules! measure {
	($n:expr, $v:expr) => {{
		let timer = ::helpers::new_timer($n);
		let res = $v;
		timer_finish!(timer);
		res
	}};
}