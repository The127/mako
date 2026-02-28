create table namespaces( \
    name text not null, \
\
    primary key (name) \
);
create table "values" \
 ( \
    namespace text not null, \
    key text not null, \
    value text not null, \
\
    primary key (namespace, key), \
    foreign key (namespace) references namespaces(name) \
);
