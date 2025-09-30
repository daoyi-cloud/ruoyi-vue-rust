/*
 Yudao Database Transfer Tool

 Source Server Type    : MySQL

 Target Server Type    : PostgreSQL

 Date: 2025-05-22 21:03:59
*/


-- ----------------------------
-- Table structure for dual
-- ----------------------------
DROP TABLE IF EXISTS dual;
CREATE TABLE dual
(
    id int4
);

COMMENT ON TABLE dual IS '数据库连接的表';

-- ----------------------------
-- Records of dual
-- ----------------------------
-- @formatter:off
INSERT INTO dual VALUES (1);
-- @formatter:on

-- ----------------------------
-- Table structure for infra_api_access_log
-- ----------------------------
DROP TABLE IF EXISTS infra_api_access_log;
CREATE TABLE infra_api_access_log
(
    id               int8         NOT NULL,
    trace_id         varchar(64)  NOT NULL DEFAULT '',
    user_id          int8         NOT NULL DEFAULT 0,
    user_type        int4         NOT NULL DEFAULT 0,
    application_name varchar(50)  NOT NULL,
    request_method   varchar(16)  NOT NULL DEFAULT '',
    request_url      varchar(255) NOT NULL DEFAULT '',
    request_params   text         NULL,
    response_body    text         NULL,
    user_ip          varchar(50)  NOT NULL,
    user_agent       varchar(512) NOT NULL,
    operate_module   varchar(50)  NULL     DEFAULT NULL,
    operate_name     varchar(50)  NULL     DEFAULT NULL,
    operate_type     int4         NULL     DEFAULT 0,
    begin_time       timestamp    NOT NULL,
    end_time         timestamp    NOT NULL,
    duration         int4         NOT NULL,
    result_code      int4         NOT NULL DEFAULT 0,
    result_msg       varchar(512) NULL     DEFAULT '',
    creator          varchar(64)  NULL     DEFAULT '',
    create_time      timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater          varchar(64)  NULL     DEFAULT '',
    update_time      timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted          int4         NOT NULL DEFAULT 0,
    tenant_id        int8         NOT NULL DEFAULT 0
);

ALTER TABLE infra_api_access_log
    ADD CONSTRAINT pk_infra_api_access_log PRIMARY KEY (id);

CREATE INDEX idx_infra_api_access_log_01 ON infra_api_access_log (create_time);

COMMENT ON COLUMN infra_api_access_log.id IS '日志主键';
COMMENT ON COLUMN infra_api_access_log.trace_id IS '链路追踪编号';
COMMENT ON COLUMN infra_api_access_log.user_id IS '用户编号';
COMMENT ON COLUMN infra_api_access_log.user_type IS '用户类型';
COMMENT ON COLUMN infra_api_access_log.application_name IS '应用名';
COMMENT ON COLUMN infra_api_access_log.request_method IS '请求方法名';
COMMENT ON COLUMN infra_api_access_log.request_url IS '请求地址';
COMMENT ON COLUMN infra_api_access_log.request_params IS '请求参数';
COMMENT ON COLUMN infra_api_access_log.response_body IS '响应结果';
COMMENT ON COLUMN infra_api_access_log.user_ip IS '用户 IP';
COMMENT ON COLUMN infra_api_access_log.user_agent IS '浏览器 UA';
COMMENT ON COLUMN infra_api_access_log.operate_module IS '操作模块';
COMMENT ON COLUMN infra_api_access_log.operate_name IS '操作名';
COMMENT ON COLUMN infra_api_access_log.operate_type IS '操作分类';
COMMENT ON COLUMN infra_api_access_log.begin_time IS '开始请求时间';
COMMENT ON COLUMN infra_api_access_log.end_time IS '结束请求时间';
COMMENT ON COLUMN infra_api_access_log.duration IS '执行时长';
COMMENT ON COLUMN infra_api_access_log.result_code IS '结果码';
COMMENT ON COLUMN infra_api_access_log.result_msg IS '结果提示';
COMMENT ON COLUMN infra_api_access_log.creator IS '创建者';
COMMENT ON COLUMN infra_api_access_log.create_time IS '创建时间';
COMMENT ON COLUMN infra_api_access_log.updater IS '更新者';
COMMENT ON COLUMN infra_api_access_log.update_time IS '更新时间';
COMMENT ON COLUMN infra_api_access_log.deleted IS '是否删除';
COMMENT ON COLUMN infra_api_access_log.tenant_id IS '租户编号';
COMMENT ON TABLE infra_api_access_log IS 'API 访问日志表';

