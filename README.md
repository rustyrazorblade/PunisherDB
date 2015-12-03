This is a demo versioned key value database implementing RAMP transactions as outlined in http://www.bailis.org/papers/ramp-sigmod2014.pdf.  In particular, I've implemented the algorithm RAMP-Fast, which I've also documented here: http://rustyrazorblade.com/2015/11/ramp-made-easy/

This DB is not recommended for production.

This DB runs over simple sockets and a simple text protocol.

Database keys are limited to lowercase strings, no spaces, no numbers.  This is just a proof of concept, not a real database.

The server starts up on port 6000.  It's hard coded.  Don't like it?  I don't care, this isn't a database meant for prod.

Once it's up, you can telnet via:

    telnet localhost 6000

Commands are terminated by a newline and are buffered internally.  

## Commands

### prepare <key> <value> <timestamp> <comma-separated-depencies>

Prepares a timestampped key/value pair with dependencies.

### commit <timestamp>

Commits any values associated with a timestamp.  Will not error if there aren't any values.

### get <key>

Returns the latest committed version of a key

### get <key> <timestamp>

Gets the specific version of a key.   See the blog post for why you'd need this.

Syntax examples:

    > prepare jon haddad 1 sam,pete
    PREPARED
    > get jon
    NOT FOUND
    > commit 1
    COMMITTED
    > get jon
    haddad 1 sam,pete
    > prepare jon bacon 10 a,b,c
    PREPARED
    > commit 10
    COMMITTED
    > get jon 1
    haddad 1 sam,pete
    > get jon 10
    bacon 10 a,b,c


Transaction ids must be unique integers.  If you're trying this out and you hit weird issues, well, that's probably why.  Consider using nanosecond precision timestamps.

The database uses a last write wins for conflict resolution, similar to Cassandra.
