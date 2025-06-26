# gitopsify

Project to grab existing helm/kustomization or other kubernetes components and convert it to FluxCD manifests.

build:
 ```shell
    cargo build --release
 ```
 
Generating helmresources from existing release:
```shell
   [gitopsify](gitopsify) --namespace <namespace> --chart <chart name> --output ./output --url <helm-repo-url>  --release <relaese_name>
```

