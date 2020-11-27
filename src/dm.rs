use crate::auth::sign_url;

const API_VERSION: &str = "2015-11-23";

/// Simple Email
#[derive(Debug, Default)]
pub struct SimpleMail<'a> {
    /// same as service account on DM console
    sender: &'a str,
    /// from alias, such as Customer Support, Admin
    sender_alias: &'a str,
    /// receiver email
    receiver: &'a str,
    /// email subject
    subject: &'a str,
    /// html body
    html_body: &'a str,
    /// text body
    text_body: &'a str,
}

/// Direct Mail(DM)  https://cn.aliyun.com/product/directmail
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
        params.push(("AccountName", simple_mail.sender));
        params.push(("FromAlias", simple_mail.sender_alias));
        params.push(("AddressType", "1"));
        params.push(("ReplyToAddress", "true"));
        params.push(("Subject", simple_mail.subject));
        params.push(("ToAddress", simple_mail.receiver));
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
            sender: "support@microservices.club",
            sender_alias: "MicroServicesClub",
            subject: "this is hello",
            receiver: "libing.chen@gmail.com",
            html_body: "<p>This is hello!</p>",
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
