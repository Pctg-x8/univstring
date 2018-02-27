
extern crate widestring;use widestring::*;
use widestring::NulError as WideNulError;
use std::ffi::*; use std::ffi::NulError as CNulError;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::string::FromUtf16Error;

/// An error description while converting strings
#[derive(Debug)]
pub enum ConversionError<NE> { Nul(NE), InvalidChar(Utf8Error), InvalidWChar(FromUtf16Error) }
impl<NE: Display> Display for ConversionError<NE>
{
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult
    {
        match *self
        {
            ConversionError::Nul(ref n) => n.fmt(fmt),
            ConversionError::InvalidChar(ref c) => c.fmt(fmt),
            ConversionError::InvalidWChar(ref c) => c.fmt(fmt)
        }
    }
}
impl<NE: Error> Error for ConversionError<NE>
{
    fn description(&self) -> &str
    {
        match *self
        {
            ConversionError::Nul(_) => "Null character found in string",
            ConversionError::InvalidChar(_) => "Invalid charcode for UTF-8",
            ConversionError::InvalidWChar(_) => "Couldn't convert UTF-16 charcode to UTF-8"
        }
    }
    fn cause(&self) -> Option<&Error>
    {
        match *self
        {
            ConversionError::Nul(ref e) => Some(e),
            ConversionError::InvalidChar(ref e) => Some(e),
            ConversionError::InvalidWChar(ref e) => Some(e)
        }
    }
}
impl From<CNulError> for ConversionError<CNulError> { fn from(e: CNulError) -> Self { ConversionError::Nul(e) } }
impl From<WideNulError> for ConversionError<WideNulError> { fn from(e: WideNulError) -> Self { ConversionError::Nul(e) } }
impl<NE> From<Utf8Error> for ConversionError<NE> { fn from(e: Utf8Error) -> Self { ConversionError::InvalidChar(e) } }
impl<NE> From<FromUtf16Error> for ConversionError<NE> { fn from(e: FromUtf16Error) -> Self { ConversionError::InvalidWChar(e) } }

pub trait UnivString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>>;
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>>;
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>>;
}
impl UnivString for str
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self).map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { WideCString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { Ok(ToString::to_string(self).into()) }
}
impl UnivString for String
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self as &str).map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { WideCString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { Ok(ToString::to_string(self).into()) }
}
impl UnivString for CStr
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { self.to_str()?.to_wcstr() }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_str().map(Cow::Borrowed).map_err(Into::into) }
}
impl UnivString for CString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { self.to_str()?.to_wcstr() }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_str().map(Cow::Borrowed).map_err(Into::into) }
}
impl UnivString for WideCStr
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self.to_string()?).map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_string().map(Cow::Owned).map_err(Into::into) }
}
impl UnivString for WideCString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(WideCStr::to_string(self)?).map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { WideCStr::to_string(self).map(Cow::Owned).map_err(Into::into) }
}
