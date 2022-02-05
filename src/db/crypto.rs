use rand::Rng;
use sha2::{Sha256,Digest};

// Default length of salt
const SALT_LENGTH:     usize = 32;

pub struct DbCrypto {
}

impl DbCrypto {
    // Calculates a SHA256 hash of the input
    fn get_sha256_hash(input: String) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(input);

        format!("{:X}", sha256.finalize())
    }

    // Generates hash of password + salt
    pub fn password_to_hash(password: &String, salt: &String) -> String {
        DbCrypto::get_sha256_hash(password.to_owned() + salt)
    }

    // Generates a random alpha string with both caps and lower case
    // Used as a salt for hashing password
    pub fn gen_rand_salt() -> String {
        let mut rng = rand::thread_rng();
        let mut arr: [u8; SALT_LENGTH] = rng.gen();

        // Modify random num array to only have alpha characters values
        for i in 0..SALT_LENGTH{
            let val = arr[i]%52;

            if val < 26 { // Capitals
                arr[i] = 65+val;
            }
            else { // Lowercase
                arr[i] = 97+(val/2);
            }
        }

        std::str::from_utf8(&arr).unwrap().to_string()
    }
}
