# flake8: noqa

# import all models into this package
# if you have many models here with many references from one model to another this may
# raise a RecursionError
# to avoid this, import only the models that you directly need like:
# from from clients/python.model.pet import Pet
# or import this package, but before doing it, use:
# import sys
# sys.setrecursionlimit(n)

from clients/python.model.add_email_channel_body import AddEmailChannelBody
from clients/python.model.add_telegram_channel_body import AddTelegramChannelBody
from clients/python.model.remove_channel_body import RemoveChannelBody
from clients/python.model.send_notification_body import SendNotificationBody
