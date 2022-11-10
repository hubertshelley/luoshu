import threading
from time import sleep

import luoshu_python
import asyncio


async def process(namespace, name, host, port):
    print("process")
    await luoshu_python.sleep(namespace, name, host, port)


def async_running():
    t = threading.Thread(target=lambda: asyncio.run(process("default", "test", "127.0.0.1", 8000)))
    t.setDaemon(True)
    t.start()


if __name__ == "__main__":
    # result = luoshu.sum_as_string(1, 2)
    # print(result)
    async_running()
    while True:
        sleep(1)
