# -*- coding: utf-8 -*-
import socket
import json
from MessageReceiver import MessageReceiver
"from MessageParser import MessageParser"

class Client:
    """
    This is the chat client class
    """

    def __init__(self, host, server_port):
        """
        This method is run when creating a new Client object
        """
        self.active=True
        # Set up the socket connection to the server
        self.connection = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        #initializing all variables
        self.host=host
        self.server_port=server_port

        self.connection.connect((self.host, self.server_port))
        
        print ("Welcome to the chatting server!\n")
        self.run()

    def run(self):
        # Initiate the connection to the server
        s_thread = MessageReceiver(self, self.connection)
        s_thread.daemon = True
        s_thread.start()
        
        while self.active:
        	self.get_input()


    def disconnect(self):
        self.connection.close()
       
    def receive_message(self, message):
        pass


    def send_payload(self, data):
        self.data = data
        msg = json.dumps(self.data)
        #print("JSON: ",msg)
        self.connection.sendall(msg)

    def get_input(self):
        a = raw_input('Please enter request: ')
        b = raw_input('Please enter content: ')
        data = {'request':a, 'content': b}
        self.send_payload(data)
        if a == 'logout':
            self.active=False



if __name__ == '__main__':
    """
    This is the main method and is executed when you type "python Client.py"
    in your terminal.

    No alterations are necessary
    """
    client = Client('localhost', 9998)
