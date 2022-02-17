use mongodb::Client;

#[derive(Default)]
struct Args {
    url: Option<String>,
}

const HELPTEXT: &str = r#"
USAGE

    minimal-mongodb-binary --url <database-url>
"#;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut parsed: Args = Default::default();
    let mut args = std::env::args().into_iter();
    args.next().unwrap();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--url" => match args.next() {
                Some(url) => {
                    parsed.url = Some(url);
                    break;
                }
                None => bail_with_helptext(),
            },
            _ => bail_with_helptext(),
        }
    }

    let url = parsed.url.unwrap();

    let client =
        Client::with_options(mongodb::options::ClientOptions::parse(url).await.unwrap()).unwrap();

    for db in client.list_databases(None, None).await.unwrap() {
        println!("Found database: {}", &db.name);
    }

    Ok(())
}

fn bail_with_helptext() -> ! {
    eprintln!("{}", HELPTEXT);
    std::process::exit(1);
}
