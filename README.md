# gitopsify

Project to grab existing helm/kustomization or other kubernetes components and convert it to FluxCD manifests.

Generating helmresources from existing release:
```shell
   cargo run -- convert --namespace <namespace> --chart <chart name> --output ./output --url <helm-repo-url>  --release <relaese_name>
```

