create table namespaces ( \
    path text not null, \
\
    primary key (path) \
);

create table permissions ( \
    subject_id text not null, \
    path text not null, \
    permissions text not null, \
\
    primary key (subject_id, path), \
    foreign key (path) references namespaces (path) on delete cascade \
);

create table "values" ( \
    path  text not null, \
    key   text not null, \
    value text not null, \
    version integer not null, \
\
    primary key (path, key), \
    foreign key (path) references namespaces (path) on delete cascade \
);
