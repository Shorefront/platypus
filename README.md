# Platypus

![Cargo Build](https://github.com/rruckley/platypus/actions/workflows/rust.yml/badge.svg)

## Open Source Telco Platform

### Description

This platform provides ODA compliant components to provide base TMF ODA aligned functionality using the Rust programming language.

### Features

Functionalality can be enabled by either TMF API number or via TMFC component number.

Each will have a v4 and v5 variant (where available).

## TLS Support

Server is coded for TLS by default. This requires a certificate and key file in PEM format in the 'tls' folder (by default) to be able to start. The location of these files can be overridden by environment variables TLS_CERT and TLS_KEY respectively.

[Configuration details](TLS-CONFIG.md)
