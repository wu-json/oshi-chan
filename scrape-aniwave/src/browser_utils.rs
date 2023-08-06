use headless_chrome::Browser;
use std::sync::Arc;
use thiserror::Error;

pub struct BrowserUtils {}
pub trait TabUtils {
    fn create_browser_tab() -> Result<(Browser, Arc<headless_chrome::Tab>), CreateBrowserTabError>;
}

#[derive(Error, Debug)]
pub enum CreateBrowserTabError {
    #[error("Browser create error.")]
    CreateError(String),
    #[error("Browser new tab error.")]
    NewTabError(String),
    #[error("Stealth mode error.")]
    StealthModeError(String),
    #[error("Set user agent error.")]
    SetUserAgentError(String),
}

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9,hi;q=0.8,es;q=0.7,lt;q=0.6";
const PLATFORM: &str = "macOS";

impl TabUtils for BrowserUtils {
    fn create_browser_tab() -> Result<(Browser, Arc<headless_chrome::Tab>), CreateBrowserTabError> {
        let browser =
            Browser::default().map_err(|e| CreateBrowserTabError::CreateError(e.to_string()))?;

        let tab = browser
            .new_context().unwrap()
            .new_tab()
            .map_err(|e| CreateBrowserTabError::NewTabError(e.to_string()))?;

        tab.enable_stealth_mode()
            .map_err(|e| CreateBrowserTabError::StealthModeError(e.to_string()))?;

        tab.set_user_agent(USER_AGENT, Some(ACCEPT_LANGUAGE), Some(PLATFORM))
            .map_err(|e| CreateBrowserTabError::SetUserAgentError(e.to_string()))?;

        return Ok((browser, tab));
    }
}
