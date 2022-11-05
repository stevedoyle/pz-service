# Power Zone Service

A simple HTTP service for calculating cycling power zones.

This is being used as a small project to explore the various serverless
environments available. The goal is to publish this simple service on different
serverless environments and to capture how-tos, and notes on the experience and
learnings.

## Usage
Right now the service sets up a server on port 3030 on `localhost`

```bash
cargo run
curl -L http://127.0.0.1:3030/ftp/210
```
