
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionValue {
    Synchronize,
    Opened,
    Reopened,
    Closed,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    pub action: ActionValue,
}
