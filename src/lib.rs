
use widestring::*;
use widestring::NulError as WideNulError;
use std::ffi::*; use std::ffi::NulError as CNulError;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::string::FromUtf16Error;

/// An error description while converting strings
#[derive(Debug)]
pub enum ConversionError<NE> { Nul(NE), InvalidChar(Utf8Error), InvalidUChar16(FromUtf16Error), InvalidUChar32(FromUtf32Error) }
impl<NE: Display> Display for ConversionError<NE>
{
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult
    {
        match self
        {
            ConversionError::Nul(n) => n.fmt(fmt),
            ConversionError::InvalidChar(c) => c.fmt(fmt),
            ConversionError::InvalidUChar16(c) => c.fmt(fmt),
            ConversionError::InvalidUChar32(c) => c.fmt(fmt)
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
            ConversionError::InvalidUChar16(_) => "Couldn't convert UTF-16 charcode to UTF-8",
            ConversionError::InvalidUChar32(_) => "Couldn't convert UTF-32 charcode to UTF-8"
        }
    }
    fn cause(&self) -> Option<&dyn Error>
    {
        match self
        {
            ConversionError::Nul(e) => Some(e),
            ConversionError::InvalidChar(e) => Some(e),
            ConversionError::InvalidUChar16(e) => Some(e),
            ConversionError::InvalidUChar32(e) => Some(e)
        }
    }
}
impl From<CNulError> for ConversionError<CNulError> { fn from(e: CNulError) -> Self { ConversionError::Nul(e) } }
impl<U: UChar> From<WideNulError<U>> for ConversionError<WideNulError<U>> { fn from(e: WideNulError<U>) -> Self { ConversionError::Nul(e) } }
impl<NE> From<Utf8Error> for ConversionError<NE> { fn from(e: Utf8Error) -> Self { ConversionError::InvalidChar(e) } }
impl<NE> From<FromUtf16Error> for ConversionError<NE> { fn from(e: FromUtf16Error) -> Self { ConversionError::InvalidUChar16(e) } }
impl<NE> From<FromUtf32Error> for ConversionError<NE> { fn from(e: FromUtf32Error) -> Self { ConversionError::InvalidUChar32(e) } }

/// The Universal String trait
/// 
/// # Examples
/// 
/// ```
/// # extern crate univstring; extern crate widestring;
/// # fn main() {
/// use univstring::UnivString;
/// use std::ffi::CString;
/// use widestring::U32CString;
/// use std::borrow::Cow;
/// 
/// let org: &str = "Hello World";
/// assert_eq!(org.to_cstr().unwrap(), Cow::Owned(CString::new("Hello World").unwrap()));
/// assert_eq!(org.to_ucstr32().unwrap(), Cow::Owned(U32CString::from_str("Hello World").unwrap()));
/// 
/// // more optimal way to take some cstrings as argument
/// fn take_ucstr16<S: UnivString + ?Sized>(s: &S)
/// {
///   let _ws = s.to_ucstr16().unwrap();
///   // do something with the WideCString...
/// }
/// // call the function
/// take_ucstr16("test");
/// let existing_cstr = CString::new("...").unwrap();
/// take_ucstr16(&existing_cstr);
/// # }
/// ```
pub trait UnivString
{
    /// Converts a string to `CString` or `CStr`(if possible)
    /// 
    /// # Errors
    /// 
    /// - This function will return a CNulError(`std::ffi::NulError`) if the string contains an internal 0 byte.
    /// - This function will return a `FromUtf16Error` if the string contains unrecognizable UTF-16 characters as UTF-8.
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>>;
    /// Converts a string to `U16CString` or `U16CStr`(if possible)
    /// 
    /// # Errors
    /// 
    /// - This function will return a WideNulError(`std::ffi::WideNulError`) if the string contains an internal 0 byte.
    /// - This function will return a `Utf8Error` if the string contains unrecognizable characters as UTF-8.
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>>;
    /// Converts a string to `U32CString` or `U32CStr`(if possible)
    /// 
    /// # Errors
    /// 
    /// - This function will return a WideNulError(`std::ffi::WideNulError`) if the string contains an internal 0 byte.
    /// - This function will return a `Utf8Error` if the string contains unrecognizable characters as UTF-8.
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>>;
    /// Converts a string to `String` or `str`(if possible)
    /// 
    /// # Errors
    /// 
    /// - This function will return a `Utf8Error` or a `FromUtf16Error` if the string contains unrecognizable characters as UTF-8.
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>>;
    /// Converts a string to `String` or `str`(if possible)
    /// 
    /// # Errors
    /// 
    /// - This function will return a `Utf8Error` or a `FromUtf16Error` if the string contains unrecognizable characters as UTF-8.
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>>;
}

