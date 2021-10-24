targets = x86_64-pc-windows-gnu x86_64-unknown-linux-gnu

release:
	for target in $(targets) ; do \
		echo $$target ; \
		cargo build --release --target=$$target ; \
	done

