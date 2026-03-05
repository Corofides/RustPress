CREATE TABLE posts (
  id integer primary key autoincrement,
  title TEXT,
  content TEXT,
  author integer
);

CREATE TABLE postsmeta (
  id integer primary key autoincrement,
  post integer,
  key TEXT,
  value TEXT
);

CREATE TABLE user (
  id integer primary key autoincrement,
  first_name TEXT,
  last_name TEXT,
  display_name TEXT
);

CREATE TABLE usersmeta (
  id integer primary key autoincrement,
  user integer,
  key TEXT,
  value TEXT
);
