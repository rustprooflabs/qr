# Experimental:  QR Code extension for Postgres

### Disclaimer

This extension probably isn't a great idea.  This is the result of experimenting
with a few tangential ideas.  Ultimately, this functionality is better
implemented in the application layer.


## How it works


Inputs:

* Base URL: `https://localhost`
* Table: `widget`
* ID: `42`

Output:

SVG format QR Code to direct user to `https://localhost/widget/42`.


