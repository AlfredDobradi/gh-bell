fn main() -> anyhow::Result<()> {
    let ghp = std::env::var("GH_RELEASE")?;

    let payload = gh_bell::get_notifications(ghp)?;

    if payload.is_empty() {
        println!("You have no new notifications");
        return Ok(());
    }

    payload.iter()
        .for_each(|item| {
            println!("#{} - [{}/{}] {} ({})", item.id, item.reason, item.subject.kind, item.subject.title, item.subject.url);
        });

    Ok(())
}
