use std::{
    any::type_name,
    io::{self, Read},
};

pub trait ReadBytes: Read {
    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u8(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u8().unwrap());
    /// ```
    fn read_u8(&mut self) -> Result<u8, io::Error> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(u8::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i8(-100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i8().unwrap());
    /// ```
    fn read_i8(&mut self) -> Result<i8, io::Error> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(i8::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u16(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u16().unwrap());
    /// ```
    fn read_u16(&mut self) -> Result<u16, io::Error> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;

        Ok(u16::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i16(-100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i16().unwrap());
    /// ```
    fn read_i16(&mut self) -> Result<i16, io::Error> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;

        Ok(i16::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u32(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u32().unwrap());
    /// ```
    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(u32::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i32(-100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i32().unwrap());
    /// ```
    fn read_i32(&mut self) -> Result<i32, io::Error> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(i32::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u64(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u64().unwrap());
    /// ```
    fn read_u64(&mut self) -> Result<u64, io::Error> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;

        Ok(u64::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i64(-100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i64().unwrap());
    /// ```
    fn read_i64(&mut self) -> Result<i64, io::Error> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;

        Ok(i64::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_u128(100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_u128().unwrap());
    /// ```
    fn read_u128(&mut self) -> Result<u128, io::Error> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;

        Ok(u128::from_be_bytes(buffer))
    }

    /// # Examples
    ///
    /// ```
    /// let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    /// cursor.write_i128(-100).unwrap();
    ///
    /// println!("My number: {:?}", cursor.read_i128().unwrap());
    /// ```
    fn read_i128(&mut self) -> Result<i128, io::Error> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;

        Ok(i128::from_be_bytes(buffer))
    }

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
