hwiinet / minimuptime

minimuptime is a minimal uptime tracker written in Rust. I recommend pairing this with
hwiinet/mmu-web, the official web ui, so you can look at your stats more easily.

directions:

create the files log.txt and iplist.txt. log.txt can stay empty, but put whichever ip
addresses you want to monitor in iplist.txt in this format:

192.168.1.1
192.168.1.2
  etc etc...

then you should be able to just run minimuptime (though it might require sudo), and it'll
automatically scan every 300 seconds (five minutes). the web ui is just a ruby script which
requires sinatra. you can place it in the same directory as minimuptime and log.txt, and it
should do its job without issue.

if you have questions or need support, mail me at null at hwii dot net.

TODO: Rewrite README and add dns and http support

