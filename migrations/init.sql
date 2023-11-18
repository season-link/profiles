create table candidate (
    id uuid primary key,
    first_name VARCHAR(255) not null,
    last_name VARCHAR(255) not null,
    birth_date timestamp not null,
    nationality_country_id VARCHAR(255) not null,
    description text not null,
    email VARCHAR(255) not null,
    phone_number VARCHAR(30) not null,
    address VARCHAR(255) not null,
    gender smallint not null,
    is_available boolean not null,
    available_from timestamp not null,
    available_to timestamp not null,
    place VARCHAR(255) not null,
    job_id uuid not null
);

create table experience (
    id uuid primary key,
    candidate_id uuid references candidate(id) not null,
    company_name VARCHAR(255) not null,
    job_id uuid not null,
    start_time timestamp not null,
    end_time timestamp not null,
    description text not null
);

create table reference (
    id uuid primary key,
    candidate_id uuid references candidate(id) not null,
    first_name VARCHAR(50) not null,
    last_name VARCHAR(50) not null,
    email VARCHAR(255) not null,
    phone_number VARCHAR(30) not null,
    company_name VARCHAR(255) not null
);