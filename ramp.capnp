@0xbcc55cbe2b79513a;

interface RampInterface {
    prepare @0 (key:Text, value:Text, dependencies:List(Text), timestamp:Int64);
    commit @1 (timestamp:Int64);
    get @2 (key:Text) -> (result: GetResult);
    getVersion @3 (key:Text, timestamp:Int64) -> (result: GetResult);
}

# since a Get or GetVersion call can have no result we need a union here
struct GetResult {
    union {
        none @0 : Void;
        version @1 : Version;
    }
}

struct Version {
    value @0: Text;
    timestamp @1: Int64;
    dependencies @2: List(Text);
}
