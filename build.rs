fn main() {
    println!("cargo:rustc-env=LITCRYPT_ENCRYPT_KEY=MY-SECRET-SPELL");  // I couldn't get this to work without passing this as an environment variable
    println!("cargo:rustc-env=SECRET_ENV=Shhhhhh");
}