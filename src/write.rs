use std::{
    any::type_name,
    io::{self, Write},
};

macro_rules! generate_write_number {
    ($func_name:ident, $type:ty) => {
        /// # Examples
        ///
        /// ```
        /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        #[doc = concat!("cursor.", stringify!($func_name), "(100).unwrap();")]
        ///
        #[doc = concat!("println!(\"My number: {:?}\", cursor.read_", stringify!($type), "().unwrap());")]
        /// ```
        fn $func_name(&mut self, number: $type) -> Result<(), io::Error> {
            self.write_all(&number.to_be_bytes())?;

            Ok(())
        }
    };
}

pub trait WriteBytes: Write {
    generate_write_number!(write_u8, u8);
    generate_write_number!(write_i8, i8);
    generate_write_number!(write_u16, u16);
    generate_write_number!(write_i16, i16);
    generate_write_number!(write_u32, u32);
    generate_write_number!(write_i32, i32);
    generate_write_number!(write_u64, u64);
    generate_write_number!(write_i64, i64);
    generate_write_number!(write_u128, u128);
    generate_write_number!(write_i128, i128);

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_string::<u8>("Hello".to_string()).unwrap();
    ///
    /// println!("My string: {:?}", cursor.read_string::<u8>().unwrap());
    /// ```
    fn write_string<T>(&mut self, string: String) -> Result<(), io::Error> {
        match type_name::<T>() {
            "u8" => self.write_u8(string.len().try_into().unwrap())?,
            "u16" => self.write_u16(string.len().try_into().unwrap())?,
            "u32" => self.write_u32(string.len().try_into().unwrap())?,
            _ => unimplemented!(),
        };

        self.write_all(string.as_bytes())?;

        Ok(())
    }
}

impl<R: Write + ?Sized> WriteBytes for R {}
