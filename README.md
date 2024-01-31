# Webhook Transformer

> [!WARNING]
> Alpha software

A small service to transform incoming HTTP POST requests using Jsonnet and
forwards them to the configured destination.

## Usage

There's a pre-compiled Docker image that you can use:

```sh
docker run \
    -p 8080 \
    -v /path/to/config.jsonnet:/config.jsonnet \
    ghcr.io/khuedoan/webhook-transformer:v0.0.1 --config /config.jsonnet
```

I personally run it as a Kubernetes sidecar to transform Alertmanager webhooks
to ntfy format, but you can use it to transform anything you like.
There are some examples in the `examples/` directory:

- [Basic](./examples/basic/config.jsonnet)
- [Alertmanager to ntfy](./examples/alertmanager-to-ntfy/config.jsonnet)

## Development

Open the Nix shell:

```sh
nix develop
```

Run the server:

```sh
make dev
```

The default development server will transform Alertmanager request and forward
it to ntfy. You can visit <https://ntfy.sh/webhook-transformer> and try it out:

```sh
curl \
    --request POST \
    --header "Content-Type: application/json" \
    --data @examples/alertmanager-to-ntfy/input.json \
    http://localhost:8080
```

After making your changes, run test suite with:

```sh
make test
```
