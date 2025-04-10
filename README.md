# proc-singleton
Rust proc-macros for impl singleton

## macros

### derive macro usage
```rust
use std::sync::LazyLock;
use uuid::Uuid;
use proc_singleton::Singleton;

static IDENT: LazyLock<Identifier> = LazyLock::new(|| {
    Identifier {
        id: Uuid::new_v4(),
    }
});
#[derive(Singleton)]
#[singleton(IDENT)]
struct Identifier {
    id: Uuid,
}

fn main() {
    let instance = Identifier::get_instance();
    let ptr = instance as *const Identifier;
    let same_ptr = Identifier::get_instance() as *const Identifier;

    assert_eq!(ptr, same_ptr);
}
```


### attribute macro usage
```rust
 use std::sync::LazyLock;
 use uuid::Uuid;
 use proc_singleton::singleton_from_static;

 #[singleton_from_static(Identifier)]
 static IDENT: LazyLock<Identifier> = LazyLock::new(|| {
     Identifier {
         id: Uuid::new_v4(),
     }
 });

 struct Identifier {
     id: Uuid,
 }

 fn main() {
     let instance = Identifier::get_instance();
     let ptr = instance as *const Identifier;
     let same_ptr = Identifier::get_instance() as *const Identifier;

     assert_eq!(ptr, same_ptr);
 }
```
