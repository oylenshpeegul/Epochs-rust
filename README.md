# Epochs-rust
Convert various epoch times to [chrono::NaiveDateTime](https://lifthrasiir.github.io/rust-chrono/chrono/naive/datetime/struct.NaiveDateTime.html) times in [Rust](https://www.rust-lang.org).

```bash
$ cat use_epochs.rs 
extern crate epochs;

fn main() {
    let ndt = epochs::unix(1234567890);
    println!("{:?}", ndt);

    let ndt2 = epochs::chrome(12879041490654321);
    println!("{:?}", ndt2);
}

$ rustc -L./Epochs-rust/target/debug/deps use_epochs.rs

$ ./use_epochs 
2009-02-13T23:31:30
2009-02-13T23:31:30.654321
```

## See Also

This project was originally done in [Perl](https://github.com/oylenshpeegul/Time-Moment-Epoch). See [the Time::Moment::Epoch web page](http://oylenshpeegul.github.io/Time-Moment-Epoch/) for motivation.

There are also versions in:
- [Go](https://github.com/oylenshpeegul/epochs)
- [Elixir](https://github.com/oylenshpeegul/Epochs-elixir)
- [PowerShell](https://github.com/oylenshpeegul/Epochs-powershell)
