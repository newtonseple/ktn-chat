Requirements
============
There must be 2 executables: server, client

Client
------
On STDIN, sends it as JSON with fields request (word 1), content (word 2 or None)
On network recieve (will be JSON), print the fields on a line (timestamp, sender, response, content)
