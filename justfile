

run:
    cargo r --bin configurator


# require to git clone https://github.com/json-schema-org/JSON-Schema-Test-Suite
test_suite:
    cargo test test_all_suite -- --nocapture