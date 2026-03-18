# JB-API

<https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/>

<https://actix.rs/docs/server/>

`sea-orm-cli generate entity --database-url postgres://postgres:<PASSWORD>@localhost:5432/<DB_NAME>?options=--search_path=public --output-dir ./src/entities`

## TODO

1. ~~CRUD - Classes~~
2. ~~CRUD - Types~~
3. ~~CRUD - Brands~~
4. ~~CRUD - Products~~
   - ~~Readjust price~~
5. Budget
    - (Product, Amount, Size, Weight and Price)
    - Calc. Total Value
    - Gen. PDF

6. ~~Fix error~~
   - ~~Return generic message and log the error~~
7. ~~Remove `.clone` from DTO stuff~~
8. ~~Substitute `serde_json::json!` for `json!`~~
9. Transform find and retrieve block into function?
10. Blocked users should not be able to use protected routes
11. ~~Fix Logs~~
12. Products images
13. Delete images on delete (product and brands)
14. Expose `uploads` dir in `compose.yaml`?
