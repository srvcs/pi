# srvcs-pi

## Name

| Field | Value |
| --- | --- |
| Service | `srvcs-pi` |
| Slug | `pi` |
| Repository | `srvcs/pi` |
| Package | `srvcs-pi` |
| Kind | `constant` |

## Function

constant: pi

This is a zero-argument constant. `POST /` returns pi in the `result` field;
any request body is ignored.

## Dependencies

None.

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | Service identity |
| `POST` | `/` | Evaluate the service function |
| `GET` | `/healthz` | Liveness probe |
| `GET` | `/readyz` | Readiness probe |
| `GET` | `/metrics` | Prometheus metrics |
| `GET` | `/openapi.json` | OpenAPI document |

## Inputs

This service accepts an empty or ignored request body.

## Outputs

| Name | Type |
| --- | --- |
| `result` | `number` |

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SRVCS_BIND_ADDR` | `0.0.0.0:8080` | Bind address |
| `SRVCS_ENV` | `development` | Environment label for logs |
| `RUST_LOG` | `info,tower_http=info` | Tracing filter |

## Error Behavior

This constant service has no input fields and no dependencies, so normal
evaluation returns `200`.

## Local Checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

See the [srvcs service standard](https://github.com/srvcs/platform/blob/main/STANDARD.md) for the full operational contract.

## Metadata

Machine-readable service metadata lives in `srvcs.yaml`. Keep it aligned with this README when the service contract changes.
