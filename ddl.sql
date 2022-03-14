-- start comment/base fields
-- CREATE TABLE public.shop_base (
--    id int8 primary key,
--    created_at timestamp without time zone default current_timestamp not null,
--    updated_at timestamp without time zone default current_timestamp not null,
--    is_deleted boolean default false not null,
--    version int4 default 0 not null
-- );
-- COMMENT ON COLUMN public.shop_base.id IS '分布式唯一ID';
-- COMMENT ON COLUMN public.shop_base.created_at IS '创建时间';
-- COMMENT ON COLUMN public.shop_base.updated_at IS '上次修改时间';
-- COMMENT ON COLUMN public.shop_base.is_deleted IS '是否被逻辑删除';
-- COMMENT ON COLUMN public.shop_base."version" IS '乐观锁版本号';
-- end comment/base fields
CREATE OR REPLACE FUNCTION trigger_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- start orders
create type order_status as enum('not_paid', 'user_cancel', 'paid_timeout_cancel');
create type order_type as enum('purchase', 'renewal');
create table if not exists orders(
    team_id int8 not null,
    product_id int8 not null,
    user_id int8 not null,
    service_id int8 not null,
    status order_status not null,
    type order_type not null,
    paid_at timestamp without time zone not null,
    price int4 not null,

    id int8 primary key,
    created_at timestamp without time zone default current_timestamp not null,
    updated_at timestamp without time zone default current_timestamp not null,
    is_deleted boolean default false not null,
    version int4 default 0 not null
);
comment on table orders is '订单表';
comment on column orders.service_id is '订单关联的已购服务 id';
comment on column orders.paid_at is '支付/开通时间';
comment on type order_status is '订单状态: 未支付、用户取消订单、支付超时取消订单';
comment on column orders.status is '订单状态: 未支付、用户取消订单、支付超时取消订单';
comment on type order_type is '订单类型: 首次购买、续费订单';
comment on column orders.type is '订单类型: 首次购买、续费订单';
CREATE TRIGGER set_updated_at_when_update
BEFORE UPDATE ON orders
FOR EACH ROW
EXECUTE FUNCTION trigger_updated_at();
-- end orders

-- start shop_resource_setmeal
CREATE TABLE public.resource_setmeal (
    type varchar NOT NULL,
    cpu_core int NOT NULL check (cpu_core > 0),
    memsize int NOT NULL,
    gpu_core int NOT NULL DEFAULT 0,

    id int8 primary key,
    created_at timestamp without time zone default current_timestamp not null,
    updated_at timestamp without time zone default current_timestamp not null,
    is_deleted boolean default false not null,
    version int4 default 0 not null
);
COMMENT ON TABLE public.resource_setmeal IS '资源套餐表 记录某个套餐的配置信息';
COMMENT ON COLUMN public.resource_setmeal.type IS '套餐类型:CPU/GPU';
COMMENT ON COLUMN public.resource_setmeal.cpu_core IS 'CPU核心数';
COMMENT ON COLUMN public.resource_setmeal.memsize IS '内存大小';
-- end resource_setmeal


--start shop_product
CREATE TABLE public.shop_product (
    "name" varchar not null,
    resource_setmeal_id bigint,
    "type" varchar not null,
    detail varchar,
    price int,

    id int8 primary key,
    created_at timestamp without time zone default current_timestamp not null,
    updated_at timestamp without time zone default current_timestamp not null,
    is_deleted boolean default false not null,
    version int4 default 0 not null
);
COMMENT ON TABLE public.shop_product IS '销售产品/SKU/服务 表';
comment on column public .shop_product."name" is '产品名字 标题';
comment on column public .shop_product.resource_setmeal_id  is '产品对应的资源的套餐id,仅仅产品是资源类型才会有';
comment on column public .shop_product."type" is '产品类型 可能是资源/服务';
comment on column public .shop_product.detail is '产品描述详情';
comment on column public .shop_product.price is '产品价格 单位是人民币的分';
--end shop_product

--start finance_account
CREATE TABLE if not exists public.finance_account (
                                        id bigint primary KEY,
                                        created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                        updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                        is_deleted bool NOT NULL DEFAULT false,
                                        "version" int NOT NULL DEFAULT 0,
                                   sea-     total_balance int not null default 0,
                                        avl_balance int not null default 0,
                                        blocked_balance int not null default 0,
                                        team_id bigint not null
);
COMMENT ON COLUMN public.finance_account.id IS 'ID,应该与team_id一致比较好';
COMMENT ON COLUMN public.finance_account.created_at IS '创建时间';
COMMENT ON COLUMN public.finance_account.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.finance_account.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.finance_account."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.finance_account.total_balance  IS '总余额';
COMMENT ON COLUMN public.finance_account.avl_balance  IS '可用余额';
COMMENT ON COLUMN public.finance_account.blocked_balance  IS '冻结余额';
COMMENT ON COLUMN public.finance_account.team_id  IS '账户对应团队di';
-- end finance_account

