ENDPOINT ?= eth.substreams.pinax.network:443
START_BLOCK_1 ?= 15096140
START_BLOCK_2 ?= 18678565

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run submitted_bids -e $(ENDPOINT) -s $(START_BLOCK_2) -t +10

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
