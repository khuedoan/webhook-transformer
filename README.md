# Webhook Transformer

> [!WARNING]
> Alpha software

A small application that listens for incoming HTTP POST requests, transforms the
payload using a provided Jsonnet configuration, and then forwards the transformed
payload to an upstream host. This is useful as an adapter to integrate applications
that don't support each other's webhook formats.

## Usage

There's a pre-compiled Docker image that you can use:

```sh
docker run \
    -p 8080 \
    -v /path/to/config.jsonnet:/config.jsonnet \
    ghcr.io/khuedoan/webhook-transformer:v0.0.1 --config /config.jsonnet
```

The transformer automatically injects some global variables:

| Name   | Type     | Description                              |
| ------ | -------- | -----------------------------------------|
| `body` | `object` | The JSON payload from the webhook        |
| `env`  | `object` | Key-value pairs of environment variables |

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

After making your changes, run the test suite:

```sh
make test
```
