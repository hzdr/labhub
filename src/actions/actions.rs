
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionValue {
    ReviewRequested,
    Synchronize,
    Opened,
    Reopened,
    Closed,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    pub action: ActionValue,
}
