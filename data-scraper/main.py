from urllib.request import urlopen
from bs4 import BeautifulSoup
import requests
import json
import os

url = input("url:\n")

page = urlopen(url=url)

html_bytes = page.read()

html = html_bytes.decode("utf-8")

soup = BeautifulSoup(html)

content = soup.find_all(class_="post-content")[0]

headings = content.find_all('h3')

names = content.find_all('ul')

for i in range(len(headings)):
    data_array = []
    heading = str(headings[i]).split(" ")[0].replace("<h3>", "")
    nms = names[i].find_all('li')
    for name in nms:
        name = str(name).replace("<li>", "").replace("</li>", "")
        if "(" in str(name):
            name = "".join(str(x) for x in str(name).split(" ")[:1])
        name_url = requests.get(f"https://de.wikipedia.org/w/api.php?action=opensearch&format=json&formatversion=2"
                                f"&search={name}&namespace=0&limit=10".format(name=name))
        try:
            source = name_url.json()[3][0]
        except:
            source = "Not found"
        data_array.append({"name": name, "description": name, "source": source})

    if not os.path.exists(f"./{heading}.json"):
        f = open(f"./{heading}.json", "w+")
        f.write("{}")
        f.close()

    with open(f"./{heading}.json", "r", encoding="utf-8") as file:
        data = json.load(file)

    try:
        values = data[str(str(headings[i]).split(" ")[-1].replace("</h3>", "")).lower()]

    except:
        data[str(str(headings[i]).split(" ")[-1].replace("</h3>", "")).lower()] = data_array

    with open(f"./{heading}.json", "w+", encoding="utf-8") as file_write:
        file_write.write(json.dumps(data, indent=4))

