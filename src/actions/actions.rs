
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionValue {
    Synchronize,
    Opened,
    Reopened,
    Merged,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    pub action: ActionValue,
}
