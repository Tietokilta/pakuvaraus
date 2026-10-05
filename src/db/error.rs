use sqlx::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("constraint violation: {0}")]
    Constraint(&'static str),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error)
}

// taken from github:launchbadge/realworld-axum-sqlx
/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&ctxt.db)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> DbError,
    ) -> Result<T, DbError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<DbError>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> DbError,
    ) -> Result<T, DbError> {
        self.map_err(|e| match e.into() {
            DbError::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}

