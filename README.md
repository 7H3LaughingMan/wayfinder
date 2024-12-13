# Wayfinder

[![gitlocalized](https://gitlocalize.com/repo/9859/whole_project/badge.svg)](https://gitlocalize.com/repo/9859?utm_source=badge)
[![ko-fi](https://img.shields.io/badge/Support%20me%20on%20Ko--fi-FF5E5B?logo=ko-fi&logoColor=FFFFFF)](https://ko-fi.com/7h3laughingman)

A wayfinder for your everyday needs! It let's you navigate treacherous paths and plan the shortest route to your destination.

![Wayfinder Example](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/example.webp)

## Troubleshooting

**Is the PF2e Drag Measurement enabled?**

Pathfinding is only available when using the system's drag measurement tool.

![PF2e Drag Measurement Setting](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/pf2e-drag-measurement-setting.png)

---

**Is the pathfinding toggle (purple icon as seen below) enabled?**

Without this toggle on it will not attempt to try and find a path.

![Pathfinding Toggle](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/pathfinding-toggle.png)

---

**Do you have the Fog Exploration Restiction enabled? Do you also have Fog Exploration enabled for the current scene?**

Wayfinder can be setup to restrict pathfinding only through explored parts of the map, in order for this to work you have to enable it and have fog exploration setup for the current scene. If there is no available path through the explored area then it will obviously fail to find a path.

![Wayfinder Fog Exploration Restriction](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/wayfinder-fog-exploration-restriction.png)

![Scene Fog Exploration](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/scene-fog-exploration.png)

---

**Does the token you are moving have vision enabled? Or do you have GM Vision (the purple lightbulb under Lighting Controls) enabled?**

If the token's vision is not enabled then Wayfinder will act like fog exploration is disabled and try to find a path through unexplored parts of the map. This also applies if you have GM Vision enabled.

![Token Vision Enabled](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/token-vision-enabled.png)

![GM Vision](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/gm-vision.png)

---

**Is Wayfinder having issues finding a path through explored parts of the map?**

If you find Wayfinder having issues finding a path through explored parts of the map then I highly advise using the Export Fog Exploration macro to view what is considered "explored". In the Compendium Packs tab you should find a compendium called Wayfinder Macros in the Macros folder where you can locate this macro.

This will provide an image that represents what is considered explored to you on the current scene. I have included an example below, anything that is red is considered explored.

Note: GM's have fog of war as well, if most of the tokens you use don't have vision enabled than it will never update your fog of war. So when you switch to a token that does have vision it will have problems since most of the map isn't considered explored. I would advise either disabling the token's vision or enabling GM's vision.

![Wayfinder Macros](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/wayfinder-macros.png)

![Wayfinder Macros](https://raw.githubusercontent.com/7H3LaughingMan/assets/main/wayfinder/fog-example.webp)

---

**Still having issues?**

Sometimes your browser will cache parts of the Wayfinder module and use that instead of downloading a fresh copy of the module when it's updated. If everything should work based on the above troubleshotting steps then I would advise clearing your browser's cache and try again. I have included some links below if you don't know how to do this.

[Chrome](https://support.google.com/accounts/answer/32050)

[Firefox](https://support.mozilla.org/en-US/kb/how-clear-firefox-cache)

[Edge](https://www.microsoft.com/en-us/edge/learning-center/how-to-manage-and-clear-your-cache-and-cookies)

If all else fails please take a look at the browser's console for any error messages. You can access this by opening the Developer Tools by pressing F12 and switching to the console tab. This will sometimes tell you some more information on if there is a problem with another module or if something else is going on.

If all else fails please submit an issue on [GitHub](https://github.com/7H3LaughingMan/wayfinder/issues) or pop into [Discord](https://discord.com/channels/880968862240239708/1253103891692654613) for assistance. Please provide as much details as possible, screenshots, and even a short clip of what is going on.


