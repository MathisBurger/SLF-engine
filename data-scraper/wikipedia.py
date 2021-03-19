import requests


def get_wikipedia_link(name):
    name_url = requests.get(f"https://de.wikipedia.org/w/api.php?action=opensearch&format=json&formatversion=2"
                            f"&search={name}&namespace=0&limit=10".format(name=name))
    try:
        source = name_url.json()[3][0]
    except:
        source = "Not found"
    return source
