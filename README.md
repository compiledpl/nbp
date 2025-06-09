# Client for National Bank of Poland Web API

This is a client for the National Bank of Poland [Web API](https://api.nbp.pl/en.html), which provides access to various financial data such as exchange rates, interest rates, and more.

The client is designed to be easy to use and provides a simple interface for accessing the data.

## Development

### Prerequisites

* [openapi-generator-cli](https://openapi-generator.tech/docs/installation/)
* [just](https://github.com/casey/just)

All additional tasks are described in the *Justfile*. Currently it consists of bindings generation based on the OpenAPI spec found under `spec/nbp.json`.

**All files under `src/openapi` are generated. Do not modify them manually as they can be overwritten.**

#### Updating Open API Spec

Useful resources:
* [Open API 3.0 Guide](https://swagger.io/docs/specification/v3_0/about/)

1. Update the spec in spec/nbp.json
2. Generate bindings

#### Bindings generation

Run `just generate-api`. The generated files will be put in `src/openapi/`.

