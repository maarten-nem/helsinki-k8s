# k8s-random-string

Prints a timestamp and random UUID every 5 seconds.

## Build

```bash
docker build -t k8s-random-string .
```

## Run

```bash
docker run k8s-random-string
```

## Deploy

```bash
kubectl apply -f manifests/
```
