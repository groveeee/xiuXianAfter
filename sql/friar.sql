create table friar
(
    id      uuid not null
        constraint xx_friar_pk
            primary key,
    name    varchar(6),
    reiki   bigint    default 0,
    birth   timestamp default now(),
    life    bigint    default 120,
    die     boolean   default false,
    realm   integer   default 1,
    account varchar(16)
        constraint name_unique
            unique,
    passwd  varchar
);

comment on table friar is '修士';

comment on column friar.id is '主键ID';

comment on column friar.name is '修士姓名';

comment on column friar.reiki is '灵气';

comment on column friar.birth is '创始日期';

comment on column friar.life is '寿命(年)';

comment on column friar.die is '死亡';

comment on column friar.realm is '境界';

comment on column friar.account is '账户名';

alter table friar
    owner to grovee;

INSERT INTO xiuxian.friar (id, name, reiki, birth, life, die, realm, account, passwd) VALUES ('62ccd189-d948-47a0-87fa-7f0996939e65', null, 0, '2023-08-13 14:11:10.541526', 120, false, 1, 'jack', 'e10adc3949ba59abbe56e057f20f883e');
INSERT INTO xiuxian.friar (id, name, reiki, birth, life, die, realm, account, passwd) VALUES ('62b89e97-7d98-4deb-b77a-7c06073ab7dd', null, 0, '2023-08-13 14:21:12.430958', 120, false, 1, 'smith', 'e10adc3949ba59abbe56e057f20f883e');
INSERT INTO xiuxian.friar (id, name, reiki, birth, life, die, realm, account, passwd) VALUES ('ec3b3c53-44e2-49a5-9d4f-1054ce58449f', null, 999, '2023-08-13 13:44:33.269552', 120, false, 999, 'grovee', 'e10adc3949ba59abbe56e057f20f883e');
