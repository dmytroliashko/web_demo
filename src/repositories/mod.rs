mod post_repository;

use diesel::MysqlConnection;

pub type Repo = gotham_middleware_diesel::Repo<MysqlConnection>;
