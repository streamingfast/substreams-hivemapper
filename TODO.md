# TODO

## Extract various mint payments
1. mint to a token split contract for a fleet -> DONE
   - token splitting contract: EEjwuvCMVYjgHUeX1BM9qmUog59Pft88c3jbt2ATwcJw
   - this will create 2 `mintTo` instructions where the amount will be sent to the accounts (account 1 and account 2)
     we cannot know what account is the manager
2. payments from the foundation
   - top-up (out of region) program (assumption that fleet and non-fleet is the same thing)
     - ```bash
          {
             "info": {
                "amount": "19808387735",
                "authority": "CzcjceA4TvMguDjE7ZyZ4HWV9qxwv2ndHVHZSSeythfp",
                "destination": "9Ez4DVJxv74sGpjjYXAJjrMsuFRFRF3su6jkXwzSF6TP",
                "source": "AwjomjhbNqkgEZN1ADvbEcTYsDfBuX4AKzNmxwgtKvxM"
             },
             "type": "transfer"
          }
       ```
         > destination's owner is the account of a driver in an out of region drive / payment
         > source: where the money comes out of
     - fleet:
     - non-fleet:
   - bounties (to incentives certain locations for mapping)
3. regular mints -> regular (not part of a fleet)
4. all ai-trainer rewards

----------
Fleet Payment:
- 2aaq76rcgymcFdMt3urR3HZDPxsQS46iKzrTdcYQxzMzhPYnfNYq9tXETkkGqxn6TosQA9fpD1vd64Bj6ewTYYHJ
- 29nMoMk2RCUWWZe73vR8Vgqif4uvCZV3PCgwDCLZ6GJwPxfdoQDMUHJs8QYmwcKkwj2NWZLxsQfYRnuwKrV1TfuR
- 5EPQfSvbqhTc9JuY2huqWe26Hd51isfbjitCXMor6LdDSMXzLJ1UyFcrgMAL37vxnpRYoR6Ym9fhessH3NHUknqq

Fleet Payment with no split:
- 3gzmPixZHfZqaTEsjsoa2Jb3psvViEqpe7tjz467wNRLH75F8HvSmuaYV8C3NBuWi8RMoERNDL6DH7YfiidT3odw

Non Fleet Payment:
- 5CpDmrzqVccA8jHFotu3yysGbB4PFsv1s5VNy8g9Q1i9acovwiDfyFGmv9ytRuRVJNrfAJknmNZZniShFtUzUQsM

Ai Trainer Payment:
- 2FcqAY7dGmHD3fHqYKTFjgeTRVygrFpVfg55wPdznk2du1VJaQkP5MS2pgSSjXTdCsiDAL4MS7DAUCWRxc7ekS37