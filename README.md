# Webhook Transformer

## Usage

This tool uses Jsonnet to transform your webhook payload,
please see the `examples/` directory for example configurations.

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
