# -*- coding: utf-8 -*-
import SocketServer
import json
import time

"""
Variables and functions that must be used by all the ClientHandler objects
must be written here (e.g. a dictionary for connected clients)
"""
history = []
usernames = {}

helptext = 'defined commands: \n -help: list of legal commands \n -login <username> logs in to the server \n -msg <content> sends a message to all online users (requires login) \n -names: lists all online users (requires login) \n logout: terminates the session (requires login)'


class ClientHandler(SocketServer.BaseRequestHandler):
    """
    This is the ClientHandler class. Everytime a new client connects to the
    server, a new ClientHandler object will be created. This class represents
    only connected clients, and not the server itself. If you want to write
    logic for the server, you must write it outside this class
    """
    username = ''

    def handle(self):
        """
        This method handles the connection between a client and the server.
        """
        self.ip = self.client_address[0]
        self.port = self.client_address[1]
        self.connection = self.request
        self.active = True

        # Loop that listens for messages from the client
        while self.active:
            received_string = self.connection.recv(4096)
            request = json.loads(received_string);
            print request;

            #Login request
            if request['request'] == 'login':
               if not self.check_username(request['content']):
                   self.error('invalid username, only letters and numbers allowed')
               elif usernames.has_key(request['content']):
                   self.error('username has already been taken')
               else:
                   self.login(request['content'])
                   
            #Help request       
            elif request['request'] == 'help':
                self.halp()

            #Check if the user has logged in.
            elif len(self.username) != 0:  
                if request['request'] == 'logout':
                    self.logout()
                    
                elif request['request'] == 'names':
                    self.names()

                elif request['request'] == 'msg':
                    if len(request['content']) == 0:
                        self.error('Invalid characters in username')
                    else:
                        self.message(request['content'])
                else:
                    self.error('invalid request, try "help" for a list of supported requests')
            else:
                
                self.error('invalid request, try "help" for a list of supported requests')

    
    #Lagre valgt navn som username, og (andre ting?)
    def login(self,nick):
        self.username = nick
        usernames[nick] = self
        msg = self.format_msg('history','server',history)
        self.connection.sendall(msg)

    #Enkode melding og sende til server sånn at den kan broadcastes til alle og lagres i historie. 
    def message(self,content):
        msg = self.format_msg('message',self.username,content)
        self.broadcast(msg)
        history.append(msg)
        
    #Terminere tilkobling og fjerne brukernavn fra aktive brukere.
    def logout(self):
        msg = self.format_msg('info','server','logging out...')
        self.connection.send(msg)
        del usernames[self.username]
        self.connection.close()
        self.active = False
        
    #Hente liste av brukernavn og sende de til klient
    def names(self):
        userlist = 'List of active users: \n'
        for user in usernames:
            userlist += '-'+user + ' \n'
        msg = self.format_msg('info','server',userlist)
        self.connection.sendall(msg)        

    #Sende forhondsdefinert
    def halp(self):
        msg = self.format_msg('info','server',helptext)
        self.connection.sendall(msg)

    #Sende feilmelding med en beskrivelse (error_msg)
    def error(self,error_msg):
        msg = self.format_msg('error','server',error_msg)
        self.connection.sendall(msg)
        
    def broadcast(self,msg):
        for user in usernames:
            usernames[user].connection.sendall(msg)

    def check_username(self,username):
        if len(username)==0:
            return False
        for i in username:
            if (ord(i) <48):
                return False
            if ((ord(i) > 57)and(ord(i)<65)):
                return False
            if ((ord(i)>90)and(ord(i)<97)):
                return False
            if (ord(i)>122):
                return False
        return True
        
    def format_msg(self, response, sender, content):
        msg = {}
        msg['response'] = response
        msg['timestamp'] = time.asctime( time.localtime(time.time()) )
        msg['sender'] = sender
        msg['content'] = content
        msg = json.dumps(msg)
        return msg
    

class ThreadedTCPServer(SocketServer.ThreadingMixIn, SocketServer.TCPServer):
    """
    This class is present so that each client connected will be ran as a own
    thread. In that way, all clients will be served by the server.

    No alterations are necessary
    """
    allow_reuse_address = True
      

if __name__ == "__main__":
    """
    This is the main method and is executed when you type "python Server.py"
    in your terminal.

    No alterations are necessary
    """
    HOST, PORT = 'localhost', 9999
    print('Server running...')

    # Set up and initiate the TCP server
    server = ThreadedTCPServer((HOST, PORT), ClientHandler)
    server.serve_forever()



