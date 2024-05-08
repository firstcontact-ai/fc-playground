const TRACE_FILTER: &str = r#"
app_desktop=debug,
lib_ais=debug,
lib_core=debug,
lib_rpc=debug,
lib_utils=debug,
lib_workers=debug
"#;

pub fn init_trace() {
	let trace_filter: String = TRACE_FILTER.chars().filter(|c| !c.is_whitespace()).collect();
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(true)
		.with_env_filter(trace_filter)
		.init();
}
