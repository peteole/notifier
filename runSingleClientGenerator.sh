docker run -v $(pwd)/clients:/local openapitools/openapi-generator-cli generate -i local/openAPI.json --git-user-id peteole -g go -o /local/go2