use serde::Serialize;
use serde_yaml;

#[derive(Serialize)]
pub struct HelmRelease {
    apiVersion: String,
    kind: String,
    metadata: Metadata,
    spec: Spec,
}

#[derive(Serialize)]
struct Metadata {
    name: String,
    namespace: String,
}

#[derive(Serialize)]
struct Spec {
    interval: String,
    chart: Chart,
    values: serde_yaml::Value,
}

#[derive(Serialize)]
struct Chart {
    spec: ChartSpec,
}

#[derive(Serialize)]
struct SourceRef {
    kind: String,
    name: String,
    namespace: String,
}

#[derive(Serialize)]
struct ChartSpec {
  chart: String,
  interval: String,
  sourceRef: SourceRef,
  version: String,
}

#[derive(Serialize)]
struct HelmRepository {
    apiVersion: String,
    kind: String,
    metadata: Metadata,
    spec: HelmRepoSpec,
}

#[derive(Serialize)]
struct HelmRepoSpec {
    interval: String,
    url: String,
}

pub fn generate_helmrepo(chart_name: &str, namespace: &str,  repo_url: &str) -> String {
    let helmrepo = HelmRepository {
        apiVersion: "source.toolkit.fluxcd.io/v1".into(),
        kind: "HelmRepository".into(),
        metadata: Metadata {
                name: format!("{}-repo", chart_name),
                namespace: namespace.into(),
        },
       spec: HelmRepoSpec {
           interval: "1h".into(),
           url: repo_url.into(),
       },
    };

    serde_yaml::to_string(&helmrepo).unwrap()
}

pub fn generate_helmrelease(chart_name: &str, namespace: &str, values_yaml: &str) -> String {
    let values: serde_yaml::Value = serde_yaml::from_str(values_yaml).unwrap_or_else(|_| { 
        serde_yaml::Value::Null
    });

    let helmrelease = HelmRelease {
        apiVersion: "helm.toolkit.fluxcd.io/v2".into(),
        kind: "HelmRelease".into(),
        metadata: Metadata {
            name: chart_name.into(),
            namespace: namespace.into(),
        },
        spec: Spec {
            interval: "5m".into(),
            chart: Chart {
                spec: ChartSpec {
                    interval: "5m".into(),
                    chart: chart_name.into(),
                    version: "1.9.0".into(), // Fixme: Hardcoded!!
                    sourceRef: SourceRef {
                        namespace: namespace.into(),
                        kind: "HelmRepository".into(),
                        name: format!("{}-charts", chart_name),
                    },
                },
            },
            values,
        },
    };

    serde_yaml::to_string(&helmrelease).unwrap()
}


