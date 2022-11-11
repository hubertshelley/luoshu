import threading
from time import sleep

import luoshu_python
import asyncio

luoshu = luoshu_python.Luoshu("default", "test", "127.0.0.1", 8000)

luoshu.config_subscribe("default|test", lambda: print("1"))


async def process():
    print("process")
    await luoshu.process()


def async_running():
    t = threading.Thread(target=lambda: asyncio.run(process()))
    t.setDaemon(True)
    t.start()


if __name__ == "__main__":
    # result = luoshu.sum_as_string(1, 2)
    # print(result)
    async_running()
    while True:
        sleep(1)
