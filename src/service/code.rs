use rand::Rng;

use crate::service::error::Result;

pub fn generate_code(email: String) -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(100000..999999).to_string())
}
