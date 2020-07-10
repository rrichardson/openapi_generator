# openapi_generator

Generate code from OpenAPI specifications.

# Quickstart

Run `just watch` to format, compile and run the server on changes and in another terminal run `just gen` to regenerate the server.

# ignore file

you can crate an ignore file in the target directory, the file should be named `openapi-generator.ignore`.

example:

```
# OpenAPI Generator Ignore
lib.rs
```