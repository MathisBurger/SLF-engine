import time
import wikipedia
import json


starttime = time.time()

files = ["city", "country", "river"]

for file in files:
    with open(f"./lists/{file}.txt", "r") as f:
        names = f.readlines()
    data = []
    for name in names:
        link = wikipedia.get_wikipedia_link(name)
        print(f"name: {name} link: {link}")
        data.append({"name": str(name).replace("\n", ""), "description": file, "source": link})
    with open(f"./data/init_template.json", "r") as read:
        all_data = json.load(read)
    for el in data:
        try:
            lst = list(all_data[str(el["name"])[0].lower()])
        except:
            print(str(el["name"])[0].lower(), el)
            exit(1)
        lst.append(el)
        all_data[str(el["name"])[0].lower()] = lst
    with open(f"./data/{file}.json", "w+") as write:
        write.write(json.dumps(all_data, indent=4))




endtime = time.time()

diff = endtime - starttime

print(f"Fetching data took {diff}s")