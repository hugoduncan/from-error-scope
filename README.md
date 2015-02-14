# from_error_scope

Enable conversion between non-local [`Error`][error] types.

When using libraries, your code often needs to convert between
[`Error`][error] types which aren't part of your local crate. In this
case you can't implement [`std::error::FromError`][from-error].

This is also useful if you need context dependent translation of
error types.

[Documentation](http://hugoduncan.github.io/from-error-scope/from_error_scope/index.html)

# Usage

Add `from_error_scope` to your dependencies:

```toml
[dependencies]
from_error_scope = "*"
```

# License

Copyright 2015 Hugo Duncan

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[error]: http://doc.rust-lang.org/std/error/trait.Error.html "std::error::Error trait"
[from-error]: http://doc.rust-lang.org/std/error/trait.FromError.html "std::error::FromError trait"
