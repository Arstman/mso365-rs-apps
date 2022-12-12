use graph_rs_sdk::prelude::*;

fn main() -> Result<()> {
    let client = Graph::new("ACCESS_TOKEN");

    // Some requests don't require an id.
    let response = client.v1().drives().get_drive();

    // Using a drive id.
    let response = client.v1().drive("DRIVE-ID").get_items("ITEM_ID").send()?;

    // Using me.
    let response = client.v1().me().drive().get_items("ITEM_ID").send()?;

    println!("{:#?}", response);

    // Using users.
    let response = client
        .v1()
        .users("USER_ID")
        .drive()
        .get_items("ITEM_ID")
        .send()?;

    println!("{:#?}", response);

    // Using sites.
    let response = client
        .v1()
        .sites("SITE-ID")
        .drive()
        .get_items("ITEM_ID")
        .send()?;

    println!("{:#?}", response);

    let folder: HashMap<String, serde_json::Value> = HashMap::new();

    let response = client
        .v1()
        .me()
        .drive()
        .create_folder(
            "PARENT_FOLDER_ID",
            &serde_json::json!({
               "name": "docs",
               "folder": folder,
               "@microsoft.graph.conflictBehavior": "fail"
            }),
        )
        .send()?;

    println!("{:#?}", response);
    Ok(())
}
