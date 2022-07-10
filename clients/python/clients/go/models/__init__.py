# flake8: noqa

# import all models into this package
# if you have many models here with many references from one model to another this may
# raise a RecursionError
# to avoid this, import only the models that you directly need like:
# from from clients/go.model.pet import Pet
# or import this package, but before doing it, use:
# import sys
# sys.setrecursionlimit(n)

from clients/go.model.add_email_channel_body import AddEmailChannelBody
from clients/go.model.add_telegram_channel_body import AddTelegramChannelBody
from clients/go.model.remove_channel_body import RemoveChannelBody
from clients/go.model.send_notification_body import SendNotificationBody
