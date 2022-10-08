#!/usr/bin/env python3

# This script could be used for Actix Web multipart example test
# just start server and run client.py

import json
import asyncio
import aiohttp
import sys

async def req():
    data = sys.stdin.readlines()
    data = ''.join(data)
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8080/',
        data=json.dumps({"content_type": "JSON", "content": data}),
        headers={"content-type": "application/json"})
    res = await resp.text()
    print("res : " + res)
    jsonRes = json.loads(res)
    assert 200 == resp.status
    print("json : "+jsonRes)
    print(jsonRes['response'])


asyncio.get_event_loop().run_until_complete(req())
