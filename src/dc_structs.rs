use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: u16,
    pub global_name: String,
    pub email: String,
    pub verified: bool,
    pub has_mobile: bool,
    pub needs_email_verification: bool,
    pub premium_until: Option<String>,
    pub flags: u64,
    pub phone: Option<String>,
    pub temp_banned_until: Option<String>,
    pub ip: Option<String>,
    pub connections: Vec<DiscordConnection>,

    // stuff
    #[serde(rename = "payments")]
    pub money_wastes: Vec<DiscordPayment>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordConnection {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub name: String,
    pub visibility: u8,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub verified: bool,
    pub two_way_link: bool,
    pub metadata_visibility: u8,
    pub revoked: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordPayment {
    pub id: String,
    pub created_at: String,
    pub currency: String,
    pub tax: f32, // could be int
    pub tax_inclusive: bool,
    pub amount: u32,
    pub amount_refunded: u32,
    pub status: u8,
    pub description: String, // usually "Nitro", "Nitro Basic Monthly" etc
    pub flags: u32,
    pub subscription: Option<DiscordSubscription>,
    pub sku_id: Option<String>,
    pub sku_price: Option<u32>,
    pub sku_subscription_plan_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordSubscription {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: u8,
    // TODO
}
