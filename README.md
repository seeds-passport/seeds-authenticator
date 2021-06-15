# seeds-autheticator

WIP/Prototype

### Dependencies

- diesel - `cargo install diesel_cli --no-default-features --features postgres` - libpq-dev also needed for this.

### Configuration

Configuration is split in 2 files: `.env` and `configuration/environment.rs`. Examples for both are provided. Please configure these files before running the database setup.

### Setup database

`diesel setup`

### Migrations

#### Up: `diesel migration run`

#### Down: `diesel migration undo`

#### Redo `diesel migration redo`
