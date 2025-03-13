pub trait PasswordTools {
    fn encrypt(password: String) -> String;
    fn decrypt(bas64: String) -> String;
}