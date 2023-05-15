-- Add migration screatecript here
create table posts(title varchar(255), description varchar(255),name varchar(225),Foreign key(name) references categories(name))