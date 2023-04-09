use concat_idents::concat_idents;
use std::{
    any::type_name,
    io::{self, Read},
};

macro_rules! generate_read_number {
    ($type:ty, $bytes:expr) => {
        concat_idents!(fn_name = read_, $type {
            /// # Examples
            ///
            /// ```
            /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            #[doc = concat!("cursor.write_", stringify!($type), "(100).unwrap();")]
            ///
            #[doc = concat!("println!(\"My number: {:?}\", cursor.read_", stringify!($type), "().unwrap());")]
            /// ```
            fn fn_name(&mut self) -> Result<$type, io::Error> {
                let mut buffer = [0; $bytes];
                self.read_exact(&mut buffer)?;
        
                Ok($type::from_be_bytes(buffer))
            }
        });
    };
}

pub trait ReadBytes: Read {
    generate_read_number!(u8, 1);
    generate_read_number!(i8, 1);
    generate_read_number!(u16, 2);
    generate_read_number!(i16, 2);
    generate_read_number!(u32, 4);
    generate_read_number!(i32, 4);
    generate_read_number!(u64, 8);
    generate_read_number!(i64, 8);
    generate_read_number!(u128, 16);
    generate_read_number!(i128, 16);

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_string::<u8>("Hello".to_string()).unwrap();
    ///
    /// println!("My string: {:?}", cursor.read_string::<u8>().unwrap());
    /// ```
    fn read_string<T>(&mut self) -> Result<String, io::Error> {
        let size: usize = match type_name::<T>() {
            "u8" => self.read_u8()?.try_into().unwrap(),
            "u16" => self.read_u16()?.try_into().unwrap(),
            "u32" => self.read_u32()?.try_into().unwrap(),
            _ => unimplemented!(),
        };
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;

        Ok(String::from_utf8(buffer).unwrap())
    }
}

impl<R: Read + ?Sized> ReadBytes for R {}
