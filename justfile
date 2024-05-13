css:
	cd assets/ && npm run css

run: css
	cargo watch -x check -x test -x 'shuttle run'