trait CharElement { type Char; }
impl<U: UChar> CharElement for UCStr<U> { type Char = U; }
impl<U: UChar> CharElement for UCString<U> { type Char = U; }

impl UnivString for str
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { U16CString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { U32CString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { Ok(ToString::to_string(self).into()) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> { WideCString::from_str(self).map(Cow::Owned).map_err(Into::into) }
}
impl UnivString for String
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self as &str).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { U16CString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { U32CString::from_str(self).map(Cow::Owned).map_err(Into::into) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { Ok(ToString::to_string(self).into()) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> { WideCString::from_str(self).map(Cow::Owned).map_err(Into::into) }
}
impl UnivString for CStr
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { self.to_str()?.to_ucstr16() }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { self.to_str()?.to_ucstr32() }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_str().map(Cow::Borrowed).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> { self.to_str()?.to_wcstr() }
}
impl UnivString for CString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { Ok(Cow::Borrowed(self)) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { self.to_str()?.to_ucstr16() }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { self.to_str()?.to_ucstr32() }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_str().map(Cow::Borrowed).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> { self.to_str()?.to_wcstr() }
}

impl UnivString for U16CStr
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self.to_string()?).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { Ok(Cow::Borrowed(self)) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { U32CString::from_str(self.to_string()?).map(Into::into).map_err(From::from) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_string().map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> {
        if std::mem::size_of::<<WideCStr as CharElement>::Char>() == 2 {
            // no conversion ほんとはtransmuteいらないはず
            Ok(Cow::Borrowed(unsafe { std::mem::transmute(self) }))
        } else {
            WideCString::from_str(self.to_string()?).map(Into::into).map_err(From::from)
        }
    }
}
impl UnivString for U16CString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(WideCStr::to_string(self)?).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { Ok(Cow::Borrowed(self)) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { U16CStr::to_ucstr32(self) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { std::ops::Deref::deref(self).to_string().map(Into::into).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> {
        if std::mem::size_of::<<WideCStr as CharElement>::Char>() == 2 {
            // no conversion ほんとはtransmuteいらないはず
            Ok(Cow::Borrowed(unsafe { std::mem::transmute(self as &U16CStr) }))
        } else {
            U16CStr::to_wcstr(self)
        }
    }
}
impl UnivString for U32CStr
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(self.to_string()?).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { U16CString::from_str(self.to_string()?).map(Into::into).map_err(From::from) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { Ok(Cow::Borrowed(self)) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { self.to_string().map(Cow::Owned).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> {
        if std::mem::size_of::<<WideCStr as CharElement>::Char>() == 4 {
            // no conversion ほんとはtransmuteいらないはず
            Ok(Cow::Borrowed(unsafe { std::mem::transmute(self) }))
        } else {
            WideCString::from_str(self.to_string()?).map(Into::into).map_err(From::from)
        }
    }
}
impl UnivString for U32CString
{
    fn to_cstr(&self) -> Result<Cow<CStr>, ConversionError<CNulError>> { CString::new(U32CStr::to_string(self)?).map(Cow::Owned).map_err(Into::into) }
    fn to_ucstr16(&self) -> Result<Cow<UCStr<u16>>, ConversionError<WideNulError<u16>>> { U32CStr::to_ucstr16(self) }
    fn to_ucstr32(&self) -> Result<Cow<UCStr<u32>>, ConversionError<WideNulError<u32>>> { Ok(Cow::Borrowed(self)) }
    fn to_string(&self) -> Result<Cow<str>, ConversionError<CNulError>> { std::ops::Deref::deref(self).to_string().map(Into::into).map_err(Into::into) }
    fn to_wcstr(&self) -> Result<Cow<WideCStr>, ConversionError<WideNulError<WideChar>>> {
        if std::mem::size_of::<<WideCStr as CharElement>::Char>() == 4 {
            // no conversion ほんとはtransmuteいらないはず
            Ok(Cow::Borrowed(unsafe { std::mem::transmute(self as &U32CStr) }))
        } else {
            U32CStr::to_wcstr(self)
        }
    }
}
