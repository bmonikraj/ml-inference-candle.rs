# ml-inference-candle.rs
Rust Machine Learning Inference with HF Candle framework

## Task Plan

- A consumer listening to Message queue (public test) on a particular subject/topic for "query" text message
- Consumed message is parallely passed for embedding computation ONNX ML model and chat generation through Small LLM model
- Both the results are combined along with original query text message into a result.json file 
- This result.json file is prepended with an 8 character random hash number and date for identification in the file name as : YYYYMMDD-<8 chars hash>-result.json
- The resulting file is then stored in a Minio bucket (public test)

## Development plan

- 4 layer structure
- Dev and Test for each module/func in parallel
- Async and sync combination
- Binary / CLI service as output