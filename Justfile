help:
    just --list

generate-api: && cleanup
    openapi-generator-cli generate -g rust -i spec/nbp.json -o openapi-generator-output
    cp -r openapi-generator-output/src/{apis,models} src/openapi
    # Fixup imports
    sed -i -e 's/use crate::/use crate::openapi::/' src/openapi/**/*.rs

cleanup:
    rm -r openapi-generator-output