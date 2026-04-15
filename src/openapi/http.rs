use crate::{Error, Result};
use reqwest::{Client, RequestBuilder, Response};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub retry_statuses: Vec<u16>,
    pub retry_on_timeout: bool,
    pub retry_on_connect: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(2),
            retry_statuses: vec![429, 500, 502, 503, 504],
            retry_on_timeout: true,
            retry_on_connect: true,
        }
    }
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    retry: RetryPolicy,
}

impl HttpClient {
    pub fn new(client: Client, retry: RetryPolicy) -> Self {
        Self { client, retry }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn retry(&self) -> &RetryPolicy {
        &self.retry
    }

    pub async fn send_with_retry(&self, mut builder: RequestBuilder) -> Result<Response> {
        let mut attempt = 0usize;

        loop {
            let retry_builder = builder.try_clone();
            let request = builder.build().map_err(Error::Http)?;

            let resp = self.client.execute(request).await;
            match resp {
                Ok(resp) => {
                    if attempt < self.retry.max_retries
                        && self.retry.retry_statuses.contains(&resp.status().as_u16())
                    {
                        if let Some(cloned) = retry_builder {
                            attempt += 1;
                            let delay = backoff_delay(&self.retry, attempt);
                            sleep(delay).await;
                            builder = cloned;
                            continue;
                        }
                    }
                    return Ok(resp);
                }
                Err(err) => {
                    let should_retry = (self.retry.retry_on_timeout && err.is_timeout())
                        || (self.retry.retry_on_connect && err.is_connect());

                    if attempt < self.retry.max_retries && should_retry {
                        if let Some(cloned) = retry_builder {
                            attempt += 1;
                            let delay = backoff_delay(&self.retry, attempt);
                            sleep(delay).await;
                            builder = cloned;
                            continue;
                        }
                    }

                    return Err(Error::Http(err));
                }
            }
        }
    }
}

fn backoff_delay(policy: &RetryPolicy, attempt: usize) -> Duration {
    let pow = 2u64.saturating_pow(attempt as u32);
    let base = policy.base_delay.as_millis() as u64 * pow;
    let capped = base.min(policy.max_delay.as_millis() as u64);
    Duration::from_millis(capped)
}
