# rpc-proxy

`cargo run` to run and then `curl -v localhost:3000/health` or `curl -X POST -H "Content-Type:application/json" "http://localhost:3000/v1?chainId=eip155:5&projectId=someid" -d '{"id":"1660887896683","jsonrpc":"2.0","method":"eth_chainId","params":[]}'`

### Docker

```console
$ docker build . --tag rpc-proxy:
$ docker run -p 3000:3000 \
    -e INFURA_PROJECT_ID=<some_id> \
    -e REGISTRY_API_URL=<registry_url> \
    -e REGISTRY_API_AUTH_TOKEN=<token> \
    --name rpc -it rpc-proxy
```
