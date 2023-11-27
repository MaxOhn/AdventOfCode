pub fn main() {
    let logs = tracing_subscriber::fmt();

    #[cfg(feature = "log-all")]
    let logs = logs.with_env_filter("aoc_solver=trace,info");

    logs.with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .without_time()
        .with_ansi(false)
        .with_target(false)
        .init();

    console_error_panic_hook::set_once();

    leptos::mount_to_body(aoc_solver::App);
}
