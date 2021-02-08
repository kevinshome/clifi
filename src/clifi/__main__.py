#!/usr/bin/env -S python3 -B

# clifi
# play your favorite streams straight from the command line
#
# Copyright (c) 2020 kevinshome
# This software is released under the terms of the MIT License, 
# which can be found either in the LICENSE file in the root directory of this
# source code, or if unavailable, can also be found at https://opensource.org/licenses/MIT

# initial python version checker
from sys import version_info, platform

_pyminver = '3.5.9' # latest 3.5.x security update

_currentpyver = str('{0}.{1}.{2}'.format(version_info[0], version_info[1], version_info[2]))
if _currentpyver < _pyminver:
    exit('Sorry, the minimum supported python version is 3.5.9 :(\n\
however, we recommend you go ahead and update to the latest version of python')

from subprocess import Popen, STDOUT
from os import devnull, remove, getenv, mkdir
from os.path import exists
from sys import stderr
from argparse import ArgumentParser
from time import sleep
from pathlib import Path
from json import load as json_load
from clifi.utils import config_read, vprint, write_json
from clifi import __version__

import shutil

if 'linux' in platform:
    vlc = '/usr/bin/vlc'
elif platform == 'darwin':
    vlc = '/Applications/VLC.app/Contents/MacOS/VLC'
elif platform == 'win32':
    vlc = "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

if platform == 'win32':
    clifi_dir = '{}\.clifi\\'.format(str(Path.home()))
    lockfile = clifi_dir + 'clifi.lck'
else:
    clifi_dir = '{}/.clifi/'.format(str(Path.home()))
    lockfile = '/tmp/clifi.lck'

def clifi_exit(exit_code):
    if exists(lockfile):
        try:
            vprint(args, 'a', 'attempting to remove \'/tmp/clifi.lck\'')
            remove(lockfile)
            exit(0)
        except OSError:
            exit('\'/tmp/clifi.lck\' not found...')
    exit(exit_code)

if exists(clifi_dir + 'clifi.cfg'):

    cfgdata = config_read()
    streamfile = clifi_dir + cfgdata['streamfile']

else:

    print('configuration file not found, creating it now...')
    sleep(0.5)

        
    try:
        mkdir(clifi_dir)
    except: # if $HOME/.clifi directory already exists, obviously we can't create it again, so just skip mkdir(), and touch the config file
        pass
    
    if platform != 'win32':
        shutil.copy('/etc/default/clifi/clifi.cfg', clifi_dir)
    else:
        shutil.copy(clifi_dir + '\defaults\clifi.cfg', clifi_dir)

    cfgdata = config_read()
    streamfile = clifi_dir + cfgdata['streamfile']

### DEPRECATION NOTICE ###
try:
    cfgdata['suppress_deprecation_notice']
except KeyError:
    print("\
:::DeprecationNotice:::\n\
This version of clifi is deprecated as of June 2020, we recommend you switch to the Rust version located at \
https://github.com/kevinshome/clifi/tree/master. If you insist on using this version, however, please keep \
in mind that it is no longer maintained, and as such, no help will be provided by the development team, and \
no new features will be added\n\n\
To suppress this notice, set 'suppress_deprecation_notice' to any value in your clifi.cfg file\n\
")

        

parser = ArgumentParser(prog='clifi',
                        description='clifi: play your favorite streams straight from the command line',
                        epilog='created with love in 2020 by kevinshome')

parser.add_argument('stream',
                    help='name of stream to be launched',
                    type=str,
                    nargs='?',
                    default=cfgdata['default_stream'])
parser.add_argument('-v', '--verbose',
                    help='be verbose', 
                    action='store_true')
parser.add_argument('-k', '--kill',
                    help='kill an active session',
                    action='store_true')
parser.add_argument('-n', '--new-stream',
                    help='add a new stream to \'streams.json\'',
                    action='store_true')
parser.add_argument('-rm', '--rm-stream',
                    help='remove a stream from \'streams.json\'',
                    action='store_true')
