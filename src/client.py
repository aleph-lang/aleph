#!/usr/bin/env python3

# This script could be used for Actix Web multipart example test
# just start server and run client.py

import json
import asyncio
import aiohttp
import sys
import argparse

async def req(args):
    data = sys.stdin.readlines()
    data = ''.join(data)

    if args.in_type == None:
        args.in_type = "json"

    if args.out_type == None:
        args.out_type = "ale"

    if args.transformer_list == None:
        args.transformer_list = []
    else:
        args.transformer_list = args.transformer_list.split(",")


    session = aiohttp.ClientSession()
    resp = await session.request(
        "post", 'http://localhost:8080/',
        data=json.dumps({"content_type": args.in_type, "content": data,
                            "return_type": args.out_type, "transformer_list":args.transformer_list}),
        headers={"content-type": "application/json"})
    res = await resp.text()
    jsonRes = json.loads(res)
    assert 200 == resp.status
    print(jsonRes["response"])
    await session.close()


parser = argparse.ArgumentParser(description ='Compile given file')
parser.add_argument('-i', dest ='in_type', action ='store', help ='input type')
parser.add_argument('-o', dest ='out_type',action ='store', help ='output type')
parser.add_argument('-t', dest ='transformer_list',action ='store', help ='list of transforms functions to be used (separed by a comma)')

args = parser.parse_args()

asyncio.get_event_loop().run_until_complete(req(args))
