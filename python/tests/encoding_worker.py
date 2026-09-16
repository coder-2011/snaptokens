"""Run the GIL/lock probe in a process that pytest can time out on deadlock."""

import sys
import threading

from snaptokens import Tokenizer

tokenizer = Tokenizer.from_json_str(sys.argv[1])
ready, begin, changed = (threading.Event() for _ in range(3))


def configure():
    ready.set()
    begin.wait()
    tokenizer.enable_truncation(17)
    changed.set()


thread = threading.Thread(target=configure, daemon=True)
thread.start()
ready.wait()
# Prevent a Python bytecode timeslice from masquerading as a native GIL release.
sys.setswitchinterval(10)
text = 'ab' * 40000
begin.set()
method = sys.argv[2]
if method == 'encode':
    ids = tokenizer.encode(text).ids
elif method == 'encode_batch':
    ids = tokenizer.encode_batch([text])[0].ids
else:
    data, offsets = tokenizer.encode_batch_flat([text])
    ids = list(memoryview(data).cast('I'))
    assert list(memoryview(offsets).cast('Q')) == [0, len(ids)]
assert changed.is_set(), 'Python settings thread did not progress during encode'
assert len(ids) in (17, 40000) and ids == [2] * len(ids)
thread.join()
assert tokenizer.encode(text).ids == [2] * 17