parser.add_argument('-s', '--switch',
                    help='switch from one stream to another',
                    action='store_true')
parser.add_argument('-S', '--streams',
                    help='return list of streams',
                    action='store_true')
parser.add_argument('-U', '--url',
                    type=str,
                    help='stream directly from a URL, rather than an entry in your streamfile')
parser.add_argument('-SS', '--start-stream',
                    type=str,
                    help='start a stream')
parser.add_argument('--version',
                    help='print program version',
                    action='store_true')
parser.add_argument('--devel',
                    help='for development use only, serves no purpose in regular releases',
                    action='store_true')

args = parser.parse_args()

def main(cfgdata, stream_name=args.stream, url=False):
    
    if not url:
        with open(streamfile) as f:
            streams = json_load(f)

        for i in range(len(streams['streams'])):
            stream_name = stream_name
            if streams['streams'][i]['name'] == stream_name:
                stream_url = streams['streams'][i]['url']

    try:
        open(vlc, 'r')
        vprint(args, 's', '{} found!'.format(vlc))
    except:
        vprint(args, 'e', '{} not found...'.format(vlc))
        print('failed to open VLC, please make sure it is installed properly and updated to the latest version')
        clifi_exit(1)

    if url:
        stream_url = args.url
        s = Popen("{} -I dummy -q --no-video {} &".format(vlc, stream_url), shell=True, stdout=open(devnull, 'w'), stderr=STDOUT) # run vlc quietly in the background
        print('Running stream from the URL \'{}\' on PID {}'.format(stream_url, s.pid + 1))
        exit(0)

    if stream_name == cfgdata['default_stream']:
        vprint(args, 'a', "no stream entered...")
        vprint(args, 'a', "starting default stream '{}' from {}".format(stream_name, stream_url))
    else:
        try:
            vprint(args, 'a', "starting stream '{}' from {}".format(stream_name, stream_url))
        except UnboundLocalError:
            clifi_exit(1)

    s = Popen("{} -I dummy -q --no-video {} &".format(vlc, stream_url), shell=True, stdout=open(devnull, 'w'), stderr=STDOUT) # run vlc quietly in the background

    print('Running stream {} on PID {}'.format(stream_name, s.pid + 1))
    exit(0)

    

