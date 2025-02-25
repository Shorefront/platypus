# TLS Configuration

TLS is enabled by default behind the feature ["tls"]

## Configuration variables

The TLS configuration requires a secret key and a certificate to operate. These are set by the following variables:

- TLS_CERT : Deafults to tls/cert.pem
- TLS_KEY : Defaults to tls/key.pem

### Environment Variables

Any configuration variable can be overriden (if there's a default) by setting an environment variable with the same name.
