# Conga
Basic HTTP queue that stores JSON objects in memory. 

Allows users to POST JSON objects that are then stored in a queue. JSON Objects can then be previewed and fetched (Ingested and Removed) from the queue.

API keys can be configured by supplying the `api_keys` string array in the config (see sample provided in config/conga.toml). If no keys are supplied, auth is disabled.

## WIP
Project is in active development, and is currently missing some features (e.g static storage).

# Links
Some useful links
- [Docker](https://hub.docker.com/r/coombszy/conga) ![Docker](https://img.shields.io/docker/pulls/coombszy/conga)
- [Github](https://github.com/Coombszy/conga)