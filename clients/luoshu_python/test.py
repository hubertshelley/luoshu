import threading
from time import sleep

import luoshu_python
import asyncio

luoshu = luoshu_python.Luoshu("default", "test", "127.0.0.1", 8000)


# luoshu.config_subscribe("default|test", lambda x: print(x))


async def process():
    print("process")
    await luoshu.process()


# def config_subscribe():
#     print("config_subscribe")
#     t = threading.Thread(target=luoshu.config_subscribe, args=("default|test", lambda x: print(x)))
#     t.setDaemon(True)
#     t.start()


def async_running():
    print("async_running")
    t = threading.Thread(target=lambda: asyncio.run(process()))
    t.setDaemon(True)
    t.start()


if __name__ == "__main__":
    # result = luoshu.sum_as_string(1, 2)
    # print(result)
    # config_subscribe()
    async_running()
    sleep(1)
    # luoshu.test_subscribe("default|test")
    # t = threading.Thread(target=lambda: luoshu.config_subscribe("default", "test", lambda x: print("python recv", x)))
    # t.setDaemon(True)
    # t.start()
    luoshu.config_subscribe("default", "test", lambda x: print("python recv", x))

    # while True:
    #     sleep(1)
