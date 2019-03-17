//! SQLite dialect
#![feature(proc_macro_hygiene)]

use phf::phf_map;
use std::str;

mod token;
pub use crate::token::TokenType;

/// Token value (lexeme)
pub type Token = Option<String>;

impl TokenType {
    // TODO try Cow<&'static, str> (Borrowed<&'static str> for keyword and Owned<String> for below),
    // => Syntax error on keyword will be better
    // => `from_token` will become unecessary
    pub fn into_token(&self, value: &[u8]) -> Token {
        match self {
            TokenType::TK_CTIME_KW => Some(from_bytes(value)),
            TokenType::TK_JOIN_KW => Some(from_bytes(value)),
            TokenType::TK_LIKE_KW => Some(from_bytes(value)),
            // Identifiers
            TokenType::TK_STRING => Some(from_bytes(value)),
            TokenType::TK_ID => Some(from_bytes(value)),
            TokenType::TK_VARIABLE => Some(from_bytes(value)),
            // Values
            TokenType::TK_ANY => Some(from_bytes(value)),
            TokenType::TK_BLOB => Some(from_bytes(value)),
            TokenType::TK_INTEGER => Some(from_bytes(value)),
            TokenType::TK_FLOAT => Some(from_bytes(value)),
            _ => None,
        }
    }
}

pub fn from_bytes(bytes: &[u8]) -> String {
    unsafe { str::from_utf8_unchecked(bytes).to_owned() }
}

