import os

cnt0 = 0

for item in os.listdir():
    if os.path.isfile(item):
        if '.' not in item or '.exe' in item:
            os.remove(item)
            cnt0 += 1

cnt1 = 0

for item in os.listdir():
    os.rename(item, "_".join(item.split(" ")))
    cnt1 += 1

print("%d item(s) cleaned" % cnt0)
print("%d item(s) renamed" % cnt1)
