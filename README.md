This is a demo versioned key value database implementing RAMP transactions as outlined in http://www.bailis.org/papers/ramp-sigmod2014.pdf.  In particular, I've implemented the algorithm RAMP-Fast, which I've also documented here: http://rustyrazorblade.com/2015/11/ramp-made-easy/

This DB is not recommended for production.

This DB runs over simple sockets and a simple text protocol.

Database keys are limited to lowercase strings, no spaces, no numbers.  This is just a proof of concept, not a real database.

Syntax examples:

    prepare jon haddad 1 sam,pete
    commit 1
    get jon
    get jon 1

That's it.
