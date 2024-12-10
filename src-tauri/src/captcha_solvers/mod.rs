pub mod capsolver;

use async_trait::async_trait;
use capsolver::CapSolver;

#[async_trait]
pub trait CaptchaSolver: Send + Sync {
    async fn solve(
        &self,
        website_url: &str,
        website_key: &str,
        task_type: &str,
        page_action: Option<&str>,
        proxy: Option<&String>,
    ) -> Result<String, String>;
}

pub fn create_solver(captcha_solver: &str, captcha_solver_api_key: &str) -> Result<Box<dyn CaptchaSolver>, String> {
    match captcha_solver {
        "cap-solver" => Ok(Box::new(CapSolver {
            client_key: captcha_solver_api_key.to_string().clone(),
        })),
        _ => Err("Unsupported CAPTCHA solver".to_string())
    }
}