static KEYWORDS: phf::Map<&[u8], TokenType> = phf_map! {
    b"ABORT" => TokenType::TK_ABORT,
    b"ACTION" => TokenType::TK_ACTION,
    b"ADD" => TokenType::TK_ADD,
    b"AFTER" => TokenType::TK_AFTER,
    b"ALL" => TokenType::TK_ALL,
    b"ALTER" => TokenType::TK_ALTER,
    b"ANALYZE" => TokenType::TK_ANALYZE,
    b"AND" => TokenType::TK_AND,
    b"AS" => TokenType::TK_AS,
    b"ASC" => TokenType::TK_ASC,
    b"ATTACH" => TokenType::TK_ATTACH,
    b"AUTOINCREMENT" => TokenType::TK_AUTOINCR,
    b"BEFORE" => TokenType::TK_BEFORE,
    b"BEGIN" => TokenType::TK_BEGIN,
    b"BETWEEN" => TokenType::TK_BETWEEN,
    b"BY" => TokenType::TK_BY,
    b"CASCADE" => TokenType::TK_CASCADE,
    b"CASE" => TokenType::TK_CASE,
    b"CAST" => TokenType::TK_CAST,
    b"CHECK" => TokenType::TK_CHECK,
    b"COLLATE" => TokenType::TK_COLLATE,
    b"COLUMN" => TokenType::TK_COLUMNKW,
    b"COMMIT" => TokenType::TK_COMMIT,
    b"CONFLICT" => TokenType::TK_CONFLICT,
    b"CONSTRAINT" => TokenType::TK_CONSTRAINT,
    b"CREATE" => TokenType::TK_CREATE,
    b"CROSS" => TokenType::TK_JOIN_KW,
    b"CURRENT" => TokenType::TK_CURRENT,
    b"CURRENT_DATE" => TokenType::TK_CTIME_KW,
    b"CURRENT_TIME" => TokenType::TK_CTIME_KW,
    b"CURRENT_TIMESTAMP" => TokenType::TK_CTIME_KW,
    b"DATABASE" => TokenType::TK_DATABASE,
    b"DEFAULT" => TokenType::TK_DEFAULT,
    b"DEFERRABLE" => TokenType::TK_DEFERRABLE,
    b"DEFERRED" => TokenType::TK_DEFERRED,
    b"DELETE" => TokenType::TK_DELETE,
    b"DESC" => TokenType::TK_DESC,
    b"DETACH" => TokenType::TK_DETACH,
    b"DISTINCT" => TokenType::TK_DISTINCT,
    b"DO" => TokenType::TK_DO,
    b"DROP" => TokenType::TK_DROP,
    b"EACH" => TokenType::TK_EACH,
    b"ELSE" => TokenType::TK_ELSE,
    b"END" => TokenType::TK_END,
    b"ESCAPE" => TokenType::TK_ESCAPE,
    b"EXCEPT" => TokenType::TK_EXCEPT,
    b"EXCLUSIVE" => TokenType::TK_EXCLUSIVE,
    b"EXISTS" => TokenType::TK_EXISTS,
    b"EXPLAIN" => TokenType::TK_EXPLAIN,
    b"FAIL" => TokenType::TK_FAIL,
    b"FILTER" => TokenType::TK_FILTER,
    b"FOLLOWING" => TokenType::TK_FOLLOWING,
    b"FOR" => TokenType::TK_FOR,
    b"FOREIGN" => TokenType::TK_FOREIGN,
    b"FROM" => TokenType::TK_FROM,
    b"FULL" => TokenType::TK_JOIN_KW,
    b"GLOB" => TokenType::TK_LIKE_KW,
    b"GROUP" => TokenType::TK_GROUP,
    b"HAVING" => TokenType::TK_HAVING,
    b"IF" => TokenType::TK_IF,
    b"IGNORE" => TokenType::TK_IGNORE,
    b"IMMEDIATE" => TokenType::TK_IMMEDIATE,
    b"IN" => TokenType::TK_IN,
    b"INDEX" => TokenType::TK_INDEX,
    b"INDEXED" => TokenType::TK_INDEXED,
    b"INITIALLY" => TokenType::TK_INITIALLY,
    b"INNER" => TokenType::TK_JOIN_KW,
    b"INSERT" => TokenType::TK_INSERT,
    b"INSTEAD" => TokenType::TK_INSTEAD,
    b"INTERSECT" => TokenType::TK_INTERSECT,
    b"INTO" => TokenType::TK_INTO,
    b"IS" => TokenType::TK_IS,
    b"ISNULL" => TokenType::TK_ISNULL,
    b"JOIN" => TokenType::TK_JOIN,
    b"KEY" => TokenType::TK_KEY,
    b"LEFT" => TokenType::TK_JOIN_KW,
    b"LIKE" => TokenType::TK_LIKE_KW,
    b"LIMIT" => TokenType::TK_LIMIT,
    b"MATCH" => TokenType::TK_MATCH,
    b"NATURAL" => TokenType::TK_JOIN_KW,
    b"NO" => TokenType::TK_NO,
    b"NOT" => TokenType::TK_NOT,
    b"NOTHING" => TokenType::TK_NOTHING,
    b"NOTNULL" => TokenType::TK_NOTNULL,
    b"NULL" => TokenType::TK_NULL,
    b"OF" => TokenType::TK_OF,
    b"OFFSET" => TokenType::TK_OFFSET,
    b"ON" => TokenType::TK_ON,
    b"OR" => TokenType::TK_OR,
    b"ORDER" => TokenType::TK_ORDER,
    b"OUTER" => TokenType::TK_JOIN_KW,
    b"OVER" => TokenType::TK_OVER,
    b"PARTITION" => TokenType::TK_PARTITION,
    b"PLAN" => TokenType::TK_PLAN,
    b"PRAGMA" => TokenType::TK_PRAGMA,
    b"PRECEDING" => TokenType::TK_PRECEDING,
    b"PRIMARY" => TokenType::TK_PRIMARY,
    b"QUERY" => TokenType::TK_QUERY,
    b"RAISE" => TokenType::TK_RAISE,
    b"RANGE" => TokenType::TK_RANGE,
    b"RECURSIVE" => TokenType::TK_RECURSIVE,
    b"REFERENCES" => TokenType::TK_REFERENCES,
    b"REGEXP" => TokenType::TK_LIKE_KW,
    b"REINDEX" => TokenType::TK_REINDEX,
    b"RELEASE" => TokenType::TK_RELEASE,
    b"RENAME" => TokenType::TK_RENAME,
    b"REPLACE" => TokenType::TK_REPLACE,
    b"RESTRICT" => TokenType::TK_RESTRICT,
    b"RIGHT" => TokenType::TK_JOIN_KW,
    b"ROLLBACK" => TokenType::TK_ROLLBACK,
    b"ROW" => TokenType::TK_ROW,
    b"ROWS" => TokenType::TK_ROWS,
    b"SAVEPOINT" => TokenType::TK_SAVEPOINT,
    b"SELECT" => TokenType::TK_SELECT,
    b"SET" => TokenType::TK_SET,
    b"TABLE" => TokenType::TK_TABLE,
    b"TEMP" => TokenType::TK_TEMP,
    b"TEMPORARY" => TokenType::TK_TEMP,
    b"THEN" => TokenType::TK_THEN,
    b"TO" => TokenType::TK_TO,
    b"TRANSACTION" => TokenType::TK_TRANSACTION,
    b"TRIGGER" => TokenType::TK_TRIGGER,
    b"UNBOUNDED" => TokenType::TK_UNBOUNDED,
    b"UNION" => TokenType::TK_UNION,
    b"UNIQUE" => TokenType::TK_UNIQUE,
    b"UPDATE" => TokenType::TK_UPDATE,
    b"USING" => TokenType::TK_USING,
    b"VACUUM" => TokenType::TK_VACUUM,
    b"VALUES" => TokenType::TK_VALUES,
    b"VIEW" => TokenType::TK_VIEW,
    b"VIRTUAL" => TokenType::TK_VIRTUAL,
    b"WHEN" => TokenType::TK_WHEN,
    b"WHERE" => TokenType::TK_WHERE,
    b"WINDOW" => TokenType::TK_WINDOW,
    b"WITH" => TokenType::TK_WITH,
    b"WITHOUT" => TokenType::TK_WITHOUT
};
pub const MAX_KEYWORD_LEN: usize = 17;

