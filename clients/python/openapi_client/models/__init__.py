# flake8: noqa

# import all models into this package
# if you have many models here with many references from one model to another this may
# raise a RecursionError
# to avoid this, import only the models that you directly need like:
# from from openapi_client.model.pet import Pet
# or import this package, but before doing it, use:
# import sys
# sys.setrecursionlimit(n)

from openapi_client.model.add_channel_body import AddChannelBody
from openapi_client.model.channel_response import ChannelResponse
from openapi_client.model.channels_response import ChannelsResponse
from openapi_client.model.get_telegram_chat_id_body import GetTelegramChatIdBody
from openapi_client.model.notify_body import NotifyBody
from openapi_client.model.remove_channel_body import RemoveChannelBody
