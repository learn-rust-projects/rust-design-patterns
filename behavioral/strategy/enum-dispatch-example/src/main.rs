use enum_dispatch::enum_dispatch;
use strum::{Display, EnumString, IntoStaticStr};
#[derive(Debug, Clone, Copy, IntoStaticStr, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(CmdExc)]
enum OutputFormat {
    Json(JsonObts),
    Csv(CsvObts),
    Yaml(YamlObts),
}
#[derive(Debug, Clone, Copy, Default)]
struct JsonObts;
impl CmdExc for JsonObts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Executing JSON output format");
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, Default)]
struct CsvObts;
impl CmdExc for CsvObts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Executing CSV output format");
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, Default)]
struct YamlObts;
impl CmdExc for YamlObts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Executing YAML output format");
        Ok(())
    }
}

#[enum_dispatch]
#[allow(async_fn_in_trait)] // cmd don't need send
pub trait CmdExc {
    async fn execute(self) -> anyhow::Result<()>;
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let formats = vec![
        OutputFormat::Json(JsonObts),
        OutputFormat::Csv(CsvObts),
        OutputFormat::Yaml(YamlObts),
    ];
    for format in formats {
        format.execute().await?;
    }
    "json".parse::<OutputFormat>()?.execute().await?;

    println!("please input format:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();
    let format = input.parse::<OutputFormat>()?;
    format.execute().await?;
    Ok(())
}
