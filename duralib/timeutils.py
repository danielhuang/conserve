# Copyright 2012 Martin Pool
# Licensed under the Apache License, Version 2.0 (the "License").

"""Time and date utilities"""

import time


def isotime(unixtime):
    return time.strftime('%Y-%m-%dT%H:%M:%S', time.localtime(unixtime))

def reltime(seconds):
    r = '%ds' % seconds
    if seconds > 120:
        r += ' (%d minutes)' % (seconds/60)
    return r
