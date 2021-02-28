use bcrypt;

const DEFAULT_BCRYPT_COST: u32 = 5;

/// Mendapatkan passhash dari sebuah password.
/// Algo menggunakan Bcrypt.
pub fn get_passhash(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_BCRYPT_COST)
        .unwrap_or_else(|_| panic!("Cannot bcrypt password `{}`", password) )
}

/// Memverifikasi apakah password match (verified) dengan hash-nya?
pub fn password_match(password: &str, hashed:&str) -> bool {
    bcrypt::verify(password, hashed).expect("Cannot verify bcrypt hash")
}