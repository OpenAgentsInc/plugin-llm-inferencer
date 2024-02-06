## Build
`cargo build --target wasm32-unknown-unknown --release`

## Test build locally
`extism call target/wasm32-unknown-unknown/release/plugin_llm_inferencer.wasm inference --input '{"model_name": "gpt-3.5-turbo", "input_content": "Say this is a test!", "api_key": "<OPENAI_API_KEY>"}' --allow-host="*"`