DROP SEQUENCE IF EXISTS infra_api_access_log_seq;
CREATE SEQUENCE infra_api_access_log_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_api_access_log
    ALTER COLUMN id SET DEFAULT nextval('infra_api_access_log_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_api_access_log_seq
    OWNED BY infra_api_access_log.id;

-- ----------------------------
-- Table structure for infra_api_error_log
-- ----------------------------
DROP TABLE IF EXISTS infra_api_error_log;
CREATE TABLE infra_api_error_log
(
    id                           int8          NOT NULL,
    trace_id                     varchar(64)   NOT NULL,
    user_id                      int8          NOT NULL DEFAULT 0,
    user_type                    int4          NOT NULL DEFAULT 0,
    application_name             varchar(50)   NOT NULL,
    request_method               varchar(16)   NOT NULL,
    request_url                  varchar(255)  NOT NULL,
    request_params               varchar(8000) NOT NULL,
    user_ip                      varchar(50)   NOT NULL,
    user_agent                   varchar(512)  NOT NULL,
    exception_time               timestamp     NOT NULL,
    exception_name               varchar(128)  NOT NULL DEFAULT '',
    exception_message            text          NOT NULL,
    exception_root_cause_message text          NOT NULL,
    exception_stack_trace        text          NOT NULL,
    exception_class_name         varchar(512)  NOT NULL,
    exception_file_name          varchar(512)  NOT NULL,
    exception_method_name        varchar(512)  NOT NULL,
    exception_line_number        int4          NOT NULL,
    process_status               int4          NOT NULL,
    process_time                 timestamp     NULL     DEFAULT NULL,
    process_user_id              int4          NULL     DEFAULT 0,
    creator                      varchar(64)   NULL     DEFAULT '',
    create_time                  timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater                      varchar(64)   NULL     DEFAULT '',
    update_time                  timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted                      int4          NOT NULL DEFAULT 0,
    tenant_id                    int8          NOT NULL DEFAULT 0
);

ALTER TABLE infra_api_error_log
    ADD CONSTRAINT pk_infra_api_error_log PRIMARY KEY (id);

COMMENT ON COLUMN infra_api_error_log.id IS '编号';
COMMENT ON COLUMN infra_api_error_log.trace_id IS '链路追踪编号';
COMMENT ON COLUMN infra_api_error_log.user_id IS '用户编号';
COMMENT ON COLUMN infra_api_error_log.user_type IS '用户类型';
COMMENT ON COLUMN infra_api_error_log.application_name IS '应用名';
COMMENT ON COLUMN infra_api_error_log.request_method IS '请求方法名';
COMMENT ON COLUMN infra_api_error_log.request_url IS '请求地址';
COMMENT ON COLUMN infra_api_error_log.request_params IS '请求参数';
COMMENT ON COLUMN infra_api_error_log.user_ip IS '用户 IP';
COMMENT ON COLUMN infra_api_error_log.user_agent IS '浏览器 UA';
COMMENT ON COLUMN infra_api_error_log.exception_time IS '异常发生时间';
COMMENT ON COLUMN infra_api_error_log.exception_name IS '异常名';
COMMENT ON COLUMN infra_api_error_log.exception_message IS '异常导致的消息';
COMMENT ON COLUMN infra_api_error_log.exception_root_cause_message IS '异常导致的根消息';
COMMENT ON COLUMN infra_api_error_log.exception_stack_trace IS '异常的栈轨迹';
COMMENT ON COLUMN infra_api_error_log.exception_class_name IS '异常发生的类全名';
COMMENT ON COLUMN infra_api_error_log.exception_file_name IS '异常发生的类文件';
COMMENT ON COLUMN infra_api_error_log.exception_method_name IS '异常发生的方法名';
COMMENT ON COLUMN infra_api_error_log.exception_line_number IS '异常发生的方法所在行';
COMMENT ON COLUMN infra_api_error_log.process_status IS '处理状态';
COMMENT ON COLUMN infra_api_error_log.process_time IS '处理时间';
COMMENT ON COLUMN infra_api_error_log.process_user_id IS '处理用户编号';
COMMENT ON COLUMN infra_api_error_log.creator IS '创建者';
COMMENT ON COLUMN infra_api_error_log.create_time IS '创建时间';
COMMENT ON COLUMN infra_api_error_log.updater IS '更新者';
COMMENT ON COLUMN infra_api_error_log.update_time IS '更新时间';
COMMENT ON COLUMN infra_api_error_log.deleted IS '是否删除';
COMMENT ON COLUMN infra_api_error_log.tenant_id IS '租户编号';
COMMENT ON TABLE infra_api_error_log IS '系统异常日志';

DROP SEQUENCE IF EXISTS infra_api_error_log_seq;
CREATE SEQUENCE infra_api_error_log_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_api_error_log
    ALTER COLUMN id SET DEFAULT nextval('infra_api_error_log_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_api_error_log_seq
    OWNED BY infra_api_error_log.id;

-- ----------------------------
-- Table structure for infra_codegen_column
-- ----------------------------
DROP TABLE IF EXISTS infra_codegen_column;
CREATE TABLE infra_codegen_column
(
    id                       int8         NOT NULL,
    table_id                 int8         NOT NULL,
    column_name              varchar(200) NOT NULL,
    data_type                varchar(100) NOT NULL,
    column_comment           varchar(500) NOT NULL,
    nullable                 bool         NOT NULL,
    primary_key              bool         NOT NULL,
    ordinal_position         int4         NOT NULL,
    java_type                varchar(32)  NOT NULL,
    java_field               varchar(64)  NOT NULL,
    dict_type                varchar(200) NULL     DEFAULT '',
    example                  varchar(64)  NULL     DEFAULT NULL,
    create_operation         bool         NOT NULL,
    update_operation         bool         NOT NULL,
    list_operation           bool         NOT NULL,
    list_operation_condition varchar(32)  NOT NULL DEFAULT '=',
    list_operation_result    bool         NOT NULL,
    html_type                varchar(32)  NOT NULL,
    creator                  varchar(64)  NULL     DEFAULT '',
    create_time              timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater                  varchar(64)  NULL     DEFAULT '',
    update_time              timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted                  int4         NOT NULL DEFAULT 0
);

ALTER TABLE infra_codegen_column
    ADD CONSTRAINT pk_infra_codegen_column PRIMARY KEY (id);

COMMENT ON COLUMN infra_codegen_column.id IS '编号';
COMMENT ON COLUMN infra_codegen_column.table_id IS '表编号';
COMMENT ON COLUMN infra_codegen_column.column_name IS '字段名';
COMMENT ON COLUMN infra_codegen_column.data_type IS '字段类型';
COMMENT ON COLUMN infra_codegen_column.column_comment IS '字段描述';
COMMENT ON COLUMN infra_codegen_column.nullable IS '是否允许为空';
COMMENT ON COLUMN infra_codegen_column.primary_key IS '是否主键';
COMMENT ON COLUMN infra_codegen_column.ordinal_position IS '排序';
COMMENT ON COLUMN infra_codegen_column.java_type IS 'Java 属性类型';
COMMENT ON COLUMN infra_codegen_column.java_field IS 'Java 属性名';
COMMENT ON COLUMN infra_codegen_column.dict_type IS '字典类型';
COMMENT ON COLUMN infra_codegen_column.example IS '数据示例';
COMMENT ON COLUMN infra_codegen_column.create_operation IS '是否为 Create 创建操作的字段';
COMMENT ON COLUMN infra_codegen_column.update_operation IS '是否为 Update 更新操作的字段';
COMMENT ON COLUMN infra_codegen_column.list_operation IS '是否为 List 查询操作的字段';
COMMENT ON COLUMN infra_codegen_column.list_operation_condition IS 'List 查询操作的条件类型';
COMMENT ON COLUMN infra_codegen_column.list_operation_result IS '是否为 List 查询操作的返回字段';
COMMENT ON COLUMN infra_codegen_column.html_type IS '显示类型';
COMMENT ON COLUMN infra_codegen_column.creator IS '创建者';
COMMENT ON COLUMN infra_codegen_column.create_time IS '创建时间';
COMMENT ON COLUMN infra_codegen_column.updater IS '更新者';
COMMENT ON COLUMN infra_codegen_column.update_time IS '更新时间';
COMMENT ON COLUMN infra_codegen_column.deleted IS '是否删除';
COMMENT ON TABLE infra_codegen_column IS '代码生成表字段定义';

DROP SEQUENCE IF EXISTS infra_codegen_column_seq;
CREATE SEQUENCE infra_codegen_column_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_codegen_column
    ALTER COLUMN id SET DEFAULT nextval('infra_codegen_column_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_codegen_column_seq
    OWNED BY infra_codegen_column.id;

-- ----------------------------
-- Table structure for infra_codegen_table
-- ----------------------------
DROP TABLE IF EXISTS infra_codegen_table;
CREATE TABLE infra_codegen_table
(
    id                    int8         NOT NULL,
    data_source_config_id int8         NOT NULL,
    scene                 int4         NOT NULL DEFAULT 1,
    table_name            varchar(200) NOT NULL DEFAULT '',
    table_comment         varchar(500) NOT NULL DEFAULT '',
    remark                varchar(500) NULL     DEFAULT NULL,
    module_name           varchar(30)  NOT NULL,
    business_name         varchar(30)  NOT NULL,
    class_name            varchar(100) NOT NULL DEFAULT '',
    class_comment         varchar(50)  NOT NULL,
    author                varchar(50)  NOT NULL,
    template_type         int4         NOT NULL DEFAULT 1,
    front_type            int4         NOT NULL,
    parent_menu_id        int8         NULL     DEFAULT NULL,
    master_table_id       int8         NULL     DEFAULT NULL,
    sub_join_column_id    int8         NULL     DEFAULT NULL,
    sub_join_many         bool         NULL     DEFAULT NULL,
    tree_parent_column_id int8         NULL     DEFAULT NULL,
    tree_name_column_id   int8         NULL     DEFAULT NULL,
    creator               varchar(64)  NULL     DEFAULT '',
    create_time           timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater               varchar(64)  NULL     DEFAULT '',
    update_time           timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted               int4         NOT NULL DEFAULT 0
);

ALTER TABLE infra_codegen_table
    ADD CONSTRAINT pk_infra_codegen_table PRIMARY KEY (id);

COMMENT ON COLUMN infra_codegen_table.id IS '编号';
COMMENT ON COLUMN infra_codegen_table.data_source_config_id IS '数据源配置的编号';
COMMENT ON COLUMN infra_codegen_table.scene IS '生成场景';
COMMENT ON COLUMN infra_codegen_table.table_name IS '表名称';
COMMENT ON COLUMN infra_codegen_table.table_comment IS '表描述';
COMMENT ON COLUMN infra_codegen_table.remark IS '备注';
COMMENT ON COLUMN infra_codegen_table.module_name IS '模块名';
COMMENT ON COLUMN infra_codegen_table.business_name IS '业务名';
COMMENT ON COLUMN infra_codegen_table.class_name IS '类名称';
COMMENT ON COLUMN infra_codegen_table.class_comment IS '类描述';
COMMENT ON COLUMN infra_codegen_table.author IS '作者';
COMMENT ON COLUMN infra_codegen_table.template_type IS '模板类型';
COMMENT ON COLUMN infra_codegen_table.front_type IS '前端类型';
COMMENT ON COLUMN infra_codegen_table.parent_menu_id IS '父菜单编号';
COMMENT ON COLUMN infra_codegen_table.master_table_id IS '主表的编号';
COMMENT ON COLUMN infra_codegen_table.sub_join_column_id IS '子表关联主表的字段编号';
COMMENT ON COLUMN infra_codegen_table.sub_join_many IS '主表与子表是否一对多';
COMMENT ON COLUMN infra_codegen_table.tree_parent_column_id IS '树表的父字段编号';
COMMENT ON COLUMN infra_codegen_table.tree_name_column_id IS '树表的名字字段编号';
COMMENT ON COLUMN infra_codegen_table.creator IS '创建者';
COMMENT ON COLUMN infra_codegen_table.create_time IS '创建时间';
COMMENT ON COLUMN infra_codegen_table.updater IS '更新者';
COMMENT ON COLUMN infra_codegen_table.update_time IS '更新时间';
COMMENT ON COLUMN infra_codegen_table.deleted IS '是否删除';
COMMENT ON TABLE infra_codegen_table IS '代码生成表定义';

DROP SEQUENCE IF EXISTS infra_codegen_table_seq;
CREATE SEQUENCE infra_codegen_table_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_codegen_table
    ALTER COLUMN id SET DEFAULT nextval('infra_codegen_table_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_codegen_table_seq
    OWNED BY infra_codegen_table.id;

-- ----------------------------
-- Table structure for infra_config
-- ----------------------------
DROP TABLE IF EXISTS infra_config;
CREATE TABLE infra_config
(
    id          int8         NOT NULL,
    category    varchar(50)  NOT NULL,
    type        int4         NOT NULL,
    name        varchar(100) NOT NULL DEFAULT '',
    config_key  varchar(100) NOT NULL DEFAULT '',
    value       varchar(500) NOT NULL DEFAULT '',
    visible     bool         NOT NULL,
    remark      varchar(500) NULL     DEFAULT NULL,
    creator     varchar(64)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     int4         NOT NULL DEFAULT 0
);

ALTER TABLE infra_config
    ADD CONSTRAINT pk_infra_config PRIMARY KEY (id);

COMMENT ON COLUMN infra_config.id IS '参数主键';
COMMENT ON COLUMN infra_config.category IS '参数分组';
COMMENT ON COLUMN infra_config.type IS '参数类型';
COMMENT ON COLUMN infra_config.name IS '参数名称';
COMMENT ON COLUMN infra_config.config_key IS '参数键名';
COMMENT ON COLUMN infra_config.value IS '参数键值';
COMMENT ON COLUMN infra_config.visible IS '是否可见';
COMMENT ON COLUMN infra_config.remark IS '备注';
COMMENT ON COLUMN infra_config.creator IS '创建者';
COMMENT ON COLUMN infra_config.create_time IS '创建时间';
COMMENT ON COLUMN infra_config.updater IS '更新者';
COMMENT ON COLUMN infra_config.update_time IS '更新时间';
COMMENT ON COLUMN infra_config.deleted IS '是否删除';
COMMENT ON TABLE infra_config IS '参数配置表';

-- ----------------------------
-- Records of infra_config
-- ----------------------------
-- @formatter:off
BEGIN;
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (2, 'biz', 1, '用户管理-账号初始密码', 'system.user.init-password', '123456', '0', '初始化密码 123456', 'admin', '2021-01-05 17:03:48', '1', '2024-07-20 17:22:47', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (7, 'url', 2, 'MySQL 监控的地址', 'url.druid', '', '1', '', '1', '2023-04-07 13:41:16', '1', '2023-04-07 14:33:38', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (8, 'url', 2, 'SkyWalking 监控的地址', 'url.skywalking', '', '1', '', '1', '2023-04-07 13:41:16', '1', '2023-04-07 14:57:03', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (9, 'url', 2, 'Spring Boot Admin 监控的地址', 'url.spring-boot-admin', '', '1', '', '1', '2023-04-07 13:41:16', '1', '2023-04-07 14:52:07', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (10, 'url', 2, 'Swagger 接口文档的地址', 'url.swagger', '', '1', '', '1', '2023-04-07 13:41:16', '1', '2023-04-07 14:59:00', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (11, 'ui', 2, '腾讯地图 key', 'tencent.lbs.key', 'TVDBZ-TDILD-4ON4B-PFDZA-RNLKH-VVF6E', '1', '腾讯地图 key', '1', '2023-06-03 19:16:27', '1', '2023-06-03 19:16:27', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (12, 'test2', 2, 'test3', 'test4', 'test5', '1', 'test6', '1', '2023-12-03 09:55:16', '1', '2025-04-06 21:00:09', '0');
INSERT INTO infra_config (id, category, type, name, config_key, value, visible, remark, creator, create_time, updater, update_time, deleted) VALUES (13, '用户管理-账号初始密码', 2, '用户管理-注册开关', 'system.user.register-enabled', 'true', '0', '', '1', '2025-04-26 17:23:41', '1', '2025-04-26 17:23:41', '0');
COMMIT;
-- @formatter:on

DROP SEQUENCE IF EXISTS infra_config_seq;
CREATE SEQUENCE infra_config_seq
    START 14;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_api_access_log
    ALTER COLUMN id SET DEFAULT nextval('infra_config_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_config_seq
    OWNED BY infra_config.id;

-- ----------------------------
-- Table structure for infra_data_source_config
-- ----------------------------
DROP TABLE IF EXISTS infra_data_source_config;
CREATE TABLE infra_data_source_config
(
    id          int8          NOT NULL,
    name        varchar(100)  NOT NULL DEFAULT '',
    url         varchar(1024) NOT NULL,
    username    varchar(255)  NOT NULL,
    password    varchar(255)  NOT NULL DEFAULT '',
    creator     varchar(64)   NULL     DEFAULT '',
    create_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)   NULL     DEFAULT '',
    update_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     int4          NOT NULL DEFAULT 0
);

ALTER TABLE infra_data_source_config
    ADD CONSTRAINT pk_infra_data_source_config PRIMARY KEY (id);

COMMENT ON COLUMN infra_data_source_config.id IS '主键编号';
COMMENT ON COLUMN infra_data_source_config.name IS '参数名称';
COMMENT ON COLUMN infra_data_source_config.url IS '数据源连接';
COMMENT ON COLUMN infra_data_source_config.username IS '用户名';
COMMENT ON COLUMN infra_data_source_config.password IS '密码';
COMMENT ON COLUMN infra_data_source_config.creator IS '创建者';
COMMENT ON COLUMN infra_data_source_config.create_time IS '创建时间';
COMMENT ON COLUMN infra_data_source_config.updater IS '更新者';
COMMENT ON COLUMN infra_data_source_config.update_time IS '更新时间';
COMMENT ON COLUMN infra_data_source_config.deleted IS '是否删除';
COMMENT ON TABLE infra_data_source_config IS '数据源配置表';

DROP SEQUENCE IF EXISTS infra_data_source_config_seq;
CREATE SEQUENCE infra_data_source_config_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_data_source_config
    ALTER COLUMN id SET DEFAULT nextval('infra_data_source_config_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_data_source_config_seq
    OWNED BY infra_data_source_config.id;

-- ----------------------------
-- Table structure for infra_file
-- ----------------------------
DROP TABLE IF EXISTS infra_file;
CREATE TABLE infra_file
(
    id          int8          NOT NULL,
    config_id   int8          NULL     DEFAULT NULL,
    name        varchar(256)  NULL     DEFAULT NULL,
    path        varchar(512)  NOT NULL,
    url         varchar(1024) NOT NULL,
    type        varchar(128)  NULL     DEFAULT NULL,
    size        int4          NOT NULL,
    creator     varchar(64)   NULL     DEFAULT '',
    create_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)   NULL     DEFAULT '',
    update_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     int4          NOT NULL DEFAULT 0
);

ALTER TABLE infra_file
    ADD CONSTRAINT pk_infra_file PRIMARY KEY (id);

COMMENT ON COLUMN infra_file.id IS '文件编号';
COMMENT ON COLUMN infra_file.config_id IS '配置编号';
COMMENT ON COLUMN infra_file.name IS '文件名';
COMMENT ON COLUMN infra_file.path IS '文件路径';
COMMENT ON COLUMN infra_file.url IS '文件 URL';
COMMENT ON COLUMN infra_file.type IS '文件类型';
COMMENT ON COLUMN infra_file.size IS '文件大小';
COMMENT ON COLUMN infra_file.creator IS '创建者';
COMMENT ON COLUMN infra_file.create_time IS '创建时间';
COMMENT ON COLUMN infra_file.updater IS '更新者';
COMMENT ON COLUMN infra_file.update_time IS '更新时间';
COMMENT ON COLUMN infra_file.deleted IS '是否删除';
COMMENT ON TABLE infra_file IS '文件表';

DROP SEQUENCE IF EXISTS infra_file_seq;
CREATE SEQUENCE infra_file_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_file
    ALTER COLUMN id SET DEFAULT nextval('infra_file_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_file_seq
    OWNED BY infra_file.id;

-- ----------------------------
-- Table structure for infra_file_config
-- ----------------------------
DROP TABLE IF EXISTS infra_file_config;
CREATE TABLE infra_file_config
(
    id          int8          NOT NULL,
    name        varchar(63)   NOT NULL,
    storage     int4          NOT NULL,
    remark      varchar(255)  NULL     DEFAULT NULL,
    master      bool          NOT NULL,
    config      varchar(4096) NOT NULL,
    creator     varchar(64)   NULL     DEFAULT '',
    create_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)   NULL     DEFAULT '',
    update_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     int4          NOT NULL DEFAULT 0
);

ALTER TABLE infra_file_config
    ADD CONSTRAINT pk_infra_file_config PRIMARY KEY (id);

COMMENT ON COLUMN infra_file_config.id IS '编号';
COMMENT ON COLUMN infra_file_config.name IS '配置名';
COMMENT ON COLUMN infra_file_config.storage IS '存储器';
COMMENT ON COLUMN infra_file_config.remark IS '备注';
COMMENT ON COLUMN infra_file_config.master IS '是否为主配置';
COMMENT ON COLUMN infra_file_config.config IS '存储配置';
COMMENT ON COLUMN infra_file_config.creator IS '创建者';
COMMENT ON COLUMN infra_file_config.create_time IS '创建时间';
COMMENT ON COLUMN infra_file_config.updater IS '更新者';
COMMENT ON COLUMN infra_file_config.update_time IS '更新时间';
COMMENT ON COLUMN infra_file_config.deleted IS '是否删除';
COMMENT ON TABLE infra_file_config IS '文件配置表';

-- ----------------------------
-- Records of infra_file_config
-- ----------------------------
-- @formatter:off
BEGIN;
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (4, '数据库（示例）', 1, '我是数据库', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.db.DBFileClientConfig","domain":"http://127.0.0.1:48080"}', '1', '2022-03-15 23:56:24', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (22, '七牛存储器（示例）', 20, '请换成你自己的密钥！！！', '1', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"s3.cn-south-1.qiniucs.com","domain":"http://test.yudao.iocoder.cn","bucket":"ruoyi-vue-pro","accessKey":"***","accessSecret":"***","enablePathStyleAccess":false}', '1', '2024-01-13 22:11:12', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (24, '腾讯云存储（示例）', 20, '请换成你的密钥！！！', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"https://cos.ap-shanghai.myqcloud.com","domain":"http://tengxun-oss.iocoder.cn","bucket":"aoteman-1255880240","accessKey":"***","accessSecret":"***"}', '1', '2024-11-09 16:03:22', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (25, '阿里云存储（示例）', 20, '', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"oss-cn-beijing.aliyuncs.com","domain":"http://ali-oss.iocoder.cn","bucket":"yunai-aoteman","accessKey":"***","accessSecret":"***","enablePathStyleAccess":false}', '1', '2024-11-09 16:47:08', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (26, '火山云存储（示例）', 20, '', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"tos-s3-cn-beijing.volces.com","domain":null,"bucket":"yunai","accessKey":"***","accessSecret":"***","enablePathStyleAccess":false}', '1', '2024-11-09 16:56:42', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (27, '华为云存储（示例）', 20, '', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"obs.cn-east-3.myhuaweicloud.com","domain":"","bucket":"yudao","accessKey":"***","accessSecret":"***","enablePathStyleAccess":false}', '1', '2024-11-09 17:18:41', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (28, 'MinIO 存储（示例）', 20, '', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.s3.S3FileClientConfig","endpoint":"http://127.0.0.1:9000","domain":"http://127.0.0.1:9000/yudao","bucket":"yudao","accessKey":"***","accessSecret":"***","enablePathStyleAccess":false}', '1', '2024-11-09 17:43:10', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (29, '本地存储（示例）', 10, '仅适合 mac 或 windows', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.local.LocalFileClientConfig","basePath":"/Users/yunai/tmp/file","domain":"http://127.0.0.1:48080"}', '1', '2025-05-02 11:25:45', '1', '2025-05-02 18:30:28', '0');
INSERT INTO infra_file_config (id, name, storage, remark, master, config, creator, create_time, updater, update_time, deleted) VALUES (30, 'SFTP 存储（示例）', 12, '', '0', '{"@class":"cn.iocoder.yudao.module.infra.framework.file.core.client.sftp.SftpFileClientConfig","basePath":"/upload","domain":"http://127.0.0.1:48080","host":"127.0.0.1","port":2222,"username":"foo","password":"pass"}', '1', '2025-05-02 16:34:10', '1', '2025-05-02 18:30:28', '0');
COMMIT;
-- @formatter:on

DROP SEQUENCE IF EXISTS infra_file_config_seq;
CREATE SEQUENCE infra_file_config_seq
    START 31;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_file_config
    ALTER COLUMN id SET DEFAULT nextval('infra_file_config_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_file_config_seq
    OWNED BY infra_file_config.id;

-- ----------------------------
-- Table structure for infra_file_content
-- ----------------------------
DROP TABLE IF EXISTS infra_file_content;
CREATE TABLE infra_file_content
(
    id          int8         NOT NULL,
    config_id   int8         NOT NULL,
    path        varchar(512) NOT NULL,
    content     bytea        NOT NULL,
    creator     varchar(64)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     int4         NOT NULL DEFAULT 0
);

ALTER TABLE infra_file_content
    ADD CONSTRAINT pk_infra_file_content PRIMARY KEY (id);

COMMENT ON COLUMN infra_file_content.id IS '编号';
COMMENT ON COLUMN infra_file_content.config_id IS '配置编号';
COMMENT ON COLUMN infra_file_content.path IS '文件路径';
COMMENT ON COLUMN infra_file_content.content IS '文件内容';
COMMENT ON COLUMN infra_file_content.creator IS '创建者';
COMMENT ON COLUMN infra_file_content.create_time IS '创建时间';
COMMENT ON COLUMN infra_file_content.updater IS '更新者';
COMMENT ON COLUMN infra_file_content.update_time IS '更新时间';
COMMENT ON COLUMN infra_file_content.deleted IS '是否删除';
COMMENT ON TABLE infra_file_content IS '文件表';

DROP SEQUENCE IF EXISTS infra_file_content_seq;
CREATE SEQUENCE infra_file_content_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_file_content
    ALTER COLUMN id SET DEFAULT nextval('infra_file_content_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_file_content_seq
    OWNED BY infra_file_content.id;

-- ----------------------------
-- Table structure for infra_job
-- ----------------------------
DROP TABLE IF EXISTS infra_job;
CREATE TABLE infra_job
(
    id              int8         NOT NULL,
    name            varchar(32)  NOT NULL,
    status          int4         NOT NULL,
    handler_name    varchar(64)  NOT NULL,
    handler_param   varchar(255) NULL     DEFAULT NULL,
    cron_expression varchar(32)  NOT NULL,
    retry_count     int4         NOT NULL DEFAULT 0,
    retry_interval  int4         NOT NULL DEFAULT 0,
    monitor_timeout int4         NOT NULL DEFAULT 0,
    creator         varchar(64)  NULL     DEFAULT '',
    create_time     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater         varchar(64)  NULL     DEFAULT '',
    update_time     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted         int4         NOT NULL DEFAULT 0
);

ALTER TABLE infra_job
    ADD CONSTRAINT pk_infra_job PRIMARY KEY (id);

COMMENT ON COLUMN infra_job.id IS '任务编号';
COMMENT ON COLUMN infra_job.name IS '任务名称';
COMMENT ON COLUMN infra_job.status IS '任务状态';
COMMENT ON COLUMN infra_job.handler_name IS '处理器的名字';
COMMENT ON COLUMN infra_job.handler_param IS '处理器的参数';
COMMENT ON COLUMN infra_job.cron_expression IS 'CRON 表达式';
COMMENT ON COLUMN infra_job.retry_count IS '重试次数';
COMMENT ON COLUMN infra_job.retry_interval IS '重试间隔';
COMMENT ON COLUMN infra_job.monitor_timeout IS '监控超时时间';
COMMENT ON COLUMN infra_job.creator IS '创建者';
COMMENT ON COLUMN infra_job.create_time IS '创建时间';
COMMENT ON COLUMN infra_job.updater IS '更新者';
COMMENT ON COLUMN infra_job.update_time IS '更新时间';
COMMENT ON COLUMN infra_job.deleted IS '是否删除';
COMMENT ON TABLE infra_job IS '定时任务表';

-- ----------------------------
-- Records of infra_job
-- ----------------------------
-- @formatter:off
BEGIN;
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (5, '支付通知 Job', 2, 'payNotifyJob', NULL, '* * * * * ?', 0, 0, 0, '1', '2021-10-27 08:34:42', '1', '2024-09-12 13:32:48', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (17, '支付订单同步 Job', 2, 'payOrderSyncJob', NULL, '0 0/1 * * * ?', 0, 0, 0, '1', '2023-07-22 14:36:26', '1', '2023-07-22 15:39:08', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (18, '支付订单过期 Job', 2, 'payOrderExpireJob', NULL, '0 0/1 * * * ?', 0, 0, 0, '1', '2023-07-22 15:36:23', '1', '2023-07-22 15:39:54', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (19, '退款订单的同步 Job', 2, 'payRefundSyncJob', NULL, '0 0/1 * * * ?', 0, 0, 0, '1', '2023-07-23 21:03:44', '1', '2023-07-23 21:09:00', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (21, '交易订单的自动过期 Job', 2, 'tradeOrderAutoCancelJob', '', '0 * * * * ?', 3, 0, 0, '1', '2023-09-25 23:43:26', '1', '2023-09-26 19:23:30', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (22, '交易订单的自动收货 Job', 2, 'tradeOrderAutoReceiveJob', '', '0 * * * * ?', 3, 0, 0, '1', '2023-09-26 19:23:53', '1', '2023-09-26 23:38:08', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (23, '交易订单的自动评论 Job', 2, 'tradeOrderAutoCommentJob', '', '0 * * * * ?', 3, 0, 0, '1', '2023-09-26 23:38:29', '1', '2023-09-27 11:03:10', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (24, '佣金解冻 Job', 2, 'brokerageRecordUnfreezeJob', '', '0 * * * * ?', 3, 0, 0, '1', '2023-09-28 22:01:46', '1', '2023-09-28 22:01:56', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (25, '访问日志清理 Job', 2, 'accessLogCleanJob', '', '0 0 0 * * ?', 3, 0, 0, '1', '2023-10-03 10:59:41', '1', '2023-10-03 11:01:10', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (26, '错误日志清理 Job', 2, 'errorLogCleanJob', '', '0 0 0 * * ?', 3, 0, 0, '1', '2023-10-03 11:00:43', '1', '2023-10-03 11:01:12', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (27, '任务日志清理 Job', 2, 'jobLogCleanJob', '', '0 0 0 * * ?', 3, 0, 0, '1', '2023-10-03 11:01:33', '1', '2024-09-12 13:40:34', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (33, 'demoJob', 2, 'demoJob', '', '0 * * * * ?', 1, 1, 0, '1', '2024-10-27 19:38:46', '1', '2025-05-10 18:13:54', '0');
INSERT INTO infra_job (id, name, status, handler_name, handler_param, cron_expression, retry_count, retry_interval, monitor_timeout, creator, create_time, updater, update_time, deleted) VALUES (35, '转账订单的同步 Job', 2, 'payTransferSyncJob', '', '0 * * * * ?', 0, 0, 0, '1', '2025-05-10 17:35:54', '1', '2025-05-10 18:13:52', '0');
COMMIT;
-- @formatter:on

DROP SEQUENCE IF EXISTS infra_job_seq;
CREATE SEQUENCE infra_job_seq
    START 36;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_job
    ALTER COLUMN id SET DEFAULT nextval('infra_job_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_job_seq
    OWNED BY infra_job.id;

-- ----------------------------
-- Table structure for infra_job_log
-- ----------------------------
DROP TABLE IF EXISTS infra_job_log;
CREATE TABLE infra_job_log
(
    id            int8          NOT NULL,
    job_id        int8          NOT NULL,
    handler_name  varchar(64)   NOT NULL,
    handler_param varchar(255)  NULL     DEFAULT NULL,
    execute_index int4          NOT NULL DEFAULT 1,
    begin_time    timestamp     NOT NULL,
    end_time      timestamp     NULL     DEFAULT NULL,
    duration      int4          NULL     DEFAULT NULL,
    status        int4          NOT NULL,
    result        varchar(4000) NULL     DEFAULT '',
    creator       varchar(64)   NULL     DEFAULT '',
    create_time   timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(64)   NULL     DEFAULT '',
    update_time   timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       int4          NOT NULL DEFAULT 0
);

ALTER TABLE infra_job_log
    ADD CONSTRAINT pk_infra_job_log PRIMARY KEY (id);

COMMENT ON COLUMN infra_job_log.id IS '日志编号';
COMMENT ON COLUMN infra_job_log.job_id IS '任务编号';
COMMENT ON COLUMN infra_job_log.handler_name IS '处理器的名字';
COMMENT ON COLUMN infra_job_log.handler_param IS '处理器的参数';
COMMENT ON COLUMN infra_job_log.execute_index IS '第几次执行';
COMMENT ON COLUMN infra_job_log.begin_time IS '开始执行时间';
COMMENT ON COLUMN infra_job_log.end_time IS '结束执行时间';
COMMENT ON COLUMN infra_job_log.duration IS '执行时长';
COMMENT ON COLUMN infra_job_log.status IS '任务状态';
COMMENT ON COLUMN infra_job_log.result IS '结果数据';
COMMENT ON COLUMN infra_job_log.creator IS '创建者';
COMMENT ON COLUMN infra_job_log.create_time IS '创建时间';
COMMENT ON COLUMN infra_job_log.updater IS '更新者';
COMMENT ON COLUMN infra_job_log.update_time IS '更新时间';
COMMENT ON COLUMN infra_job_log.deleted IS '是否删除';
COMMENT ON TABLE infra_job_log IS '定时任务日志表';

DROP SEQUENCE IF EXISTS infra_job_log_seq;
CREATE SEQUENCE infra_job_log_seq
    START 1;
-- 将序列与表的 id 字段关联
ALTER TABLE infra_job_log
    ALTER COLUMN id SET DEFAULT nextval('infra_job_log_seq');

-- 设置序列由表拥有
ALTER SEQUENCE infra_job_log_seq
    OWNED BY infra_job_log.id;
