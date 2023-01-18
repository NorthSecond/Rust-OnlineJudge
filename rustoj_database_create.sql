drop database if exists rustoj;

create database rustoj;

use rustoj;

/*==============================================================*/
/* Table: tb_user                                               */
/*==============================================================*/
create table tb_user
(
    user_name     varchar(64) primary key not null,
    user_password varchar(32) not null,
    sex           int,
		email         varchar(32) not null,
);



/*==============================================================*/
/* Table: tb_problem                                               */
/*==============================================================*/
create table tb_problem
(
    problem_id        int primary key auto_increment,
    problem_title     varchar(64) not null,
    problem_level     varchar(32) not null,
    problem_total     int,
		problem_acrate    double,
		problem_content   text not null
);




/*==============================================================*/
/* Table: tb_contest                                               */
/*==============================================================*/
create table tb_contest
(
		contest_id          int primary key auto_increment,
		contest_name  	    varchar(64) not null,
		contest_status      varchar(32) not null,
    contest_intro       text,
		contest_startTime   datetime,
		contest_endTime     datetime,
		contest_type        varchar(32),
		contest_rule        varchar(32),
		contest_creator     varchar(32),
		contest_password    varchar(32)
);



/*==============================================================*/
/* Table: tb_rank                                               */
/*==============================================================*/
create table tb_rank
(
		rank                int,
		contest_id          int,
		user_name     			varchar(64),
		penalty             int,
);
alter table tb_rank add foreign key(contest_id) references tb_contest(contest_id);
alter table tb_rank add foreign key(user_name) references tb_user(user_name);



