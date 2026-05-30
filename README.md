# srvcs-pi

A floating-point **constant** microservice for srvcs.cloud.

- **Concern:** `constant: pi`
- **Depends on:** nothing (leaf constant)
- **Result type:** `f64` (a JSON number with a fractional part)

It returns the mathematical constant pi: `std::f64::consts::PI`
(`3.141592653589793`). It has no dependencies, performs no input validation, and
calls no other service.

## API

### `GET /`

Service identity.

```json
{ "service": "srvcs-pi", "concern": "constant: pi", "depends_on": [] }
```

### `POST /`

Returns the constant. The request body is **ignored** — it may be empty, absent,
`{}`, or any JSON value.

```sh
curl -s -X POST localhost:8080/ -H 'content-type: application/json' -d '{}'
```

```json
{ "result": 3.141592653589793 }
```

`result` is an `f64`. Compare it approximately (`|got - expected| < 1e-9`),
never with exact float equality.

### Standard endpoints

`GET /healthz`, `GET /readyz`, `GET /metrics`, `GET /openapi.json`.

## Local checks

```sh
nix flake check -L
nix develop -c sh -euc 'cargo fmt --check; cargo clippy --all-targets -- -D warnings; cargo test'
nix build .#default -L
```

If the OpenAPI snapshot drifts, regenerate it:

```sh
UPDATE_OPENAPI=1 cargo test --test openapi_snapshot
```

See [`srvcs/platform`](https://github.com/srvcs/platform) for the shared service
standard and CI workflow.
