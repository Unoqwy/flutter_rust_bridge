# Language translations

Here is a brief glance showing what the code generator can generate (non-exhaustive). Some rows have hyper-links pointing to more detailed explanations.

| Rust                                              | Dart                        |
| ------------------------------------------------- | --------------------------- |
| [`Vec<u8>`, `Vec<i8>`..](lang_vec.md)             | `Uint8List`, `Int8List`, .. |
| [`Vec<T>`](lang_vec.md)                           | `List<T>`                   |
| [`struct { .. }`, `struct( .. )`](lang_struct.md) | `class`                     |
| [`enum { A, B }`](lang_enum.md)                   | `enum`                      |
| [`enum { A(..) }`](lang_enum.md)                  | `@freezed class`            |
| [`use ...`](lang_use.md)                          | act normally                |
| [`Option<T>`](lang_option.md)                     | `T?`                        |
| `Box<T>`                                          | `T`                         |
| comments                                          | same                        |
| `Result::Err`, panic                              | `throw Exception`           |
| `i8`, `u8`, ..                                    | `int`                       |
| `f32`, `f64`                                      | `double`                    |
| `bool`                                            | `bool`                      |
| `String`                                          | `String`                    |
| `()`                                              | `void`                      |

