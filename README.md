# snooper

Uses a webdriver to crawl common social media sites and grab usernames and biographies on a username.
Notes on usage:

This will NOT work properly unless you are operating in an environment that supports sandboxing. This requires some special instructions, and will frequently throw errors in the crates that this is dependant on that are confusing without further research (and since they are in a dependancy, I cannot handle them more gracefully without forking). Furthermore, the same dependancy is supposed to download chrome from github on compilation. Occasionally, when testing, this does not work for me, so ensure you have chromium installed beforehand.

I will link the relevant reading here. If you still cannot get it to work, put in an issue and I will attempt to find out why.

Enabling sandboxing:
https://github.com/puppeteer/puppeteer/blob/main/docs/troubleshooting.md


