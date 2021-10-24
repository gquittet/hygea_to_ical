targets = x86_64-pc-windows-gnu x86_64-unknown-linux-gnu

release:
	@echo "Releasing the new version 🚀" ;
	for target in $(targets) ; do \
		echo "Building $$target" ; \
		cargo build --release --target=$$target ; \
	done ;
	@echo "Done! 🎉" ;

