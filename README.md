# SubjecView Backend
[![Rust](https://img.shields.io/badge/rust-3670A0?style=for-the-badge&logo=rust&logoColor=ffdd54)](https://www.rust-lang.org/) [![PostgreSQL](https://img.shields.io/badge/postgresql-003545?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/) [![Rocket.rs](https://img.shields.io/badge/Rocket.rs-092E20?style=for-the-badge&logo=Rocket.rs&logoColor=white)](https://rocket.rs/) [![Railway](https://img.shields.io/badge/railway-%23FF9900.svg?style=for-the-badge&logo=railway&logoColor=white)](https://railway.app/) [![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/) [![Github](https://img.shields.io/badge/github-%230047B3.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/)

# Crates
| Crate            | Path                                           | Description                          |
| ---------------- | ---------------------------------------------- | ------------------------------------ |
| `migrator`       | [migrator](migrator)                           | Migrations                           |
| `orbit`          | [orbit](orbit)                                 | REST API server                      |
| `quark`          | [quark](quark)                                 | Logic & More                         |

## Setting up local development
We only need docker, vscode and the [DevContainers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension

## Apply migrations
`sea-orm-cli migrate -d ./migrator`

to connect to the database you can forward the 5432 port on the vscode PORTS tab

## Run the api server
`cargo run`

## API Documentation
This project uses multiple documentation

- **Scalar Docs**: [/scalar]()
- **ReDoc**: [/redoc]()
- **Swagger UI**: [/swagger-ui]()
- **Rapidoc**: [/rapidoc]()

## Deployed on Railway
https://subjectview-api-production.up.railway.app/