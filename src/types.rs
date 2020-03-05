//use serde::{Deserialize, Serialize};

/*
"app_eui\":\"0000000100000046\",
\"app_key\":\"CB67C92DD5898D07872224202DED7E76\",
\"dev_eui\":\"1234567890110000\",
\"id\":\"5bdb0128-f0e9-4458-86cf-305faf8f48c2\",
\"name\":\"Okapi\",
\"organization_id\":
\"07273bc4-4bc9-44ec-b4d5-ad320f162e15\",
\"oui\":1}
*/
#[derive(Clone, Deserialize, Debug)]
pub struct Device {
	app_eui: String,
	app_key: String,
	dev_eui: String,
	id: String,
	name: String,
	organization_id: String,
	oui: usize,
}