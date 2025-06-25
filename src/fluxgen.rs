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
struct ChartRef {
    kind: String,
    name: String,
    namespace: String,
}

#[derive(Serialize)]
struct Spec {
    interval: String,
    chartRef: ChartRef,
    releaseName: String,
    values: serde_yaml::Value,
}

#[derive(Serialize)]
struct Chart {
    apiVersion: String,
    kind: String,
    metadata: Metadata,
    spec: ChartSpec,
}

#[derive(Serialize)]
struct SourceRef {
    kind: String,
    name: String,
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
pub fn get_repo_name(name: &str) -> String {
  format!("{}-repo", name)
}
pub fn generate_helmrepo(name: &str, namespace: &str,  repo_url: &str) -> String {
    let helmrepo = HelmRepository {
        apiVersion: "source.toolkit.fluxcd.io/v1".into(),
        kind: "HelmRepository".into(),
        metadata: Metadata {
                name: get_repo_name(&name),
                namespace: namespace.into(),
        },
       spec: HelmRepoSpec {
           interval: "1h".into(),
           url: repo_url.into(),
       },
    };

    serde_yaml::to_string(&helmrepo).unwrap()
}

pub fn generate_helmrelease(chart: &str, name: &str, namespace: &str, values_yaml: &str) -> String {
    let values: serde_yaml::Value = serde_yaml::from_str(values_yaml).unwrap_or_else(|_| { 
        serde_yaml::Value::Null
    });

    let helmrelease = HelmRelease {
        apiVersion: "helm.toolkit.fluxcd.io/v2".into(),
        kind: "HelmRelease".into(),
        metadata: Metadata {
            name: name.into(),
            namespace: namespace.into(),
        },
        spec: Spec {
            interval: "5m".into(),
            chartRef: ChartRef {
              kind: "HelmChart".into(),
              name: chart.into(),
              namespace: namespace.into(),
            },
            releaseName: name.into(),
            values,
        },
    };

    serde_yaml::to_string(&helmrelease).unwrap()
}

pub fn generate_helmchart(name: &str, namespace: &str, version: &str) -> String {
        let helm_chart =  Chart {
                apiVersion: "source.toolkit.fluxcd.io/v1".into(),
                kind: "HelmChart".into(),
                metadata: Metadata {
                    name: name.into(),
                    namespace: namespace.into(),
                },
                spec: ChartSpec {
                    interval: "5m".into(),
                    chart: name.into(),
                    version: version.into(),
                    sourceRef: SourceRef {
                        kind: "HelmRepository".into(),
                        name: get_repo_name(&name),
                    },
                },
            };
        serde_yaml::to_string(&helm_chart).unwrap()
}
