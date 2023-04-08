use std::{
    any::type_name,
    io::{self, Read, Write},
};

pub trait ReadBytes: Read {
    fn read_u8(&mut self) -> Result<u8, io::Error> {
        let mut buffer = [0; 1];
        let n = self.read(&mut buffer[..])?;

        Ok(u8::from_be_bytes(buffer[..n].try_into().unwrap()))
    }

    fn read_u16(&mut self) -> Result<u16, io::Error> {
        let mut buffer = [0; 2];
        let n = self.read(&mut buffer[..])?;

        Ok(u16::from_be_bytes(buffer[..n].try_into().unwrap()))
    }

    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut buffer = [0; 4];
        let n = self.read(&mut buffer[..])?;

        Ok(u32::from_be_bytes(buffer[..n].try_into().unwrap()))
    }

    fn read_u64(&mut self) -> Result<u64, io::Error> {
        let mut buffer = [0; 8];
        let n = self.read(&mut buffer[..])?;

        Ok(u64::from_be_bytes(buffer[..n].try_into().unwrap()))
    }

    fn read_u128(&mut self) -> Result<u128, io::Error> {
        let mut buffer = [0; 16];
        let n = self.read(&mut buffer[..])?;

        Ok(u128::from_be_bytes(buffer[..n].try_into().unwrap()))
    }

    fn read_string<T>(&mut self) -> Result<String, io::Error> {
        let size: u32 = match type_name::<T>() {
            "u8" => self.read_u8()?.try_into().unwrap(),
            "u16" => self.read_u16()?.try_into().unwrap(),
            "u32" => self.read_u32()?,
            _ => unimplemented!(),
        };
        let mut buffer = vec![0; size as usize];
        let n = self.read(&mut buffer[..])?;

        Ok(String::from_utf8(buffer[..n].try_into().unwrap()).unwrap())
    }
}

pub trait WriteBytes: Write {
    fn write_u8(&mut self, number: u8) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    fn write_u16(&mut self, number: u16) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    fn write_u32(&mut self, number: u32) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    fn write_u64(&mut self, number: u64) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

    fn write_u128(&mut self, number: u128) -> Result<(), io::Error> {
        self.write_all(&number.to_be_bytes())?;

        Ok(())
    }

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

impl<R: Read + ?Sized> ReadBytes for R {}
impl<R: Write + ?Sized> WriteBytes for R {}
