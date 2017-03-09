import json

class MessageParser():
    def __init__(self):
        self.possible_responses = {
            'error': self.parse_error,
            'info': self.parse_info,
            'message': self.parse_message,
            'history': self.parse_history
        }
	
    def parse(self, payload):
        payload = json.loads(payload)

        if payload['response'] in self.possible_responses:
            return self.possible_responses[payload['response']](payload)
        else:
            # Response not valid
            print('The server has sent us a nonvalid response. Please revise your code.')
    def parse_error(self, payload):
        output=''
        output += payload['response'] + ': '
        output += payload['content']
        print(output)

    def parse_info(self, payload):
    	self.formated_print(payload)

    def parse_message(self, payload):
        self.formated_print(payload)

    def parse_history(self, payload):
       print('Message history: \n')
       if len(payload['content']) == 0:
        return 1
       for msg in payload['content']:
        output = json.loads(msg)
        self.formated_print(output)

    def formated_print(self,payload):
        output =''
        output += payload['timestamp'] + ' -'
        output += payload['sender'] + ' '
        output += payload['response'] + ': '
        output += payload['content']
        print(output)


