import requests
from bs4 import BeautifulSoup

urls = [
	"https://www.unicode.org/emoji/charts/full-emoji-list.html",
	"https://www.unicode.org/emoji/charts/full-emoji-modifiers.html"
]

emoji = []
for url in urls:
	html = requests.get(url).text
	these_emoji = BeautifulSoup(html, "html.parser").find_all("td", class_="chars")

	for emojis in these_emoji:
		for single_emoji in emojis:
			emoji.append(single_emoji)

import json
with open('all_emoji.json', 'w') as outfile:
	json.dump(emoji, outfile, indent=4)
