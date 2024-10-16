.PHONY: stream_local
stream_local: build
	substreams run substreams.yaml map_outputs --plaintext -e localhost:9000 -s $(START_BLOCK) -t +1

.PHONY: stream_fleet_payments
stream_fleet_payments: build
	substreams run substreams.yaml map_outputs -e mainnet.sol.streamingfast.io:443 -s 180279461 -t +1

.PHONY: stream_regular_payments
stream_regular_payments: build
	substreams run substreams.yaml map_outputs -e mainnet.sol.streamingfast.io:443 -s 200974959 -t +1

.PHONY: stream_ai_trainer_payments
stream_ai_trainer_payments: build
	substreams run substreams.yaml map_outputs -e mainnet.sol.streamingfast.io:443 -s 200975925 -t +1
