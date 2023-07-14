.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONY: stream_local
stream_local: build
	substreams run substreams.yaml map_outputs --plaintext -e localhost:9000 -s $(START_BLOCK) -t +1
