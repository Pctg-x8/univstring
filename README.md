
The Universal String trait | [Documentation](https://docs.rs/univstring)

```rust
// more optimal way to take some cstrings as argument
fn take_wstr<S: UnivString + ?Sized>(s: &S)
{
  let _ws = s.to_wcstr().unwrap();
  // do something with the WideCString...
}
// call the function
take_wstr("test");
let existing_cstr = CString::new("...").unwrap();
take_wstr(&existing_cstr);
```