/// word must be uppercase
pub fn keyword_token(word: &[u8]) -> Option<TokenType> {
    KEYWORDS.get(word).cloned()
}

pub fn is_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let bytes = name.as_bytes();
    is_identifier_start(bytes[0])
        && (bytes.len() == 1 || bytes[1..].iter().all(|b| is_identifier_continue(*b)))
}

pub fn is_identifier_start(b: u8) -> bool {
    (b >= b'A' && b <= b'Z') || b == b'_' || (b >= b'a' && b <= b'z') || b > b'\x7F'
}

pub fn is_identifier_continue(b: u8) -> bool {
    b == b'$'
        || (b >= b'0' && b <= b'9')
        || (b >= b'A' && b <= b'Z')
        || b == b'_'
        || (b >= b'a' && b <= b'z')
        || b > b'\x7F'
}

// keyword may become an identifier
pub fn from_token(ty: u16, value: Token) -> String {
    if let Some(str) = value {
        return str;
    }
    match ty {
        x if x == TokenType::TK_ABORT as u16 => "ABORT".to_owned(),
        x if x == TokenType::TK_ACTION as u16 => "ACTION".to_owned(),
        //x if x == TokenType::TK_ADD as u16 => "ADD".to_owned(),
        x if x == TokenType::TK_AFTER as u16 => "AFTER".to_owned(),
        //x if x == TokenType::TK_ALL as u16 => "ALL".to_owned(),
        //x if x == TokenType::TK_ALTER as u16 => "ALTER".to_owned(),
        x if x == TokenType::TK_ANALYZE as u16 => "ANALYZE".to_owned(),
        //x if x == TokenType::TK_AND as u16 => "AND".to_owned(),
        //x if x == TokenType::TK_AS as u16 => "AS".to_owned(),
        x if x == TokenType::TK_ASC as u16 => "ASC".to_owned(),
        x if x == TokenType::TK_ATTACH as u16 => "ATTACH".to_owned(),
        //x if x == TokenType::TK_AUTOINCR as u16 => "AUTOINCREMENT".to_owned(),
        x if x == TokenType::TK_BEFORE as u16 => "BEFORE".to_owned(),
        x if x == TokenType::TK_BEGIN as u16 => "BEGIN".to_owned(),
        //x if x == TokenType::TK_BETWEEN as u16 => "BETWEEN".to_owned(),
        x if x == TokenType::TK_BY as u16 => "BY".to_owned(),
        x if x == TokenType::TK_CASCADE as u16 => "CASCADE".to_owned(),
        //x if x == TokenType::TK_CASE as u16 => "CASE".to_owned(),
        x if x == TokenType::TK_CAST as u16 => "CAST".to_owned(),
        //x if x == TokenType::TK_CHECK as u16 => "CHECK".to_owned(),
        //x if x == TokenType::TK_COLLATE as u16 => "COLLATE".to_owned(),
        x if x == TokenType::TK_COLUMNKW as u16 => "COLUMN".to_owned(),
        //x if x == TokenType::TK_COMMIT as u16 => "COMMIT".to_owned(),
        x if x == TokenType::TK_CONFLICT as u16 => "CONFLICT".to_owned(),
        //x if x == TokenType::TK_CONSTRAINT as u16 => "CONSTRAINT".to_owned(),
        //x if x == TokenType::TK_CREATE as u16 => "CREATE".to_owned(),
        //x if x == TokenType::TK_CURRENT as u16 => "CURRENT".to_owned(),
        x if x == TokenType::TK_DATABASE as u16 => "DATABASE".to_owned(),
        x if x == TokenType::TK_DEFAULT as u16 => "DEFAULT".to_owned(),
        //x if x == TokenType::TK_DEFERRABLE as u16 => "DEFERRABLE".to_owned(),
        x if x == TokenType::TK_DEFERRED as u16 => "DEFERRED".to_owned(),
        x if x == TokenType::TK_DELETE as u16 => "DELETE".to_owned(),
        x if x == TokenType::TK_DESC as u16 => "DESC".to_owned(),
        x if x == TokenType::TK_DETACH as u16 => "DETACH".to_owned(),
        //x if x == TokenType::TK_DISTINCT as u16 => "DISTINCT".to_owned(),
        x if x == TokenType::TK_DO as u16 => "DO".to_owned(),
        //x if x == TokenType::TK_DROP as u16 => "DROP".to_owned(),
        x if x == TokenType::TK_EACH as u16 => "EACH".to_owned(),
        //x if x == TokenType::TK_ELSE as u16 => "ELSE".to_owned(),
        x if x == TokenType::TK_END as u16 => "END".to_owned(),
        //x if x == TokenType::TK_ESCAPE as u16 => "ESCAPE".to_owned(),
        //x if x == TokenType::TK_EXCEPT as u16 => "EXCEPT".to_owned(),
        x if x == TokenType::TK_EXCLUSIVE as u16 => "EXCLUSIVE".to_owned(),
        //x if x == TokenType::TK_EXISTS as u16 => "EXISTS".to_owned(),
        x if x == TokenType::TK_EXPLAIN as u16 => "EXPLAIN".to_owned(),
        x if x == TokenType::TK_FAIL as u16 => "FAIL".to_owned(),
        //x if x == TokenType::TK_FILTER as u16 => "FILTER".to_owned(),
        //x if x == TokenType::TK_FOLLOWING as u16 => "FOLLOWING".to_owned(),
        x if x == TokenType::TK_FOR as u16 => "FOR".to_owned(),
        //x if x == TokenType::TK_FOREIGN as u16 => "FOREIGN".to_owned(),
        //x if x == TokenType::TK_FROM as u16 => "FROM".to_owned(),
        //x if x == TokenType::TK_GROUP as u16 => "GROUP".to_owned(),
        //x if x == TokenType::TK_HAVING as u16 => "HAVING".to_owned(),
        x if x == TokenType::TK_IF as u16 => "IF".to_owned(),
        x if x == TokenType::TK_IGNORE as u16 => "IGNORE".to_owned(),
        x if x == TokenType::TK_IMMEDIATE as u16 => "IMMEDIATE".to_owned(),
        //x if x == TokenType::TK_IN as u16 => "IN".to_owned(),
        //x if x == TokenType::TK_INDEX as u16 => "INDEX".to_owned(),
        x if x == TokenType::TK_INDEXED as u16 => "INDEXED".to_owned(),
        x if x == TokenType::TK_INITIALLY as u16 => "INITIALLY".to_owned(),
        //x if x == TokenType::TK_INSERT as u16 => "INSERT".to_owned(),
        x if x == TokenType::TK_INSTEAD as u16 => "INSTEAD".to_owned(),
        //x if x == TokenType::TK_INTERSECT as u16 => "INTERSECT".to_owned(),
        //x if x == TokenType::TK_INTO as u16 => "INTO".to_owned(),
        //x if x == TokenType::TK_IS as u16 => "IS".to_owned(),
        //x if x == TokenType::TK_ISNULL as u16 => "ISNULL".to_owned(),
        //x if x == TokenType::TK_JOIN as u16 => "JOIN".to_owned(),
        x if x == TokenType::TK_KEY as u16 => "KEY".to_owned(),
        //x if x == TokenType::TK_LIMIT as u16 => "LIMIT".to_owned(),
        x if x == TokenType::TK_MATCH as u16 => "MATCH".to_owned(),
        x if x == TokenType::TK_NO as u16 => "NO".to_owned(),
        //x if x == TokenType::TK_NOT as u16 => "NOT".to_owned(),
        //x if x == TokenType::TK_NOTHING as u16 => "NOTHING".to_owned(),
        //x if x == TokenType::TK_NOTNULL as u16 => "NOTNULL".to_owned(),
        //x if x == TokenType::TK_NULL as u16 => "NULL".to_owned(),
        x if x == TokenType::TK_OF as u16 => "OF".to_owned(),
        x if x == TokenType::TK_OFFSET as u16 => "OFFSET".to_owned(),
        x if x == TokenType::TK_ON as u16 => "ON".to_owned(),
        //x if x == TokenType::TK_OR as u16 => "OR".to_owned(),
        //x if x == TokenType::TK_ORDER as u16 => "ORDER".to_owned(),
        //x if x == TokenType::TK_OVER as u16 => "OVER".to_owned(),
        //x if x == TokenType::TK_PARTITION as u16 => "PARTITION".to_owned(),
        x if x == TokenType::TK_PLAN as u16 => "PLAN".to_owned(),
        x if x == TokenType::TK_PRAGMA as u16 => "PRAGMA".to_owned(),
        //x if x == TokenType::TK_PRECEDING as u16 => "PRECEDING".to_owned(),
        //x if x == TokenType::TK_PRIMARY as u16 => "PRIMARY".to_owned(),
        x if x == TokenType::TK_QUERY as u16 => "QUERY".to_owned(),
        x if x == TokenType::TK_RAISE as u16 => "RAISE".to_owned(),
        //x if x == TokenType::TK_RANGE as u16 => "RANGE".to_owned(),
        x if x == TokenType::TK_RECURSIVE as u16 => "RECURSIVE".to_owned(),
        //x if x == TokenType::TK_REFERENCES as u16 => "REFERENCES".to_owned(),
        //x if x == TokenType::TK_REINDEX as u16 => "REINDEX".to_owned(),
        x if x == TokenType::TK_RELEASE as u16 => "RELEASE".to_owned(),
        //x if x == TokenType::TK_RENAME as u16 => "RENAME".to_owned(),
        x if x == TokenType::TK_REPLACE as u16 => "REPLACE".to_owned(),
        x if x == TokenType::TK_RESTRICT as u16 => "RESTRICT".to_owned(),
        x if x == TokenType::TK_ROLLBACK as u16 => "ROLLBACK".to_owned(),
        x if x == TokenType::TK_ROW as u16 => "ROW".to_owned(),
        x if x == TokenType::TK_ROWS as u16 => "ROWS".to_owned(),
        x if x == TokenType::TK_SAVEPOINT as u16 => "SAVEPOINT".to_owned(),
        //x if x == TokenType::TK_SELECT as u16 => "SELECT".to_owned(),
        //x if x == TokenType::TK_SET as u16 => "SET".to_owned(),
        //x if x == TokenType::TK_TABLE as u16 => "TABLE".to_owned(),
        x if x == TokenType::TK_TEMP as u16 => "TEMP".to_owned(),
        //x if x == TokenType::TK_TEMP as u16 => "TEMPORARY".to_owned(),
        //x if x == TokenType::TK_THEN as u16 => "THEN".to_owned(),
        //x if x == TokenType::TK_TO as u16 => "TO".to_owned(),
        //x if x == TokenType::TK_TRANSACTION as u16 => "TRANSACTION".to_owned(),
        x if x == TokenType::TK_TRIGGER as u16 => "TRIGGER".to_owned(),
        //x if x == TokenType::TK_UNBOUNDED as u16 => "UNBOUNDED".to_owned(),
        //x if x == TokenType::TK_UNION as u16 => "UNION".to_owned(),
        //x if x == TokenType::TK_UNIQUE as u16 => "UNIQUE".to_owned(),
        //x if x == TokenType::TK_UPDATE as u16 => "UPDATE".to_owned(),
        //x if x == TokenType::TK_USING as u16 => "USING".to_owned(),
        x if x == TokenType::TK_VACUUM as u16 => "VACUUM".to_owned(),
        x if x == TokenType::TK_VALUES as u16 => "VALUES".to_owned(),
        x if x == TokenType::TK_VIEW as u16 => "VIEW".to_owned(),
        x if x == TokenType::TK_VIRTUAL as u16 => "VIRTUAL".to_owned(),
        //x if x == TokenType::TK_WHEN as u16 => "WHEN".to_owned(),
        //x if x == TokenType::TK_WHERE as u16 => "WHERE".to_owned(),
        //x if x == TokenType::TK_WINDOW as u16 => "WINDOW".to_owned(),
        x if x == TokenType::TK_WITH as u16 => "WITH".to_owned(),
        x if x == TokenType::TK_WITHOUT as u16 => "WITHOUT".to_owned(),
        _ => unreachable!(),
    }
}
