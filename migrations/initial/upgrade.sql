create table namespaces( \
    path text not null, \
\
    primary key (path) \
);

create table "values" \
 ( \
    path text not null, \
    key text not null, \
    value text not null, \
\
    primary key (path, key), \
    foreign key (path) references namespaces(path) \
);
