# Storm storage daemon

Storage daemon for Storm and other LNP/BP nodes. It is a microservice frontend
for different storage backends

## Run with Docker

### Build

Clone the repository and checkout to the desired version (here `v0.8.0`):

```console
$ git clone https://github.com/Storm-WG/storm-stored
$ cd storm-stored
$ git checkout v0.8.0
```

Build and tag the Docker image:

```console
$ docker build -t storm-stored:v0.8.0 .
```
