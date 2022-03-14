#!/bin/bash
set -exu

# TODO use gitlab CI variable
ROBOT_API_KEY=47b52a76-0379-446b-8649-e822432e70b3

# idp_shop_demo 新的代码提交，请尽快 pull 最新 commit
curl "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=$ROBOT_API_KEY" \
   -H 'Content-Type: application/json' \
   -d '{ "msgtype": "text", "text": { "content": "gitlab: 新代码提交的 CI 检查已通过"} }'