-- start setemal-price
CREATE TABLE setemal_price (
    "type" varchar NOT NULL,
    cpu_core int4 NOT NULL,
    mem_size int4 NOT NULL,
    gpu_core int4 NOT NULL DEFAULT 0,
    "storage" int4 NULL,
    price_id int8 NOT NULL,

    id int8 primary key,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted bool NOT NULL DEFAULT false,
    version int4 NOT NULL DEFAULT 0
);
-- end setmeal price

);
COMMENT ON TABLE public.resource_setmeal IS '资源套餐表 记录某个套餐的配置信息';

-- Column comments

COMMENT ON COLUMN public.resource_setmeal.id IS '资源套餐id';
COMMENT ON COLUMN public.resource_setmeal.created_at IS '创建时间';
COMMENT ON COLUMN public.resource_setmeal.updated_at IS '更新时间';
COMMENT ON COLUMN public.resource_setmeal.is_deleted IS '逻辑删除';
COMMENT ON COLUMN public.resource_setmeal."version" IS '乐观锁版本';
COMMENT ON COLUMN public.resource_setmeal."type" IS '套餐类型:CPU/GPU';
COMMENT ON COLUMN public.resource_setmeal.cpu_core IS 'CPU核心数';
COMMENT ON COLUMN public.resource_setmeal.memsize IS '内存大小';
COMMENT ON COLUMN public.resource_setmeal.gpu_core IS 'gpu核心数';
COMMENT ON COLUMN public.resource_setmeal."storage" IS '存储空间大小';
COMMENT ON COLUMN public.resource_setmeal.price_id IS '对应的价格表id,包月/包年/包年';
-- end resource_setmeal


--start setmeal_price
CREATE TABLE public.setmeal_price (
                                      id bigint primary key,
                                      created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                      updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                      is_deleted bool NOT NULL DEFAULT false,
                                      "version" int NOT NULL DEFAULT 0,
                                      hourly_price int not null,
                                      monthly_price int not null,
                                      annual_price int not null,
                                      discount int

);
COMMENT ON TABLE public.setmeal_price IS '资源套餐价格表 1对多 每个资源套餐对应的价格都对应一条价格记录 包含包月包年包时 折扣不同条件下的金额';
COMMENT ON COLUMN public.setmeal_price.id IS '套餐价格ID';
COMMENT ON COLUMN public.setmeal_price.created_at IS '创建时间';
COMMENT ON COLUMN public.setmeal_price.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.setmeal_price.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.setmeal_price."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.setmeal_price.hourly_price IS '按时付费每小时价格';
COMMENT ON COLUMN public.setmeal_price.monthly_price  IS '包月价格';
COMMENT ON COLUMN public.setmeal_price.annual_price  IS '包年价格';
COMMENT ON COLUMN public.setmeal_price.discount IS '折扣率 记录是整数 使用之前除以100先';
--end setmeal_price



-- start operation_service

CREATE TABLE operation_service (
    start_time timestamp without time zone not null default current_timestamp,
    expire_time timestamp without time zone not null,
    product_id bigint not null,
    resource_setmeal_id bigint not null,
    status varchar not null,

    id int8 primary key,
    created_at timestamp without time zone default current_timestamp not null,
    updated_at timestamp without time zone default current_timestamp not null,
    is_deleted boolean default false not null,
    version int4 default 0 not null
);
COMMENT ON TABLE operation_service IS '运营服务相关表 记录售出服务的到期时间 是否自动续费 开启服务的订单id,对应产品id,资源套餐id等信息';
COMMENT ON COLUMN operation_service.id IS '运营服务ID';
COMMENT ON COLUMN operation_service.created_at IS '创建时间';
COMMENT ON COLUMN operation_service.updated_at IS '上次修改时间';
COMMENT ON COLUMN operation_service.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN operation_service."version" IS '乐观锁版本号';

COMMENT ON COLUMN operation_service.start_time IS '服务开始时间';
COMMENT ON COLUMN operation_service.expire_time IS '服务过期时间';
COMMENT ON COLUMN operation_service.product_id IS '服务对应售卖产品id';
COMMENT ON COLUMN operation_service.resource_setmeal_id IS '服务对应资源套餐id';
COMMENT ON COLUMN operation_service.status IS '服务当前状态 生效中 准备中 已过期等';
-- end operation_service


