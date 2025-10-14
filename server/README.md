## How to build and run

At the repo root (so `cd ..` first)

```
docker build . -t reth-server:latest

docker run --gpus all -p 8000:8000 reth-server:latest
```
