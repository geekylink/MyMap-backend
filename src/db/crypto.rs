use std::time::SystemTime;
use rand::Rng;
use sha2::{Digest, Sha256};
use totp_rs::{Algorithm, TOTP};

// Default length of salt & totp secret
const SALT_LENGTH: usize    = 32;
const SECRET_LENGTH: usize  = 16;

// Would be nice to probably move some of this stuff to a settings.json
const WEBSITE_URL: &str = "gekinzuku.github.io";

pub struct DbCrypto {}

impl DbCrypto {
    // Generates a random string used as a salt for hashing password
    pub fn gen_rand_salt() -> String {
        DbCrypto::gen_rand_string(SALT_LENGTH)
    }

    // Generates a random string used as a totp secret for a user
    pub fn gen_rand_secret() -> String {
        DbCrypto::gen_rand_string(SECRET_LENGTH)
    }

    // Generates a Base64 encoding of a QR code containing
    // the totp secret and user/website information
    pub fn gen_totp_qr(username: &str, totp_secret: &str) -> String {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, totp_secret);

        let label = format!("{}@{}", username, WEBSITE_URL);
        let issuer = WEBSITE_URL;

        totp.get_qr(&label, issuer).expect("creating qr code")
    }

    // Checks if the provided totp_code matches for the secret
    pub fn is_valid_totp(totp_secret: &str, totp_code: &str) -> bool {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, totp_secret);
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

        let token = totp.generate(time);

        println!("TOTP secret: {}", totp_secret);
        println!("Token is: {}", totp_code);
        println!("Token should be: {}", token);

        token.eq(totp_code)
    }

    // Generates hash of password + salt
    pub fn password_to_hash(password: &str, salt: &str) -> String {
        DbCrypto::get_sha256_hash(&(password.to_owned() + salt))
    }

    // Private helpers:

    // Generates a random string of size length
    fn gen_rand_string(length: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut rand_str = "".to_string();
    
        for _ in 0..length {
            let val = rng.gen::<u8>()%62;
            if val < 26 { // Capitals
                rand_str.push(char::from(65+val));
            } 
            else if val < 52 { // Lowercase
                rand_str.push(char::from(97+(val/2)));
            }
            else { // Numbers
                rand_str.push(char::from(48+(val-52)));
            }
        }
    
        rand_str
    }

    // Calculates a SHA256 hash of the input
    fn get_sha256_hash(input: &str) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(input);

        format!("{:X}", sha256.finalize())
    }

}
