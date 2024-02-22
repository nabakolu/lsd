release: target/release/lsd

install: release
	install -Dm 755 "target/release/lsd" "/usr/local/bin/lsd"
	install -Dm 644 "usr/bash-completion/completions/lsd" "/usr/local/share/bash-completion/completions/lsd"
	install -Dm 644 "usr/zsh/site-functions/_lsd" "/usr/local/share/zsh/site-functions/_lsd"
	install -Dm 644 "usr/man/man1/lsd.1.gz" "/usr/local/share/man/man1/lsd.1.gz"


target/release/lsd: src/
	cargo build --release


uninstall:
	rm "/usr/local/bin/lsd"
	rm "/usr/local/share/bash-completion/completions/lsd"
	rm "/usr/local/share/zsh/site-functions/_lsd"
	rm "/usr/local/share/man/man1/lsd.1.gz"
