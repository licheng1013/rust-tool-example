use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey};

/// 用户数据
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// 用户id
    id: i64,
    /// 过期时间
    exp: usize,
}

pub struct JwtUtil {}
static KEY: &'static [u8] = b"secret";
pub static TOKEN: &'static str = "Authorization";

impl JwtUtil {
    /// 生成token
    pub fn token(id: i64) -> String {
        // 创建过期时间，默认为30天过期
        let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
        let my_claims = Claims {
            id,
            exp,
        };
        // Encode
        let token = encode(&Header::new(Algorithm::HS256), &my_claims,
                           &EncodingKey::from_secret(KEY)).unwrap();
        return token;
    }

    /// 获取用户id
    pub fn id(token: &str) -> i64 {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(KEY), &validation).unwrap();
        // 判断是不是已经过期
        if token_data.claims.exp < (Utc::now().timestamp() as usize) {
            panic!("token is expired"); // token过期
        }
        return token_data.claims.id;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jwt() {
        let token = JwtUtil::token(12313);
        println!("{:?}", token);
        // Decode
        let token_data = JwtUtil::id(&token);
        println!("{:?}", token_data);
    }
}


