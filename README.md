# ml-inference-candle.rs
Rust Machine Learning Inference with HF Candle framework

## TODO

- [x] A consumer listening to Message queue (public test) on a particular subject/topic for "query" text message
- [] Testing for consumer and factory to be added
- [] Message is processed for embedding computation ONNX ML model and chat generation through Small LLM model
- [] Testing for embedding model and slm to be added
- [] Write outbound for object storage interfaces.
- [] Both the results are combined along with original query text message into a result.json file. This result.json file is prepended with an 8 character random hash number and date for identification in the file name as : YYYYMMDD-<8 chars hash>-result.json. The resulting file is then stored in a Minio bucket (public test)
- [] Testing for outbound object storages and file activities to be added

## Development plan

- 4 layer structure
- Dev and Test for each module/func in parallel
- Async and sync combination
- Binary / CLI service as output

## Installations

`cargo install cargo-llvm-cov` - To install coverage tool
`cargo uninstall cargo-llvm-cov` - To uninstall coverage tool

## NATS Telnet communication

`telnet demo.nats.io 4222`
`CONNECT {}`
`PING`
`PUB bmonikraj.ml-inference-candle.rs 49`
`{"query": "How are you doing this fine evening?"}`