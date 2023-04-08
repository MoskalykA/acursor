use std::{
    any::type_name,
    io::{self, Write},
};

pub trait WriteBytes: Write {
    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u8(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u8().unwrap());
    /// ```
    fn write_u8(&mut self, number: u8) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i8(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i8().unwrap());
    /// ```
    fn write_i8(&mut self, number: i8) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u16(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u16().unwrap());
    /// ```
    fn write_u16(&mut self, number: u16) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i16(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i16().unwrap());
    /// ```
    fn write_i16(&mut self, number: i16) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u32(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u32().unwrap());
    /// ```
    fn write_u32(&mut self, number: u32) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i32(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i32().unwrap());
    /// ```
    fn write_i32(&mut self, number: i32) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u64(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u64().unwrap());
    /// ```
    fn write_u64(&mut self, number: u64) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i64(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i64().unwrap());
    /// ```
    fn write_i64(&mut self, number: i64) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u128(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u128().unwrap());
    /// ```
    fn write_u128(&mut self, number: u128) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i128(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i128().unwrap());
    /// ```
    fn write_i128(&mut self, number: i128) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

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
