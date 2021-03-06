fn enumerate_processes() {
	use sysinfo::SystemExt;
	let sys = sysinfo::System::new_all();
	for (pid, process) in sys.get_processes() {
		println!("[TRACE] {} {:?}", pid, process);
	}
}

/// アプリケーションのエントリーポイント
fn main() {
	enumerate_processes();
}
