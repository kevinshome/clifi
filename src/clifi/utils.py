from os import getenv
from json import dump as json_dump

def config_read(): # turns config data into dict
    config = open('{}/.clifi/clifi.cfg'.format(getenv('HOME')), 'r')
    data = {}
    for line in config:
        line = line.strip()
        if '#' in line or line == '':
            pass # ignore comments
        else:
            line = line.split('=')
            setting = line[0]
            value = line[1]

            data[setting] = value
    return data

def vprint(args, type, output): # verbose printouts
        if args.verbose:
            if type == 's':
                print("SUCCESS: {}".format(output))
            if type == 'a':
                print("ALERT: {}".format(output))
            if type == 'e':
                print("ERR: {}".format(output))
            if type == 'f':
                print("FAIL: {}".format(output))

def write_json(streamfile, data): 
        with open(streamfile,'w') as f: 
            json_dump(data, f, indent=4) 