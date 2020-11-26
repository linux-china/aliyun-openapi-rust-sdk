use crate::auth::sign_url;

const API_VERSION: &str = "2015-11-23";

#[derive(Debug, Default)]
pub struct SimpleMail<'a> {
    account_name: &'a str,
    from_alias: &'a str,
    to_address: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

/// 邮件推送  https://help.aliyun.com/product/29412.html
#[doc(alias = "mail")]
pub struct DM<'a> {
    /// endpoint, such as `dm.aliyuncs.com`
    pub endpoint: &'a str,
    /// global reqwest::Client
    pub http_client: &'a reqwest::Client,
}

impl<'a> DM<'a> {
    pub async fn send(&self, simple_mail: &SimpleMail<'_>) -> reqwest::Result<bool> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("AccountName", simple_mail.account_name));
        params.push(("FromAlias", simple_mail.from_alias));
        params.push(("AddressType", "1"));
        params.push(("ReplyToAddress", "true"));
        params.push(("Subject", simple_mail.subject));
        params.push(("ToAddress", simple_mail.to_address));
        params.push(("HtmlBody", simple_mail.html_body));
        let url = sign_url("POST", self.endpoint, API_VERSION,
                           "SingleSendMail", params.as_ref());
        let resp = self.http_client
            .post(&url)
            .send().await?;
        return resp.error_for_status().map(|response| response.status().is_success());
    }
}

#[cfg(test)]
mod tests {
    use crate::dm::{SimpleMail, DM};

    #[tokio::test]
    async fn test_send() -> Result<(), Box<dyn std::error::Error>> {
        let mail = SimpleMail {
            account_name: "support@microservices.club",
            from_alias: "MicroServicesClub",
            subject: "this is hello",
            to_address: "libing.chen@gmail.com",
            html_body: "This is hello!",
            text_body: "This is hello!",
        };
        let endpoint = "dm.aliyuncs.com";
        let ref http_client = reqwest::Client::new();
        let dm = DM { endpoint, http_client };
        let result = dm.send(&mail).await?;
        println!("{}", result);
        Ok(())
    }
}
