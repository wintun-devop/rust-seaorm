###
```
cargo new your_project
```
###
```
cargo add tokio --features full
```
```
cargo add dotenvy
```
```
cargo add serde serde_json
```

### sea-orm basic packages
- 
```
cargo add sea-orm --features sqlx-postgres,runtime-tokio-rustls,macros
```
```
cargo add serde --features derive
```
- sea orm migration
```
cargo add sea-orm-migration --features sqlx-postgres,runtime-tokio-rustls
```

### sea-orm migration
```
cargo install sea-orm-cli
```
```
sea-orm-cli migrate init
```
```
sea-orm-cli migrate generate create_tc_user
```
```
sea-orm-cli migrate up
```