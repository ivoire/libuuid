struct UUID {
    data: [u8; 16],
}

// TODO: implement the right Trait for comparison
enum Types {
    Null,     // Empty UUID
    Time,     // UUIDv1
    Random,   // UUIDv4
}

#[link(name = "uuid")]
extern {
    fn uuid_generate(out: *mut [u8; 16]);
    fn uuid_generate_random(out: *mut [u8; 16]);
    fn uuid_generate_time(out: *mut [u8; 16]);
    fn uuid_generate_time_safe(out: *mut [u8; 16]);

    fn uuid_type(uu: *const [u8; 16]) -> i32;
}

impl UUID {
    fn new(uuid_type: Types) -> UUID {
        let mut data: [u8; 16] = [0; 16];
        match uuid_type {
            Types::Null => { }
            Types::Time => { unsafe { uuid_generate_time(&mut data) } }
            Types::Random => { unsafe { uuid_generate_random(&mut data) } }
      }
      UUID{ data: data }
    }

    fn get_type(&self) -> Types {
        if self.data == [0; 16] { Types::Null }
        else {
            match unsafe { uuid_type(&self.data) } {
                1 => { Types::Time }
                4 => { Types::Random }
                _ => { Types::Null }    // TODO: should raise an error
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;
    use super::Types;

    #[test]
    fn new_null_uuid() {
        let uuid = UUID::new(Types::Null);
        assert!(uuid.data == [0; 16]);
        match uuid.get_type() {
            Types::Null => { assert!(true) }
            _ => { assert!(false) }
        }
    }

    #[test]
    fn new_time_uuid() {
        let uuid = UUID::new(Types::Time);
        assert!(uuid.data != [0; 16]);
        match uuid.get_type() {
            Types::Time => { assert!(true) }
            _ => { assert!(false) }
        }
    }

    #[test]
    fn new_random_uuid() {
        let uuid = UUID::new(Types::Random);
        assert!(uuid.data != [0; 16]);
        match uuid.get_type() {
            Types::Random => { assert!(true) }
            _ => { assert!(false) }
        }
    }
}
