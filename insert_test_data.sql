insert into orders(
    id,team_id,
    product_id,
    user_id,
    service_id,
    status,
    type,
    paid_at,
    price
) values(
    1,
    1,
    1,
    1,
    1,
    'not_paid',
    'purchase',
    now(),
    1
);
insert into operation_service(id,expire_time,product_id,resource_setmeal_id,status) values(1,now(),1,1,'expired');