-- start charge_record
CREATE TABLE public.charge_record (
                                      id bigint primary key,
                                      created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                      updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                      is_deleted bool NOT NULL DEFAULT false,
                                      "version" int NOT NULL DEFAULT 0,
                                      payment_id bigint not null,
                                      user_id bigint not null,
                                      team_id bigint not null,
                                      amount int not null,
                                      status varchar not null
);
COMMENT ON TABLE public.charge_record IS '充值记录表';
COMMENT ON COLUMN public.charge_record.id IS '充值记录ID';
COMMENT ON COLUMN public.charge_record.created_at IS '创建时间';
COMMENT ON COLUMN public.charge_record.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.charge_record.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.charge_record."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.charge_record.payment_id IS '对应的支付记录的id';
COMMENT ON COLUMN public.charge_record.user_id  IS '充值操作人账户id';
COMMENT ON COLUMN public.charge_record.team_id  IS '团队id';
COMMENT ON COLUMN public.charge_record.amount  IS '充值金额 单位为分';
COMMENT ON COLUMN public.charge_record.status  IS '充值状态 充值中 已完成 充值失败';
-- end charge_record




--start payment_record
CREATE TABLE if not exists public.payment_record (
                                       id bigint primary key,
                                       created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                       updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                       is_deleted bool NOT NULL DEFAULT false,
                                       "version" int NOT NULL DEFAULT 0,
                                       "type" varchar not null,
                                       amount int not null,
                                       out_trade_no varchar,
                                       subject varchar,
                                       detail varchar,
                                       status varchar not null
);
COMMENT ON COLUMN public.payment_record.id IS '支付记录ID';
COMMENT ON COLUMN public.payment_record.created_at IS '创建时间';
COMMENT ON COLUMN public.payment_record.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.payment_record.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.payment_record."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.payment_record."type" IS '支付方式 支付宝/微信/银行/线下付款';
COMMENT ON COLUMN public.payment_record.amount  IS '支付金额';
COMMENT ON COLUMN public.payment_record.out_trade_no  IS '第三方交易后 可能是支付宝微信的可能是银行流水号';
COMMENT ON COLUMN public.payment_record.subject  IS '支付标题，购买物标题等';
COMMENT ON COLUMN public.payment_record.detail  IS '其他细节详情';
COMMENT ON COLUMN public.payment_record.status  IS '支付状态 支付中 已完成 支付失败';
--end payment_record





--start expense_report
CREATE TABLE public.expense_report (
    id bigint primary key,
    created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
    updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
    is_deleted bool NOT NULL DEFAULT false,
    "version" int NOT NULL DEFAULT 0,
    order_id bigint not null,
    service_id bigint not null,
    amount int not null default 0,
    team_id int8 not null,
    service_name varchar not null,
    product_id bigint not null,
    product_name varchar not null
);
COMMENT ON TABLE public.expense_report IS '账单 每日生成的扣款账单记录 发往给用户';
COMMENT ON COLUMN public.expense_report.id IS '账单ID';
COMMENT ON COLUMN public.expense_report.created_at IS '创建时间';
COMMENT ON COLUMN public.expense_report.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.expense_report.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.expense_report."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.expense_report.order_id  IS '相关联的订单id';
COMMENT ON COLUMN public.expense_report.service_id  IS '相关联的已购服务id';
COMMENT ON COLUMN public.expense_report.amount  IS '账单金额';
COMMENT ON COLUMN public.expense_report.team_id  IS '账单所属团队id';
COMMENT ON COLUMN public.expense_report.service_name IS '相关联服务名 冗余字段';
COMMENT ON COLUMN public.expense_report.product_name  IS '相关联产品名 冗余字段';
COMMENT ON COLUMN public.expense_report.product_id  IS '相关联产品id';
--end expense_report

-- start finance_bill
CREATE TABLE if not exists public.finance_bill (
                                     id bigint primary key,
                                     created_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                     updated_at timestamp without time zone NOT NULL DEFAULT current_timestamp,
                                     is_deleted bool NOT NULL DEFAULT false,
                                     "version" int NOT NULL DEFAULT 0,
                                     "type" varchar not null,
                                     amount int not null,
                                     "source" varchar not null,
                                     link_id bigint not null,
                                     description varchar,
                                     team_id bigint not null
);

COMMENT ON COLUMN public.finance_bill.id IS '流水ID';
COMMENT ON COLUMN public.finance_bill.created_at IS '创建时间';
COMMENT ON COLUMN public.finance_bill.updated_at IS '上次修改时间';
COMMENT ON COLUMN public.finance_bill.is_deleted IS '是否被逻辑删除';
COMMENT ON COLUMN public.finance_bill."version" IS '乐观锁版本号';
COMMENT ON COLUMN public.finance_bill."type" IS '类型 支出/收入';
COMMENT ON COLUMN public.finance_bill."source" IS '来源';
COMMENT ON COLUMN public.finance_bill.amount IS '金额';
COMMENT ON COLUMN public.finance_bill.link_id  IS '关联id 可能是充值记录id 或账单id';
COMMENT ON COLUMN public.finance_bill.description  IS '详情描述';
COMMENT ON COLUMN public.finance_bill.team_id IS '所属团队id';
--end fiance bill
