drop database if exists rustoj;
create database rustoj;
use rustoj;



/*==============================================================*/
/* Table: tb_user                                               */
/*==============================================================*/
drop table if exists `tb_user`;
create table `tb_user` (
	`user_name` varchar(64) primary key not null,
	`user_password` varchar(32) not null,
	`sex` int,
	`email` varchar(32) not null
);

insert into `tb_user` values('Durant', '123456', 1, 'durant@email.com');


/*==============================================================*/
/* Table: tb_problem                                            */
/*==============================================================*/
drop table if exists `tb_problem`;
create table `tb_problem` (
	`problem_id` int primary key auto_increment,
	`problem_title` varchar(64) not null,
	`problem_path` varchar(255) not null
);

INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (1, '字符串编辑', './problems/1');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (2, '数制转换', './problems/2');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (3, '砝码称重', './problems/3');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (4, '麦森数', './problems/4');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (5, '侦探推理', './problems/5');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (6, '传染病控制', './problems/6');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (7, '不高兴的津津', './problems/7');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (8, '花生采摘', './problems/8');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (9, 'FBI树', './problems/9');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (10, '传纸条', './problems/10');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (11, '关押罪犯', './problems/11');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (12, '观光公交（Day 2）', './problems/12');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (13, '疫情控制（Day 2）', './problems/13');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (14, '表达式求值', './problems/14');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (15, '花匠（Day 2）', './problems/15');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (16, '表达式求值', './problems/16');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (17, '丽洁体', './problems/17');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (18, '话旧', './problems/18');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (19, '[ZJOI2010]任务安排', './problems/19');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (20, '图书管理员', './problems/20');
INSERT INTO `tb_problem` (`problem_id`, `problem_title`, `problem_path`)
VALUES (100, '图书管理员', './problems/100');


/*==============================================================*/
/* Table: tb_contest                                               */
/*==============================================================*/
drop table if exists `tb_contest`;
create table `tb_contest` (
	`contest_id` int primary key auto_increment,
	`contest_name` varchar(64) not null,
	`contest_status` varchar(32) not null,
	`contest_intro` text,
	`contest_startTime` datetime,
	`contest_endTime` datetime,
	`contest_rule` varchar(32),
	`contest_creator` varchar(32),
	`contest_password` varchar(32),
	`contest_realTimeRank` boolean,
	`contest_visible`  boolean,
	`contest_lastUpdateTime` datetime
);
/*==============================================================*/
/* Table: tb_rank                                               */
/*==============================================================*/
drop table if exists `tb_rank`;
create table `tb_rank` (
	`rank` int,
	`contest_id` int,
	`user_name` varchar(64),
	`penalty` int
);
alter table `tb_rank`
add foreign key(`contest_id`) references `tb_contest`(`contest_id`);
alter table `tb_rank`
add foreign key(`user_name`) references `tb_user`(`user_name`);


drop table if exists `tb_submission`;
create table tb_submission(
   id int primary key auto_increment,
   contest int ,
   problem int,
   create_time datetime,
   username varchar(32),
   language varchar(32),
   code text,
   result int,
   time_cost   int default 0,
   memory_cost int default 0,
   err_info    varchar(32) default '',
   score       int default 0
);

select * from tb_user;


insert into `tb_submission`(`id`,`contest`, `problem`, `username`,`code`,`result`,`create_time`)
    values (null,1, 1, '12','123',0,now());
select * from tb_submission;

