# -*- coding: utf-8 -*-
from threading import Thread
from MessageParser import MessageParser

class MessageReceiver(Thread):
    """
    This is the message receiver class. The class inherits Thread, something that
    is necessary to make the MessageReceiver start a new thread, and it allows
    the chat client to both send and receive messages at the same time
    """

    def __init__(self, client, connection):
        """
        This method is executed when creating a new MessageReceiver object
        """
        self.connection = connection
        super(MessageReceiver,self).__init__()
       
    def run(self):
      
        self.MessageParser = MessageParser()
        while True:
            payload = self.connection.recv(45096)
            self.MessageParser.parse(payload)
            
