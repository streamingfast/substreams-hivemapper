# substreams-hivemapper

## Usage

[`hivemapper` on **substreams.dev**](https://substreams.dev/streamingfast/hivemapper/v0.1.8)

```bash
substreams gui https://spkg.io/streamingfast/hivemapper-v0.1.8.spkg
```

## Modules

### `map_outputs`

Outputs:

```protobuf
package hivemapper.types.v1;

message Output {
  repeated TokenSplittingPayment token_splitting_payments = 10;
  repeated RegularDriverPayment regular_driver_payments = 20;
  repeated NoSplitPayment no_split_payments = 30;

  repeated AiTrainerPayment ai_trainer_payments = 50;
  repeated OperationalPayment operational_payments = 51;
  repeated RewardPayment reward_payments = 52;
  repeated MapCreate map_create = 53;
  repeated MapConsumptionReward map_consumption_reward = 54;

  repeated Transfer transfers = 60;
  repeated Mint mints = 70;
  repeated Burn burns = 80;

  repeated InitializedAccount initialized_account = 120;
}
```

See details on [the registry](https://substreams.dev/streamingfast/hivemapper/v0.1.8?tab=modules&proto=hivemapper.types.v1.Output).

## License

Apache 2.0

## Contribute

```bash
substreams build
substreams auth
substreams gui
```