if __name__ == '__main__':

    if args.devel:
        exit(0)

    if not exists(streamfile):
        
        print('\'{}\' not found, creating it now...'.format(streamfile))
        sleep(0.5)

        try:
            mkdir('{}/.clifi'.format(str(Path.home())))
        except: # if $HOME/.clifi directory already exists, obviously we can't create it again, so just skip mkdir(), and touch the JSON file
            pass
        
        if platform != 'win32':
            shutil.copy('/etc/default/clifi/streams.json', clifi_dir)
        else:
            shutil.copy(clifi_dir + '\defaults\streams.json', clifi_dir)

        '''
        f = open(streamfile, 'w')

        default_json = {
            "_description":"JSON file containing all streams to be used by clifi",
            "streams":[
                {
                    "name":"lofi",
                    "full-name":"lofi hip hop radio - beats to relax/study to",
                    "url":"https://www.youtube.com/watch?v=5qap5aO4i9A"
                }
            ]
        }

        try:
            write_json(streamfile, default_json)
        except:
            clifi_exit('failed to write data to \'streams.json\'')
        
        f.close()
        ''' # deprecated

    if args.start_stream:
        s = Popen("C:\\Program Files\\VideoLAN\\VLC\\vlc.exe -I dummy -q --no-video 'C:\\Users\\noaht\\Desktop\\technical difficulties' --sout '#transcode{vcodec=none,acodec=mp3,ab=128,channels=2,samplerate=44100,scodec=none}:http{mux=mp3,dst=:8080/}' &", shell=True, stdout=open(devnull, 'w'), stderr=STDOUT) # run vlc quietly in the background
        #s = Popen(vlc + " -I dummy -q --no-video " + args.start_stream + " --sout '#transcode{vcodec=none,acodec=mp3,ab=128,channels=2,samplerate=44100,scodec=none}:http{mux=mp3,dst=:8080/}' &", shell=True, stdout=open(devnull, 'w'), stderr=STDOUT) # run vlc quietly in the background
        #s = Popen(vlc + " -I dummy -q --no-video " + args.start_stream + " --sout '#standard{access=" + cfgdata['local_stream_protocol'] + ",mux=" + cfgdata['local_stream_profile'] + ",dst=" + cfgdata['local_stream_host'] + ':' + cfgdata['local_stream_port'] + "}' &", shell=True, stdout=open(devnull, 'w'), stderr=STDOUT) # run vlc quietly in the background
        print('Streaming {} at {}://{}:{} on PID {}'.format(args.start_stream, cfgdata['local_stream_protocol'], cfgdata['local_stream_host'], cfgdata['local_stream_port'], s.pid + 1))
        exit(0)

    if args.rm_stream:
        with open(streamfile) as f:
            data = json_load(f)
        
        for i in range(len(data['streams'])):
            if data['streams'][i]['name'] == args.stream:
                del data['streams'][i]

        
        try:
            write_json(streamfile, data)
        except:
            clifi_exit('failed to write data to \'streams.json\'')

        f.close()
        clifi_exit(0)    
                
    if args.url:
        open(lockfile, 'w').close()
        main(cfgdata, url=True)



    if args.switch:
        args.kill = True

    if args.streams:
        with open(streamfile) as f:
            streams = json_load(f)
        
        for i in range(len(streams['streams'])):
            if streams['streams'][i]['name'] == cfgdata['default_stream']:
                print('*' + streams['streams'][i]['name'] + ': \'' + streams['streams'][i]['full-name'] + '\' ({})'.format(streams['streams'][i]['url']))
            else:
                print(streams['streams'][i]['name'] + ': \'' + streams['streams'][i]['full-name'] + '\' ({})'.format(streams['streams'][i]['url']))
        clifi_exit(0)

    if args.new_stream:
        stream_name = input('Short name for stream (this is the name you\'ll use to launch the stream): ')
        stream_full_name = input('Full name of stream (not required, will only be used when \'--full-name\' argument is called): ')
        stream_url = input('Stream URL: ')

        with open(streamfile) as f: 
            data = json_load(f) 
            inject = {"name":stream_name, "full-name":stream_full_name, "url":stream_url}
            data['streams'].append(inject)
        
        try:
            write_json(streamfile, data)
        except:
            clifi_exit('failed to write data to \'streams.json\'')
        
        print('Successfully added {} to stream list!'.format(stream_name))
        clifi_exit(0)

    if args.version:
        stderr.write('clifi {} - this version is hella wavy\n'.format(__version__))
        stderr.flush()
        clifi_exit(0)

    if args.kill:
        vprint(args, 'a', 'attempting to kill all vlc instances')

        if 'linux' in platform:
            killall = Popen('killall vlc', shell=True, stdout=open(devnull, 'w'), stderr=STDOUT)
        elif platform == 'darwin':
            killall = Popen('killall VLC', shell=True, stdout=open(devnull, 'w'), stderr=STDOUT)
        elif platform == 'win32':
            killall = Popen('taskkill /IM "vlc.exe" /F', shell=True, stdout=open(devnull, 'w'), stderr=STDOUT)

        killall.communicate()[0]
        if killall.returncode != 0:
            clifi_exit('no instances currently running...')
        else:
            vprint(args, 's', 'killed running vlc instances')

        if not args.switch:
            clifi_exit(0)
        
        if args.switch:
            vprint(args, 'a', 'Launching new stream \'{}\''.format(args.stream))
            main(cfgdata)

    if exists(lockfile):
        print('sorry, it looks like there\'s already an instance running...')
        exit('if not, delete \'/tmp/clifi.lck\' and try again')
    else:
        open(lockfile, 'w').close()
        main(cfgdata)
