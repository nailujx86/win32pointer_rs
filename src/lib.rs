pub mod pointers;
pub mod monitors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pointer = pointers::create_pen().unwrap();
        let _ = pointers::inject_pen(&pointer, 400, 400, 0, pointers::devicestates::pen::PEN_DEFAULT, pointers::pointerstates::pen::PEN_DOWN.0, false, None, None);
        for i in 400..600 {
            match pointers::inject_pen(&pointer, i, i, 4096, pointers::devicestates::pen::PEN_DEFAULT, pointers::pointerstates::touch::TOUCH_CONTACT.0, false, None, None) {
                Ok(_) => (),
                Err(_) => panic!()
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let _ = pointers::inject_pen(&pointer, 400, 400, 0, pointers::devicestates::pen::PEN_DEFAULT, pointers::pointerstates::pen::PEN_UP.0, false, None, None);
        
    }
}
